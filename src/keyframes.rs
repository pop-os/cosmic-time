mod button;
#[cfg(feature = "libcosmic")]
mod cards;
mod column;
mod container;
mod helpers;
mod row;
mod space;
mod style_button;
mod style_container;
mod toggler;

use crate::reexports::iced_core::{widget, Length};

pub use button::Button;
#[cfg(feature = "libcosmic")]
pub use cards::Cards;
pub use column::Column;
pub use container::Container;
#[cfg(feature = "libcosmic")]
pub use helpers::cards;
pub use helpers::id;
pub use helpers::lazy;
pub use helpers::{
    button, chain, column, container, row, space, style_button, style_container, toggler,
};
pub use row::Row;
pub use space::Space;
pub use style_button::StyleButton;
pub use style_container::StyleContainer;
pub use toggler::Toggler;

use crate::Timeline;

/// The macro used to cleanly and efficently build an animation chain.
/// Works for ann Id's that implement `into_chain` and `into_chain_with_children`
#[macro_export]
macro_rules! chain{
  ($id:expr) => {
    $id.clone().into_chain()
  };
  ($id:expr, $($x:expr),+ $(,)?) => {
    $id.clone().into_chain_with_children(vec![$($x),+])
  };
}

/// The macro used to clean up animation's view code.
#[macro_export]
macro_rules! anim{
  ($id:expr, $($x:expr),+ $(,)?) => {
    $id.clone().as_widget($($x),+)
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
        .map_or(default, |m| Length::Fixed(m.value))
}

fn as_f32(length: Option<Length>) -> Option<f32> {
    match length {
        Some(Length::Fixed(i)) => Some(i),
        _ => None,
    }
}
