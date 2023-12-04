//! Allow your users to perform actions by pressing a button.
//!
//! A [`Button`] has some local [`State`].
use cosmic::iced_core::gradient::{ColorStop, Linear};
use cosmic::iced_runtime::core::widget::Id;
use cosmic::iced_runtime::{keyboard, Command};

use crate::utils::static_array_from_iter;
use crate::widget::StyleType;
use cosmic::iced_core::event::{self, Event};
use cosmic::iced_core::overlay;
use cosmic::iced_core::renderer;
use cosmic::iced_core::touch;
use cosmic::iced_core::widget::tree::{self, Tree};
use cosmic::iced_core::widget::Operation;
use cosmic::iced_core::{layout, Gradient};
use cosmic::iced_core::{mouse, Radians};
use cosmic::iced_core::{
    Background, Clipboard, Color, Element, Layout, Length, Padding, Point, Rectangle, Shell,
    Vector, Widget,
};
use cosmic::iced_renderer::core::widget::{operation, OperationOutputWrapper};

pub use cosmic::iced_style::button::{Appearance, StyleSheet};

/// A generic widget that produces a message when pressed.
#[allow(missing_debug_implementations)]
pub struct Button<'a, Message, Renderer = cosmic::Renderer>
where
    Renderer: cosmic::iced_core::Renderer,
    Renderer::Theme: StyleSheet,
{
    id: Id,
    #[cfg(feature = "a11y")]
    name: Option<Cow<'a, str>>,
    #[cfg(feature = "a11y")]
    description: Option<iced_accessibility::Description<'a>>,
    #[cfg(feature = "a11y")]
    label: Option<Vec<iced_accessibility::accesskit::NodeId>>,
    content: Element<'a, Message, Renderer>,
    on_press: Option<Message>,
    width: Length,
    height: Length,
    padding: Padding,
    style: StyleType<<Renderer::Theme as StyleSheet>::Style>,
}

