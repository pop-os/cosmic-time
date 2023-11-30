//! Reexports of all the modules in this crate.

#[cfg(feature = "libcosmic")]
mod libcosmic;
#[cfg(feature = "libcosmic")]
pub use self::libcosmic::{iced, iced_core, iced_futures, iced_runtime, iced_style, iced_widget};

#[cfg(feature = "iced")]
#[path = "iced.rs"]
mod _iced;
#[cfg(feature = "iced")]
pub use self::_iced::{iced, iced_core, iced_futures, iced_runtime, iced_style, iced_widget};
