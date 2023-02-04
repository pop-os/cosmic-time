use iced::widget::{button, column, text};
use iced::{
    executor, Alignment, Application, Command, Element, Length, Settings, Subscription, Theme,
};
use std::time::{Duration, Instant};

use cosmic_time::{self, keyframes, Timeline};

use once_cell::sync::Lazy;

static CONTAINER: Lazy<keyframes::container::Id> = Lazy::new(keyframes::container::Id::unique);

pub fn main() -> iced::Result {
    Counter::run(Settings::default())
}

struct Counter {
    value: i32,
    timeline: Timeline,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    IncrementPressed,
    DecrementPressed,
    Tick,
}

impl Application for Counter {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        // This is new! This is how we build a timeline!
        // These values can be created at anytime, but because this example is
        // simple and we want to animate from application init, we will build the
        // timeline Struct and the "timeline" itself here.
        // Though more complicated applications will likely do this in the `update`
        let mut timeline = Timeline::new();
        let animation = cosmic_time::container::Chain::new(CONTAINER.clone())
            // .loop_forever() // Uncomment this line to loop the animation!
            .link(
                keyframes::Container::new(Duration::ZERO)
                    .width(Length::Units(0))
                    .height(Length::Units(100)),
            )
            .link(
                keyframes::Container::new(Duration::from_secs(2))
                    .width(Length::Units(200))
                    .height(Length::Units(100)),
            )
            .link(
                keyframes::Container::new(Duration::from_secs(4))
                    .width(Length::Units(200))
                    .height(Length::Units(300))
                    .padding([0, 0, 0, 0]),
            )
            .link(
                keyframes::Container::new(Duration::from_secs(6))
                    .width(Length::Units(700))
                    .height(Length::Units(300))
                    .padding([0, 0, 0, 500]),
            )
            .link(
                keyframes::Container::new(Duration::from_secs(8))
                    .width(Length::Units(150))
                    .height(Length::Units(150))
                    .padding([0, 0, 0, 0]),
            );

        // Notice how we had to specify the start and end of the widget dimensions?
        // Iced's default values for widgets are usually not animatable, because
        // they are unknown until the layout is built after the update.
        // because the goal of cosmic-time is to output regular widgets in the view,
        // we do the same here. Thus we must specify the start and end values of the
        // animation. To animate from a width of 50 to 100, we need two keyframes.
        // This example has multiple animated values, but if you look each one specifies
        // the value at each keyframe.
        // Notice how we specify the height of 300 again in `four`? That is because
        // cosmic-time assumes that the timeline is continuous. Try deleting it,
        // the height will animate smoothly from 300 to 150 right through keyframe `four`!

        timeline.set_chain(animation.into()).start();
        // `Start` is very important! Your animation won't "start" without it.
        // Cosmic-time tries to be atomic, meaning that keyframes defined in the
        // same function call all start at the same time. Because there is process time
        // between creating each keyframe it would be possible that two keyframes defined
        // with the same delay may lag behind eachother! Most of the time this would be
        // a single digit number of microseconds, but it might not!
        // So just be aware, when adding keyframes with a `Duration`, that keyframe's
        // time length is "`Duration` from the next `start` function call."

        (Self { value: 0, timeline }, Command::none())
    }

    fn title(&self) -> String {
        String::from("Counter - Iced")
    }

    fn subscription(&self) -> Subscription<Message> {
        // This is the magic that lets us animaate. Cosmic-time looks
        // at what timeline you have built and decides for you how often your
        // application should redraw for you! When the animation is done idle
        // or finished, cosmic-time will keep your applicaiton idle!
        self.timeline.as_subscription().map(|_| Message::Tick)
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed => {
                self.value -= 1;
            }
            Message::Tick => {}
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        // Now we build a contaienr widget from the timeline above.
        // Cosmic-time just generates standard iced widgets. Style them like you would
        // any other! If you have a widget with a constant width, and animated height,
        // just define the width with a `width` method like any other widget, then
        // animate the height in your view! Only control the animatable values with
        // cosmic-time, all others should be in your view!
        keyframes::Container::as_widget(
            CONTAINER.clone(),
            &self.timeline,
            column![
                button("Increment")
                    .on_press(Message::IncrementPressed)
                    .width(Length::Fill),
                text(self.value).size(50).height(Length::Fill),
                button("Decrement")
                    .on_press(Message::DecrementPressed)
                    .width(Length::Fill)
            ]
            .padding(20)
            .align_items(Alignment::Center),
        )
        .into()
    }
}
