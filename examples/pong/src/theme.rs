use iced::widget::{container, text};
use iced::{application, color};

#[derive(Debug, Clone, Copy, Default)]
pub struct Theme;

impl application::StyleSheet for Theme {
    type Style = ();

    fn appearance(&self, _style: &Self::Style) -> application::Appearance {
        application::Appearance {
            background_color: color!(0x00, 0x00, 0x00),
            text_color: color!(0xff, 0xff, 0xff),
        }
    }
}

impl text::StyleSheet for Theme {
    type Style = ();

    fn appearance(&self, _style: Self::Style) -> text::Appearance {
        text::Appearance {
            color: color!(0xeb, 0xdb, 0xb2).into(),
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum Container {
    #[default]
    Default,
    Paddle,
    Ball,
}

impl container::StyleSheet for Theme {
    type Style = Container;

    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        match style {
            Container::Default => container::Appearance {
                background: Some(color!(0, 0, 0).into()),
                ..Default::default()
            },
            Container::Paddle => container::Appearance {
                background: Some(color!(0xff, 0xff, 0xff).into()),
                ..Default::default()
            },
            Container::Ball => container::Appearance {
                background: Some(color!(0xff, 0xff, 0xff).into()),
                border_radius: 100000.0.into(),
                ..Default::default()
            },
        }
    }
}

pub mod widget {
    #![allow(dead_code)]
    use crate::theme::Theme;

    pub type Renderer = iced::Renderer<Theme>;
    pub type Element<'a, Message> = iced::Element<'a, Message, Renderer>;
    pub type Container<'a, Message> = iced::widget::Container<'a, Message, Renderer>;
}
