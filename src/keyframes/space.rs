use iced_native::{widget, Length};

use crate::keyframes::{as_f32, get_length, Repeat};
use crate::timeline::Frame;
use crate::{Ease, Linear, MovementType};

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

    pub fn into_chain(self) -> Chain {
        Chain::new(self)
    }

    pub fn into_chain_with_children(self, children: Vec<Space>) -> Chain {
        Chain::with_children(self, children)
    }

    pub fn as_widget(self, timeline: &crate::Timeline) -> widget::Space {
        Space::as_widget(self, timeline)
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

    pub fn with_children(id: Id, children: Vec<Space>) -> Self {
        Chain {
            id,
            links: children,
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

impl From<Chain> for crate::timeline::Chain {
    fn from(chain: Chain) -> Self {
        crate::timeline::Chain::new(
            chain.id.into(),
            chain.repeat,
            chain
                .links
                .into_iter()
                .map(|s| s.into())
                .collect::<Vec<_>>(),
        )
    }
}

#[must_use = "Keyframes are intended to be used in an animation chain."]
#[derive(Debug, Clone, Copy)]
pub struct Space {
    at: MovementType,
    ease: Ease,
    width: Option<Length>,
    height: Option<Length>,
    is_eager: bool,
}

impl Space {
    pub fn new(at: impl Into<MovementType>) -> Self {
        let at = at.into();
        Space {
            at,
            ease: Linear::InOut.into(),
            width: None,
            height: None,
            is_eager: true,
        }
    }

    pub fn lazy(at: impl Into<MovementType>) -> Self {
        let at = at.into();
        Space {
            at,
            ease: Linear::InOut.into(),
            width: None,
            height: None,
            is_eager: false,
        }
    }

    pub fn as_widget(id: Id, timeline: &crate::Timeline) -> widget::Space {
        let id: widget::Id = id.into();

        widget::Space::new(
            get_length(&id, timeline, 0, Length::Shrink),
            get_length(&id, timeline, 1, Length::Shrink),
        )
    }

    // does nothing if lazy
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = Some(width.into());
        self
    }

    // does nothing if lazy
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = Some(height.into());
        self
    }

    pub fn ease<E: Into<Ease>>(mut self, ease: E) -> Self {
        self.ease = ease.into();
        self
    }
}

#[rustfmt::skip]
impl From<Space> for Vec<Option<Frame>> {
    fn from(space: Space) -> Vec<Option<Frame>> {
      if space.is_eager {
        vec![as_f32(space.width).map(|w| Frame::eager(space.at, w, space.ease)), // 0 = width
          as_f32(space.height).map(|h| Frame::eager(space.at, h, space.ease)) // 1 = height
        ]
      } else {
        vec![Some(Frame::lazy(space.at, 0., space.ease)); 2] // lazy calculates for both width & height
      }
    }
}
