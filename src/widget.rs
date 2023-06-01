#![allow(clippy::too_many_arguments)]

#[cfg(feature = "libcosmic")]
pub mod cosmic_button;
#[cfg(feature = "libcosmic")]
pub mod cosmic_container;
#[cfg(feature = "libcosmic")]
pub mod cosmic_toggler;

#[cfg(feature = "libcosmic")]
pub use cosmic_button::Button;
#[cfg(feature = "libcosmic")]
pub use cosmic_container::Container;
#[cfg(feature = "libcosmic")]
pub use cosmic_toggler::Toggler;

#[cfg(not(feature = "libcosmic"))]
pub mod button;
#[cfg(not(feature = "libcosmic"))]
pub mod container;
#[cfg(not(feature = "libcosmic"))]
pub mod toggler;

#[cfg(not(feature = "libcosmic"))]
pub use button::Button;
#[cfg(not(feature = "libcosmic"))]
pub use container::Container;
#[cfg(not(feature = "libcosmic"))]
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
