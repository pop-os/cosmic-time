//! Reexports of all the modules in this crate.

#[cfg(feature = "libcosmic")]
mod libcosmic;
#[cfg(feature = "libcosmic")]
pub use self::libcosmic::{iced_core, iced_futures, iced_runtime, iced_style, iced_widget};

#[cfg(feature = "iced")]
mod iced;
#[cfg(feature = "iced")]
pub use self::iced::{iced_core, iced_futures, iced_runtime, iced_style, iced_widget};
