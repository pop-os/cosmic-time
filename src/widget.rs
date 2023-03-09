pub mod button;
pub mod container;

pub use button::Button;
pub use container::Container;

pub enum StyleType<T> {
    Static(T),
    Blend(T, T, f32),
}
