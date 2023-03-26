use iced::event;
use iced::keyboard::{self, KeyCode};
use iced::subscription;
use iced::widget::{column, container, row, Space};
use iced::{executor, Application, Command, Event, Length, Settings, Subscription};
use iced_native::window;

use cosmic_time::{self, keyframes, Duration, Instant, Speed, Timeline};

use once_cell::sync::Lazy;
use rand::prelude::*;

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
    in_play: bool,
    rng: ThreadRng,
    left: Direction,
    right: Direction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
}

#[derive(Debug, Clone, Copy, Default)]
struct Window {
    width: f32,
    height: f32,
    paddle_height: f32,
    paddle_width: f32,
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
                window: Window::default(),
                rng: thread_rng(),
                in_play: false,
                left: Direction::Up,
                right: Direction::Up,
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

                // Start game on first player movement
                if !self.in_play {
                    self.in_play = true;
                    let vertical_bounce = self.rand_vertical_bounce();
                    let horizontal_bounce = self.rand_horizontal_bounce();
                    let _ = self
                        .timeline
                        .set_chain(vertical_bounce)
                        .set_chain(horizontal_bounce);
                }
                if let Some(a) = animation {
                    self.timeline.set_chain(a)
                } else {
                    &mut self.timeline
                }
                .start();
            }
            Message::WindowResized(width, height) => {
                let width = width as f32;
                let height = height as f32;
                self.window = Window {
                    width,
                    height,
                    paddle_width: width * 0.03,
                    paddle_height: height * 0.2,
                };

                self.in_play = false;
                let x = self.init_ball_x();
                let y = self.init_ball_y();
                self.timeline.set_chain(x).set_chain(y).start();
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let width = self.window.paddle_width;
        let height = self.window.paddle_height;

        let paddle_left = container(Space::new(width, height)).style(theme::Container::Paddle);
        let paddle_right = container(Space::new(width, height)).style(theme::Container::Paddle);

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

        let ball = column![
            keyframes::Space::as_widget(BALL_Y.clone(), &self.timeline),
            row![
                keyframes::Space::as_widget(BALL_X.clone(), &self.timeline),
                container(Space::new(width, width)).style(theme::Container::Ball)
            ]
        ];

        Layer::new(content, ball).into()
    }
}

impl Pong {
    fn anim_left(&mut self, direction: Direction) -> Option<cosmic_time::space::Chain> {
        if self.left != direction {
            self.left = direction;
            Some(match direction {
                Direction::Down => cosmic_time::space::Chain::new(PADDLE_LEFT.clone())
                    // OOh here are the lazy keyframes!
                    // This means that this animation will start at wherever the previous
                    // animation left off!
                    // Lazy still takes a duration, this will usually be `Duration::ZERO`
                    // like regular animations, but you can put them anywhere in your
                    // animation chain. Meaning that you would have an animation start
                    // at the previous animations's interupted location, animate to elsewhere,
                    // then go back to that spot!
                    //
                    // Also notice the speed here is per_millis! This is important.
                    // The animation is only as granular as your definition in the chain.
                    // If you animation time is not in exact seconds, I highly recommend
                    // using a smaller unit.
                    .link(keyframes::Space::lazy(Duration::ZERO))
                    .link(
                        keyframes::Space::new(Speed::per_millis(0.3))
                            .height(self.window.height - 100.),
                    ),
                Direction::Up => cosmic_time::space::Chain::new(PADDLE_LEFT.clone())
                    .link(keyframes::Space::lazy(Duration::ZERO))
                    .link(keyframes::Space::new(Speed::per_millis(0.3)).height(0.)),
            })
        } else {
            None
        }
    }

    fn anim_right(&mut self, direction: Direction) -> Option<cosmic_time::space::Chain> {
        if self.right != direction {
            self.right = direction;
            Some(match direction {
                Direction::Down => cosmic_time::space::Chain::new(PADDLE_RIGHT.clone())
                    .link(keyframes::Space::lazy(Duration::ZERO))
                    .link(
                        keyframes::Space::new(Speed::per_millis(0.3))
                            .height(self.window.height - 100.),
                    ),
                Direction::Up => cosmic_time::space::Chain::new(PADDLE_RIGHT.clone())
                    .link(keyframes::Space::lazy(Duration::ZERO))
                    .link(keyframes::Space::new(Speed::per_millis(0.3)).height(0.)),
            })
        } else {
            None
        }
    }

    fn init_ball_y(&mut self) -> cosmic_time::space::Chain {
        let min = self.window.height * 0.3;
        let max = self.window.height - min - self.window.paddle_height;
        cosmic_time::space::Chain::new(BALL_Y.clone())
            .link(keyframes::Space::new(Duration::ZERO).height(self.rng.gen_range(min..max)))
    }

    fn init_ball_x(&mut self) -> cosmic_time::space::Chain {
        let min = self.window.width * 0.3;
        let max = self.window.width - min - self.window.paddle_width;
        cosmic_time::space::Chain::new(BALL_X.clone())
            .link(keyframes::Space::new(Duration::ZERO).width(self.rng.gen_range(min..max)))
    }

    fn rand_vertical_bounce(&mut self) -> cosmic_time::space::Chain {
        let speed = 100. * self.rng.gen_range(0.9..1.1);
        if self.rng.gen() {
            cosmic_time::space::Chain::new(BALL_Y.clone())
                .link(keyframes::Space::lazy(Duration::ZERO))
                .link(
                    keyframes::Space::new(Speed::per_secs(speed))
                        .height(self.window.height - self.window.paddle_width),
                )
                .link(keyframes::Space::new(Speed::per_secs(speed)).height(0.))
                .link(keyframes::Space::lazy(Speed::per_secs(speed)))
                .loop_forever()
        } else {
            cosmic_time::space::Chain::new(BALL_Y.clone())
                .link(keyframes::Space::lazy(Duration::ZERO))
                .link(keyframes::Space::new(Speed::per_secs(speed)).height(0.))
                .link(
                    keyframes::Space::new(Speed::per_secs(speed))
                        .height(self.window.height - self.window.paddle_width),
                )
                .link(keyframes::Space::lazy(Speed::per_secs(speed)))
                .loop_forever()
        }
    }

    fn rand_horizontal_bounce(&mut self) -> cosmic_time::space::Chain {
        let speed = 100. * self.rng.gen_range(0.9..1.1);
        if self.rng.gen() {
            cosmic_time::space::Chain::new(BALL_X.clone())
                .link(keyframes::Space::lazy(Duration::ZERO))
                .link(
                    keyframes::Space::new(Speed::per_secs(speed))
                        .width(self.window.width - self.window.paddle_width),
                )
                .link(keyframes::Space::new(Speed::per_secs(speed)).width(0.))
                .link(keyframes::Space::lazy(Speed::per_secs(speed)))
                .loop_forever()
        } else {
            cosmic_time::space::Chain::new(BALL_X.clone())
                .link(keyframes::Space::lazy(Duration::ZERO))
                .link(keyframes::Space::new(Speed::per_secs(speed)).width(0.))
                .link(
                    keyframes::Space::new(Speed::per_secs(speed))
                        .width(self.window.width - self.window.paddle_width),
                )
                .link(keyframes::Space::lazy(Speed::per_secs(speed)))
                .loop_forever()
        }
    }
}
