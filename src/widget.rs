#![allow(clippy::too_many_arguments)]

use crate::reexports::*;

#[cfg(feature = "libcosmic")]
pub mod cards;
#[cfg(feature = "libcosmic")]
pub mod cosmic_button;
#[cfg(feature = "libcosmic")]
pub mod cosmic_container;
#[cfg(feature = "libcosmic")]
pub mod cosmic_toggler;

#[cfg(feature = "libcosmic")]
pub use cards::Cards;
#[cfg(feature = "libcosmic")]
pub use cosmic_button::Button;
#[cfg(feature = "libcosmic")]
pub use cosmic_container::Container;
#[cfg(feature = "libcosmic")]
pub use cosmic_toggler::Toggler;

#[cfg(not(feature = "libcosmic"))]
pub mod button;
#[cfg(not(feature = "libcosmic"))]
pub mod container;
#[cfg(not(feature = "libcosmic"))]
pub mod toggler;

#[cfg(not(feature = "libcosmic"))]
pub use button::Button;
#[cfg(not(feature = "libcosmic"))]
pub use container::Container;
#[cfg(not(feature = "libcosmic"))]
pub use toggler::Toggler;

/// A convenience type to optimize style-able widgets,
/// to only do the "expensize" style calculations if needed.
#[derive(Debug)]
pub enum StyleType<T> {
    /// The style is either default, or set manually in the `view`.
    Static(T),
    /// The stlye is being animated. Blend between the two values.
    Blend(T, T, f32),
}

use iced_core::{
    gradient::{ColorStop, Linear},
    Background, Color, Gradient, Radians, Vector,
};

/// Blend between two button appearances.
pub fn container_blend_appearances(
    one: iced_style::container::Appearance,
    mut two: iced_style::container::Appearance,
    percent: f32,
) -> iced_style::container::Appearance {
    use crate::lerp;

    // background
    let background_mix: Background = match (one.background, two.background) {
        (Some(Background::Color(c1)), Some(Background::Color(c2))) => {
            let background_mix: [f32; 4] = c1
                .into_linear()
                .iter()
                .zip(c2.into_linear().iter())
                .map(|(o, t)| lerp(*o, *t, percent))
                .collect::<Vec<f32>>()
                .try_into()
                .unwrap();
            Background::from(Color::from(background_mix))
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
                    ) => {
                        let color: [f32; 4] = c1
                            .into_linear()
                            .iter()
                            .zip(c2.into_linear().iter())
                            .map(|(o, t)| lerp(*o, *t, percent))
                            .collect::<Vec<f32>>()
                            .try_into()
                            .unwrap();
                        Some(ColorStop {
                            color: color.into(),
                            offset: lerp(*o1, *o2, percent),
                        })
                    }
                    (a, b) => *if percent < 0.5 { a } else { b },
                })
                .collect::<Vec<Option<ColorStop>>>();
            Background::Gradient(
                Linear {
                    angle: Radians(angle),
                    stops: stops.try_into().unwrap(),
                }
                .into(),
            )
        }
        _ => Background::from(Color::from([0.0, 0.0, 0.0, 0.0])),
    };
    // boarder color
    let border_color: [f32; 4] = one
        .border_color
        .into_linear()
        .iter()
        .zip(two.border_color.into_linear().iter())
        .map(|(o, t)| lerp(*o, *t, percent))
        .collect::<Vec<f32>>()
        .try_into()
        .unwrap();

    // text
    let text = one
        .text_color
        .map(|t| {
            let ret: [f32; 4] = t
                .into_linear()
                .iter()
                .zip(two.text_color.unwrap_or(t).into_linear().iter())
                .map(|(o, t)| lerp(*o, *t, percent))
                .collect::<Vec<f32>>()
                .try_into()
                .unwrap();
            ret
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

/// Blend between two button appearances.
pub fn button_blend_appearances(
    one: iced_style::button::Appearance,
    mut two: iced_style::button::Appearance,
    percent: f32,
) -> iced_style::button::Appearance {
    use crate::lerp;

    // shadow offet
    let x1 = one.shadow_offset.x;
    let y1 = one.shadow_offset.y;
    let x2 = two.shadow_offset.x;
    let y2 = two.shadow_offset.y;

    // background
    let background_mix: Background = match (one.background, two.background) {
        (Some(Background::Color(c1)), Some(Background::Color(c2))) => {
            let background_mix: [f32; 4] = c1
                .into_linear()
                .iter()
                .zip(c2.into_linear().iter())
                .map(|(o, t)| lerp(*o, *t, percent))
                .collect::<Vec<f32>>()
                .try_into()
                .unwrap();
            Background::from(Color::from(background_mix))
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
                    ) => {
                        let color: [f32; 4] = c1
                            .into_linear()
                            .iter()
                            .zip(c2.into_linear().iter())
                            .map(|(o, t)| lerp(*o, *t, percent))
                            .collect::<Vec<f32>>()
                            .try_into()
                            .unwrap();
                        Some(ColorStop {
                            color: color.into(),
                            offset: lerp(*o1, *o2, percent),
                        })
                    }
                    (a, b) => *if percent < 0.5 { a } else { b },
                })
                .collect::<Vec<Option<ColorStop>>>();
            Background::Gradient(
                Linear {
                    angle: Radians(angle),
                    stops: stops.try_into().unwrap(),
                }
                .into(),
            )
        }
        _ => Background::from(Color::from([0.0, 0.0, 0.0, 0.0])),
    };

    // boarder color
    let border_color: [f32; 4] = one
        .border_color
        .into_linear()
        .iter()
        .zip(two.border_color.into_linear().iter())
        .map(|(o, t)| lerp(*o, *t, percent))
        .collect::<Vec<f32>>()
        .try_into()
        .unwrap();

    // text
    let text: [f32; 4] = one
        .text_color
        .into_linear()
        .iter()
        .zip(two.text_color.into_linear().iter())
        .map(|(o, t)| lerp(*o, *t, percent))
        .collect::<Vec<f32>>()
        .try_into()
        .unwrap();

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
