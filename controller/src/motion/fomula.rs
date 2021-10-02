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
    let (same_sign, prev_is_zero, next_is_zero) = {
        let p_v: V = prev.into();
        let n_v: V = next.into();
        (p_v.signum() == n_v.signum(), p_v.is_zero(), n_v.is_zero())
    };

    if same_sign {
        get_speed(half(prev + next), dur.to_seconds())
    } else {
        let s = dur.to_seconds();
        let a = if prev_is_zero {
            V::zero().into()
        } else if next_is_zero {
            get_speed(half(prev), s)
        } else {
            let s = s * (prev / next).reduction();
            get_speed(half(prev), s.scalar())
        };
        let b = if next_is_zero {
            V::zero().into()
        } else if prev_is_zero {
            get_speed(half(prev), s)
        } else {
            let s = s * (next / prev).reduction();
            get_speed(half(prev), s.scalar())
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
