#![allow(clippy::too_many_arguments)]

use crate::{reexports::iced_core, utils::static_array_from_iter};

pub mod cards;
pub mod cosmic_toggler;

pub use cards::Cards;
pub use cosmic_toggler::Toggler;

/// A convenience type to optimize style-able widgets,
/// to only do the "expensize" style calculations if needed.
#[derive(Debug)]
pub enum StyleType<T> {
    /// The style is either default, or set manually in the `view`.
    Static(T),
    /// The stlye is being animated. Blend between the two values.
    Blend(T, T, f32),
}

use self::iced_core::{
    gradient::{ColorStop, Linear},
    Background, Color, Gradient, Radians, Vector,
};
