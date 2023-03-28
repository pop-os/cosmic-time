use iced_native::{widget, Length, Padding, Pixels};

use crate::keyframes::{as_f32, get_length, Repeat};
use crate::timeline::Frame;
use crate::{Ease, Linear, MovementType};

/// A Row's animation Id. Used for linking animation built in `update()` with widget output in `view()`
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

    pub fn to_chain_with_children(self, children: Vec<Row>) -> Chain {
        Chain::with_children(self, children)
    }

    pub fn as_widget<'a, Message, Renderer>(
        self,
        timeline: &crate::Timeline,
    ) -> widget::Row<'a, Message, Renderer>
    where
        Renderer: iced_native::Renderer,
        Renderer::Theme: widget::container::StyleSheet,
    {
        Row::as_widget(self, timeline)
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
    links: Vec<Row>,
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

    pub fn with_children(id: Id, children: Vec<Row>) -> Self {
        Chain {
            id,
            links: children,
            repeat: Repeat::Never,
        }
    }

    pub fn link(mut self, container: Row) -> Self {
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
                .map(|r| r.into())
                .collect::<Vec<_>>(),
        )
    }
}

#[must_use = "Keyframes are intended to be used in an animation chain."]
#[derive(Debug, Clone, Copy)]
pub struct Row {
    at: MovementType,
    ease: Ease,
    spacing: Option<f32>,
    padding: Option<Padding>,
    width: Option<Length>,
    height: Option<Length>,
    is_eager: bool,
}

impl Row {
    pub fn new(at: impl Into<MovementType>) -> Row {
        let at = at.into();
        Row {
            at,
            ease: Linear::InOut.into(),
            spacing: None,
            width: None,
            height: None,
            padding: None,
            is_eager: true,
        }
    }

    pub fn lazy(at: impl Into<MovementType>) -> Row {
        let at = at.into();
        Row {
            at,
            ease: Linear::InOut.into(),
            spacing: None,
            width: None,
            height: None,
            padding: None,
            is_eager: false,
        }
    }

    pub fn as_widget<'a, Message, Renderer>(
        id: Id,
        timeline: &crate::Timeline,
    ) -> widget::Row<'a, Message, Renderer>
    where
        Renderer: iced_native::Renderer,
        Renderer::Theme: widget::container::StyleSheet,
    {
        let id: widget::Id = id.into();

        widget::Row::new()
            .spacing(timeline.get(&id, 0).map(|m| m.value).unwrap_or(0.))
            .padding([
                timeline.get(&id, 1).map(|m| m.value).unwrap_or(0.),
                timeline.get(&id, 2).map(|m| m.value).unwrap_or(0.),
                timeline.get(&id, 3).map(|m| m.value).unwrap_or(0.),
                timeline.get(&id, 4).map(|m| m.value).unwrap_or(0.),
            ])
            .width(get_length(&id, timeline, 5, Length::Shrink))
            .height(get_length(&id, timeline, 6, Length::Shrink))
    }

    pub fn spacing(mut self, spacing: impl Into<Pixels>) -> Self {
        self.spacing = Some(spacing.into().0);
        self
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = Some(width.into());
        self
    }

    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = Some(height.into());
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
impl From<Row> for Vec<Option<Frame>> {
    fn from(row: Row) -> Vec<Option<Frame>> {
      if row.is_eager {
        vec![row.spacing.map(|s| Frame::eager(row.at, s, row.ease)),        // 0 = spacing
             row.padding.map(|p| Frame::eager(row.at, p.top, row.ease)),    // 1 = padding[0] (top)
             row.padding.map(|p| Frame::eager(row.at, p.right, row.ease)),  // 2 = padding[1] (right)
             row.padding.map(|p| Frame::eager(row.at, p.bottom, row.ease)), // 3 = padding[2] (bottom)
             row.padding.map(|p| Frame::eager(row.at, p.left, row.ease)),  // 4 = padding[3] (left)
             as_f32(row.width).map(|w| Frame::eager(row.at, w, row.ease)),  // 5 = width
             as_f32(row.height).map(|h| Frame::eager(row.at, h, row.ease)), // 6 = height
        ]
      } else {
        vec![Some(Frame::lazy(row.at, 0., row.ease)); 7] // lazy evaluates for all values
      }
    }
}
