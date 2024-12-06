use cosmic::iced_core::widget::Id as IcedId;
use cosmic::widget::icon::Handle;
use cosmic::Element;

use crate::keyframes::Repeat;
use crate::timeline::Frame;
use crate::{cards, chain, lazy::cards as lazy, Duration, Ease, Linear, MovementType};

/// A Cards's animation Id. Used for linking animation built in `update()` with widget output in `view()`
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
    pub fn into_chain_with_children(self, children: Vec<Cards>) -> Chain {
        Chain::with_children(self, children)
    }

    /// Used by [`crate::anim!`] macro
    #[allow(clippy::too_many_arguments)]
    pub fn as_widget<'a, Message, F, G>(
        self,
        timeline: &crate::Timeline,
        card_inner_elements: Vec<Element<'a, Message>>,
        on_clear_all: Message,
        on_show_more: Option<F>,
        on_activate: Option<G>,
        show_more_label: &'a str,
        show_less_label: &'a str,
        clear_all_label: &'a str,
        show_less_icon: Option<Handle>,
        expanded: bool,
    ) -> crate::widget::Cards<'a, Message, cosmic::Renderer>
    where
        F: 'a + Fn(Chain, bool) -> Message,
        G: 'a + Fn(usize) -> Message,

        Message: 'static + Clone,
    {
        Cards::as_widget(
            self,
            timeline,
            card_inner_elements,
            on_clear_all,
            on_show_more,
            on_activate,
            show_more_label,
            show_less_label,
            clear_all_label,
            show_less_icon,
            expanded,
        )
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
    links: Vec<Cards>,
    repeat: Repeat,
}

impl Chain {
    /// Crate a new [`Cards`] animation chain.
    /// You probably don't want to use use directly, and should
    /// use the [`chain`] macro.
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
    /// use the [`chain`] macro.
    #[must_use]
    pub fn with_children(id: Id, children: Vec<Cards>) -> Self {
        Chain {
            id,
            links: children,
            repeat: Repeat::Never,
        }
    }

    /// Link another keyframe, (very similar to push)
    /// You probably don't want to use use directly, and should
    /// use the [`chain`] macro.
    #[must_use]
    pub fn link(mut self, toggler: Cards) -> Self {
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

    /// Returns the default animation for animating the cards to "on"
    #[must_use]
    pub fn on(id: Id, anim_multiplier: f32) -> Self {
        let duration = (ANIM_DURATION * anim_multiplier.round()) as u64;
        chain!(
            id,
            lazy(Duration::ZERO),
            cards(Duration::from_millis(duration)).percent(1.0),
        )
    }

    /// Returns the default animation for animating the cards to "off"
    #[must_use]
    pub fn off(id: Id, anim_multiplier: f32) -> Self {
        let duration = (ANIM_DURATION * anim_multiplier.round()) as u64;
        chain!(
            id,
            lazy(Duration::ZERO),
            cards(Duration::from_millis(duration)).percent(0.0),
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
pub struct Cards {
    at: MovementType,
    ease: Ease,
    percent: f32,
    is_eager: bool,
}

impl Cards {
    pub fn new(at: impl Into<MovementType>) -> Cards {
        let at = at.into();
        Cards {
            at,
            ease: Linear::InOut.into(),
            percent: 1.0,
            is_eager: true,
        }
    }

    pub fn lazy(at: impl Into<MovementType>) -> Cards {
        let at = at.into();
        Cards {
            at,
            ease: Linear::InOut.into(),
            percent: 1.0,
            is_eager: false,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn as_widget<'a, Message, F, G>(
        id: Id,
        timeline: &crate::Timeline,
        card_inner_elements: Vec<Element<'a, Message>>,
        on_clear_all: Message,
        on_show_more: Option<F>,
        on_activate: Option<G>,
        show_more_label: &'a str,
        show_less_label: &'a str,
        clear_all_label: &'a str,
        show_less_icon: Option<Handle>,
        expanded: bool,
    ) -> crate::widget::Cards<'a, Message, cosmic::Renderer>
    where
        F: 'a + Fn(Chain, bool) -> Message,
        G: 'a + Fn(usize) -> Message,

        Message: Clone + 'static,
    {
        crate::widget::Cards::new(
            id.clone(),
            card_inner_elements,
            on_clear_all,
            on_show_more,
            on_activate,
            show_more_label,
            show_less_label,
            clear_all_label,
            show_less_icon,
            expanded,
        )
        .percent(
            timeline
                .get(&id.into(), 0)
                .map_or(if expanded { 1.0 } else { 0.0 }, |m| m.value),
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
impl From<Cards> for Vec<Option<Frame>> {
    fn from(cards: Cards) -> Vec<Option<Frame>> {
      if cards.is_eager {
        vec![Some(Frame::eager(cards.at, cards.percent, cards.ease))]  // 0 = animation percent completion
      } else {
        vec![Some(Frame::lazy(cards.at, 0., cards.ease))] // lazy evaluates for all values
      }
    }
}
