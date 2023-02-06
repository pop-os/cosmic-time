pub mod keyframes;
pub mod timeline;

pub use crate::keyframes::container;
pub use crate::timeline::Timeline;

pub fn lerp(start: f32, end: f32, percent_complete: f32) -> f32 {
    let percent_complete = percent_complete.clamp(0.0, 1.0);
    (1.0 - percent_complete) * start + percent_complete * end
}

pub fn flip(num: f32) -> f32 {
    1.0 - num
}

pub trait Tween: std::fmt::Debug + Copy {
    fn tween(&self, percent_complete: f32) -> f32;
}

macro_rules! tween {
    ($($x:ident),*) => {
        #[derive(Debug, Copy, Clone)]
        pub enum Ease {
            $(
                $x($x),
            )*
        }

        impl Tween for Ease {
            fn tween(&self, percent_complete: f32) -> f32 {
                match self {
                    $(
                        Ease::$x(ease) => ease.tween(percent_complete),
                    )*
                }
            }
        }
    };
}

tween!(
    Linear,
    Quadratic,
    Cubic,
    Quartic,
    Quintic,
    Sinusoidal,
    Exponential,
    Circular,
    Elastic,
    Back,
    Bounce
);

#[derive(Debug, Copy, Clone)]
pub enum Linear {
    InOut,
}

impl Tween for Linear {
    fn tween(&self, percent_complete: f32) -> f32 {
        percent_complete
    }
}

impl From<Linear> for Ease {
    fn from(linear: Linear) -> Self {
        Ease::Linear(linear)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Quadratic {
    In(i32),
    Out(i32),
    InOut(i32),
    Bezier(i32, i32),
}

impl Tween for Quadratic {
    fn tween(&self, percent_complete: f32) -> f32 {
        match self {
            Quadratic::In(n) => percent_complete.powi(*n),
            Quadratic::Out(n) => flip(flip(percent_complete).powi(*n)),
            Quadratic::InOut(n) => lerp(
                percent_complete.powi(*n),
                flip(flip(percent_complete).powi(*n)),
                percent_complete,
            ),
            Quadratic::Bezier(i, o) => lerp(
                percent_complete.powi(*i),
                flip(percent_complete).powi(*o),
                percent_complete,
            ),
        }
    }
}

impl From<Quadratic> for Ease {
    fn from(quadratic: Quadratic) -> Self {
        Ease::Quadratic(quadratic)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Cubic {
    In,
    Out,
    InOut,
}

impl Tween for Cubic {
    fn tween(&self, percent_complete: f32) -> f32 {
        percent_complete
    }
}

impl From<Cubic> for Ease {
    fn from(cubic: Cubic) -> Self {
        Ease::Cubic(cubic)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Quartic {
    In,
    Out,
    InOut,
}

impl Tween for Quartic {
    fn tween(&self, percent_complete: f32) -> f32 {
        percent_complete
    }
}

impl From<Quartic> for Ease {
    fn from(quartic: Quartic) -> Self {
        Ease::Quartic(quartic)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Quintic {
    In,
    Out,
    InOut,
}

impl Tween for Quintic {
    fn tween(&self, percent_complete: f32) -> f32 {
        percent_complete
    }
}

impl From<Quintic> for Ease {
    fn from(quintic: Quintic) -> Self {
        Ease::Quintic(quintic)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Sinusoidal {
    In,
    Out,
    InOut,
}

impl Tween for Sinusoidal {
    fn tween(&self, percent_complete: f32) -> f32 {
        percent_complete
    }
}

impl From<Sinusoidal> for Ease {
    fn from(sinusoidal: Sinusoidal) -> Self {
        Ease::Sinusoidal(sinusoidal)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Exponential {
    In,
    Out,
    InOut,
}

impl Tween for Exponential {
    fn tween(&self, percent_complete: f32) -> f32 {
        percent_complete
    }
}

impl From<Exponential> for Ease {
    fn from(exponential: Exponential) -> Self {
        Ease::Exponential(exponential)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Circular {
    In,
    Out,
    InOut,
}

impl Tween for Circular {
    fn tween(&self, percent_complete: f32) -> f32 {
        percent_complete
    }
}

impl From<Circular> for Ease {
    fn from(circular: Circular) -> Self {
        Ease::Circular(circular)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Elastic {
    In,
    Out,
    InOut,
}

impl Tween for Elastic {
    fn tween(&self, percent_complete: f32) -> f32 {
        percent_complete
    }
}

impl From<Elastic> for Ease {
    fn from(elastic: Elastic) -> Self {
        Ease::Elastic(elastic)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Back {
    In,
    Out,
    InOut,
}

impl Tween for Back {
    fn tween(&self, percent_complete: f32) -> f32 {
        percent_complete
    }
}

impl From<Back> for Ease {
    fn from(back: Back) -> Self {
        Ease::Back(back)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Bounce {
    In,
    Out,
    InOut,
}

impl Tween for Bounce {
    fn tween(&self, percent_complete: f32) -> f32 {
        percent_complete
    }
}

impl From<Bounce> for Ease {
    fn from(bounce: Bounce) -> Self {
        Ease::Bounce(bounce)
    }
}
