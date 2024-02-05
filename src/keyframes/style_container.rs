use crate::reexports::iced_core::{
    widget::Id as IcedId, Element, Length, Padding, Pixels, Renderer as IcedRenderer,
};
use crate::reexports::iced_style::container::StyleSheet;

use crate::keyframes::{as_f32, get_length, Repeat};
use crate::timeline::{Frame, Interped};
use crate::{Ease, Linear, MovementType};

/// A `StyleContainer`'s animation Id. Used for linking animation built in `update()` with widget output in `view()`
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
    pub fn into_chain_with_children(self, children: Vec<StyleContainer>) -> Chain {
        Chain::with_children(self, children)
    }

    #[cfg(feature = "iced")]
    /// Used by [`crate::anim!`] macro
    pub fn as_widget<'a, Message, Theme, Renderer>(
        self,
        style: fn(u8) -> <Theme as StyleSheet>::Style,
        timeline: &crate::Timeline,
        content: impl Into<Element<'a, Message, Theme, Renderer>>,
    ) -> crate::widget::Container<'a, Message, Theme, Renderer>
    where
        Renderer: IcedRenderer,
        Theme: StyleSheet,
    {
        StyleContainer::as_widget(self, style, timeline, content)
    }

    #[cfg(feature = "libcosmic")]
    /// Used by [`crate::anim!`] macro
    pub fn as_widget<'a, Message, Theme, Renderer>(
        self,
        style: fn(u8) -> <Theme as StyleSheet>::Style,
        timeline: &crate::Timeline,
        content: impl Into<Element<'a, Message, Theme, Renderer>>,
    ) -> crate::widget::Container<'a, Message, Theme, Renderer>
    where
        Renderer: IcedRenderer,
        Theme: StyleSheet + cosmic::cosmic_theme::LayeredTheme,
        <Theme as crate::reexports::iced_style::container::StyleSheet>::Style:
            From<cosmic::theme::Container>,
    {
        StyleContainer::as_widget(self, style, timeline, content)
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

    pub fn with_children(id: Id, children: Vec<StyleContainer>) -> Self {
        Chain {
            id,
            links: children,
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
pub struct StyleContainer {
    at: MovementType,
    ease: Ease,
    width: Option<Length>,
    height: Option<Length>,
    padding: Option<Padding>,
    max_width: Option<f32>,
    max_height: Option<f32>,
    style: Option<u8>,
    is_eager: bool,
}

impl StyleContainer {
    pub fn new(at: impl Into<MovementType>) -> StyleContainer {
        let at = at.into();
        StyleContainer {
            at,
            ease: Linear::InOut.into(),
            width: None,
            height: None,
            padding: None,
            max_width: None,
            max_height: None,
            style: None,
            is_eager: true,
        }
    }

    pub fn lazy(at: impl Into<MovementType>) -> StyleContainer {
        let at = at.into();
        StyleContainer {
            at,
            ease: Linear::InOut.into(),
            width: None,
            height: None,
            padding: None,
            max_width: None,
            max_height: None,
            style: None,
            is_eager: false,
        }
    }

    // Returns a cosmic-time container, not a default iced button. The difference shouldn't
    // matter to the end user. Though it is an implementation detail.
    #[cfg(feature = "iced")]
    pub fn as_widget<'a, Message, Theme, Renderer>(
        id: Id,
        style: fn(u8) -> <Theme as StyleSheet>::Style,
        timeline: &crate::Timeline,
        content: impl Into<Element<'a, Message, Theme, Renderer>>,
    ) -> crate::widget::Container<'a, Message, Theme, Renderer>
    where
        Renderer: IcedRenderer,
        Theme: StyleSheet,
    {
        let id: IcedId = id.into();

        let container = crate::widget::Container::new(content)
            .width(get_length(&id, timeline, 0, Length::Shrink))
            .height(get_length(&id, timeline, 1, Length::Shrink))
            .padding([
                timeline.get(&id, 2).map_or(0., |m| m.value),
                timeline.get(&id, 3).map_or(0., |m| m.value),
                timeline.get(&id, 4).map_or(0., |m| m.value),
                timeline.get(&id, 5).map_or(0., |m| m.value),
            ])
            .max_width(timeline.get(&id, 6).map_or(f32::INFINITY, |m| m.value))
            .max_height(timeline.get(&id, 7).map_or(f32::INFINITY, |m| m.value));

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

    #[cfg(feature = "libcosmic")]
    pub fn as_widget<'a, Message, Theme, Renderer>(
        id: Id,
        style: fn(u8) -> <Theme as StyleSheet>::Style,
        timeline: &crate::Timeline,
        content: impl Into<Element<'a, Message, Theme, Renderer>>,
    ) -> crate::widget::Container<'a, Message, Theme, Renderer>
    where
        Renderer: IcedRenderer,
        Theme: StyleSheet + cosmic::cosmic_theme::LayeredTheme,
        <Theme as crate::reexports::iced_style::container::StyleSheet>::Style:
            From<cosmic::theme::Container>,
    {
        let id: IcedId = id.into();

        let container = crate::widget::Container::new(content)
            .width(get_length(&id, timeline, 0, Length::Shrink))
            .height(get_length(&id, timeline, 1, Length::Shrink))
            .padding([
                timeline.get(&id, 2).map_or(0., |m| m.value),
                timeline.get(&id, 3).map_or(0., |m| m.value),
                timeline.get(&id, 4).map_or(0., |m| m.value),
                timeline.get(&id, 5).map_or(0., |m| m.value),
            ])
            .max_width(timeline.get(&id, 6).map_or(f32::INFINITY, |m| m.value))
            .max_height(timeline.get(&id, 7).map_or(f32::INFINITY, |m| m.value));

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

    pub fn style(mut self, style: impl Into<u8>) -> Self {
        let style = style.into();
        self.style = Some(style);
        self
    }
}

#[rustfmt::skip]
impl From<StyleContainer> for Vec<Option<Frame>> {
    fn from(container: StyleContainer) -> Vec<Option<Frame>> {
      if container.is_eager {
        vec![as_f32(container.width).map(|w| Frame::eager(container.at, w, container.ease)),  // 0 = width
             as_f32(container.height).map(|h| Frame::eager(container.at, h, container.ease)), // 1 = height
             container.padding.map(|p| Frame::eager(container.at, p.top, container.ease)),    // 2 = padding[0] (top)
             container.padding.map(|p| Frame::eager(container.at, p.right, container.ease)),  // 3 = padding[1] (right)
             container.padding.map(|p| Frame::eager(container.at, p.bottom, container.ease)), // 4 = padding[2] (bottom)
             container.padding.map(|p| Frame::eager(container.at, p.left, container.ease)),   // 5 = padding[3] (left)
             container.max_width.map(|w| Frame::eager(container.at, w, container.ease)),      // 6 = max_width
             container.max_height.map(|h| Frame::eager(container.at, h, container.ease)),     // 7 = max_height
             container.style.map(|s| Frame::eager(container.at, f32::from(s), container.ease)),   // 6 = style blend (passed to widget to mix values at `draw` time)
        ]
      } else {
        vec![Some(Frame::lazy(container.at, 0., container.ease)); 9] // lazy evaluates for all values
      }
    }
}
