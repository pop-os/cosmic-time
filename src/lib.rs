pub mod keyframes;
pub mod timeline;

pub use crate::keyframes::container;
pub use crate::timeline::Timeline;

const PI: f32 = std::f32::consts::PI;

// p = percent_complete in decimal form
pub fn lerp(start: f32, end: f32, p: f32) -> f32 {
    let p = p.clamp(0.0, 1.0);
    (1.0 - p) * start + p * end
}

pub fn flip(num: f32) -> f32 {
    1.0 - num
}

pub trait Tween: std::fmt::Debug + Copy {
    // p = percent complete as decimal
    fn tween(&self, p: f32) -> f32;
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
    // Modeled after the line y = x
    fn tween(&self, p: f32) -> f32 {
        p
    }
}

impl From<Linear> for Ease {
    fn from(linear: Linear) -> Self {
        Ease::Linear(linear)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Quadratic {
    In,
    Out,
    InOut,
    Bezier(i32),
}

impl Tween for Quadratic {
    fn tween(&self, p: f32) -> f32 {
        match self {
            // Modeled after the parabola y = x^2
            Quadratic::In => p.powi(2),
            // Modeled after the parabola y = -x^2
            Quadratic::Out => -(p * (p - 2.)),
            // Modeled after the parabola y = -x^2 + 2x
            Quadratic::InOut => {
                if p < 0.5 {
                    2. * p.powi(2)
                } else {
                    (2. * p.powi(2)) + p.mul_add(4., -1.)
                }
            }
            // A Bezier Curve TODO
            Quadratic::Bezier(_n) => p,
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
    fn tween(&self, p: f32) -> f32 {
        match self {
            // Modeled after the cubic y = x^3
            Cubic::In => p.powi(3),
            // Modeled after the cubic y = (x-1)^3 + 1
            Cubic::Out => {
                let q = p - 1.;
                q.powi(3) + 1.
            }
            // Modeled after the piecewise cubic
            // y = (1/2)((2x)^3)       ; [0, 0.5]
            // y = (1/2)((2x=2)^3 + 2) ; [0.5, 1]
            Cubic::InOut => {
                if p < 0.5 {
                    4. * p.powi(3)
                } else {
                    let q = p.mul_add(2., -2.);
                    (q.powi(3)).mul_add(0.5, 1.)
                }
            }
        }
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
    fn tween(&self, p: f32) -> f32 {
        match self {
            // Modeled after the quartic y = x^4
            Quartic::In => p.powi(4),
            // Modeled after the quartic y = 1 - (x - 1)^4
            Quartic::Out => {
                let q = p - 1.;
                (q.powi(4)).mul_add(1. - p, 1.)
            }
            // Modeled after the piecewise quartic
            // y = (1/2)((2x)^4)       ; [0, 0.5]
            // y = -(1/2)((2x-2)^4 -2) ; [0.5, 1]
            Quartic::InOut => {
                if p < 0.5 {
                    8. * p.powi(4)
                } else {
                    let q = p - 1.;
                    (q.powi(4)).mul_add(-8., 1.)
                }
            }
        }
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
    fn tween(&self, p: f32) -> f32 {
        match self {
            // Modeled after the quintic y = x^5
            Quintic::In => p.powi(5),
            // Modeled after the quintic y = (x - 1)^5 + 1
            Quintic::Out => {
                let q = p - 1.;
                q.powi(5) + 1.
            },
            // Modeled after the piecewise quintic
            // y = (1/2)((2x)^5)       ; [0, 0.5]
            // y = (1/2)((2x-2)^5 + 2) ; [0.5, 1]
            Quintic::InOut => {
                if p < 0.5 {
                    16. * p.powi(5)
                } else {
                    let q = (2. * p) - 2.;
                    q.powi(5).mul_add(-0.5, 1.)
                }
            }
        }
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
    fn tween(&self, p: f32) -> f32 {
        match self {
            // Modeled after quarter-cycle of sine wave
            Sinusoidal::In => {
                let q = (p - 1.) * PI;
                q.sin() + 1.
            },
            // Modeled after quarter-cycle of sine wave (different phase)
            Sinusoidal::Out => (p * PI).sin(),
            // Modeled after half sine wave
            Sinusoidal::InOut => {
                if p < 0.5 {
                    0.5 * (1. - p.powi(2).mul_add(-4., 1.).sqrt())
                } else {
                    0.5 * (p.mul_add(02., 3.) * p.mul_add(2., -1.)).sqrt() + 1.
                }
            },
        }
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
    fn tween(&self, p: f32) -> f32 {
        match self {
            // Modeled after the exponential function y = 2^(10(x-1))
            Exponential::In => {
                if p == 0. { 0. }
                else { (10. * (p - 1.)).powi(2)}
            }
            // Modeled after the exponential function y = -2^(-10x) + 1
            Exponential::Out => {
                if p == 1. { 1. }
                else { 1. - (-10. * p).powi(2)}
            },
            // Modeled after the piecewise exponential
            // y = (1/2)*2^(10(2x -1))        ; [0, 0.5]
            // y = -(1/2)*2^(-10(2x - 1)) + 1 ; [0.5, 1]
            Exponential::InOut => {
                if p == 0. { 0. }
                else if p == 1. { 1. }
                else if p < 0.5 { p.mul_add(20., -10.).powi(2) * 0.5}
                else {p.mul_add(-20., 10.).powi(2).mul_add(-0.5, 1.)}
            },
        }
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
    fn tween(&self, p: f32) -> f32 {
        match self {
            // Modeled after shifted quadrant IV of unit circle
            Circular::In => 1.0 - (1. - (p.powi(2))).sqrt(),
            // Modeled after shifted quadrant II of unit circle
            Circular::Out => ((2. - p) * p).sqrt(),
            // Modeled after the piecewise circular function
            // y = (1/2)(1 - sqrt(1 - 4x^2))           ; [0, 0.5)
            // y = (1/2)(sqrt(-(2x - 3)*(2x - 1)) + 1) ; [0.5, 1]
            Circular::InOut => {
                if p < 0.5 {
                    0.5 * (1. - (1. - 4. * p.powi(2)).sqrt())
                } else {
                    0.5 * (-(((2. * p) -3.) * ((2. * p) - 1.)) + 1.).sqrt()
                }
            },
        }
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
    fn tween(&self, p: f32) -> f32 {
        match self {
            // Modeled after the demped sine wave y = sin(13pi/2x)*(10 * (x - 1)^2)
            Elastic::In => (13. * PI * p).sin() * (10. * (p - 1.)).powi(2),
            // Modeled after the damped sin wave y = sin(-13pi/2(x + 1))*(-10x)^2 + 1
            Elastic::Out => (-13. * PI * (p + 1.)).sin() * (-10. * p).powi(2) + 1.,
            // Modeled after the piecewise exponentially-damped sine wave:
            // y = (1/2)*sin(13pi/2(2*x))*(2, 10 * ((2*x) - 1))^2      ; [0,0.5)
            // y = (1/2)*(sin(-13pi/2*((2x-1)+1))*(2,-10(2*x-1))^2 + 2) ; [0.5, 1]
            Elastic::InOut => {
                if p < 0.5 {
                    0.5 * (13. * PI * 2. * p).sin() * (10. * ((2. * p) - 1.)).powi(2)
                } else {
                    0.5 * ((-13. * PI * ((2. * p - 1.) + 1.)).sin() * (-20. * p + 10.).powi(2) + 2.)
                }
            },
        }
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
    fn tween(&self, p: f32) -> f32 {
        match self {
            Back::In => p.powi(4) * (p * PI).sin(),
            Back::Out => {
                let q: f32 = 1. - p;
                1. - (q.powi(4) * (q * PI).sin())
            },
            Back::InOut => {
                if p < 0.5 {
                    let q = 2. * p;
                    0.5 * (q.powi(4) * (q * PI).sin())
                } else {
                    let q: f32 = 1. - (2. * p - 1.);
                    0.5 * (1. - (q.powi(4) * (q * PI).sin())) + 0.5
                }
            },
        }
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

impl Bounce {
    fn bounce_ease_out(p: f32) -> f32 {
        if p < 4./11. {
            (121. * p.powi(2))/16.
        } else if p < 8./11. {
            (363./40. * p.powi(2)) - 99./10. * p + 17./5.
        } else if p < 9./10. {
            4356./361. * p.powi(2) - 35442./1805. * p + 16061./1805.
        } else {
            54./5. * p.powi(2) - 513./25. * p + 268./25.
        }
    }
}

impl Tween for Bounce {
    fn tween(&self, p: f32) -> f32 {
        match self {
            Bounce::In => 1. - Bounce::bounce_ease_out(1. - p),
            Bounce::Out => Bounce::bounce_ease_out(p),
            Bounce::InOut => {
                if p < 0.5 {
                    0.5 * (1. - Bounce::bounce_ease_out(p * 2.))
                } else {
                    -0.5 * Bounce::bounce_ease_out(p * 2. - 1.) + 0.5
                }
            }
        }
    }
}

impl From<Bounce> for Ease {
    fn from(bounce: Bounce) -> Self {
        Ease::Bounce(bounce)
    }
}

    //Linear,
    //Quadratic,
    //Cubic,
    //Quartic,
    //Quintic,
    //Sinusoidal,
    //Exponential,
    //Circular,
    //Elastic,
    //Back,
    //Bounce
