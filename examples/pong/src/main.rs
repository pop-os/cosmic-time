use iced::event::{self, Event as E};
use iced::keyboard::{self, KeyCode};
use iced::subscription;
use iced::widget::{column, container, row, text, Space};
use iced::{executor, Application, Command, Event, Length, Settings, Subscription};
use iced_native::window;

use cosmic_time::{self, keyframes, Duration, Instant, Speed, Timeline};

use once_cell::sync::Lazy;

mod layer;
mod theme;
use layer::Layer;
use theme::{widget::Element, Theme};

static PADDLE_LEFT: Lazy<keyframes::space::Id> = Lazy::new(keyframes::space::Id::unique);
static PADDLE_RIGHT: Lazy<keyframes::space::Id> = Lazy::new(keyframes::space::Id::unique);
static BALL_X: Lazy<keyframes::space::Id> = Lazy::new(keyframes::space::Id::unique);
static BALL_Y: Lazy<keyframes::space::Id> = Lazy::new(keyframes::space::Id::unique);

pub fn main() -> iced::Result {
    Pong::run(Settings::default())
}

struct Pong {
    timeline: Timeline,
    window: Window,
    paddle_left: Direction,
    paddle_right: Direction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
}

struct Window {
    width: u32,
    height: u32,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Tick(Instant),
    Paddle(Paddle),
    WindowResized(u32, u32),
}

#[derive(Debug, Clone, Copy)]
enum Paddle {
    LeftUp,
    LeftDown,
    RightUp,
    RightDown,
}

impl Application for Pong {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Pong {
                timeline: Timeline::new(),
                window: Window {
                    width: 0,
                    height: 0,
                },
                paddle_left: Direction::Up,
                paddle_right: Direction::Up,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Pong - Cosmic-Time")
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::batch(vec![
            self.timeline.as_subscription::<Event>().map(Message::Tick),
            subscription::events_with(|event, status| match (event, status) {
                (
                    Event::Keyboard(keyboard::Event::KeyPressed {
                        key_code,
                        modifiers: _,
                    }),
                    event::Status::Ignored,
                ) => match key_code {
                    KeyCode::W => Some(Message::Paddle(Paddle::LeftUp)),
                    KeyCode::S => Some(Message::Paddle(Paddle::LeftDown)),
                    KeyCode::Up => Some(Message::Paddle(Paddle::RightUp)),
                    KeyCode::Down => Some(Message::Paddle(Paddle::RightDown)),
                    _ => None,
                },
                (
                    Event::Window(window::Event::Resized { width, height }),
                    event::Status::Ignored,
                ) => Some(Message::WindowResized(width, height)),
                _ => None,
            }),
        ])
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Tick(now) => self.timeline.now(now),
            Message::Paddle(p) => {
                let animation = match p {
                    Paddle::LeftUp => self.anim_left(Direction::Up),
                    Paddle::RightUp => self.anim_right(Direction::Up),
                    Paddle::LeftDown => self.anim_left(Direction::Down),
                    Paddle::RightDown => self.anim_right(Direction::Down),
                };
                self.timeline.set_chain(animation).start()
            }
            Message::WindowResized(width, height) => self.window = Window { width, height },
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let width = self.window.width as f32 * 0.03;
        let height = self.window.height as f32 * 0.2;

        let paddle_left = container(Space::new(Length::Fixed(width), Length::Fixed(height)))
            .style(theme::Container::Paddle);
        let paddle_right = container(Space::new(Length::Fixed(width), Length::Fixed(height)))
            .style(theme::Container::Paddle);

        let content = row![
            column![
                keyframes::Space::as_widget(PADDLE_LEFT.clone(), &self.timeline),
                paddle_left
            ],
            Space::new(Length::Fill, Length::Fill),
            column![
                keyframes::Space::as_widget(PADDLE_RIGHT.clone(), &self.timeline),
                paddle_right
            ],
        ];

        let ball = container(Space::new(Length::Fixed(width), Length::Fixed(width)))
            .style(theme::Container::Ball);

        Layer::new(content, ball).into()
    }
}

impl Pong {
    fn anim_left(&mut self, direction: Direction) -> cosmic_time::space::Chain {
        match direction {
            Direction::Down => cosmic_time::space::Chain::new(PADDLE_LEFT.clone())
                // OOh here are the lazy keyframes!
                // This means that this animation will start at wherever the previous
                // animation left off!
                // Lazy still takes a duration, this will usually be `Duration::ZERO`
                // like regular animations, but you can put them anywhere in your
                // animation chain. Meaning that you would have an animation start
                // at the previous animations's interupted location, animate to elsewhere,
                // then go back to that spot!
                .link(keyframes::Space::lazy(Duration::ZERO))
                .link(
                    keyframes::Space::new(Speed::per_secs(100.))
                        .height(Length::Fixed(self.window.height as f32 - 100.)),
                ),
            Direction::Up => cosmic_time::space::Chain::new(PADDLE_LEFT.clone())
                .link(keyframes::Space::lazy(Duration::ZERO))
                .link(keyframes::Space::new(Speed::per_secs(100.)).height(Length::Fixed(0.))),
        }
    }

    fn anim_right(&mut self, direction: Direction) -> cosmic_time::space::Chain {
        match direction {
            Direction::Down => cosmic_time::space::Chain::new(PADDLE_RIGHT.clone())
                .link(keyframes::Space::lazy(Duration::ZERO))
                .link(
                    keyframes::Space::new(Speed::per_secs(100.))
                        .height(Length::Fixed(self.window.height as f32 - 100.)),
                ),
            Direction::Up => cosmic_time::space::Chain::new(PADDLE_RIGHT.clone())
                .link(keyframes::Space::lazy(Duration::ZERO))
                .link(keyframes::Space::new(Speed::per_secs(100.)).height(Length::Fixed(0.))),
        }
    }
}
