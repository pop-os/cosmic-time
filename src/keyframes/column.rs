use iced_native::{widget, Element, Length, Padding, Pixels};

use crate::keyframes::{get_length, Repeat, as_f32};
use crate::timeline::DurFrame;
use crate::{Ease, Linear, MovementType};

/// A Column's animation Id. Used for linking animation built in `update()` with widget output in `view()`
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
    links: Vec<Column>,
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

    pub fn link(mut self, container: Column) -> Self {
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

impl<T> From<Chain> for crate::timeline::Chain<T>
where
    T: ExactSizeIterator<Item = Option<DurFrame>> + std::fmt::Debug,
    Vec<T>: From<Vec<Column>>,
{
    fn from(chain: Chain) -> Self {
        crate::timeline::Chain::new(chain.id.into(), chain.repeat, chain.links.into())
    }
}

#[must_use = "Keyframes are intended to be used in an animation chain."]
#[derive(Debug)]
pub struct Column {
    index: usize,
    at: MovementType,
    ease: Ease,
    spacing: Option<f32>,
    padding: Option<Padding>,
    width: Option<Length>,
    height: Option<Length>,
}

impl Column {
    pub fn new(at: impl Into<MovementType>) -> Column {
      let at = at.into();
        Column {
            index: 0,
            at,
            ease: Linear::InOut.into(),
            width: None,
            height: None,
            padding: None,
            max_width: None,
            max_height: None,
        }
    }

    pub fn as_widget<'a, Message, Renderer>(
        id: Id,
        timeline: &crate::Timeline,
        content: impl Into<Element<'a, Message, Renderer>>,
    ) -> widget::Column<'a, Message, Renderer>
    where
        Renderer: iced_native::Renderer,
        Renderer::Theme: widget::container::StyleSheet,
    {
        let id: widget::Id = id.into();
        let now = Instant::now();

        widget::Column::new(content)
            .spacing(
                timeline
                    .get(&id, &now, 0)
                    .map(|m| m.value)
                    .unwrap_or(0.),
            )
            .padding([
                timeline.get(&id, &now, 1).map(|m| m.value).unwrap_or(0.),
                timeline.get(&id, &now, 2).map(|m| m.value).unwrap_or(0.),
                timeline.get(&id, &now, 3).map(|m| m.value).unwrap_or(0.),
                timeline.get(&id, &now, 4).map(|m| m.value).unwrap_or(0.),
            ])
            .width(get_length(&id, timeline, &now, 5, Length::Shrink))
            .height(get_length(&id, timeline, &now, 6, Length::Shrink))
    }

    pub fn spacing(mut self, spacing: impl Into<Pixels>) -> Self {
      self.spacing = spacing.into().0;
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

// 0 = spacing
// 1 = padding[1] (top)
// 2 = padding[2] (right)
// 3 = padding[3] (bottom)
// 4 = padding[4] (left)
// 5 = width
// 6 = height
impl Iterator for Column {
    type Item = Option<DurFrame>;

    fn next(&mut self) -> Option<Option<DurFrame>> {
        self.index += 1;
        match self.index - 1 {
            0 => Some(self.spacing.map(|s| DurFrame::new(self.at, s, self.ease))),
            1 => Some(
                self.padding
                    .map(|p| DurFrame::new(self.at, p.top, self.ease)),
            ),
            2 => Some(
                self.padding
                    .map(|p| DurFrame::new(self.at, p.right, self.ease)),
            ),
            3 => Some(
                self.padding
                    .map(|p| DurFrame::new(self.at, p.bottom, self.ease)),
            ),
            4 => Some(
                self.padding
                    .map(|p| DurFrame::new(self.at, p.left, self.ease)),
            ),
            5 => Some(as_f32(self.width).map(|w| DurFrame::new(self.at, w, self.ease))),
            6 => Some(as_f32(self.height).map(|h| DurFrame::new(self.at, h, self.ease))),
            _ => None,
        }
    }
}

impl ExactSizeIterator for Column {
    fn len(&self) -> usize {
        7 - self.index
    }
}
