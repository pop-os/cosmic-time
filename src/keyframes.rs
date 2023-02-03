pub mod container;

use iced::Length;
use iced_native::widget;

pub use container::{Container, Chain};

use std::time::Instant;

use crate::Timeline;

#[derive(Debug, Copy, Clone)]
pub enum Repeat {
  Never,
  Forever,
} 

impl std::default::Default for Repeat {
  fn default() -> Self {
    Repeat::Never
  }
}

pub trait IsChain {
    fn repeat(&self) -> Repeat;
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
