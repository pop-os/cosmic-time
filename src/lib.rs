pub mod keyframes;
pub mod timeline;

pub use crate::keyframes::{container, space};
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
            fn tween(&self, p: f32) -> f32 {
                match self {
                    $(
                        Ease::$x(ease) => ease.tween(p),
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
            // Modeled after the parabola y = -x^2 + 2x
            Quadratic::Out => -(p * (p - 2.)),
            // Modeled after the piecewise quadratic
            // y = (1/2)((2x)^2)             ; [0, 0.5)
            // y = -(1/2)((2x-1)*(2x-3) - 1) ; [0.5, 1]
            Quadratic::InOut => {
                if p < 0.5 {
                    2. * p.powi(2)
                } else {
                    (-2. * p.powi(2)) + p.mul_add(4., -1.)
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
            // y = (1/2)((2x-2)^3 + 2) ; [0.5, 1]
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
                (q.powi(3)).mul_add(1. - p, 1.)
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
            }
            // Modeled after the piecewise quintic
            // y = (1/2)((2x)^5)       ; [0, 0.5]
            // y = (1/2)((2x-2)^5 + 2) ; [0.5, 1]
            Quintic::InOut => {
                if p < 0.5 {
                    16. * p.powi(5)
                } else {
                    let q = (2. * p) - 2.;
                    q.powi(5).mul_add(0.5, 1.)
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
            // Modeled after eighth sinusoidal wave y = 1 - cos((x * PI) / 2)
            Sinusoidal::In => 1. - ((p * PI) / 2.).cos(),
            // Modeled after eigth sinusoidal wave y = sin((x * PI) / 2)
            Sinusoidal::Out => ((p * PI) / 2.).sin(),
            // Modeled after quarter sinusoidal wave y = -0.5 * (cos(x * PI) - 1);
            Sinusoidal::InOut => -0.5 * ((p * PI).cos() - 1.)
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
            // Modeled after the piecewise exponential
            // y = 0            ; [0, 0]
            // y = 2^(10x-10)   ; [0, 1]
            Exponential::In => {
                if p == 0. {
                    0.
                } else {
                    2_f32.powf(10. * p - 10.)
                }
            }
            // Modeled after the piecewise exponential
            // y = 1 - 2^(-10x)  ; [0, 1]
            // y = 1             ; [1, 1]
            Exponential::Out => {
                if p == 1. {
                    1.
                } else {
                    1. - 2_f32.powf(-10. * p)
                }
            }
            // Modeled after the piecewise exponential
            // y = 0                        ; [0, 0  ]
            // y = 2^(20x - 10) / 2         ; [0, 0.5]
            // y = 1 - 0.5*2^(-10(2x - 1))  ; [0.5, 1]
            // y = 1                        ; [1, 1  ]
            Exponential::InOut => {
                if p == 0. {
                    0.
                } else if p == 1. {
                    1.
                } else if p < 0.5 {
                    2_f32.powf(p.mul_add(20., -10.)) * 0.5
                } else {
                    2_f32.powf(p.mul_add(-20., 10.)).mul_add(-0.5, 1.)
                }
            }
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
                    0.5 * (-(((2. * p) - 3.) * ((2. * p) - 1.)) + 1.).sqrt()
                }
            }
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
            // y = (1/2)*sin(13pi/2(2*x))*(2, 10 * ((2*x) - 1))^2       ; [0,0.5)
            // y = (1/2)*(sin(-13pi/2*((2x-1)+1))*(2,-10(2*x-1))^2 + 2) ; [0.5, 1]
            Elastic::InOut => {
                if p < 0.5 {
                    0.5 * (13. * PI * 2. * p).sin() * (10. * ((2. * p) - 1.)).powi(2)
                } else {
                    0.5 * ((-13. * PI * ((2. * p - 1.) + 1.)).sin() * (-20. * p + 10.).powi(2) + 2.)
                }
            }
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
            }
            Back::InOut => {
                if p < 0.5 {
                    let q = 2. * p;
                    0.5 * (q.powi(4) * (q * PI).sin())
                } else {
                    let q: f32 = 1. - (2. * p - 1.);
                    0.5 * (1. - (q.powi(4) * (q * PI).sin())) + 0.5
                }
            }
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
        if p < 4. / 11. {
            (121. * p.powi(2)) / 16.
        } else if p < 8. / 11. {
            (363. / 40. * p.powi(2)) - 99. / 10. * p + 17. / 5.
        } else if p < 9. / 10. {
            4356. / 361. * p.powi(2) - 35442. / 1805. * p + 16061. / 1805.
        } else {
            54. / 5. * p.powi(2) - 513. / 25. * p + 268. / 25.
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

#[cfg(test)]
mod test {
    use super::*;

    fn r(val: f32) -> f32 {
        (val * 10E+5).round() / 10E+5
    }

    #[test]
    fn linear() {
        assert_eq!(0.0, Linear::InOut.tween(0.0));
        assert_eq!(0.1, Linear::InOut.tween(0.1));
        assert_eq!(0.2, Linear::InOut.tween(0.2));
        assert_eq!(0.3, Linear::InOut.tween(0.3));
        assert_eq!(0.4, Linear::InOut.tween(0.4));
        assert_eq!(0.5, Linear::InOut.tween(0.5));
        assert_eq!(0.6, Linear::InOut.tween(0.6));
        assert_eq!(0.7, Linear::InOut.tween(0.7));
        assert_eq!(0.8, Linear::InOut.tween(0.8));
        assert_eq!(0.9, Linear::InOut.tween(0.9));
        assert_eq!(1.0, Linear::InOut.tween(1.0));
    }

    #[test]
    // Modeled after the parabola y = x^2
    fn quadratic_in() {
        assert_eq!(0.00, r(Quadratic::In.tween(0.0)));
        assert_eq!(0.01, r(Quadratic::In.tween(0.1)));
        assert_eq!(0.04, r(Quadratic::In.tween(0.2)));
        assert_eq!(0.09, r(Quadratic::In.tween(0.3)));
        assert_eq!(0.16, r(Quadratic::In.tween(0.4)));
        assert_eq!(0.25, r(Quadratic::In.tween(0.5)));
        assert_eq!(0.36, r(Quadratic::In.tween(0.6)));
        assert_eq!(0.49, r(Quadratic::In.tween(0.7)));
        assert_eq!(0.64, r(Quadratic::In.tween(0.8)));
        assert_eq!(0.81, r(Quadratic::In.tween(0.9)));
        assert_eq!(1.00, r(Quadratic::In.tween(1.0)));
    }

    #[test]
    // Modeled after the parabola y = -x^2 + 2x
    fn quadratic_out() {
        assert_eq!(0.00, r(Quadratic::Out.tween(0.0)));
        assert_eq!(0.19, r(Quadratic::Out.tween(0.1)));
        assert_eq!(0.36, r(Quadratic::Out.tween(0.2)));
        assert_eq!(0.51, r(Quadratic::Out.tween(0.3)));
        assert_eq!(0.64, r(Quadratic::Out.tween(0.4)));
        assert_eq!(0.75, r(Quadratic::Out.tween(0.5)));
        assert_eq!(0.84, r(Quadratic::Out.tween(0.6)));
        assert_eq!(0.91, r(Quadratic::Out.tween(0.7)));
        assert_eq!(0.96, r(Quadratic::Out.tween(0.8)));
        assert_eq!(0.99, r(Quadratic::Out.tween(0.9)));
        assert_eq!(1.00, r(Quadratic::Out.tween(1.0)));
    }

    #[test]
    // Modeled after the piecewise quadratic
    // y = (1/2)((2x)^2)             ; [0, 0.5)
    // y = -(1/2)((2x-1)*(2x-3) - 1) ; [0.5, 1]
    fn quadratic_inout() {
        assert_eq!(0.00, r(Quadratic::InOut.tween(0.0)));
        assert_eq!(0.02, r(Quadratic::InOut.tween(0.1)));
        assert_eq!(0.08, r(Quadratic::InOut.tween(0.2)));
        assert_eq!(0.18, r(Quadratic::InOut.tween(0.3)));
        assert_eq!(0.32, r(Quadratic::InOut.tween(0.4)));
        assert_eq!(0.50, r(Quadratic::InOut.tween(0.5)));
        assert_eq!(0.68, r(Quadratic::InOut.tween(0.6)));
        assert_eq!(0.82, r(Quadratic::InOut.tween(0.7)));
        assert_eq!(0.92, r(Quadratic::InOut.tween(0.8)));
        assert_eq!(0.98, r(Quadratic::InOut.tween(0.9)));
        assert_eq!(1.00, r(Quadratic::InOut.tween(1.0)));
    }

    // TODO Bezier
    
    #[test]
    // Modeled after the cubic y = x^3
    fn cubic_in() {
        assert_eq!(0.000, r(Cubic::In.tween(0.0)));
        assert_eq!(0.001, r(Cubic::In.tween(0.1)));
        assert_eq!(0.008, r(Cubic::In.tween(0.2)));
        assert_eq!(0.027, r(Cubic::In.tween(0.3)));
        assert_eq!(0.064, r(Cubic::In.tween(0.4)));
        assert_eq!(0.125, r(Cubic::In.tween(0.5)));
        assert_eq!(0.216, r(Cubic::In.tween(0.6)));
        assert_eq!(0.343, r(Cubic::In.tween(0.7)));
        assert_eq!(0.512, r(Cubic::In.tween(0.8)));
        assert_eq!(0.729, r(Cubic::In.tween(0.9)));
        assert_eq!(1.000, r(Cubic::In.tween(1.0)));
    }

    #[test]
    // Modeled after the cubic y = (x-1)^3 + 1
    fn cubic_out() {
        assert_eq!(0.000, r(Cubic::Out.tween(0.0)));
        assert_eq!(0.271, r(Cubic::Out.tween(0.1)));
        assert_eq!(0.488, r(Cubic::Out.tween(0.2)));
        assert_eq!(0.657, r(Cubic::Out.tween(0.3)));
        assert_eq!(0.784, r(Cubic::Out.tween(0.4)));
        assert_eq!(0.875, r(Cubic::Out.tween(0.5)));
        assert_eq!(0.936, r(Cubic::Out.tween(0.6)));
        assert_eq!(0.973, r(Cubic::Out.tween(0.7)));
        assert_eq!(0.992, r(Cubic::Out.tween(0.8)));
        assert_eq!(0.999, r(Cubic::Out.tween(0.9)));
        assert_eq!(1.000, r(Cubic::Out.tween(1.0)));
    }

    #[test]
    // Modeled after the piecewise cubic
    // y = (1/2)((2x)^3)       ; [0, 0.5]
    // y = (1/2)((2x-2)^3 + 2) ; [0.5, 1]
    fn cubic_inout() {
        assert_eq!(0.000, r(Cubic::InOut.tween(0.0)));
        assert_eq!(0.004, r(Cubic::InOut.tween(0.1)));
        assert_eq!(0.032, r(Cubic::InOut.tween(0.2)));
        assert_eq!(0.108, r(Cubic::InOut.tween(0.3)));
        assert_eq!(0.256, r(Cubic::InOut.tween(0.4)));
        assert_eq!(0.500, r(Cubic::InOut.tween(0.5)));
        assert_eq!(0.744, r(Cubic::InOut.tween(0.6)));
        assert_eq!(0.892, r(Cubic::InOut.tween(0.7)));
        assert_eq!(0.968, r(Cubic::InOut.tween(0.8)));
        assert_eq!(0.996, r(Cubic::InOut.tween(0.9)));
        assert_eq!(1.000, r(Cubic::InOut.tween(1.0)));
    }

    #[test]
    // Modeled after the quartic y = x^4
    fn quartic_in() {
        assert_eq!(0.0000, r(Quartic::In.tween(0.0)));
        assert_eq!(0.0001, r(Quartic::In.tween(0.1)));
        assert_eq!(0.0016, r(Quartic::In.tween(0.2)));
        assert_eq!(0.0081, r(Quartic::In.tween(0.3)));
        assert_eq!(0.0256, r(Quartic::In.tween(0.4)));
        assert_eq!(0.0625, r(Quartic::In.tween(0.5)));
        assert_eq!(0.1296, r(Quartic::In.tween(0.6)));
        assert_eq!(0.2401, r(Quartic::In.tween(0.7)));
        assert_eq!(0.4096, r(Quartic::In.tween(0.8)));
        assert_eq!(0.6561, r(Quartic::In.tween(0.9)));
        assert_eq!(1.0000, r(Quartic::In.tween(1.0)));
    }

    #[test]
    // Modeled after the quartic y = 1 - (x - 1)^4
    fn quartic_out() {
        assert_eq!(0.0000, r(Quartic::Out.tween(0.0)));
        assert_eq!(0.3439, r(Quartic::Out.tween(0.1)));
        assert_eq!(0.5904, r(Quartic::Out.tween(0.2)));
        assert_eq!(0.7599, r(Quartic::Out.tween(0.3)));
        assert_eq!(0.8704, r(Quartic::Out.tween(0.4)));
        assert_eq!(0.9375, r(Quartic::Out.tween(0.5)));
        assert_eq!(0.9744, r(Quartic::Out.tween(0.6)));
        assert_eq!(0.9919, r(Quartic::Out.tween(0.7)));
        assert_eq!(0.9984, r(Quartic::Out.tween(0.8)));
        assert_eq!(0.9999, r(Quartic::Out.tween(0.9)));
        assert_eq!(1.0000, r(Quartic::Out.tween(1.0)));
    }

    #[test]
    // Modeled after the piecewise quartic
    // y = (1/2)((2x)^4)       ; [0, 0.5]
    // y = -(1/2)((2x-2)^4 -2) ; [0.5, 1]
    fn quartic_inout() {
        assert_eq!(0.0000, r(Quartic::InOut.tween(0.0)));
        assert_eq!(0.0008, r(Quartic::InOut.tween(0.1)));
        assert_eq!(0.0128, r(Quartic::InOut.tween(0.2)));
        assert_eq!(0.0648, r(Quartic::InOut.tween(0.3)));
        assert_eq!(0.2048, r(Quartic::InOut.tween(0.4)));
        assert_eq!(0.5000, r(Quartic::InOut.tween(0.5)));
        assert_eq!(0.7952, r(Quartic::InOut.tween(0.6)));
        assert_eq!(0.9352, r(Quartic::InOut.tween(0.7)));
        assert_eq!(0.9872, r(Quartic::InOut.tween(0.8)));
        assert_eq!(0.9992, r(Quartic::InOut.tween(0.9)));
        assert_eq!(1.0000, r(Quartic::InOut.tween(1.0)));
    }

    #[test]
    // Modeled after the quartic y = x^5
    fn quintic_in() {
        assert_eq!(0.00000, r(Quintic::In.tween(0.0)));
        assert_eq!(0.00001, r(Quintic::In.tween(0.1)));
        assert_eq!(0.00032, r(Quintic::In.tween(0.2)));
        assert_eq!(0.00243, r(Quintic::In.tween(0.3)));
        assert_eq!(0.01024, r(Quintic::In.tween(0.4)));
        assert_eq!(0.03125, r(Quintic::In.tween(0.5)));
        assert_eq!(0.07776, r(Quintic::In.tween(0.6)));
        assert_eq!(0.16807, r(Quintic::In.tween(0.7)));
        assert_eq!(0.32768, r(Quintic::In.tween(0.8)));
        assert_eq!(0.59049, r(Quintic::In.tween(0.9)));
        assert_eq!(1.00000, r(Quintic::In.tween(1.0)));
    }

    #[test]
    // Modeled after the quintic y = (x - 1)^5 + 1
    fn quintic_out() {
        assert_eq!(0.00000, r(Quintic::Out.tween(0.0)));
        assert_eq!(0.40951, r(Quintic::Out.tween(0.1)));
        assert_eq!(0.67232, r(Quintic::Out.tween(0.2)));
        assert_eq!(0.83193, r(Quintic::Out.tween(0.3)));
        assert_eq!(0.92224, r(Quintic::Out.tween(0.4)));
        assert_eq!(0.96875, r(Quintic::Out.tween(0.5)));
        assert_eq!(0.98976, r(Quintic::Out.tween(0.6)));
        assert_eq!(0.99757, r(Quintic::Out.tween(0.7)));
        assert_eq!(0.99968, r(Quintic::Out.tween(0.8)));
        assert_eq!(0.99999, r(Quintic::Out.tween(0.9)));
        assert_eq!(1.00000, r(Quintic::Out.tween(1.0)));
    }

    #[test]
    // Modeled after the piecewise quintic
    // y = (1/2)((2x)^5)       ; [0, 0.5]
    // y = (1/2)((2x-2)^5 + 2) ; [0.5, 1]
    fn quintic_inout() {
        assert_eq!(0.00000, r(Quintic::InOut.tween(0.0)));
        assert_eq!(0.00016, r(Quintic::InOut.tween(0.1)));
        assert_eq!(0.00512, r(Quintic::InOut.tween(0.2)));
        assert_eq!(0.03888, r(Quintic::InOut.tween(0.3)));
        assert_eq!(0.16384, r(Quintic::InOut.tween(0.4)));
        assert_eq!(0.50000, r(Quintic::InOut.tween(0.5)));
        assert_eq!(0.83616, r(Quintic::InOut.tween(0.6)));
        assert_eq!(0.96112, r(Quintic::InOut.tween(0.7)));
        assert_eq!(0.99488, r(Quintic::InOut.tween(0.8)));
        assert_eq!(0.99984, r(Quintic::InOut.tween(0.9)));
        assert_eq!(1.00000, r(Quintic::InOut.tween(1.0)));
    }

    #[test]
    // Modeled after eighth sinusoidal wave y = 1 - cos((x * PI) / 2)
    fn sinusoidal_in() {
        assert_eq!(0.000000, r(Sinusoidal::In.tween(0.0)));
        assert_eq!(0.012312, r(Sinusoidal::In.tween(0.1)));
        assert_eq!(0.048943, r(Sinusoidal::In.tween(0.2)));
        assert_eq!(0.108993, r(Sinusoidal::In.tween(0.3)));
        assert_eq!(0.190983, r(Sinusoidal::In.tween(0.4)));
        assert_eq!(0.292893, r(Sinusoidal::In.tween(0.5)));
        assert_eq!(0.412215, r(Sinusoidal::In.tween(0.6)));
        assert_eq!(0.546010, r(Sinusoidal::In.tween(0.7)));
        assert_eq!(0.690983, r(Sinusoidal::In.tween(0.8)));
        assert_eq!(0.843566, r(Sinusoidal::In.tween(0.9)));
        assert_eq!(1.000000, r(Sinusoidal::In.tween(1.0)));
    }

    #[test]
    // Modeled after eigth sinusoidal wave y = sin((x * PI) / 2)
    fn sinusoidal_out() {
        assert_eq!(0.000000, r(Sinusoidal::Out.tween(0.0)));
        assert_eq!(0.156434, r(Sinusoidal::Out.tween(0.1)));
        assert_eq!(0.309017, r(Sinusoidal::Out.tween(0.2)));
        assert_eq!(0.453991, r(Sinusoidal::Out.tween(0.3)));
        assert_eq!(0.587785, r(Sinusoidal::Out.tween(0.4)));
        assert_eq!(0.707107, r(Sinusoidal::Out.tween(0.5)));
        assert_eq!(0.809017, r(Sinusoidal::Out.tween(0.6)));
        assert_eq!(0.891007, r(Sinusoidal::Out.tween(0.7)));
        assert_eq!(0.951057, r(Sinusoidal::Out.tween(0.8)));
        assert_eq!(0.987688, r(Sinusoidal::Out.tween(0.9)));
        assert_eq!(1.000000, r(Sinusoidal::Out.tween(1.0)));
    }

    #[test]
    // Modeled after quarter sinusoidal wave y = -0.5 * (cos(x * PI) - 1);
    fn sinusoidal_inout() {
        assert_eq!(0.000000, r(Sinusoidal::InOut.tween(0.0)));
        assert_eq!(0.024472, r(Sinusoidal::InOut.tween(0.1)));
        assert_eq!(0.095492, r(Sinusoidal::InOut.tween(0.2)));
        assert_eq!(0.206107, r(Sinusoidal::InOut.tween(0.3)));
        assert_eq!(0.345492, r(Sinusoidal::InOut.tween(0.4)));
        assert_eq!(0.500000, r(Sinusoidal::InOut.tween(0.5)));
        assert_eq!(0.654509, r(Sinusoidal::InOut.tween(0.6)));
        assert_eq!(0.793893, r(Sinusoidal::InOut.tween(0.7)));
        assert_eq!(0.904509, r(Sinusoidal::InOut.tween(0.8)));
        assert_eq!(0.975528, r(Sinusoidal::InOut.tween(0.9)));
        assert_eq!(1.000000, r(Sinusoidal::InOut.tween(1.0)));
    }

    // Modeled after the piecewise exponential
    // y = 0            ; [0, 0]
    // y = 2^(10x-10)   ; [0, 1]
    fn exponential_in() {
        assert_eq!(0.000000, r(Exponential::In.tween(0.0)));
        assert_eq!(0.001953, r(Exponential::In.tween(0.1)));
        assert_eq!(0.003906, r(Exponential::In.tween(0.2)));
        assert_eq!(0.007813, r(Exponential::In.tween(0.3)));
        assert_eq!(0.015625, r(Exponential::In.tween(0.4)));
        assert_eq!(0.031250, r(Exponential::In.tween(0.5)));
        assert_eq!(0.062500, r(Exponential::In.tween(0.6)));
        assert_eq!(0.125000, r(Exponential::In.tween(0.7)));
        assert_eq!(0.250000, r(Exponential::In.tween(0.8)));
        assert_eq!(0.500000, r(Exponential::In.tween(0.9)));
        assert_eq!(1.000000, r(Exponential::In.tween(1.0)));
    }

    // Modeled after the piecewise exponential
    // y = 1 - 2^(-10x)  ; [0, 1]
    // y = 1             ; [1, 1]
    fn exponential_out() {
        assert_eq!(0.000000, r(Exponential::Out.tween(0.0)));
        assert_eq!(0.050000, r(Exponential::Out.tween(0.1)));
        assert_eq!(0.750000, r(Exponential::Out.tween(0.2)));
        assert_eq!(0.875000, r(Exponential::Out.tween(0.3)));
        assert_eq!(0.937500, r(Exponential::Out.tween(0.4)));
        assert_eq!(0.968750, r(Exponential::Out.tween(0.5)));
        assert_eq!(0.984375, r(Exponential::Out.tween(0.6)));
        assert_eq!(0.992188, r(Exponential::Out.tween(0.7)));
        assert_eq!(0.996094, r(Exponential::Out.tween(0.8)));
        assert_eq!(0.998047, r(Exponential::Out.tween(0.9)));
        assert_eq!(1.000000, r(Exponential::Out.tween(1.0)));
    }

    // Modeled after the piecewise exponential
    // y = 0                        ; [0, 0  ]
    // y = 2^(20x - 10) / 2         ; [0, 0.5]
    // y = 1 - 0.5*2^(-20x + 10))   ; [0.5, 1]
    // y = 1                        ; [1, 1  ]
    fn exponential_inout() {
        assert_eq!(0.000000, r(Exponential::InOut.tween(0.0)));
        assert_eq!(0.003906, r(Exponential::InOut.tween(0.1)));
        assert_eq!(0.015625, r(Exponential::InOut.tween(0.2)));
        assert_eq!(0.062500, r(Exponential::InOut.tween(0.3)));
        assert_eq!(0.250000, r(Exponential::InOut.tween(0.4)));
        assert_eq!(0.500000, r(Exponential::InOut.tween(0.5)));
        assert_eq!(0.654509, r(Exponential::InOut.tween(0.6)));
        assert_eq!(0.793893, r(Exponential::InOut.tween(0.7)));
        assert_eq!(0.904509, r(Exponential::InOut.tween(0.8)));
        assert_eq!(0.975528, r(Exponential::InOut.tween(0.9)));
        assert_eq!(1.000000, r(Exponential::InOut.tween(1.0)));
    }
}
