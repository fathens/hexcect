use std::ops::Add;

use crate::model::*;
use measure_units::*;

use nalgebra::{matrix, RealField};
use num_traits::{Float, FromPrimitive};

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

/**
ジャイロの微小角を現在の姿勢の回転に足し込む。
See: http://watako-lab.com/2019/02/28/3axis_gyro/
*/
pub fn gyro_delta<V>(base: &Radians3D<V>, e: &Radians3D<V>) -> Radians3D<V>
where
    V: Float,
    V: RealField,
    V: From<Radians<V>>,
{
    let sin_cos = |r: Radians<V>| (r.sin(), r.cos());

    let (s_r, c_r) = sin_cos(base.roll());
    let (s_p, c_p) = sin_cos(base.pitch());

    let zero = V::zero();
    let one = V::one();

    let m = matrix![
        one, s_r * s_p / c_p, c_r * s_p / c_p;
        zero, c_r, -s_r;
        zero, s_r / c_p, c_r / c_p;
    ];
    (m * e.as_matrix()).into()
}

pub fn rotate<V, A>(src: &Vector3D<A>, ds: &Radians3D<V>) -> Vector3D<A>
where
    V: Float,
    V: FromPrimitive,
    V: RealField,
    V: From<Degrees<V>>,
    V: From<Scalar<V>>,
    V: From<A>,
    A: Copy,
    A: From<V>,
{
    let zero = V::zero();
    let one = V::one();
    let sin_cos = |r: Radians<V>| (r.sin(), r.cos());

    let roll = {
        let (sin, cos) = sin_cos(ds.roll());
        matrix![
            one, zero, zero;
            zero, cos, -sin;
            zero, sin, cos;
        ]
    };

    let pitch = {
        let (sin, cos) = sin_cos(ds.pitch());
        matrix![
            cos, zero, sin;
            zero, one, zero;
            -sin, zero, cos;
        ]
    };

    let yaw = {
        let (sin, cos) = sin_cos(ds.yaw());
        matrix![
            cos, -sin, zero;
            sin, cos, zero;
            zero, zero, zero;
        ]
    };

    (yaw * pitch * roll * src.as_matrix()).into()
}

/**
垂直に対する回転を求める。
See: http://watako-lab.com/2019/02/15/3axis_acc/
*/
pub fn vector_angle<F, A>(v: &Vector3D<A>) -> Radians3D<F>
where
    A: Copy,
    F: Float,
    F: From<A>,
    F: Into<Radians<F>>,
{
    let x: F = v.x().into();
    let y: F = v.y().into();
    let z: F = v.z().into();

    let roll = y.atan2(z);
    let pitch = (-x).atan2((y.powi(2) + z.powi(2)).sqrt());
    let yaw = F::zero();

    Radians3D::new(roll.into(), pitch.into(), yaw.into())
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
    use approx::assert_ulps_eq;
    use nalgebra::vector;

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

    #[test]
    fn gyro_delta_simple() {
        let e = Radians3D::new(5_f64.into(), 6_f64.into(), 7_f64.into());
        let base = Radians3D::new(1_f64.into(), 2_f64.into(), 3_f64.into());
        let delta = gyro_delta(&base, &e);

        let r = base.roll();
        let p = base.pitch();

        let m = matrix![
            1.0, r.sin() * p.sin() / p.cos(), r.cos() * p.sin() / p.cos();
            0.0, r.cos(), -r.sin();
            0.0, r.sin() / p.cos(), r.cos() / p.cos();
        ];
        let v = vector![5_f64, 6_f64, 7_f64];
        let a = m * v;

        assert_ulps_eq!(a.x, delta.roll().into());
        assert_ulps_eq!(a.y, delta.pitch().into());
        assert_ulps_eq!(a.z, delta.yaw().into());
    }

    #[test]
    fn rotate_simple() {
        let src = Vector3D::new(1_f64.meters(), 2_f64.meters(), 3_f64.meters());
        let dst = rotate(
            &src,
            &Degrees3D::new(10_f64.into(), 15_f64.into(), 20_f64.into()).into(),
        );

        let roll = 10_f64.to_radians();
        let pitch = 15_f64.to_radians();
        let yaw = 20_f64.to_radians();

        let r_x = matrix![
            1.0, 0.0, 0.0;
            0.0, roll.cos(), -roll.sin();
            0.0, roll.sin(), roll.cos();
        ];
        let r_y = matrix![
            pitch.cos(), 0.0, pitch.sin();
            0.0, 1.0, 0.0;
            -pitch.sin(), 0.0, pitch.cos();
        ];
        let r_z = matrix![
            yaw.cos(), -yaw.sin(), 0.0;
            yaw.sin(), yaw.cos(), 0.0;
            0.0, 0.0, 0.0;
        ];
        let v = vector![1_f64, 2_f64, 3_f64];

        let check = |a: nalgebra::Vector3<f64>| {
            assert_ulps_eq!(a.x, dst.x().into());
            assert_ulps_eq!(a.y, dst.y().into());
            assert_ulps_eq!(a.z, dst.z().into());
        };

        check(r_z * r_y * r_x * v);
        check((r_z * r_y) * (r_x * v));
        check(r_z * r_y * (r_x * v));
        check(r_z * (r_y * (r_x * v)));
        check(r_z * (r_y * r_x * v));
        check(r_z * (r_y * r_x) * v);
    }
}
