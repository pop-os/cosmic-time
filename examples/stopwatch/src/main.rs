use iced::alignment;
use iced::executor;
use iced::theme::{self, Theme};
use iced::time;
use iced::widget::{button, column, container, row, text};
use iced::{Alignment, Application, Command, Element, Length, Settings, Subscription, Event};

use cosmic_time::{self, Timeline, style_button::{self, StyleButton}};
use once_cell::sync::Lazy;

static BUTTON: Lazy<style_button::Id> = Lazy::new(style_button::Id::unique);

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
        (
            Stopwatch {
                timeline: Timeline::new(),
                duration: Duration::default(),
                state: State::Idle,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Stopwatch - Iced")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Toggle => match self.state {
                State::Idle => {
                    self.state = State::Ticking {
                        last_tick: Instant::now(),
                    };
                    self.timeline
                        .set_chain(anim_to_destructive().into())
                        .start();
                }
                State::Ticking { .. } => {
                    self.state = State::Idle;
                    self.timeline.set_chain(anim_to_primary().into()).start();
                }
            },
            Message::Tick(now) => {
                if let State::Ticking { last_tick } = &mut self.state {
                    self.duration += now - *last_tick;
                    *last_tick = now;
                }
            }
            Message::Reset => {
                self.duration = Duration::default();
            }
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::batch(vec![
            match self.state {
                State::Idle => Subscription::none(),
                State::Ticking { .. } => time::every(Duration::from_millis(10)).map(Message::Tick),
            },
            self.timeline.as_subscription::<Event>().map(Message::Tick),
        ])
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

            StyleButton::as_widget(
                BUTTON.clone(),
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

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

fn anim_to_primary() -> style_button::Chain {
    style_button::Chain::new(BUTTON.clone())
        .link(StyleButton::new(Duration::ZERO).style(as_u8(theme::Button::Destructive)))
        .link(
            StyleButton::new(Duration::from_millis(500)).style(as_u8(theme::Button::Primary)),
        )
}

fn anim_to_destructive() -> style_button::Chain {
    style_button::Chain::new(BUTTON.clone())
        .link(StyleButton::new(Duration::ZERO).style(as_u8(theme::Button::Primary)))
        .link(
            StyleButton::new(Duration::from_millis(500))
                .style(as_u8(theme::Button::Destructive)),
        )
}

// Style implementations

// the enum's default must be 0
fn as_u8(style: theme::Button) -> u8 {
    match style {
        theme::Button::Primary => 0,
        theme::Button::Secondary => 1,
        theme::Button::Positive => 2,
        theme::Button::Destructive => 3,
        theme::Button::Text => 4,
        _ => panic!("Custom is not supported"),
    }
}
