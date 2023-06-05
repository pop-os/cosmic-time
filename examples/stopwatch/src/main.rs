use iced::alignment;
use iced::executor;
use iced::widget::{button, column, row, text};
use iced::{Alignment, Application, Command, Event, Length, Settings, Subscription};

mod theme;
use self::widget::Element;
use theme::Theme;

use cosmic_time::{self, anim, chain, id, once_cell::sync::Lazy, Sinusoidal, Timeline};

static BUTTON: Lazy<id::StyleButton> = Lazy::new(id::StyleButton::unique);
static CONTAINER: Lazy<id::StyleContainer> = Lazy::new(id::StyleContainer::unique);

use std::time::{Duration, Instant};

pub fn main() -> iced::Result {
    Stopwatch::run(Settings::default())
}

struct Stopwatch {
    timeline: Timeline,
    duration: Duration,
    state: State,
}

enum State {
    Idle,
    Ticking { last_tick: Instant },
}

#[derive(Debug, Clone)]
enum Message {
    Toggle,
    Reset,
    Tick(Instant),
}

impl Application for Stopwatch {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Stopwatch, Command<Message>) {
        let mut timeline = Timeline::new();
        timeline.set_chain_paused(anim_background()).start();
        (
            Stopwatch {
                timeline,
                duration: Duration::default(),
                state: State::Idle,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Stopwatch - Cosmic-Time")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Toggle => match self.state {
                State::Idle => {
                    self.state = State::Ticking {
                        last_tick: Instant::now(),
                    };
                    self.timeline
                        .set_chain(anim_to_destructive())
                        .resume(CONTAINER.clone())
                        .start();
                }
                State::Ticking { .. } => {
                    self.state = State::Idle;
                    self.timeline
                        .set_chain(anim_to_primary())
                        .pause(CONTAINER.clone())
                        .start();
                }
            },
            Message::Tick(now) => {
                self.timeline.now(now);
                if let State::Ticking { last_tick } = &mut self.state {
                    self.duration += now - *last_tick;
                    *last_tick = now;
                }
            }
            Message::Reset => {
                self.duration = Duration::default();
                match self.state {
                    State::Idle => self.timeline.set_chain_paused(anim_background()).start(),
                    State::Ticking { .. } => self.timeline.set_chain(anim_background()).start(),
                }
            }
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        self.timeline.as_subscription::<Event>().map(Message::Tick)
    }

    fn view(&self) -> Element<Message> {
        const MINUTE: u64 = 60;
        const HOUR: u64 = 60 * MINUTE;

        let seconds = self.duration.as_secs();

        let duration = text(format!(
            "{:0>2}:{:0>2}:{:0>2}.{:0>2}",
            seconds / HOUR,
            (seconds % HOUR) / MINUTE,
            seconds % MINUTE,
            self.duration.subsec_millis() / 10,
        ))
        .size(40);

        let button = |label| {
            button(text(label).horizontal_alignment(alignment::Horizontal::Center))
                .padding(10)
                .width(Length::Fixed(80.))
        };

        // must match the same order that the function used to parse into `u8`s
        let buttons = |i: u8| match i {
            0 => theme::Button::Primary,
            1 => theme::Button::Secondary,
            2 => theme::Button::Positive,
            3 => theme::Button::Destructive,
            4 => theme::Button::Text,
            _ => panic!("custom is not supported"),
        };

        let toggle_button = {
            let label = match self.state {
                State::Idle => "Start",
                State::Ticking { .. } => "Stop",
            };

            anim!(
                BUTTON,
                buttons,
                &self.timeline,
                text(label).horizontal_alignment(alignment::Horizontal::Center),
            )
            .padding(10)
            .width(Length::Fixed(80.))
            .on_press(Message::Toggle)
        };

        let reset_button = button("Reset")
            .style(theme::Button::Secondary)
            .on_press(Message::Reset);

        let controls = row![toggle_button, reset_button].spacing(20);

        let content = column![duration, controls]
            .align_items(Alignment::Center)
            .spacing(20);

        anim!(
            CONTAINER,
            // Cool! Because we implemented the function on our custom, theme's type, adding
            // the map argument is easy!
            theme::Container::map(),
            &self.timeline,
            content,
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
    }
}

fn anim_to_primary() -> cosmic_time::Chain {
    use cosmic_time::style_button;
    chain![
        BUTTON,
        style_button(Duration::ZERO).style(button_u8(theme::Button::Destructive)),
        style_button(Duration::from_millis(500)).style(button_u8(theme::Button::Primary))
    ]
    .into()
}

fn anim_to_destructive() -> cosmic_time::Chain {
    use cosmic_time::style_button;
    chain![
        BUTTON,
        style_button(Duration::ZERO).style(button_u8(theme::Button::Primary)),
        style_button(Duration::from_millis(500)).style(button_u8(theme::Button::Destructive))
    ]
    .into()
}

fn anim_background() -> cosmic_time::Chain {
    use cosmic_time::style_container;
    chain![
        CONTAINER,
        style_container(Duration::ZERO).style(theme::Container::Red),
        style_container(Duration::from_secs(1))
            // Notice how we can just pass the enum value here, where in the `anim_to_primary/destructive`
            // we have to use the fucntion `button_u8`? Because we use a implemented a custom iced theme,
            // we can just impl Into<u8> on the enum, and it works here!
            .style(theme::Container::Green)
            .ease(Sinusoidal::In),
        style_container(Duration::from_secs(2))
            .style(theme::Container::Blue)
            .ease(Sinusoidal::In),
        style_container(Duration::from_secs(3))
            .style(theme::Container::Red)
            .ease(Sinusoidal::In)
    ]
    .loop_forever()
    .into()
}

// Style implementations

// Here the button example uses Iced's default theme
// enum. So we have to have some helper functions to make it work.
// we also have another closture, `buttons`, in `fn view()`
//
// For themining reasons, this actually isn't iced's default
// button theme, but the implementation here for button is what you
// would have to do to use the iced type in your project.

// the enum's default must be 0
fn button_u8(style: theme::Button) -> u8 {
    match style {
        theme::Button::Primary => 0,
        theme::Button::Secondary => 1,
        theme::Button::Positive => 2,
        theme::Button::Destructive => 3,
        theme::Button::Text => 4,
        _ => panic!("Custom is not supported"),
    }
}

// But! if we are useing a custom theme then
// the code cleans up quite a bit.

impl From<theme::Container> for u8 {
    fn from(style: theme::Container) -> Self {
        match style {
            theme::Container::White => 0,
            theme::Container::Red => 1,
            theme::Container::Green => 2,
            theme::Container::Blue => 3,
        }
    }
}

impl theme::Container {
    fn map() -> fn(u8) -> theme::Container {
        |i: u8| match i {
            0 => theme::Container::White,
            1 => theme::Container::Red,
            2 => theme::Container::Green,
            3 => theme::Container::Blue,
            _ => panic!("Impossible"),
        }
    }
}

// Just for themeing, not a part of this example.
mod widget {
    #![allow(dead_code)]
    use crate::theme::Theme;

    pub type Renderer = iced::Renderer<Theme>;
    pub type Element<'a, Message> = iced::Element<'a, Message, Renderer>;
    pub type Container<'a, Message> = iced::widget::Container<'a, Message, Renderer>;
    pub type Button<'a, Message> = iced::widget::Button<'a, Message, Renderer>;
}
