use iced::{Length, Padding};
use iced_native::{widget, Element, Renderer};
use iced_style::button::{ StyleSheet};

use std::time::{Duration, Instant};

use crate::keyframes::{clamp_u16, get_length, Repeat};
use crate::timeline::DurFrame;
use crate::{Ease, Linear};

/// A Button's animation Id. Used for linking animation built in `update()` with widget output in `view()`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Id(iced_native::widget::Id);

impl Id {
    /// Creates a custom [`Id`].
    pub fn new(id: impl Into<std::borrow::Cow<'static, str>>) -> Self {
        Self(widget::Id::new(id))
    }

    /// Creates a unique [`Id`].
    ///
    /// This function produces a different [`Id`] every time it is called.
    pub fn unique() -> Self {
        Self(widget::Id::unique())
    }
}

impl From<Id> for widget::Id {
    fn from(id: Id) -> Self {
        id.0
    }
}

#[derive(Debug)]
pub struct Chain {
    id: Id,
    links: Vec<Button>,
    repeat: Repeat,
}

impl Chain {
    pub fn new(id: Id) -> Self {
        Chain {
            id,
            links: Vec::new(),
            repeat: Repeat::Never,
        }
    }

    pub fn link(mut self, button: Button) -> Self {
        self.links.push(button);
        self
    }

    pub fn loop_forever(mut self) -> Self {
        self.repeat = Repeat::Forever;
        self
    }

    pub fn loop_once(mut self) -> Self {
        self.repeat = Repeat::Never;
        self
    }
}

impl<T> From<Chain> for crate::timeline::Chain<T>
where
    T: ExactSizeIterator<Item = Option<DurFrame>> + std::fmt::Debug,
    Vec<T>: From<Vec<Button>>,
{
    fn from(chain: Chain) -> Self {
        crate::timeline::Chain::new(chain.id.into(), chain.repeat, chain.links.into())
    }
}

#[must_use = "Keyframes are intended to be used in an animation chain."]
#[derive(Debug)]
pub struct Button {
    index: usize,
    at: Duration,
    ease: Ease,
    width: Option<Length>,
    height: Option<Length>,
    padding: Option<Padding>,
    style: iced_style::theme::Button,
}

impl Button {
    pub fn new(at: Duration) -> Button {
        Button {
            index: 0,
            at,
            ease: Linear::InOut.into(),
            width: None,
            height: None,
            padding: None,
        }
    }

    pub fn as_widget<'a, Message, Renderer>(
        id: Id,
        timeline: &crate::Timeline,
        content: impl Into<Element<'a, Message, Renderer>>,
    ) -> crate::widget::Button<'a, Message, Renderer>
    where
        Renderer: iced_native::Renderer,
        Renderer::Theme: widget::button::StyleSheet, <Renderer as iced_native::Renderer>::Theme: iced_style::button::StyleSheet
    {
        let id: widget::Id = id.into();
        let now = Instant::now();

        crate::widget::Button::new(content)
            .width(get_length(&id, timeline, &now, 0, Length::Shrink))
            .height(get_length(&id, timeline, &now, 1, Length::Shrink))
            .padding([
                clamp_u16(timeline.get(&id, &now, 2)).unwrap_or(0),
                clamp_u16(timeline.get(&id, &now, 3)).unwrap_or(0),
                clamp_u16(timeline.get(&id, &now, 4)).unwrap_or(0),
                clamp_u16(timeline.get(&id, &now, 5)).unwrap_or(0),
            ])
    }

    pub fn width(mut self, width: Length) -> Self {
        self.width = Some(width);
        self
    }

    pub fn height(mut self, height: Length) -> Self {
        self.height = Some(height);
        self
    }

    pub fn padding<P: Into<Padding>>(mut self, padding: P) -> Self {
        self.padding = Some(padding.into());
        self
    }

    pub fn ease<E: Into<Ease>>(mut self, ease: E) -> Self {
        self.ease = ease.into();
        self
    }

    pub fn style(mut self, style: iced_style::theme::Button) -> Self {
      self.style = style;
      self
    }
}

// 0 = width
// 1 = height
// 2 = padding[1] (top)
// 3 = padding[2] (right)
// 4 = padding[3] (bottom)
// 5 = padding[4] (left)
impl Iterator for Button {
    type Item = Option<DurFrame>;

    fn next(&mut self) -> Option<Option<DurFrame>> {
        self.index += 1;
        match self.index - 1 {
            0 => Some(as_isize(self.width).map(|w| DurFrame::new(self.at, w, self.ease))),
            1 => Some(as_isize(self.height).map(|h| DurFrame::new(self.at, h, self.ease))),
            2 => Some(
                self.padding
                    .map(|p| DurFrame::new(self.at, p.top as isize, self.ease)),
            ),
            3 => Some(
                self.padding
                    .map(|p| DurFrame::new(self.at, p.right as isize, self.ease)),
            ),
            4 => Some(
                self.padding
                    .map(|p| DurFrame::new(self.at, p.bottom as isize, self.ease)),
            ),
            5 => Some(
                self.padding
                    .map(|p| DurFrame::new(self.at, p.left as isize, self.ease)),
            ),
            _ => None,
        }
    }
}

impl ExactSizeIterator for Button {
    fn len(&self) -> usize {
        6 - self.index
    }
}

fn as_isize(length: Option<Length>) -> Option<isize> {
    match length {
        Some(Length::Units(i)) => Some(i as isize),
        _ => None,
    }
}
