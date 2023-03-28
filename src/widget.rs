#![allow(clippy::too_many_arguments)]
pub mod button;
pub mod container;
pub mod toggler;

pub use button::Button;
pub use container::Container;
pub use toggler::Toggler;

/// A convenience type to optimize style-able widgets,
/// to only do the "expensize" style calculations if needed.
#[derive(Debug)]
pub enum StyleType<T> {
    /// The style is either default, or set manually in the `view`.
    Static(T),
    /// The stlye is being animated. Blend between the two values.
    Blend(T, T, f32),
}
