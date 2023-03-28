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

use crate::Timeline;

#[macro_export]
macro_rules! chain{
  ($id:expr) => {
    $id.clone().to_chain()
  };
  ($id:expr, $($x:expr),+ $(,)?) => {
    $id.clone().to_chain_with_children(vec![$($x),+])
  };
}

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub enum Repeat {
    #[default]
    Never,
    Forever,
}

pub trait IsChain {
    fn repeat(&self) -> Repeat;
}

pub fn get_length(id: &widget::Id, timeline: &Timeline, index: usize, default: Length) -> Length {
    timeline
        .get(id, index)
        .map(|m| Length::Fixed(m.value))
        .unwrap_or(default)
}

fn as_f32(length: Option<Length>) -> Option<f32> {
    match length {
        Some(Length::Fixed(i)) => Some(i),
        _ => None,
    }
}
