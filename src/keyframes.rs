pub mod button;
pub mod container;
pub mod space;
pub mod style_button;
pub mod style_container;
pub mod toggler;

use iced_native::{widget, Length};

pub use button::Button;
pub use container::Container;
pub use space::Space;
pub use style_button::StyleButton;
pub use style_container::StyleContainer;
pub use toggler::Toggler;

use std::time::Instant;

use crate::Timeline;

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub enum Repeat {
    #[default]
    Never,
    Forever,
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

fn as_f32(length: Option<Length>) -> Option<f32> {
    match length {
        Some(Length::Fixed(i)) => Some(i),
        _ => None,
    }
}
