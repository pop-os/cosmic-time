use iced_native::{widget, Element, Length, Padding};
use iced_style::button::StyleSheet;

use crate::keyframes::{as_f32, get_length, Repeat};
use crate::timeline::{Frame, Interped};
use crate::{Ease, Linear, MovementType};

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

    pub fn to_chain(self) -> Chain {
        Chain::new(self)
    }

    pub fn to_chain_with_children(self, children: Vec<StyleButton>) -> Chain {
        Chain::with_children(self, children)
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
    links: Vec<StyleButton>,
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

    pub fn with_children(id: Id, children: Vec<StyleButton>) -> Self {
        Chain {
            id,
            links: children,
            repeat: Repeat::Never,
        }
    }

    pub fn link(mut self, button: StyleButton) -> Self {
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
                .map(|b| b.into())
                .collect::<Vec<_>>(),
        )
    }
}

#[must_use = "Keyframes are intended to be used in an animation chain."]
#[derive(Debug, Clone, Copy)]
pub struct StyleButton {
    at: MovementType,
    ease: Ease,
    width: Option<Length>,
    height: Option<Length>,
    padding: Option<Padding>,
    style: Option<u8>,
    is_eager: bool,
}

impl StyleButton {
    pub fn new(at: impl Into<MovementType>) -> StyleButton {
        let at = at.into();
        StyleButton {
            at,
            ease: Linear::InOut.into(),
            width: None,
            height: None,
            padding: None,
            style: None,
            is_eager: true,
        }
    }

    pub fn lazy(at: impl Into<MovementType>) -> StyleButton {
        let at = at.into();
        StyleButton {
            at,
            ease: Linear::InOut.into(),
            width: None,
            height: None,
            padding: None,
            style: None,
            is_eager: false,
        }
    }

    // Returns a cosmic-time button, not a default iced button. The difference shouldn't
    // matter to the end user. Though it is an implementation detail.
    pub fn as_widget<'a, Message, Renderer>(
        id: Id,
        style: fn(u8) -> <Renderer::Theme as StyleSheet>::Style,
        timeline: &crate::Timeline,
        content: impl Into<Element<'a, Message, Renderer>>,
    ) -> crate::widget::Button<'a, Message, Renderer>
    where
        Renderer: iced_native::Renderer,
        Renderer::Theme: widget::button::StyleSheet,
    {
        let id: widget::Id = id.into();

        let button = crate::widget::Button::new(content)
            .width(get_length(&id, timeline, 0, Length::Shrink))
            .height(get_length(&id, timeline, 1, Length::Shrink))
            .padding([
                timeline.get(&id, 2).map(|m| m.value).unwrap_or(5.0),
                timeline.get(&id, 3).map(|m| m.value).unwrap_or(5.0),
                timeline.get(&id, 4).map(|m| m.value).unwrap_or(5.0),
                timeline.get(&id, 5).map(|m| m.value).unwrap_or(5.0),
            ]);

        if let Some(Interped {
            previous,
            next,
            percent,
            ..
        }) = timeline.get(&id, 6)
        {
            button.blend_style(style(previous as u8), style(next as u8), percent)
        } else {
            button
        }
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

    pub fn style(mut self, style: impl Into<u8>) -> Self {
        let style = style.into();
        self.style = Some(style);
        self
    }
}

#[rustfmt::skip]
impl From<StyleButton> for Vec<Option<Frame>> {
    fn from(button: StyleButton) -> Vec<Option<Frame>> {
      if button.is_eager {
        vec![as_f32(button.width).map(|w| Frame::eager(button.at, w, button.ease)),  // 0 = width
             as_f32(button.height).map(|h| Frame::eager(button.at, h, button.ease)), // 1 = height
             button.padding.map(|p| Frame::eager(button.at, p.top, button.ease)),    // 2 = padding[0] (top)
             button.padding.map(|p| Frame::eager(button.at, p.right, button.ease)),  // 3 = padding[1] (right)
             button.padding.map(|p| Frame::eager(button.at, p.bottom, button.ease)), // 4 = padding[2] (bottom)
             button.padding.map(|p| Frame::eager(button.at, p.left, button.ease)),  // 5 = padding[3] (left)
             button.style.map(|s| Frame::eager(button.at, s as f32, button.ease)),  // 6 = style blend (passed to widget to mix values at `draw` time)
        ]
      } else {
        vec![Some(Frame::lazy(button.at, 0., button.ease)), // 0 = width
             Some(Frame::lazy(button.at, 0., button.ease)), // 1 = height
             Some(Frame::lazy(button.at, 5., button.ease)), // 2 = padding[0] (top)
             Some(Frame::lazy(button.at, 5., button.ease)), // 3 = padding[1] (right)
             Some(Frame::lazy(button.at, 5., button.ease)), // 4 = padding[2] (bottom)
             Some(Frame::lazy(button.at, 5., button.ease)), // 5 = padding[3] (left)
             Some(Frame::lazy(button.at, 0., button.ease)), // 6 = style blend (passed to widget to mix values at `draw` time)
        ]
      }
    }
}
