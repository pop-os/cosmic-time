use iced_native::widget;

use crate::keyframes::Repeat;
use crate::timeline::Frame;
use crate::{Ease, Linear, MovementType};

/// A Toggler's animation Id. Used for linking animation built in `update()` with widget output in `view()`
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

#[derive(Debug, Clone)]
pub struct Chain {
    id: Id,
    links: Vec<Toggler>,
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

    pub fn link(mut self, mut toggler: Toggler) -> Self {
        toggler.chain_index = self.links.len();
        self.links.push(toggler);
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
    T: ExactSizeIterator<Item = Option<Frame>> + std::fmt::Debug,
    Vec<T>: From<Vec<Toggler>>,
{
    fn from(chain: Chain) -> Self {
        crate::timeline::Chain::new(chain.id.into(), chain.repeat, chain.links.into())
    }
}

#[must_use = "Keyframes are intended to be used in an animation chain."]
#[derive(Debug, Clone, Copy)]
pub struct Toggler {
    index: usize,
    chain_index: usize,
    at: MovementType,
    ease: Ease,
    percent: f32,
}

impl Toggler {
    pub fn new(at: impl Into<MovementType>) -> Toggler {
        let at = at.into();
        Toggler {
            index: 0,
            chain_index: 0,
            at,
            ease: Linear::InOut.into(),
            percent: 1.0,
        }
    }

    pub fn as_widget<'a, Message, Renderer, F>(
        id: Id,
        timeline: &crate::Timeline,
        label: impl Into<Option<String>>,
        is_toggled: bool,
        f: F,
    ) -> crate::widget::Toggler<'a, Message, Renderer>
    where
        Renderer: iced_native::Renderer + iced_native::text::Renderer,
        Renderer::Theme: widget::toggler::StyleSheet,
        F: 'a + Fn(Chain, bool) -> Message,
    {
        crate::widget::Toggler::new(id.clone(), label, is_toggled, f).percent(
            timeline
                .get(&id.into(), 0)
                .map(|m| m.value)
                .unwrap_or(if is_toggled { 1.0 } else { 0.0 }),
        )
    }

    pub fn percent(mut self, percent: f32) -> Self {
        self.percent = percent;
        self
    }

    pub fn ease<E: Into<Ease>>(mut self, ease: E) -> Self {
        self.ease = ease.into();
        self
    }
}

// 0 = animation percent completion
impl Iterator for Toggler {
    type Item = Option<Frame>;

    fn next(&mut self) -> Option<Option<Frame>> {
        self.index += 1;
        match self.index - 1 {
            0 => Some(Some(Frame::eager(
                self.chain_index,
                self.at,
                self.percent,
                self.ease,
            ))),
            _ => None,
        }
    }
}

impl ExactSizeIterator for Toggler {
    fn len(&self) -> usize {
        1 - self.index
    }
}
