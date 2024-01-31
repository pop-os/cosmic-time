//! Show toggle controls using togglers.

use cosmic::iced_core::Border;
use iced_core::{
    alignment, event, layout, mouse, renderer, text,
    widget::{self, tree, Tree},
    Clipboard, Element, Event, Layout, Length, Pixels, Rectangle, Shell, Size, Widget,
};

use crate::{
    chain, id, lerp,
    reexports::{iced, iced_core, iced_widget},
};
pub use cosmic::iced_style::toggler::{Appearance, StyleSheet};

/// A toggler widget.
#[allow(missing_debug_implementations)]
pub struct Toggler<'a, Message, Renderer = cosmic::iced::Renderer>
where
    Renderer: text::Renderer,
{
    id: id::Toggler,
    is_toggled: bool,
    on_toggle: Box<dyn Fn(chain::Toggler, bool) -> Message + 'a>,
    label: Option<String>,
    width: Length,
    size: f32,
    text_size: Option<f32>,
    text_line_height: text::LineHeight,
    text_alignment: alignment::Horizontal,
    text_shaping: text::Shaping,
    spacing: f32,
    font: Option<Renderer::Font>,
    style: <cosmic::Theme as StyleSheet>::Style,
    percent: f32,
    anim_multiplier: f32,
}

impl<'a, Message, Renderer> Toggler<'a, Message, Renderer>
where
    Renderer: text::Renderer,
{
    /// The default size of a [`Toggler`].
    pub const DEFAULT_SIZE: f32 = 24.0;

    /// Creates a new [`Toggler`].
    ///
    /// It expects:
    ///   * a boolean describing whether the [`Toggler`] is checked or not
    ///   * An optional label for the [`Toggler`]
    ///   * a function that will be called when the [`Toggler`] is toggled. It
    ///     will receive the new state of the [`Toggler`] and must produce a
    ///     `Message`.
    pub fn new<F>(id: id::Toggler, label: impl Into<Option<String>>, is_toggled: bool, f: F) -> Self
    where
        F: 'a + Fn(chain::Toggler, bool) -> Message,
    {
        Toggler {
            id,
            is_toggled,
            on_toggle: Box::new(f),
            label: label.into(),
            width: Length::Fill,
            size: Self::DEFAULT_SIZE,
            text_size: None,
            text_line_height: text::LineHeight::default(),
            text_alignment: alignment::Horizontal::Left,
            text_shaping: text::Shaping::Advanced,
            spacing: 0.0,
            font: None,
            style: Default::default(),
            percent: if is_toggled { 1.0 } else { 0.0 },
            anim_multiplier: 1.0,
        }
    }

    /// Sets the size of the [`Toggler`].
    pub fn size(mut self, size: impl Into<Pixels>) -> Self {
        self.size = size.into().0;
        self
    }

    /// Sets the width of the [`Toggler`].
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the text size o the [`Toggler`].
    pub fn text_size(mut self, text_size: impl Into<Pixels>) -> Self {
        self.text_size = Some(text_size.into().0);
        self
    }

    /// Sets the text [`LineHeight`] of the [`Toggler`].
    pub fn text_line_height(mut self, line_height: impl Into<text::LineHeight>) -> Self {
        self.text_line_height = line_height.into();
        self
    }

    /// Sets the horizontal alignment of the text of the [`Toggler`]
    pub fn text_alignment(mut self, alignment: alignment::Horizontal) -> Self {
        self.text_alignment = alignment;
        self
    }

    /// Sets the [`text::Shaping`] strategy of the [`Toggler`].
    pub fn text_shaping(mut self, shaping: text::Shaping) -> Self {
        self.text_shaping = shaping;
        self
    }

    /// Sets the spacing between the [`Toggler`] and the text.
    pub fn spacing(mut self, spacing: impl Into<Pixels>) -> Self {
        self.spacing = spacing.into().0;
        self
    }

    /// Sets the [`Font`] of the text of the [`Toggler`]
    ///
    /// [`Font`]: cosmic::iced::text::Renderer::Font
    pub fn font(mut self, font: impl Into<Renderer::Font>) -> Self {
        self.font = Some(font.into());
        self
    }

    /// Sets the style of the [`Toggler`].
    pub fn style(mut self, style: impl Into<<cosmic::Theme as StyleSheet>::Style>) -> Self {
        self.style = style.into();
        self
    }

    /// The percent completion of the toggler animation.
    /// This is indented to automated cosmic-time use, and shouldn't
    /// need to be called manually.
    pub fn percent(mut self, percent: f32) -> Self {
        self.percent = percent;
        self
    }
}

impl<'a, Message, Renderer> Widget<Message, cosmic::Theme, Renderer>
    for Toggler<'a, Message, Renderer>
