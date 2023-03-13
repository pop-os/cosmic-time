#![allow(clippy::too_many_arguments)]
pub mod button;
pub mod container;
pub mod toggler;

pub use button::Button;
pub use container::Container;
pub use toggler::Toggler;

pub enum StyleType<T> {
    Static(T),
    Blend(T, T, f32),
}