impl<'a, Message, Renderer> Button<'a, Message, Renderer>
where
    Renderer: cosmic::iced_core::Renderer,
    Renderer::Theme: StyleSheet,
{
    /// Creates a new [`Button`] with the given content.
    pub fn new(content: impl Into<Element<'a, Message, Renderer>>) -> Self {
        Button {
            id: Id::unique(),
            #[cfg(feature = "a11y")]
            name: None,
            #[cfg(feature = "a11y")]
            description: None,
            #[cfg(feature = "a11y")]
            label: None,
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

    /// Sets the [`Id`] of the [`Button`].
    pub fn id(mut self, id: Id) -> Self {
        self.id = id;
        self
    }

    #[cfg(feature = "a11y")]
    /// Sets the name of the [`Button`].
    pub fn name(mut self, name: impl Into<Cow<'a, str>>) -> Self {
        self.name = Some(name.into());
        self
    }

    #[cfg(feature = "a11y")]
    /// Sets the description of the [`Button`].
    pub fn description_widget<T: iced_accessibility::Describes>(mut self, description: &T) -> Self {
        self.description = Some(iced_accessibility::Description::Id(
            description.description(),
        ));
        self
    }

    #[cfg(feature = "a11y")]
    /// Sets the description of the [`Button`].
    pub fn description(mut self, description: impl Into<Cow<'a, str>>) -> Self {
        self.description = Some(iced_accessibility::Description::Text(description.into()));
        self
    }

    #[cfg(feature = "a11y")]
    /// Sets the label of the [`Button`].
    pub fn label(mut self, label: &dyn iced_accessibility::Labels) -> Self {
        self.label = Some(label.label().into_iter().map(|l| l.into()).collect());
        self
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for Button<'a, Message, Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a + cosmic::iced_core::Renderer,
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

    fn diff(&mut self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_mut(&mut self.content))
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
        operation: &mut dyn Operation<OperationOutputWrapper<Message>>,
    ) {
        operation.container(None, layout.bounds(), &mut |operation| {
            self.content.as_widget().operate(
                &mut tree.children[0],
                layout.children().next().unwrap(),
                renderer,
                operation,
            );
        });
        let state = tree.state.downcast_mut::<State>();
        operation.focusable(state, Some(&self.id));
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
            self.id.clone(),
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
        style: &renderer::Style,
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
                scale_factor: style.scale_factor,
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

    #[cfg(feature = "a11y")]
    /// get the a11y nodes for the widget
    fn a11y_nodes(
        &self,
        layout: Layout<'_>,
        state: &Tree,
        p: Point,
    ) -> iced_accessibility::A11yTree {
        use iced_accessibility::{
            accesskit::{Action, DefaultActionVerb, NodeBuilder, NodeId, Rect, Role},
            A11yNode, A11yTree,
        };

        let child_layout = layout.children().next().unwrap();
        let child_tree = &state.children[0];
        let child_tree = self
            .content
            .as_widget()
            .a11y_nodes(child_layout, &child_tree, p);

        let Rectangle {
            x,
            y,
            width,
            height,
        } = layout.bounds();
        let bounds = Rect::new(x as f64, y as f64, (x + width) as f64, (y + height) as f64);
        let is_hovered = state.state.downcast_ref::<State>().is_hovered;

        let mut node = NodeBuilder::new(Role::Button);
        node.add_action(Action::Focus);
        node.add_action(Action::Default);
        node.set_bounds(bounds);
        if let Some(name) = self.name.as_ref() {
            node.set_name(name.clone());
        }
        match self.description.as_ref() {
            Some(iced_accessibility::Description::Id(id)) => {
                node.set_described_by(
                    id.iter()
                        .cloned()
                        .map(|id| NodeId::from(id))
                        .collect::<Vec<_>>(),
                );
            }
            Some(iced_accessibility::Description::Text(text)) => {
                node.set_description(text.clone());
            }
            None => {}
        }

        if let Some(label) = self.label.as_ref() {
            node.set_labelled_by(label.clone());
        }

        if self.on_press.is_none() {
            node.set_disabled()
        }
        if is_hovered {
            node.set_hovered()
        }
        node.set_default_action_verb(DefaultActionVerb::Click);

        A11yTree::node_with_child_tree(A11yNode::new(node, self.id.clone()), child_tree)
    }

    fn id(&self) -> Option<Id> {
        Some(self.id.clone())
    }

    fn set_id(&mut self, id: Id) {
        self.id = id;
    }
}

impl<'a, Message, Renderer> From<Button<'a, Message, Renderer>> for Element<'a, Message, Renderer>
where
    Message: Clone + 'a,
    Renderer: cosmic::iced_core::Renderer + 'a,
    Renderer::Theme: StyleSheet,
{
    fn from(button: Button<'a, Message, Renderer>) -> Self {
        Self::new(button)
    }
}

/// The local state of a [`Button`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct State {
    is_hovered: bool,
    is_pressed: bool,
    is_focused: bool,
}

impl State {
    /// Creates a new [`State`].
    #[must_use]
    pub fn new() -> State {
        State::default()
    }

    /// Returns whether the [`Button`] is currently focused or not.
    #[must_use]
    pub fn is_focused(&self) -> bool {
        self.is_focused
    }

    /// Returns whether the [`Button`] is currently hovered or not.
    #[must_use]
    pub fn is_hovered(&self) -> bool {
        self.is_hovered
    }

    /// Focuses the [`Button`].
    pub fn focus(&mut self) {
        self.is_focused = true;
    }

    /// Unfocuses the [`Button`].
    pub fn unfocus(&mut self) {
        self.is_focused = false;
    }
}

/// Processes the given [`Event`] and updates the [`State`] of a [`Button`]
/// accordingly.
pub fn update<'a, Message: Clone>(
    _id: Id,
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
        #[cfg(feature = "a11y")]
        Event::A11y(event_id, iced_accessibility::accesskit::ActionRequest { action, .. }) => {
            let state = state();
            if let Some(Some(on_press)) = (id == event_id
                && matches!(action, iced_accessibility::accesskit::Action::Default))
            .then(|| on_press.clone())
            {
                state.is_pressed = false;
                shell.publish(on_press);
            }
            return event::Status::Captured;
        }
        Event::Keyboard(keyboard::Event::KeyPressed { key_code, .. }) => {
            if let Some(on_press) = on_press.clone() {
                let state = state();
                if state.is_focused && key_code == keyboard::KeyCode::Enter {
                    state.is_pressed = true;
                    shell.publish(on_press);
                    return event::Status::Captured;
                }
            }
        }
        Event::Touch(touch::Event::FingerLost { .. }) => {
            let state = state();
            state.is_hovered = false;
            state.is_pressed = false;
        }
        _ => {}
    }

    event::Status::Ignored
}

