//! Allow your users to perform actions by pressing a button.
//!
//! A [`Button`] has some local [`State`].
use crate::widget::StyleType;
use iced_core::event::{self, Event};
use iced_core::layout;
use iced_core::mouse;
use iced_core::overlay;
use iced_core::renderer;
use iced_core::touch;
use iced_core::widget::tree::{self, Tree};
use iced_core::widget::Operation;
use iced_core::{
    Background, Clipboard, Color, Element, Layout, Length, Padding, Point, Rectangle, Shell,
    Vector, Widget,
};

pub use iced_style::button::{Appearance, StyleSheet};

use super::button_blend_appearances;

/// A generic widget that produces a message when pressed.
///
#[allow(missing_debug_implementations)]
pub struct Button<'a, Message, Renderer>
where
    Renderer: iced_core::renderer::Renderer,
    Renderer::Theme: StyleSheet,
{
    content: Element<'a, Message, Renderer>,
    on_press: Option<Message>,
    width: Length,
    height: Length,
    padding: Padding,
    style: StyleType<<Renderer::Theme as StyleSheet>::Style>,
}

impl<'a, Message, Renderer> Button<'a, Message, Renderer>
where
    Renderer: iced_core::renderer::Renderer,
    Renderer::Theme: StyleSheet,
{
    /// Creates a new [`Button`] with the given content.
    pub fn new(content: impl Into<Element<'a, Message, Renderer>>) -> Self {
        Button {
            content: content.into(),
            on_press: None,
            width: Length::Shrink,
            height: Length::Shrink,
            padding: Padding::new(5.0),
            style: StyleType::Static(<Renderer::Theme as StyleSheet>::Style::default()),
        }
    }

    /// Sets the width of the [`Button`].
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the height of the [`Button`].
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Sets the [`Padding`] of the [`Button`].
    pub fn padding<P: Into<Padding>>(mut self, padding: P) -> Self {
        self.padding = padding.into();
        self
    }

    /// Sets the message that will be produced when the [`Button`] is pressed.
    ///
    /// Unless `on_press` is called, the [`Button`] will be disabled.
    pub fn on_press(mut self, msg: Message) -> Self {
        self.on_press = Some(msg);
        self
    }

    /// Sets the style variant of this [`Button`].
    pub fn style(mut self, style: <Renderer::Theme as StyleSheet>::Style) -> Self {
        self.style = StyleType::Static(style);
        self
    }

    /// Sets the animatable style variant of this [`Button`].
    pub fn blend_style(
        mut self,
        style1: <Renderer::Theme as StyleSheet>::Style,
        style2: <Renderer::Theme as StyleSheet>::Style,
        percent: f32,
    ) -> Self {
        self.style = StyleType::Blend(style1, style2, percent);
        self
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for Button<'a, Message, Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a + iced_core::renderer::Renderer,
    Renderer::Theme: StyleSheet,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::new())
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_ref(&self.content));
    }

    fn width(&self) -> Length {
        self.width
    }

    fn height(&self) -> Length {
        self.height
    }

    fn layout(
        &self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        layout(
            renderer,
            limits,
            self.width,
            self.height,
            self.padding,
            |renderer, limits| {
                self.content
                    .as_widget()
                    .layout(&mut tree.children[0], renderer, limits)
            },
        )
    }

    fn operate(
        &self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation<Message>,
    ) {
        operation.container(None, layout.bounds(), &mut |operation| {
            self.content.as_widget().operate(
                &mut tree.children[0],
                layout.children().next().unwrap(),
                renderer,
                operation,
            );
        });
    }

    fn on_event(
        &mut self,
        tree: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor_position: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) -> event::Status {
        if let event::Status::Captured = self.content.as_widget_mut().on_event(
            &mut tree.children[0],
            event.clone(),
            layout.children().next().unwrap(),
            cursor_position,
            renderer,
            clipboard,
            shell,
            _viewport,
        ) {
            return event::Status::Captured;
        }

        update(
            event,
            layout,
            cursor_position,
            shell,
            &self.on_press,
            || tree.state.downcast_mut::<State>(),
        )
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Renderer::Theme,
        #[cfg(feature = "libcosmic")] style: &renderer::Style,
        #[cfg(not(feature = "libcosmic"))] _style: &renderer::Style,
        layout: Layout<'_>,
        cursor_position: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        let content_layout = layout.children().next().unwrap();

        let styling = draw(
            renderer,
            bounds,
            cursor_position,
            self.on_press.is_some(),
            theme,
            &self.style,
            || tree.state.downcast_ref::<State>(),
        );

        self.content.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            &renderer::Style {
                #[cfg(feature = "libcosmic")]
                icon_color: styling.icon_color.unwrap_or(style.icon_color),
                text_color: styling.text_color,
            },
            content_layout,
            cursor_position,
            &bounds,
        );
    }

    fn mouse_interaction(
        &self,
        _tree: &Tree,
        layout: Layout<'_>,
        cursor_position: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        mouse_interaction(layout, cursor_position, self.on_press.is_some())
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
    ) -> Option<overlay::Element<'b, Message, Renderer>> {
        self.content.as_widget_mut().overlay(
            &mut tree.children[0],
            layout.children().next().unwrap(),
            renderer,
        )
    }
}

