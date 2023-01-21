use iced::Length;
use iced_native::widget;

pub mod container;

pub use container::Container;

use std::time::{Duration, Instant};

use crate::Timeline;

pub trait IsKeyframe: ExactSizeIterator<Item = Option<isize>> {
    fn id(&self) -> widget::Id;

    fn at(&self) -> Duration;
}

pub fn clamp_u16(num: Option<isize>) -> Option<u16> {
    num.and_then(|n| Some(n.clamp(0, u16::MAX as isize) as u16))
}

pub fn get_length(
    id: &widget::Id,
    timeline: &Timeline,
    now: &Instant,
    index: usize,
    default: Length,
) -> Length {
    let out = clamp_u16(timeline.get(&id, &now, index))
        .and_then(|num| Some(Length::Units(num)))
        .unwrap_or(default);
    out
}
