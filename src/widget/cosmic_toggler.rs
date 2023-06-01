//! Show toggle controls using togglers.
use cosmic::iced_core::alignment;
use cosmic::iced_core::event;
use cosmic::iced_core::layout;
use cosmic::iced_core::mouse;
use cosmic::iced_core::renderer;
use cosmic::iced_core::text;
use cosmic::iced_core::time::Duration;
use cosmic::iced_core::widget::Tree;
use cosmic::iced_core::{
    Alignment, Clipboard, Element, Event, Layout, Length, Pixels, Point, Rectangle, Shell, Widget,
};
use cosmic::iced_widget::{Row, Text};

use crate::keyframes::{self, toggler::Chain};
pub use cosmic::iced_style::toggler::{Appearance, StyleSheet};

/// The default animation duration. Change here for custom widgets. Or at runtime with `.anim_multiplier`
const ANIM_DURATION: f32 = 100.;

/// A toggler widget.
///
/// # Example
///
/// ```no_run
/// # type Toggler<'a, Message> =
/// #     iced_widget::Toggler<'a, Message, iced_widget::renderer::Renderer<iced_widget::style::Theme>>;
/// #
/// pub enum Message {
///     TogglerToggled(bool),
/// }
///
/// let is_toggled = true;
///
/// Toggler::new(String::from("Toggle me!"), is_toggled, |b| Message::TogglerToggled(b));
/// ```
#[allow(missing_debug_implementations)]
pub struct Toggler<'a, Message, Renderer = cosmic::iced::Renderer>
where
    Renderer: text::Renderer,
    Renderer::Theme: StyleSheet,
{
    id: crate::keyframes::toggler::Id,
    is_toggled: bool,
    on_toggle: Box<dyn Fn(Chain, bool) -> Message + 'a>,
    label: Option<String>,
    width: Length,
    size: f32,
    text_size: Option<f32>,
    text_line_height: text::LineHeight,
    text_alignment: alignment::Horizontal,
    text_shaping: text::Shaping,
    spacing: f32,
    font: Option<Renderer::Font>,
    style: <Renderer::Theme as StyleSheet>::Style,
    percent: f32,
    anim_multiplier: f32,
}