impl<'a, Message, Renderer> From<Button<'a, Message, Renderer>> for Element<'a, Message, Renderer>
where
    Message: Clone + 'a,
    Renderer: iced_core::renderer::Renderer + 'a,
    Renderer::Theme: StyleSheet,
{
    fn from(button: Button<'a, Message, Renderer>) -> Self {
        Self::new(button)
    }
}

/// The local state of a [`Button`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct State {
    is_pressed: bool,
}

impl State {
    /// Creates a new [`State`].
    #[must_use]
    pub fn new() -> State {
        State::default()
    }
}

/// Processes the given [`Event`] and updates the [`State`] of a [`Button`]
/// accordingly.
pub fn update<'a, Message: Clone>(
    event: Event,
    layout: Layout<'_>,
    cursor_position: mouse::Cursor,
    shell: &mut Shell<'_, Message>,
    on_press: &Option<Message>,
    state: impl FnOnce() -> &'a mut State,
) -> event::Status {
    match event {
        Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
        | Event::Touch(touch::Event::FingerPressed { .. }) => {
            if on_press.is_some() {
                let bounds = layout.bounds();

                if cursor_position.is_over(bounds) {
                    let state = state();

                    state.is_pressed = true;

                    return event::Status::Captured;
                }
            }
        }
        Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
        | Event::Touch(touch::Event::FingerLifted { .. }) => {
            if let Some(on_press) = on_press.clone() {
                let state = state();

                if state.is_pressed {
                    state.is_pressed = false;

                    let bounds = layout.bounds();

                    if cursor_position.is_over(bounds) {
                        shell.publish(on_press);
                    }

                    return event::Status::Captured;
                }
            }
        }
        Event::Touch(touch::Event::FingerLost { .. }) => {
            let state = state();

            state.is_pressed = false;
        }
        _ => {}
    }

    event::Status::Ignored
}

/// Draws a [`Button`].
pub fn draw<'a, Renderer: iced_core::renderer::Renderer>(
    renderer: &mut Renderer,
    bounds: Rectangle,
    cursor_position: mouse::Cursor,
    is_enabled: bool,
    style_sheet: &dyn StyleSheet<Style = <Renderer::Theme as StyleSheet>::Style>,
    style: &StyleType<<Renderer::Theme as StyleSheet>::Style>,
    state: impl FnOnce() -> &'a State,
) -> Appearance
where
    Renderer::Theme: StyleSheet,
{
    let is_mouse_over = cursor_position.is_over(bounds);

    // todo disable blend if user has applied style.
    let styling = match style {
        StyleType::Static(style) => {
            if !is_enabled {
                style_sheet.disabled(style)
            } else if is_mouse_over {
                let state = state();

                if state.is_pressed {
                    style_sheet.pressed(style)
                } else {
                    style_sheet.hovered(style)
                }
            } else {
                style_sheet.active(style)
            }
        }
        StyleType::Blend(style1, style2, percent) => {
            let (one, two) = if !is_enabled {
                (style_sheet.disabled(style1), style_sheet.disabled(style2))
            } else if is_mouse_over {
                let state = state();

                if state.is_pressed {
                    (style_sheet.pressed(style1), style_sheet.pressed(style2))
                } else {
                    (style_sheet.hovered(style1), style_sheet.hovered(style2))
                }
            } else {
                (style_sheet.active(style1), style_sheet.active(style2))
            };

            button_blend_appearances(one, two, *percent)
        }
    };

    if styling.background.is_some() || styling.border_width > 0.0 {
        if styling.shadow_offset != Vector::default() {
            // TODO: Implement proper shadow support
            renderer.fill_quad(
                renderer::Quad {
                    bounds: Rectangle {
                        x: bounds.x + styling.shadow_offset.x,
                        y: bounds.y + styling.shadow_offset.y,
                        ..bounds
                    },
                    border_radius: styling.border_radius,
                    border_width: 0.0,
                    border_color: Color::TRANSPARENT,
                },
                Background::Color([0.0, 0.0, 0.0, 0.5].into()),
            );
        }

        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border_radius: styling.border_radius,
                border_width: styling.border_width,
                border_color: styling.border_color,
            },
            styling
                .background
                .unwrap_or(Background::Color(Color::TRANSPARENT)),
        );
    }

    styling
}

/// Computes the layout of a [`Button`].
pub fn layout<Renderer>(
    renderer: &Renderer,
    limits: &layout::Limits,
    width: Length,
    height: Length,
    padding: Padding,
    layout_content: impl FnOnce(&Renderer, &layout::Limits) -> layout::Node,
) -> layout::Node {
    let limits = limits.width(width).height(height);

    let mut content = layout_content(renderer, &limits.pad(padding));
    let padding = padding.fit(content.size(), limits.max());
    let size = limits.pad(padding).resolve(content.size()).pad(padding);

    content.move_to(Point::new(padding.left, padding.top));

    layout::Node::with_children(size, vec![content])
}

/// Returns the [`mouse::Interaction`] of a [`Button`].
#[must_use]
pub fn mouse_interaction(
    layout: Layout<'_>,
    cursor_position: mouse::Cursor,
    is_enabled: bool,
) -> mouse::Interaction {
    let is_mouse_over = cursor_position.is_over(layout.bounds());

    if is_mouse_over && is_enabled {
        mouse::Interaction::Pointer
    } else {
        mouse::Interaction::default()
    }
}
