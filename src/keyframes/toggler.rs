use crate::reexports::iced_core::{text, widget::Id as IcedId, Renderer as IcedRenderer};

use crate::keyframes::Repeat;
use crate::timeline::Frame;
use crate::{chain, lazy::toggler as lazy, toggler, Duration, Ease, Linear, MovementType};

/// A Toggler's animation Id. Used for linking animation built in `update()` with widget output in `view()`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Id(IcedId);
const ANIM_DURATION: f32 = 100.;

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

    /// Used by [`chain!`] macro
    #[must_use]
    pub fn into_chain(self) -> Chain {
        Chain::new(self)
    }

    /// Used by [`chain!`] macro
    #[must_use]
    pub fn into_chain_with_children(self, children: Vec<Toggler>) -> Chain {
        Chain::with_children(self, children)
    }

    /// Used by [`crate::anim!`] macro
    pub fn as_widget<'a, Message, Renderer, F>(
        self,
        timeline: &crate::Timeline,
        label: impl Into<Option<String>>,
        is_toggled: bool,
        f: F,
    ) -> crate::widget::Toggler<'a, Message, Renderer>
    where
        Renderer: IcedRenderer + text::Renderer,
        F: 'a + Fn(Chain, bool) -> Message,
    {
        Toggler::as_widget(self, timeline, label, is_toggled, f)
    }
}

impl From<Id> for IcedId {
    fn from(id: Id) -> Self {
        id.0
    }
}

#[derive(Debug, Clone)]
/// An animation, where each keyframe is "chained" together.
pub struct Chain {
    id: Id,
    links: Vec<Toggler>,
    repeat: Repeat,
}

impl Chain {
    /// Crate a new Toggler animation chain.
    /// You probably don't want to use use directly, and should
    /// use the [`chain!`] macro.
    #[must_use]
    pub fn new(id: Id) -> Self {
        Chain {
            id,
            links: Vec::new(),
            repeat: Repeat::Never,
        }
    }

    /// Create a chain pre-fulled with children.
    /// You probably don't want to use use directly, and should
    /// use the [`chain!`] macro.
    #[must_use]
    pub fn with_children(id: Id, children: Vec<Toggler>) -> Self {
        Chain {
            id,
            links: children,
            repeat: Repeat::Never,
        }
    }

    /// Link another keyframe, (very similar to push)
    /// You probably don't want to use use directly, and should
    /// use the [`chain!`] macro.
    #[must_use]
    pub fn link(mut self, toggler: Toggler) -> Self {
        self.links.push(toggler);
        self
    }

    /// Sets the animation to loop forever.
    #[must_use]
    pub fn loop_forever(mut self) -> Self {
        self.repeat = Repeat::Forever;
        self
    }

    /// Sets the animation to only loop once.
    /// This is the default, and only useful to
    /// stop an animation that was previously set
    /// to loop forever.
    #[must_use]
    pub fn loop_once(mut self) -> Self {
        self.repeat = Repeat::Never;
        self
    }

    /// Returns the default animation for animating the toggler to "on"
    #[must_use]
    pub fn on(id: Id, anim_multiplier: f32) -> Self {
        let duration = (ANIM_DURATION * anim_multiplier.round()) as u64;
        chain!(
            id,
            lazy(Duration::ZERO),
            toggler(Duration::from_millis(duration)).percent(1.0),
        )
    }

    /// Returns the default animation for animating the toggler to "off"
    #[must_use]
    pub fn off(id: Id, anim_multiplier: f32) -> Self {
        let duration = (ANIM_DURATION * anim_multiplier.round()) as u64;
        chain!(
            id,
            lazy(Duration::ZERO),
            toggler(Duration::from_millis(duration)).percent(0.0),
        )
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
pub struct Toggler {
    at: MovementType,
    ease: Ease,
    percent: f32,
    is_eager: bool,
}

impl Toggler {
    pub fn new(at: impl Into<MovementType>) -> Toggler {
        let at = at.into();
        Toggler {
            at,
            ease: Linear::InOut.into(),
            percent: 1.0,
            is_eager: true,
        }
    }

    pub fn lazy(at: impl Into<MovementType>) -> Toggler {
        let at = at.into();
        Toggler {
            at,
            ease: Linear::InOut.into(),
            percent: 1.0,
            is_eager: false,
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
        Renderer: IcedRenderer + text::Renderer,
        F: 'a + Fn(Chain, bool) -> Message,
    {
        crate::widget::Toggler::new(id.clone(), label, is_toggled, f).percent(
            timeline
                .get(&id.into(), 0)
                .map_or(if is_toggled { 1.0 } else { 0.0 }, |m| m.value),
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

#[rustfmt::skip]
impl From<Toggler> for Vec<Option<Frame>> {
    fn from(toggler: Toggler) -> Vec<Option<Frame>> {
      if toggler.is_eager {
        vec![Some(Frame::eager(toggler.at, toggler.percent, toggler.ease))]  // 0 = animation percent completion
      } else {
        vec![Some(Frame::lazy(toggler.at, 0., toggler.ease))] // lazy evaluates for all values
      }
    }
}
