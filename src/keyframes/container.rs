use iced_native::{widget, Element, Length, Padding, Pixels};

use crate::keyframes::{as_f32, get_length, Repeat};
use crate::timeline::Frame;
use crate::{Ease, Linear, MovementType};

/// A Container's animation Id. Used for linking animation built in `update()` with widget output in `view()`
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

    /// Used by [`chain!`] macro
    pub fn into_chain(self) -> Chain {
        Chain::new(self)
    }

    /// Used by [`chain!`] macro
    pub fn into_chain_with_children(self, children: Vec<Container>) -> Chain {
        Chain::with_children(self, children)
    }

    /// Used by [`anim!`] macro
    pub fn as_widget<'a, Message, Renderer>(
        self,
        timeline: &crate::Timeline,
        content: impl Into<Element<'a, Message, Renderer>>,
    ) -> widget::Container<'a, Message, Renderer>
    where
        Renderer: iced_native::Renderer,
        Renderer::Theme: widget::container::StyleSheet,
    {
        Container::as_widget(self, timeline, content)
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
    links: Vec<Container>,
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

    pub fn with_children(id: Id, children: Vec<Container>) -> Self {
        Chain {
            id,
            links: children,
            repeat: Repeat::Never,
        }
    }

    pub fn link(mut self, container: Container) -> Self {
        self.links.push(container);
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
pub struct Container {
    at: MovementType,
    ease: Ease,
    width: Option<Length>,
    height: Option<Length>,
    padding: Option<Padding>,
    max_width: Option<f32>,
    max_height: Option<f32>,
    is_eager: bool,
}

impl Container {
    pub fn new(at: impl Into<MovementType>) -> Container {
        let at = at.into();
        Container {
            at,
            ease: Linear::InOut.into(),
            width: None,
            height: None,
            padding: None,
            max_width: None,
            max_height: None,
            is_eager: true,
        }
    }

    pub fn lazy(at: impl Into<MovementType>) -> Container {
        let at = at.into();
        Container {
            at,
            ease: Linear::InOut.into(),
            width: None,
            height: None,
            padding: None,
            max_width: None,
            max_height: None,
            is_eager: false,
        }
    }

    pub fn as_widget<'a, Message, Renderer>(
        id: Id,
        timeline: &crate::Timeline,
        content: impl Into<Element<'a, Message, Renderer>>,
    ) -> widget::Container<'a, Message, Renderer>
    where
        Renderer: iced_native::Renderer,
        Renderer::Theme: widget::container::StyleSheet,
    {
        let id: widget::Id = id.into();

        widget::Container::new(content)
            .width(get_length(&id, timeline, 0, Length::Shrink))
            .height(get_length(&id, timeline, 1, Length::Shrink))
            .padding([
                timeline.get(&id, 2).map(|m| m.value).unwrap_or(0.),
                timeline.get(&id, 3).map(|m| m.value).unwrap_or(0.),
                timeline.get(&id, 4).map(|m| m.value).unwrap_or(0.),
                timeline.get(&id, 5).map(|m| m.value).unwrap_or(0.),
            ])
            .max_width(
                timeline
                    .get(&id, 6)
                    .map(|m| m.value)
                    .unwrap_or(f32::INFINITY),
            )
            .max_height(
                timeline
                    .get(&id, 7)
                    .map(|m| m.value)
                    .unwrap_or(f32::INFINITY),
            )
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = Some(width.into());
        self
    }

    pub fn max_width(mut self, max_width: impl Into<Pixels>) -> Self {
        self.max_width = Some(max_width.into().0);
        self
    }

    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = Some(height.into());
        self
    }

    pub fn max_height(mut self, max_height: impl Into<Pixels>) -> Self {
        self.max_height = Some(max_height.into().0);
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
impl From<Container> for Vec<Option<Frame>> {
    fn from(container: Container) -> Vec<Option<Frame>> {
      if container.is_eager {
        vec![as_f32(container.width).map(|w| Frame::eager(container.at, w, container.ease)),  // 0 = width
             as_f32(container.height).map(|h| Frame::eager(container.at, h, container.ease)), // 1 = height
             container.padding.map(|p| Frame::eager(container.at, p.top, container.ease)),    // 2 = padding[0] (top)
             container.padding.map(|p| Frame::eager(container.at, p.right, container.ease)),  // 3 = padding[1] (right)
             container.padding.map(|p| Frame::eager(container.at, p.bottom, container.ease)), // 4 = padding[2] (bottom)
             container.padding.map(|p| Frame::eager(container.at, p.left, container.ease)),   // 5 = padding[3] (left)
             container.max_width.map(|w| Frame::eager(container.at, w, container.ease)),      // 6 = max_width
             container.max_height.map(|h| Frame::eager(container.at, h, container.ease)),     // 7 = max_height
        ]
      } else {
        vec![Some(Frame::lazy(container.at, 0., container.ease)); 8] // lazy evaluates for all values
      }
    }
}
