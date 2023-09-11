#[cfg(feature = "libcosmic")]
use crate::keyframes::Cards;
use crate::keyframes::{
    Button, Column, Container, Row, Space, StyleButton, StyleContainer, Toggler,
};

use crate::MovementType;

/// Create a button keyframe.
/// Needs to be added into a chain. See [`crate::chain!`] macro.
pub fn button(at: impl Into<MovementType>) -> Button {
    Button::new(at)
}

/// Create a column keyframe.
/// Needs to be added into a chain. See [`crate::chain!`] macro.
pub fn column(at: impl Into<MovementType>) -> Column {
    Column::new(at)
}

/// Create a container keyframe.
/// Needs to be added into a chain. See [`crate::chain!`] macro.
pub fn container(at: impl Into<MovementType>) -> Container {
    Container::new(at)
}

/// Create a row keyframe.
/// Needs to be added into a chain. See [`crate::chain!`] macro.
pub fn row(at: impl Into<MovementType>) -> Row {
    Row::new(at)
}

/// Create a space keyframe.
/// Needs to be added into a chain. See [`crate::chain!`] macro.
pub fn space(at: impl Into<MovementType>) -> Space {
    Space::new(at)
}

/// Create a toggler keyframe.
/// Needs to be added into a chain. See [`crate::chain!`] macro.
pub fn toggler(at: impl Into<MovementType>) -> Toggler {
    Toggler::new(at)
}

#[cfg(feature = "libcosmic")]
/// Create a cards keyframe.
/// Needs to be added into a chain. See [`crate::chain!`] macro.
pub fn cards(at: impl Into<MovementType>) -> Cards {
    Cards::new(at)
}

/// Create a `style_button` keyframe.
/// Needs to be added into a chain. See [`crate::chain!`] macro.
pub fn style_button(at: impl Into<MovementType>) -> StyleButton {
    StyleButton::new(at)
}

/// Create a `style_container` keyframe.
/// Needs to be added into a chain. See [`crate::chain!`] macro.
pub fn style_container(at: impl Into<MovementType>) -> StyleContainer {
    StyleContainer::new(at)
}

/// A slightly different import to clean up makeing lazy keyframes.
pub mod lazy {
    #[cfg(feature = "libcosmic")]
    use crate::keyframes::Cards;
    use crate::keyframes::{
        Button, Column, Container, Row, Space, StyleButton, StyleContainer, Toggler,
    };
    use crate::MovementType;

    /// Create a lazy button keyframe.
    /// Needs to be added into a chain. See [`crate::chain!`] macro.
    pub fn button(at: impl Into<MovementType>) -> Button {
        Button::lazy(at)
    }

    /// Create a lazy column keyframe.
    /// Needs to be added into a chain. See [`crate::chain!`] macro.
    pub fn column(at: impl Into<MovementType>) -> Column {
        Column::lazy(at)
    }

    /// Create a lazy container keyframe.
    /// Needs to be added into a chain. See [`crate::chain!`] macro.
    pub fn container(at: impl Into<MovementType>) -> Container {
        Container::lazy(at)
    }

    /// Create a lazy row keyframe.
    /// Needs to be added into a chain. See [`crate::chain!`] macro.
    pub fn row(at: impl Into<MovementType>) -> Row {
        Row::lazy(at)
    }

    /// Create a lazy space keyframe.
    /// Needs to be added into a chain. See [`crate::chain!`] macro.
    pub fn space(at: impl Into<MovementType>) -> Space {
        Space::lazy(at)
    }

    /// Create a lazy toggler keyframe.
    /// Needs to be added into a chain. See [`crate::chain!`] macro.
    pub fn toggler(at: impl Into<MovementType>) -> Toggler {
        Toggler::lazy(at)
    }

    #[cfg(feature = "libcosmic")]
    /// Create a lazy toggler keyframe.
    /// Needs to be added into a chain. See [`crate::chain!`] macro.
    pub fn cards(at: impl Into<MovementType>) -> Cards {
        Cards::lazy(at)
    }

    /// Create a lazy `style_button` keyframe.
    /// Needs to be added into a chain. See [`crate::chain!`] macro.
    pub fn style_button(at: impl Into<MovementType>) -> StyleButton {
        StyleButton::lazy(at)
    }

    /// Create a lazy `style_container` keyframe.
    /// Needs to be added into a chain. See [`crate::chain!`] macro.
    pub fn style_container(at: impl Into<MovementType>) -> StyleContainer {
        StyleContainer::lazy(at)
    }
}

/// A slightly different import to clean up makeing animation Ids.
pub mod id {
    #[cfg(feature = "libcosmic")]
    pub use crate::keyframes::cards::Id as Cards;
    pub use crate::keyframes::{
        button::Id as Button, column::Id as Column, container::Id as Container, row::Id as Row,
        space::Id as Space, style_button::Id as StyleButton, style_container::Id as StyleContainer,
        toggler::Id as Toggler,
    };
}

/// Direct access to `Chain`s for widget that may return an animation
/// in a message.
pub mod chain {
    #[cfg(feature = "libcosmic")]
    pub use crate::keyframes::cards::Chain as Cards;
    pub use crate::keyframes::toggler::Chain as Toggler;
}
