//! Decorate content and apply alignment.
use cosmic::iced_core::alignment::{self, Alignment};
use cosmic::iced_core::event::{self, Event};
use cosmic::iced_core::gradient::{ColorStop, Linear};
use cosmic::iced_core::overlay;
use cosmic::iced_core::renderer;
use cosmic::iced_core::widget::{Id, Operation, Tree};
use cosmic::iced_core::{layout, Gradient};
use cosmic::iced_core::{mouse, Radians};
use cosmic::iced_core::{
    Background, Clipboard, Color, Element, Layout, Length, Padding, Pixels, Point, Rectangle,
    Shell, Widget,
};

use crate::utils::static_array_from_iter;
use crate::widget::StyleType;

use cosmic::iced_renderer::core::widget::OperationOutputWrapper;
pub use cosmic::iced_style::container::{Appearance, StyleSheet};

/// An element decorating some content.
///
/// It is normally used for alignment purposes.
#[allow(missing_debug_implementations)]
pub struct Container<'a, Message, Renderer = cosmic::iced::Renderer>
where
    Renderer: cosmic::iced_core::Renderer,
    Renderer::Theme: StyleSheet,
{
    id: Option<Id>,
    padding: Padding,
    width: Length,
    height: Length,
    max_width: f32,
    max_height: f32,
    horizontal_alignment: alignment::Horizontal,
    vertical_alignment: alignment::Vertical,
    style: StyleType<<Renderer::Theme as StyleSheet>::Style>,
    content: Element<'a, Message, Renderer>,
}

