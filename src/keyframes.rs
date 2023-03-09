pub mod button;
pub mod container;
pub mod space;
pub mod style_button;

use iced_native::{widget, Length};

pub use button::Button;
pub use container::Container;
pub use space::Space;
pub use style_button::StyleButton;

use std::time::Instant;

use crate::Timeline;

#[derive(Debug, Copy, Clone, PartialEq)]
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

pub fn get_length(
    id: &widget::Id,
    timeline: &Timeline,
    now: &Instant,
    index: usize,
    default: Length,
) -> Length {
    timeline
        .get(id, now, index)
        .map(|m| Length::Fixed(m.value))
        .unwrap_or(default)
}
