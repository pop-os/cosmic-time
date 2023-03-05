use iced::alignment;
use iced::executor;
use iced::theme::{self, Theme};
use iced::time;
use iced::widget::{button, column, container, row, text};
use iced::{
    Alignment, Application, Command, Element, Length, Settings, Subscription,
};

use once_cell::sync::Lazy;
use cosmic_time::{self, keyframes, Timeline};

static BUTTON: Lazy<keyframes::button::Id> = Lazy::new(keyframes::button::Id::unique);

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
                    self.timeline.set_chain(anim_to_destructive().into()).start();
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
                if let State::Ticking { .. } = self.state {
                    self.timeline.set_chain(anim_to_primary().into()).start();
                }
            }
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
      Subscription::batch( vec![
        match self.state {
            State::Idle => Subscription::none(),
            State::Ticking { .. } => {
                time::every(Duration::from_millis(10)).map(Message::Tick)
            }
        },
        self.timeline.as_subscription().map(Message::Tick)
      ]
      )
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
            button(
                text(label).horizontal_alignment(alignment::Horizontal::Center),
            )
            .padding(10)
            .width(Length::Units(80))
        };

        let toggle_button = {
            let label = match self.state {
                State::Idle => "Start",
                State::Ticking { .. } => "Stop",
            };

            keyframes::Button::as_widget(
              BUTTON.clone(),
              &self.timeline,
              label
            )
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

fn anim_to_primary() -> cosmic_time::button::Chain {
    cosmic_time::button::Chain::new(BUTTON.clone())
      .link(
        keyframes::Button::new(Duration::ZERO)
          .style(theme::Button::Destructive)
      )
      .link(
        keyframes::Button::new(Duration::from_millis(500))
          .style(theme::Button::Primary)
        )
}

fn anim_to_destructive() -> cosmic_time::button::Chain {
    cosmic_time::button::Chain::new(BUTTON.clone())
      .link(
        keyframes::Button::new(Duration::ZERO)
          .style(theme::Button::Primary)
      )
      .link(
        keyframes::Button::new(Duration::from_millis(500))
          .style(theme::Button::Destructive)
        )
}
