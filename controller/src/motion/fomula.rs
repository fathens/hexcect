use std::ops::Div;

use crate::model::*;
use measure_units::*;

use num_traits::Float;

pub fn get_speed<V>(accel: Accel<V>, time: Seconds<V>) -> Speed<V>
where
    V: num_traits::Float,
    V: From<Seconds<V>>,
    V: From<Accel<V>>,
{
    let a = time * accel; // S * ((M / S) / S)
    a.infuse_nmr().reduction_left()
}

pub fn integral_accel<V, D>(dur: D, prev: Accel<V>, next: Accel<V>) -> Speed<V>
where
    V: Float,
    V: From<Speed<V>>,
    V: From<Accel<V>>,
    V: From<Scalar<V>>,
    V: From<Seconds<V>>,
    D: Copy,
    D: Duration<V>,
{
    if prev.is_sign_positive() == next.is_sign_positive() {
        get_speed(half(prev + next), dur.to_seconds())
    } else {
        let in_rate = |a: Accel<V>| {
            if a.is_zero() {
                V::zero().into()
            } else {
                let r = (a.abs() / (prev.abs() + next.abs())).reduction();
                get_speed(half(a), (dur.to_seconds() * r).scalar())
            }
        };

        in_rate(prev) + in_rate(next)
    }
}

fn half<V, C>(a: C) -> C
where
    V: Float,
    C: From<V>,
    C: Div<Scalar<V>, Output = UnitsDiv<V, C, Scalar<V>>>,
{
    let two: Scalar<V> = (V::one() + V::one()).into();
    (a / two).scalar()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integral_accel_trapezoid_positive() {
        let dur: Seconds<f32> = 1.0.into();
        let prev: Accel<f32> = 3.0.into();
        let next: Accel<f32> = 6.0.into();
        let r = integral_accel(dur, prev, next);
        assert_eq!(r, 4.5.into());
    }

    #[test]
    fn integral_accel_trapezoid_negative() {
        let dur: Seconds<f32> = 1.0.into();
        let prev: Accel<f32> = (-3.0).into();
        let next: Accel<f32> = (-6.0).into();
        let r = integral_accel(dur, prev, next);
        assert_eq!(r, (-4.5).into());
    }

    #[test]
    fn integral_accel_prev_only() {
        let dur: Seconds<f32> = 1.0.into();
        let prev: Accel<f32> = 3.0.into();
        let next: Accel<f32> = 0.0.into();
        let r = integral_accel(dur, prev, next);
        assert_eq!(r, 1.5.into());
    }

    #[test]
    fn integral_accel_next_only() {
        let dur: Seconds<f32> = 1.0.into();
        let prev: Accel<f32> = 0.0.into();
        let next: Accel<f32> = 4.0.into();
        let r = integral_accel(dur, prev, next);
        assert_eq!(r, 2.0.into());
    }

    #[test]
    fn integral_accel_triangles() {
        let dur: Seconds<f32> = 3.0.into();
        let prev: Accel<f32> = (-2.0).into();
        let next: Accel<f32> = 4.0.into();
        let r = integral_accel(dur, prev, next);
        assert_eq!(r, 3.0.into());
    }

    #[test]
    fn integral_accel_zero() {
        let dur: Seconds<f32> = 1.0.into();
        let prev: Accel<f32> = 0.0.into();
        let next: Accel<f32> = 0.0.into();
        let r = integral_accel(dur, prev, next);
        assert_eq!(r, 0.0.into());
    }
}
