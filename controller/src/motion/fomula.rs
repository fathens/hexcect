use std::ops::Add;

use crate::model::*;
use measure_units::*;

use num_traits::{Float, FloatConst, FromPrimitive};

pub fn integral_dur<V, A, D>(dur: D, prev: UnitsDiv<V, A, D>, next: UnitsDiv<V, A, D>) -> A
where
    V: Float,
    V: FromPrimitive,
    V: From<UnitsDiv<V, A, D>>,
    V: From<Scalar<V>>,
    V: From<D>,
    A: Copy,
    A: From<V>,
    A: CalcMix<V>,
    A: Add<Output = A>,
    D: Copy,
    D: From<V>,
    D: CalcMix<V>,
    D: Duration<V>,
{
    if prev.is_sign_positive() == next.is_sign_positive() {
        product_dur(half(prev + next), dur)
    } else {
        let in_rate = |a: UnitsDiv<V, A, D>| {
            if a.is_zero() {
                V::zero().into()
            } else {
                let total = prev.abs() + next.abs();
                let r = (a.abs() / total).reduction();
                product_dur(half(a), dur.calc_mul(r).scalar())
            }
        };

        in_rate(prev) + in_rate(next)
    }
}

pub fn rotate<V, A>(_src: &Vector3D<A>, _ds: &Angle3D<Degrees<V>>) -> Vector3D<A>
where
    V: Float,
    V: FloatConst,
    A: Copy,
{
    todo!()
}

fn product_dur<V, A, D>(moving: UnitsDiv<V, A, D>, dur: D) -> A
where
    V: Float,
    V: From<UnitsDiv<V, A, D>>,
    V: From<D>,
    A: From<V>,
    D: CalcMix<V>,
{
    let a = dur.calc_mul(moving); // S * (A / S)
    a.infuse_nmr().reduction_left()
}

fn half<V, C>(a: C) -> C
where
    V: Float,
    V: From<C>,
    V: From<Scalar<V>>,
    C: From<V>,
    C: CalcMix<V>,
{
    let two: Scalar<V> = (V::one() + V::one()).into();
    a.calc_div(two).scalar()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integral_accel_trapezoid_positive() {
        let dur: Seconds<f32> = 1.0.into();
        let prev: Accel<f32> = 3.0.into();
        let next: Accel<f32> = 6.0.into();
        let r = integral_dur(dur, prev, next);
        assert_eq!(r, 4.5.into());
    }

    #[test]
    fn integral_accel_trapezoid_negative() {
        let dur: Seconds<f32> = 1.0.into();
        let prev: Accel<f32> = (-3.0).into();
        let next: Accel<f32> = (-6.0).into();
        let r = integral_dur(dur, prev, next);
        assert_eq!(r, (-4.5).into());
    }

    #[test]
    fn integral_accel_prev_only() {
        let dur: Seconds<f32> = 1.0.into();
        let prev: Accel<f32> = 3.0.into();
        let next: Accel<f32> = 0.0.into();
        let r = integral_dur(dur, prev, next);
        assert_eq!(r, 1.5.into());
    }

    #[test]
    fn integral_accel_next_only() {
        let dur: Seconds<f32> = 1.0.into();
        let prev: Accel<f32> = 0.0.into();
        let next: Accel<f32> = 4.0.into();
        let r = integral_dur(dur, prev, next);
        assert_eq!(r, 2.0.into());
    }

    #[test]
    fn integral_accel_triangles() {
        let dur: Seconds<f32> = 3.0.into();
        let prev: Accel<f32> = (-2.0).into();
        let next: Accel<f32> = 4.0.into();
        let r = integral_dur(dur, prev, next);
        assert_eq!(r, 3.0.into());
    }

    #[test]
    fn integral_accel_zero() {
        let dur: Seconds<f32> = 1.0.into();
        let prev: Accel<f32> = 0.0.into();
        let next: Accel<f32> = 0.0.into();
        let r = integral_dur(dur, prev, next);
        assert_eq!(r, 0.0.into());
    }
}
