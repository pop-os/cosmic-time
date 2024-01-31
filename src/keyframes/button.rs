use self::iced_core::{widget::Id as IcedId, Element, Length, Padding, Renderer as IcedRenderer};
use crate::reexports::{iced_core, iced_style, iced_widget};

use crate::keyframes::{as_f32, get_length, Repeat};
use crate::timeline::Frame;
use crate::{Ease, Linear, MovementType};

/// A Button's animation Id. Used for linking animation built in `update()` with widget output in `view()`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Id(IcedId);

impl Id {
    /// Creates a custom [`Id`].
    pub fn new(id: impl Into<std::borrow::Cow<'static, str>>) -> Self {
        Self(IcedId::new(id))
    }

    /// Creates a unique [`Id`].
    ///
    /// This function produces a different [`Id`] every time it is called.
    #[must_use]
    pub fn unique() -> Self {
        Self(IcedId::unique())
    }

    /// Used by [`crate::chain!`] macro
    #[must_use]
    pub fn into_chain(self) -> Chain {
        Chain::new(self)
    }

    /// Used by [`crate::chain!`] macro
    #[must_use]
    pub fn into_chain_with_children(self, children: Vec<Button>) -> Chain {
        Chain::with_children(self, children)
    }

    /// Used by [`crate::anim!`] macro
    pub fn as_widget<'a, Message, Theme, Renderer>(
        self,
        timeline: &crate::Timeline,
        content: impl Into<Element<'a, Message, Theme, Renderer>>,
    ) -> iced_widget::Button<'a, Message, Theme, Renderer>
    where
        Renderer: IcedRenderer,
        Theme: iced_style::button::StyleSheet,
    {
        Button::as_widget(self, timeline, content)
    }
}

impl From<Id> for IcedId {
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

    pub fn with_children(id: Id, children: Vec<Button>) -> Self {
        Chain {
            id,
            links: children,
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

impl From<Chain> for crate::timeline::Chain {
    fn from(chain: Chain) -> Self {
        crate::timeline::Chain::new(
            chain.id.into(),
            chain.repeat,
            chain
                .links
                .into_iter()
                .map(std::convert::Into::into)
                .collect::<Vec<_>>(),
        )
    }
}

#[must_use = "Keyframes are intended to be used in an animation chain."]
#[derive(Debug, Clone, Copy)]
pub struct Button {
    at: MovementType,
    ease: Ease,
    width: Option<Length>,
    height: Option<Length>,
    padding: Option<Padding>,
    is_eager: bool,
}

impl Button {
    pub fn new(at: impl Into<MovementType>) -> Button {
        let at = at.into();
        Button {
            at,
            ease: Linear::InOut.into(),
            width: None,
            height: None,
            padding: None,
            is_eager: true,
        }
    }

    pub fn lazy(at: impl Into<MovementType>) -> Button {
        let at = at.into();
        Button {
            at,
            ease: Linear::InOut.into(),
            width: None,
            height: None,
            padding: None,
            is_eager: false,
        }
    }

    pub fn as_widget<'a, Message, Theme, Renderer>(
        id: Id,
        timeline: &crate::Timeline,
        content: impl Into<Element<'a, Message, Theme, Renderer>>,
    ) -> iced_widget::Button<'a, Message, Theme, Renderer>
    where
        Renderer: IcedRenderer,
        Theme: iced_style::button::StyleSheet,
    {
        let id: IcedId = id.into();

        iced_widget::Button::new(content)
            .width(get_length(&id, timeline, 0, Length::Shrink))
            .height(get_length(&id, timeline, 1, Length::Shrink))
            .padding([
                timeline.get(&id, 2).map_or(5.0, |m| m.value),
                timeline.get(&id, 3).map_or(5.0, |m| m.value),
                timeline.get(&id, 4).map_or(5.0, |m| m.value),
                timeline.get(&id, 5).map_or(5.0, |m| m.value),
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
}

#[rustfmt::skip]
impl From<Button> for Vec<Option<Frame>> {
    fn from(button: Button) -> Vec<Option<Frame>> {
      if button.is_eager {
        vec![as_f32(button.width).map(|w| Frame::eager(button.at, w, button.ease)),  // 0 = width
             as_f32(button.height).map(|h| Frame::eager(button.at, h, button.ease)), // 1 = height
             button.padding.map(|p| Frame::eager(button.at, p.top, button.ease)),    // 2 = padding[0] (top)
             button.padding.map(|p| Frame::eager(button.at, p.right, button.ease)),  // 3 = padding[1] (right)
             button.padding.map(|p| Frame::eager(button.at, p.bottom, button.ease)), // 4 = padding[2] (bottom)
             button.padding.map(|p| Frame::eager(button.at, p.left, button.ease)),   // 5 = padding[3] (left)
        ]
      } else {
        vec![Some(Frame::lazy(button.at, 0., button.ease)), // 0 = width
             Some(Frame::lazy(button.at, 0., button.ease)), // 1 = height
             Some(Frame::lazy(button.at, 5., button.ease)), // 2 = padding[0] (top)
             Some(Frame::lazy(button.at, 5., button.ease)), // 3 = padding[1] (right)
             Some(Frame::lazy(button.at, 5., button.ease)), // 4 = padding[2] (bottom)
             Some(Frame::lazy(button.at, 5., button.ease)), // 5 = padding[3] (left)
        ]
      }
    }
}