where
    Renderer: text::Renderer,
{
    fn size(&self) -> Size<Length> {
        Size::new(self.width, Length::Shrink)
    }

    fn state(&self) -> tree::State {
        tree::State::new(widget::text::State::<Renderer::Paragraph>::default())
    }

    fn layout(
        &self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let limits = limits.width(self.width);

        crate::utils::next_to_each_other(
            &limits,
            self.spacing,
            |limits| {
                if let Some(label) = self.label.as_deref() {
                    let state = tree
                        .state
                        .downcast_mut::<iced_widget::text::State<Renderer::Paragraph>>();

                    let node = iced_core::widget::text::layout(
                        state,
                        renderer,
                        limits,
                        self.width,
                        Length::Shrink,
                        label,
                        self.text_line_height,
                        self.text_size.map(iced::Pixels),
                        self.font,
                        self.text_alignment,
                        alignment::Vertical::Top,
                        self.text_shaping,
                    );
                    match self.width {
                        Length::Fill => {
                            let size = node.size();
                            layout::Node::with_children(
                                Size::new(limits.width(Length::Fill).max().width, size.height),
                                vec![node],
                            )
                        }
                        _ => node,
                    }
                } else {
                    layout::Node::new(iced_core::Size::ZERO)
                }
            },
            |_| layout::Node::new(iced_core::Size::new(2.0 * self.size, self.size)),
        )
    }

    fn on_event(
        &mut self,
        _state: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor_position: mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) -> event::Status {
        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                let mouse_over = cursor_position.is_over(layout.bounds());

                if mouse_over {
                    if self.is_toggled {
                        let off_animation =
                            chain::Toggler::off(self.id.clone(), self.anim_multiplier);
                        shell.publish((self.on_toggle)(off_animation, !self.is_toggled));
                    } else {
                        let on_animation =
                            chain::Toggler::on(self.id.clone(), self.anim_multiplier);
                        shell.publish((self.on_toggle)(on_animation, !self.is_toggled));
                    }

                    event::Status::Captured
                } else {
                    event::Status::Ignored
                }
            }
            _ => event::Status::Ignored,
        }
    }

    fn mouse_interaction(
        &self,
        _state: &Tree,
        layout: Layout<'_>,
        cursor_position: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        if cursor_position.is_over(layout.bounds()) {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::default()
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &cosmic::Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor_position: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        /// Makes sure that the border radius of the toggler looks good at every size.
        const BORDER_RADIUS_RATIO: f32 = 32.0 / 13.0;

        /// The space ratio between the background Quad and the Toggler bounds, and
        /// between the background Quad and foreground Quad.
        const SPACE_RATIO: f32 = 0.05;

        let mut children = layout.children();

        if let Some(_label) = &self.label {
            let label_layout = children.next().unwrap();

            iced_widget::text::draw(
                renderer,
                style,
                label_layout,
                tree.state.downcast_ref(),
                iced_widget::text::Appearance::default(),
                viewport,
            );
        }

        let toggler_layout = children.next().unwrap();
        let bounds = toggler_layout.bounds();

        let is_mouse_over = cursor_position.is_over(bounds);

        let style = if is_mouse_over {
            blend_appearances(
                theme.hovered(&self.style, false),
                theme.hovered(&self.style, true),
                self.percent,
            )
        } else {
            blend_appearances(
                theme.active(&self.style, false),
                theme.active(&self.style, true),
                self.percent,
            )
        };

        let border_radius = bounds.height / BORDER_RADIUS_RATIO;
        let space = SPACE_RATIO * bounds.height;

        let toggler_background_bounds = Rectangle {
            x: bounds.x + space,
            y: bounds.y + space,
            width: bounds.width - (2.0 * space),
            height: bounds.height - (2.0 * space),
        };

        renderer.fill_quad(
            renderer::Quad {
                bounds: toggler_background_bounds,
                border: Border {
                    width: 1.0,
                    color: style.background_border.unwrap_or(style.background),
                    radius: border_radius.into(),
                },
                shadow: Default::default(),
            },
            style.background,
        );

        let toggler_foreground_bounds = Rectangle {
            x: bounds.x
                + lerp(
                    2.0 * space,
                    bounds.width - 2.0 * space - (bounds.height - (4.0 * space)),
                    self.percent,
                ),
            y: bounds.y + (2.0 * space),
            width: bounds.height - (4.0 * space),
            height: bounds.height - (4.0 * space),
        };

        renderer.fill_quad(
            renderer::Quad {
                bounds: toggler_foreground_bounds,
                border: Border {
                    width: 1.0,
                    color: style.foreground_border.unwrap_or(style.foreground),
                    radius: border_radius.into(),
                },
                shadow: Default::default(),
            },
            style.foreground,
        );
    }
}

impl<'a, Message, Renderer> From<Toggler<'a, Message, Renderer>>
    for Element<'a, Message, cosmic::Theme, Renderer>
where
    Message: 'a,
    Renderer: 'a + text::Renderer,
{
    fn from(
        toggler: Toggler<'a, Message, Renderer>,
    ) -> Element<'a, Message, cosmic::Theme, Renderer> {
        Element::new(toggler)
    }
}

fn blend_appearances(first: Appearance, mut other: Appearance, percent: f32) -> Appearance {
    if percent == 0. {
        first
    } else if percent == 1. {
        other
    } else {
        let first_background = first.background.into_linear();

        let other_background = std::mem::take(&mut other.background).into_linear();

        other.background = crate::utils::static_array_from_iter::<f32, 4>(
            first_background
                .iter()
                .zip(other_background.iter())
                .map(|(o, t)| o * (1.0 - percent) + t * percent),
        )
        .into();

        other
    }
}
