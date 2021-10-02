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
    D: Duration<V>,
{
    if prev.is_sign_positive() == next.is_sign_positive() {
        get_speed(half(prev + next), dur.to_seconds())
    } else {
        let a_p = prev.abs();
        let a_n = next.abs();

        let s = dur.to_seconds();
        let a = if prev.is_zero() {
            V::zero().into()
        } else if next.is_zero() {
            get_speed(half(prev), s)
        } else {
            let s = s * (a_p / (a_p + a_n)).reduction();
            get_speed(half(prev), s.scalar())
        };
        let b = if next.is_zero() {
            V::zero().into()
        } else if prev.is_zero() {
            get_speed(half(next), s)
        } else {
            let s = s * (a_n / (a_p + a_n)).reduction();
            get_speed(half(next), s.scalar())
        };
        a + b
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
}
