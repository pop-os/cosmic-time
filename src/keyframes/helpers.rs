use crate::keyframes::{Button, Column, Container, Row, Space, StyleButton, StyleContainer};
use crate::MovementType;

pub fn button(at: impl Into<MovementType>) -> Button {
    Button::new(at)
}

pub fn column(at: impl Into<MovementType>) -> Column {
    Column::new(at)
}

pub fn container(at: impl Into<MovementType>) -> Container {
    Container::new(at)
}

pub fn row(at: impl Into<MovementType>) -> Row {
    Row::new(at)
}

pub fn space(at: impl Into<MovementType>) -> Space {
    Space::new(at)
}

pub fn style_button(at: impl Into<MovementType>) -> StyleButton {
    StyleButton::new(at)
}

pub fn style_container(at: impl Into<MovementType>) -> StyleContainer {
    StyleContainer::new(at)
}

pub mod lazy {
    use crate::keyframes::{Button, Column, Container, Row, Space, StyleButton, StyleContainer};
    use crate::MovementType;

    pub fn button(at: impl Into<MovementType>) -> Button {
        Button::lazy(at)
    }

    pub fn column(at: impl Into<MovementType>) -> Column {
        Column::lazy(at)
    }

    pub fn container(at: impl Into<MovementType>) -> Container {
        Container::lazy(at)
    }

    pub fn row(at: impl Into<MovementType>) -> Row {
        Row::lazy(at)
    }

    pub fn space(at: impl Into<MovementType>) -> Space {
        Space::lazy(at)
    }

    pub fn style_button(at: impl Into<MovementType>) -> StyleButton {
        StyleButton::lazy(at)
    }

    pub fn style_container(at: impl Into<MovementType>) -> StyleContainer {
        StyleContainer::lazy(at)
    }
}

pub mod id {
    pub use crate::keyframes::{
        button::Id as Button, column::Id as Column, container::Id as Container, row::Id as Row,
        space::Id as Space, style_button::Id as StyleButton, style_container::Id as StyleContainer,
    };
}
