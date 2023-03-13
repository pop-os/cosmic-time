use iced_native::time::Duration;
use iced_native::{widget, Element, Length, Padding, Pixels};
use iced_style::container::StyleSheet;

use crate::keyframes::{as_f32, get_length, Repeat};
use crate::timeline::{DurFrame, Interped};
use crate::{Ease, Linear};

/// A StyleContainer's animation Id. Used for linking animation built in `update()` with widget output in `view()`
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
    links: Vec<StyleContainer>,
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

    pub fn link(mut self, container: StyleContainer) -> Self {
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
    Vec<T>: From<Vec<StyleContainer>>,
{
    fn from(chain: Chain) -> Self {
        crate::timeline::Chain::new(chain.id.into(), chain.repeat, chain.links.into())
    }
}

#[must_use = "Keyframes are intended to be used in an animation chain."]
#[derive(Debug)]
pub struct StyleContainer {
    index: usize,
    at: Duration,
    ease: Ease,
    width: Option<Length>,
    height: Option<Length>,
    padding: Option<Padding>,
    max_width: Option<f32>,
    max_height: Option<f32>,
    style: Option<u8>,
}

impl StyleContainer {
    pub fn new(at: Duration) -> StyleContainer {
        StyleContainer {
            index: 0,
            at,
            ease: Linear::InOut.into(),
            width: None,
            height: None,
            padding: None,
            max_width: None,
            max_height: None,
            style: None,
        }
    }

    // Returns a cosmic-time container, not a default iced button. The difference shouldn't
    // matter to the end user. Though it is an implementation detail.
    pub fn as_widget<'a, Message, Renderer>(
        id: Id,
        style: fn(u8) -> <Renderer::Theme as StyleSheet>::Style,
        timeline: &crate::Timeline,
        content: impl Into<Element<'a, Message, Renderer>>,
    ) -> crate::widget::Container<'a, Message, Renderer>
    where
        Renderer: iced_native::Renderer,
        Renderer::Theme: widget::container::StyleSheet,
    {
        let id: widget::Id = id.into();

        let container = crate::widget::Container::new(content)
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
            );

        if let Some(Interped {
            previous,
            next,
            percent,
            ..
        }) = timeline.get(&id, 8)
        {
            container.blend_style(style(previous as u8), style(next as u8), percent)
        } else {
            container
        }
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

    pub fn style(mut self, style: u8) -> Self {
        self.style = Some(style);
        self
    }
}

// 0 = width
// 1 = height
// 2 = padding[1] (top)
// 3 = padding[2] (right)
// 4 = padding[3] (bottom)
// 5 = padding[4] (left)
// 6 = max_width
// 7 = max_height
// 8 = style blend (passed to widget to mix values at `draw` time)
impl Iterator for StyleContainer {
    type Item = Option<DurFrame>;

    fn next(&mut self) -> Option<Option<DurFrame>> {
        self.index += 1;
        match self.index - 1 {
            0 => Some(as_f32(self.width).map(|w| DurFrame::new(self.at, w, self.ease))),
            1 => Some(as_f32(self.height).map(|h| DurFrame::new(self.at, h, self.ease))),
            2 => Some(
                self.padding
                    .map(|p| DurFrame::new(self.at, p.top, self.ease)),
            ),
            3 => Some(
                self.padding
                    .map(|p| DurFrame::new(self.at, p.right, self.ease)),
            ),
            4 => Some(
                self.padding
                    .map(|p| DurFrame::new(self.at, p.bottom, self.ease)),
            ),
            5 => Some(
                self.padding
                    .map(|p| DurFrame::new(self.at, p.left, self.ease)),
            ),
            6 => Some(self.max_width.map(|w| DurFrame::new(self.at, w, self.ease))),
            7 => Some(
                self.max_height
                    .map(|h| DurFrame::new(self.at, h, self.ease)),
            ),
            8 => Some(
                self.style
                    .map(|s| DurFrame::new(self.at, s as f32, self.ease)),
            ),
            _ => None,
        }
    }
}

impl ExactSizeIterator for StyleContainer {
    fn len(&self) -> usize {
        9 - self.index
    }
}
