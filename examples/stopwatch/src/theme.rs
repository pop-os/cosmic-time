/*
 * This file is not specific to cosmic-time.
 * The relevant code to this example is in main.rs.
 * This is just code to make an iced theme, so
 * the stopwatch example can be prettier, and
 * show the andvantages of style animations
 * with a custom theme.
 *
 */

use iced::theme::palette::{self, Background};
use iced::widget::{button, container, text};
use iced::Background as B;
use iced::{application, color, Color, Vector};

#[derive(Default)]
pub struct Theme;

impl application::StyleSheet for Theme {
    type Style = ();

    fn appearance(&self, _style: &Self::Style) -> application::Appearance {
        application::Appearance {
            background_color: color!(0xff, 0xff, 0xff),
            text_color: color!(0xff, 0x00, 0x00),
        }
    }
}

impl text::StyleSheet for Theme {
    type Style = ();

    fn appearance(&self, _style: Self::Style) -> text::Appearance {
        text::Appearance {
            color: color!(0xff, 0xff, 0xff).into(),
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum Container {
    #[default]
    White,
    Red,
    Green,
    Blue,
}

impl container::StyleSheet for Theme {
    type Style = Container;

    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        match style {
            Container::White => container::Appearance {
                background: Some(B::Color(color!(0xd1, 0xd5, 0xdb))),
                text_color: Some(color!(0x00, 0x00, 0x00)),
                ..Default::default()
            },
            Container::Red => container::Appearance {
                background: Some(B::Color(color!(0xfc, 0xa5, 0xa5))),
                text_color: Some(color!(0x00, 0x00, 0x00)),
                ..Default::default()
            },
            Container::Green => container::Appearance {
                background: Some(B::Color(color!(0xb3, 0xf2, 0x64))),
                text_color: Some(color!(0x00, 0x00, 0x00)),
                ..Default::default()
            },
            Container::Blue => container::Appearance {
                background: Some(B::Color(color!(0x93, 0xc5, 0xfd))),
                text_color: Some(color!(0x00, 0x00, 0x00)),
                ..Default::default()
            },
        }
    }
}

#[derive(Default)]
pub enum Button {
    #[default]
    Primary,
    Secondary,
    Positive,
    Destructive,
    Text,
    Custom(Box<dyn button::StyleSheet<Style = Theme>>),
}

impl button::StyleSheet for Theme {
    type Style = Button;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        match style {
            Button::Primary => button::Appearance {
                background: color!(0x25, 0x63, 0xeb).into(),
                text_color: color!(0x00, 0x00, 0x00),
                border_radius: 10.0,
                border_width: 10.0,
                shadow_offset: Vector::new(3., 3.),
                border_color: color!(0x25, 0x63, 0xeb),
                ..Default::default()
            },
            Button::Secondary => button::Appearance {
                background: color!(0x3c, 0x38, 0x36).into(),
                border_radius: 10.0,
                shadow_offset: Vector::new(3., 3.),
                text_color: color!(0xff, 0xff, 0xff),
                ..Default::default()
            },
            Button::Destructive => button::Appearance {
                background: color!(0xdc, 0x26, 0x26).into(),
                text_color: color!(0xff, 0xff, 0xff),
                border_radius: 10.0,
                shadow_offset: Vector::new(5., 5.),
                border_color: color!(0xdc, 0x26, 0x26),
                border_width: 10.0,
                ..Default::default()
            },
            _ => panic!("This isn't a custom style exmaple, just skipping these for now"),
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        self.active(style)
    }

    fn pressed(&self, style: &Self::Style) -> button::Appearance {
        self.active(style)
    }
}