impl<'a, Message, Renderer> Container<'a, Message, Renderer>
where
    Renderer: cosmic::iced_core::Renderer,
    Renderer::Theme: StyleSheet,
{
    /// Creates an empty [`Container`].
    pub fn new<T>(content: T) -> Self
    where
        T: Into<Element<'a, Message, Renderer>>,
    {
        Container {
            id: None,
            padding: Padding::ZERO,
            width: Length::Shrink,
            height: Length::Shrink,
            max_width: f32::INFINITY,
            max_height: f32::INFINITY,
            horizontal_alignment: alignment::Horizontal::Left,
            vertical_alignment: alignment::Vertical::Top,
            style: StyleType::Static(Default::default()),
            content: content.into(),
        }
    }

    /// Sets the [`Id`] of the [`Container`].
    pub fn id(mut self, id: Id) -> Self {
        self.id = Some(id);
        self
    }

    /// Sets the [`Padding`] of the [`Container`].
    pub fn padding<P: Into<Padding>>(mut self, padding: P) -> Self {
        self.padding = padding.into();
        self
    }

    /// Sets the width of the [`Container`].
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the height of the [`Container`].
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Sets the maximum width of the [`Container`].
    pub fn max_width(mut self, max_width: impl Into<Pixels>) -> Self {
        self.max_width = max_width.into().0;
        self
    }

    /// Sets the maximum height of the [`Container`].
    pub fn max_height(mut self, max_height: impl Into<Pixels>) -> Self {
        self.max_height = max_height.into().0;
        self
    }

    /// Sets the content alignment for the horizontal axis of the [`Container`].
    pub fn align_x(mut self, alignment: alignment::Horizontal) -> Self {
        self.horizontal_alignment = alignment;
        self
    }

    /// Sets the content alignment for the vertical axis of the [`Container`].
    pub fn align_y(mut self, alignment: alignment::Vertical) -> Self {
        self.vertical_alignment = alignment;
        self
    }

    /// Centers the contents in the horizontal axis of the [`Container`].
    pub fn center_x(mut self) -> Self {
        self.horizontal_alignment = alignment::Horizontal::Center;
        self
    }

    /// Centers the contents in the vertical axis of the [`Container`].
    pub fn center_y(mut self) -> Self {
        self.vertical_alignment = alignment::Vertical::Center;
        self
    }

    /// Sets the style of the [`Container`].
    pub fn style(mut self, style: impl Into<<Renderer::Theme as StyleSheet>::Style>) -> Self {
        self.style = StyleType::Static(style.into());
        self
    }

    /// Sets the animatable style variant of this [`Container`].
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

impl<'a, Message, Renderer> Widget<Message, Renderer> for Container<'a, Message, Renderer>
where
    Renderer: cosmic::iced_core::Renderer,
    Renderer::Theme: StyleSheet,
{
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
            self.max_width,
            self.max_height,
            self.padding,
            self.horizontal_alignment,
            self.vertical_alignment,
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
        operation.container(self.id.as_ref(), layout.bounds(), &mut |operation| {
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
        viewport: &Rectangle,
    ) -> event::Status {
        self.content.as_widget_mut().on_event(
            &mut tree.children[0],
            event,
            layout.children().next().unwrap(),
            cursor_position,
            renderer,
            clipboard,
            shell,
            viewport,
        )
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor_position: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.content.as_widget().mouse_interaction(
            &tree.children[0],
            layout.children().next().unwrap(),
            cursor_position,
            viewport,
            renderer,
        )
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Renderer::Theme,
        renderer_style: &renderer::Style,
        layout: Layout<'_>,
        cursor_position: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let style = match &self.style {
            StyleType::Static(style) => theme.appearance(style),
            StyleType::Blend(one, two, percent) => {
                blend_appearances(theme.appearance(one), theme.appearance(two), *percent)
            }
        };

        draw_background(renderer, &style, layout.bounds());

        self.content.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            &renderer::Style {
                icon_color: style.icon_color.unwrap_or(renderer_style.icon_color),
                text_color: style.text_color.unwrap_or(renderer_style.text_color),
                scale_factor: renderer_style.scale_factor,
            },
            layout.children().next().unwrap(),
            cursor_position,
            viewport,
        );
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
        let c_layout = layout.children().next().unwrap();
        let c_state = &state.children[0];
        self.content.as_widget().a11y_nodes(c_layout, c_state, p)
    }
}

impl<'a, Message, Renderer> From<Container<'a, Message, Renderer>>
    for Element<'a, Message, Renderer>
where
    Message: 'a,
    Renderer: 'a + cosmic::iced_core::Renderer,
    Renderer::Theme: StyleSheet,
{
    fn from(column: Container<'a, Message, Renderer>) -> Element<'a, Message, Renderer> {
        Element::new(column)
    }
}

/// Computes the layout of a [`Container`].
pub fn layout<Renderer>(
    renderer: &Renderer,
    limits: &layout::Limits,
    width: Length,
    height: Length,
    max_width: f32,
    max_height: f32,
    padding: Padding,
    horizontal_alignment: alignment::Horizontal,
    vertical_alignment: alignment::Vertical,
    layout_content: impl FnOnce(&Renderer, &layout::Limits) -> layout::Node,
) -> layout::Node {
    let limits = limits
        .loose()
        .max_width(max_width)
        .max_height(max_height)
        .width(width)
        .height(height);

    let mut content = layout_content(renderer, &limits.pad(padding).loose());
    let padding = padding.fit(content.size(), limits.max());
    let size = limits.pad(padding).resolve(content.size());

    content.move_to(Point::new(padding.left, padding.top));
    content.align(
        Alignment::from(horizontal_alignment),
        Alignment::from(vertical_alignment),
        size,
    );

    layout::Node::with_children(size.pad(padding), vec![content])
}

/// Draws the background of a [`Container`] given its [`Appearance`] and its `bounds`.
pub fn draw_background<Renderer>(
    renderer: &mut Renderer,
    appearance: &Appearance,
    bounds: Rectangle,
) where
    Renderer: cosmic::iced_core::Renderer,
{
    if appearance.background.is_some() || appearance.border_width > 0.0 {
        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border_radius: appearance.border_radius,
                border_width: appearance.border_width,
                border_color: appearance.border_color,
            },
            appearance
                .background
                .unwrap_or(Background::Color(Color::TRANSPARENT)),
        );
    }
}

fn blend_appearances(
    one: cosmic::iced_style::container::Appearance,
    mut two: cosmic::iced_style::container::Appearance,
    percent: f32,
) -> cosmic::iced_style::container::Appearance {
    use crate::lerp;

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
    let text = one
        .text_color
        .map(|t| {
            static_array_from_iter::<f32, 4>(
                t.into_linear()
                    .iter()
                    .zip(two.text_color.unwrap_or(t).into_linear().iter())
                    .map(|(o, t)| lerp(*o, *t, percent)),
            )
        })
        .map(Into::<Color>::into);

    let one_border_radius: [f32; 4] = one.border_radius.into();
    let two_border_radius: [f32; 4] = two.border_radius.into();
    two.background = Some(background_mix);
    two.border_radius = [
        lerp(one_border_radius[0], two_border_radius[0], percent),
        lerp(one_border_radius[1], two_border_radius[1], percent),
        lerp(one_border_radius[2], two_border_radius[2], percent),
        lerp(one_border_radius[3], two_border_radius[3], percent),
    ]
    .into();
    two.border_width = lerp(one.border_width, two.border_width, percent);
    two.border_color = border_color.into();
    two.text_color = text;
    two
}