impl<'a, Message, Renderer> Toggler<'a, Message, Renderer>
where
    Renderer: text::Renderer,
    Renderer::Theme: StyleSheet,
{
    /// The default size of a [`Toggler`].
    pub const DEFAULT_SIZE: f32 = 20.0;

    /// Creates a new [`Toggler`].
    ///
    /// It expects:
    ///   * a boolean describing whether the [`Toggler`] is checked or not
    ///   * An optional label for the [`Toggler`]
    ///   * a function that will be called when the [`Toggler`] is toggled. It
    ///     will receive the new state of the [`Toggler`] and must produce a
    ///     `Message`.
    pub fn new<F>(
        id: crate::keyframes::toggler::Id,
        label: impl Into<Option<String>>,
        is_toggled: bool,
        f: F,
    ) -> Self
    where
        F: 'a + Fn(Chain, bool) -> Message,
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
            text_shaping: text::Shaping::Basic,
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
    pub fn style(mut self, style: impl Into<<Renderer::Theme as StyleSheet>::Style>) -> Self {
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

impl<'a, Message, Renderer> Widget<Message, Renderer> for Toggler<'a, Message, Renderer>
where
    Renderer: text::Renderer,
    Renderer::Theme: StyleSheet + cosmic::iced_widget::text::StyleSheet,
{
    fn width(&self) -> Length {
        self.width
    }

    fn height(&self) -> Length {
        Length::Shrink
    }

    fn layout(&self, renderer: &Renderer, limits: &layout::Limits) -> layout::Node {
        let mut row = Row::<(), Renderer>::new()
            .width(self.width)
            .spacing(self.spacing)
            .align_items(Alignment::Center);

        if let Some(label) = &self.label {
            row = row.push(
                Text::new(label)
                    .horizontal_alignment(self.text_alignment)
                    .font(self.font.unwrap_or_else(|| renderer.default_font()))
                    .width(self.width)
                    .size(self.text_size.unwrap_or_else(|| renderer.default_size()))
                    .line_height(self.text_line_height)
                    .shaping(self.text_shaping),
            );
        }

        row = row.push(Row::new().width(2.0 * self.size).height(self.size));

        row.layout(renderer, limits)
    }

    fn on_event(
        &mut self,
        _state: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
    ) -> event::Status {
        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                let mouse_over = layout.bounds().contains(cursor_position);

                // To prevent broken animations, only send message if
                // toggler is clicked after animation is finished.
                // TODO this should be possible to fix once redirectable
                // animations are implemented.
                if mouse_over && (self.percent == 0.0 || self.percent == 1.0) {
                    if self.is_toggled {
                        let off_animation = Chain::new(self.id.clone())
                            .link(keyframes::toggler::Toggler::new(Duration::ZERO).percent(1.0))
                            .link(
                                keyframes::toggler::Toggler::new(Duration::from_millis(
                                    (ANIM_DURATION * self.anim_multiplier.round()) as u64,
                                ))
                                .percent(0.0),
                            );
                        shell.publish((self.on_toggle)(off_animation, !self.is_toggled));
                    } else {
                        let on_animation = Chain::new(self.id.clone())
                            .link(keyframes::toggler::Toggler::new(Duration::ZERO).percent(0.0))
                            .link(
                                keyframes::toggler::Toggler::new(Duration::from_millis(
                                    (ANIM_DURATION * self.anim_multiplier.round()) as u64,
                                ))
                                .percent(1.0),
                            );
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
        cursor_position: Point,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        if layout.bounds().contains(cursor_position) {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::default()
        }
    }

    fn draw(
        &self,
        _state: &Tree,
        renderer: &mut Renderer,
        theme: &Renderer::Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor_position: Point,
        _viewport: &Rectangle,
    ) {
        /// Makes sure that the border radius of the toggler looks good at every size.
        const BORDER_RADIUS_RATIO: f32 = 32.0 / 13.0;

        /// The space ratio between the background Quad and the Toggler bounds, and
        /// between the background Quad and foreground Quad.
        const SPACE_RATIO: f32 = 0.05;

        let mut children = layout.children();

        if let Some(label) = &self.label {
            let label_layout = children.next().unwrap();

            cosmic::iced_widget::text::draw(
                renderer,
                style,
                label_layout,
                label,
                self.text_size,
                self.text_line_height,
                self.font,
                Default::default(),
                self.text_alignment,
                alignment::Vertical::Center,
                self.text_shaping,
            );
        }

        let toggler_layout = children.next().unwrap();
        let bounds = toggler_layout.bounds();

        let is_mouse_over = bounds.contains(cursor_position);

        let style = if is_mouse_over {
            theme.hovered(&self.style, self.is_toggled)
        } else {
            theme.active(&self.style, self.is_toggled)
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
                border_radius: border_radius.into(),
                border_width: 1.0,
                border_color: style.background_border.unwrap_or(style.background),
            },
            style.background,
        );

        let toggler_foreground_bounds = Rectangle {
            x: bounds.x
                + if self.is_toggled {
                    bounds.width - 2.0 * space - (bounds.height - (4.0 * space))
                } else {
                    2.0 * space
                },
            y: bounds.y + (2.0 * space),
            width: bounds.height - (4.0 * space),
            height: bounds.height - (4.0 * space),
        };

        renderer.fill_quad(
            renderer::Quad {
                bounds: toggler_foreground_bounds,
                border_radius: border_radius.into(),
                border_width: 1.0,
                border_color: style.foreground_border.unwrap_or(style.foreground),
            },
            style.foreground,
        );
    }
}

impl<'a, Message, Renderer> From<Toggler<'a, Message, Renderer>> for Element<'a, Message, Renderer>
where
    Message: 'a,
    Renderer: 'a + text::Renderer,
    Renderer::Theme: StyleSheet + cosmic::iced_widget::text::StyleSheet,
{
    fn from(toggler: Toggler<'a, Message, Renderer>) -> Element<'a, Message, Renderer> {
        Element::new(toggler)
    }
}
