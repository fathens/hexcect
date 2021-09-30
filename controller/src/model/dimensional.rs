use std::ops::{Add, Div, Mul, Sub};

use super::*;

use derive_more::Constructor;
use getset::CopyGetters;
use num_traits::FloatConst;

#[derive(Debug, PartialEq, Eq, Constructor, CopyGetters)]
#[get_copy = "pub"]
pub struct Gyro3D<V: Copy + FloatConst> {
    x: AngleVelocity<V>,
    y: AngleVelocity<V>,
    z: AngleVelocity<V>,
}

#[derive(Debug, PartialEq, Eq, Constructor, CopyGetters)]
#[get_copy = "pub"]
pub struct Accel3D<V: Copy> {
    x: Accel<V>,
    y: Accel<V>,
    z: Accel<V>,
}

impl<V: Copy> From<Accel3D<V>> for Vector3D<V>
where
    V: From<Accel<V>>,
{
    fn from(src: Accel3D<V>) -> Self {
        Vector3D::new(src.x().into(), src.y().into(), src.z().into())
    }
}

#[derive(Debug, PartialEq, Eq, Constructor, CopyGetters)]
#[get_copy = "pub"]
pub struct Vector3D<V: Copy> {
    x: V,
    y: V,
    z: V,
}

impl<V: Copy, Rhs> Add<Rhs> for Vector3D<V>
where
    V: Add<Output = V>,
    Rhs: Into<Self>,
{
    type Output = Self;

    fn add(self, rhs: Rhs) -> Self::Output {
        let o: Self = rhs.into();
        Vector3D::new(self.x + o.x, self.y + o.y, self.z + o.z)
    }
}

impl<V: Copy, Rhs> Sub<Rhs> for Vector3D<V>
where
    V: Sub<Output = V>,
    Rhs: Into<Self>,
{
    type Output = Self;

    fn sub(self, rhs: Rhs) -> Self::Output {
        let o: Self = rhs.into();
        Vector3D::new(self.x - o.x, self.y - o.y, self.z - o.z)
    }
}

impl<V: Copy> Mul<V> for Vector3D<V>
where
    V: Mul<Output = V>,
{
    type Output = Self;

    fn mul(self, rhs: V) -> Self::Output {
        Vector3D::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl<V: Copy> Div<V> for Vector3D<V>
where
    V: Div<Output = V>,
{
    type Output = Self;

    fn div(self, rhs: V) -> Self::Output {
        Vector3D::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accel_to_vector() {
        let a = Accel3D::new(1_f64.into(), 2_f64.into(), 3_f64.into());
        let b: Vector3D<f64> = a.into();
        assert_eq!(b.x(), 1.0);
        assert_eq!(b.y(), 2.0);
        assert_eq!(b.z(), 3.0);
    }

    #[test]
    fn vector_add() {
        let a = Vector3D::new(1_f64, 2_f64, 3_f64);
        let b = Vector3D::new(10_f64, 20_f64, 30_f64);
        let c = a + b;
        assert_eq!(c.x(), 11.0);
        assert_eq!(c.y(), 22.0);
        assert_eq!(c.z(), 33.0);
    }

    #[test]
    fn vector_add_accel() {
        let a = Vector3D::new(1_f64, 2_f64, 3_f64);
        let b = Accel3D::new(10_f64.into(), 20_f64.into(), 30_f64.into());
        let c = a + b;
        assert_eq!(c.x(), 11.0);
        assert_eq!(c.y(), 22.0);
        assert_eq!(c.z(), 33.0);
    }

    #[test]
    fn vector_sub() {
        let a = Vector3D::new(1_f64, 2_f64, 3_f64);
        let b = Vector3D::new(10_f64, 20_f64, 30_f64);
        let c = a - b;
        assert_eq!(c.x(), -9.0);
        assert_eq!(c.y(), -18.0);
        assert_eq!(c.z(), -27.0);
    }

    #[test]
    fn vector_sub_accel() {
        let a = Vector3D::new(1_f64, 2_f64, 3_f64);
        let b = Accel3D::new(10_f64.into(), 20_f64.into(), 30_f64.into());
        let c = a - b;
        assert_eq!(c.x(), -9.0);
        assert_eq!(c.y(), -18.0);
        assert_eq!(c.z(), -27.0);
    }

    #[test]
    fn vector_mul() {
        let a = Vector3D::new(1_f64, 2_f64, 3_f64);
        let b = a * 1.5;
        assert_eq!(b.x(), 1.5);
        assert_eq!(b.y(), 3.0);
        assert_eq!(b.z(), 4.5);
    }

    #[test]
    fn vector_div() {
        let a = Vector3D::new(1_f64, 2_f64, 3_f64);
        let b = a / 2.0;
        assert_eq!(b.x(), 0.5);
        assert_eq!(b.y(), 1.0);
        assert_eq!(b.z(), 1.5);
    }
}
