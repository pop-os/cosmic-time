use iced_native::time::Duration;
use iced_native::{widget, Length};

use crate::keyframes::{as_f32, get_length, Repeat};
use crate::timeline::DurFrame;
use crate::{Ease, Linear};

/// A Space's animation Id. Used for linking animation built in `update()` with widget output in `view()`
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
    links: Vec<Space>,
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

    pub fn link(mut self, space: Space) -> Self {
        self.links.push(space);
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
    Vec<T>: From<Vec<Space>>,
{
    fn from(chain: Chain) -> Self {
        crate::timeline::Chain::new(chain.id.into(), chain.repeat, chain.links.into())
    }
}

#[must_use = "Keyframes are intended to be used in an animation chain."]
#[derive(Debug)]
pub struct Space {
    index: usize,
    at: Duration,
    ease: Ease,
    width: Option<Length>,
    height: Option<Length>,
}

impl Space {
    pub fn new(at: Duration) -> Self {
        Space {
            index: 0,
            at,
            ease: Linear::InOut.into(),
            width: None,
            height: None,
        }
    }

    pub fn as_widget(id: Id, timeline: &crate::Timeline) -> widget::Space {
        let id: widget::Id = id.into();

        widget::Space::new(
            get_length(&id, timeline, 0, Length::Shrink),
            get_length(&id, timeline, 1, Length::Shrink),
        )
    }

    pub fn width(mut self, width: Length) -> Self {
        self.width = Some(width);
        self
    }

    pub fn height(mut self, height: Length) -> Self {
        self.height = Some(height);
        self
    }

    pub fn ease<E: Into<Ease>>(mut self, ease: E) -> Self {
        self.ease = ease.into();
        self
    }
}

// 0 = width
// 1 = height
impl Iterator for Space {
    type Item = Option<DurFrame>;

    fn next(&mut self) -> Option<Option<DurFrame>> {
        self.index += 1;
        match self.index - 1 {
            0 => Some(as_f32(self.width).map(|w| DurFrame::new(self.at, w, self.ease))),
            1 => Some(as_f32(self.height).map(|h| DurFrame::new(self.at, h, self.ease))),
            _ => None,
        }
    }
}

impl ExactSizeIterator for Space {
    fn len(&self) -> usize {
        6 - self.index
    }
}