/// Draws a [`Button`].
pub fn draw<'a, Renderer: cosmic::iced_core::Renderer>(
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

            blend_appearances(one, two, *percent)
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

/// Produces a [`Command`] that focuses the [`Button`] with the given [`Id`].
pub fn focus<Message: 'static>(id: Id) -> Command<Message> {
    Command::widget(operation::focusable::focus(id))
}

impl operation::Focusable for State {
    fn is_focused(&self) -> bool {
        State::is_focused(self)
    }

    fn focus(&mut self) {
        State::focus(self)
    }

    fn unfocus(&mut self) {
        State::unfocus(self)
    }
}

fn blend_appearances(
    one: cosmic::iced_style::button::Appearance,
    mut two: cosmic::iced_style::button::Appearance,
    percent: f32,
) -> cosmic::iced_style::button::Appearance {
    use crate::lerp;

    // shadow offet
    let x1 = one.shadow_offset.x;
    let y1 = one.shadow_offset.y;
    let x2 = two.shadow_offset.x;
    let y2 = two.shadow_offset.y;

    // background
    let background_mix: Background = match (one.background, two.background) {
        (Some(Background::Color(c1)), Some(Background::Color(c2))) => {
            Background::from(Color::from(static_array_from_iter::<f32, 4>(
                c1.into_linear()
                    .iter()
                    .zip(c2.into_linear().iter())
                    .map(|(o, t)| lerp(*o, *t, percent)),
            )))
        }
        (
            Some(Background::Gradient(Gradient::Linear(l1))),
            Some(Background::Gradient(Gradient::Linear(l2))),
        ) => {
            let angle = lerp(l1.angle.0, l2.angle.0, percent);
            let stops = l1
                .stops
                .iter()
                .zip(l2.stops.iter())
                .map(|(o, t)| match (o, t) {
                    (
                        Some(ColorStop {
                            offset: o1,
                            color: c1,
                        }),
                        Some(ColorStop {
                            color: c2,
                            offset: o2,
                        }),
                    ) => Some(ColorStop {
                        color: static_array_from_iter::<f32, 4>(
                            c1.into_linear()
                                .iter()
                                .zip(c2.into_linear().iter())
                                .map(|(o, t)| lerp(*o, *t, percent)),
                        )
                        .into(),
                        offset: lerp(*o1, *o2, percent),
                    }),
                    (a, b) => *if percent < 0.5 { a } else { b },
                });
            Background::Gradient(
                Linear {
                    angle: Radians(angle),
                    stops: static_array_from_iter(stops),
                }
                .into(),
            )
        }
        _ => Background::from(Color::from([0.0, 0.0, 0.0, 0.0])),
    };

    // boarder color
    let border_color = static_array_from_iter::<f32, 4>(
        one.border_color
            .into_linear()
            .iter()
            .zip(two.border_color.into_linear().iter())
            .map(|(o, t)| lerp(*o, *t, percent)),
    );

    // text
    let text = static_array_from_iter::<f32, 4>(
        one.text_color
            .into_linear()
            .iter()
            .zip(two.text_color.into_linear().iter())
            .map(|(o, t)| lerp(*o, *t, percent)),
    );

    let br1: [f32; 4] = one.border_radius.into();
    let br2: [f32; 4] = two.border_radius.into();

    let br = [
        lerp(br1[0], br2[0], percent),
        lerp(br1[1], br2[1], percent),
        lerp(br1[2], br2[2], percent),
        lerp(br1[3], br2[3], percent),
    ];

    two.shadow_offset = Vector::new(lerp(x1, x2, percent), lerp(y1, y2, percent));
    two.background = Some(background_mix);
    two.border_radius = br.into();
    two.border_width = lerp(one.border_width, two.border_width, percent);
    two.border_color = border_color.into();
    two.text_color = text.into();
    two
}
