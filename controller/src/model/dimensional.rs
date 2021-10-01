use std::ops::{Add, Div, Mul, Sub};

use super::*;
use hardware::model::sensor::{AccelInfo, GyroInfo};

use derive_more::Constructor;
use getset::CopyGetters;
use num_traits::FloatConst;

// ================================================================

#[derive(Debug, PartialEq, Eq, Constructor, CopyGetters)]
#[get_copy = "pub"]
pub struct Gyro3D<V: Copy + FloatConst> {
    x: AngleVelocity<V>,
    y: AngleVelocity<V>,
    z: AngleVelocity<V>,
}

impl<V: Copy + FloatConst> From<GyroInfo<V>> for Gyro3D<V> {
    fn from(src: GyroInfo<V>) -> Self {
        Gyro3D::new(src.x().into(), src.y().into(), src.z().into())
    }
}

// ================================================================

pub type Accel3D<V> = Vector3D<Accel<V>>;

impl<V: Copy> From<AccelInfo<V>> for Accel3D<V> {
    fn from(src: AccelInfo<V>) -> Self {
        Vector3D::new(src.x().into(), src.y().into(), src.z().into())
    }
}

// ================================================================

#[derive(Debug, PartialEq, Eq, Constructor, CopyGetters)]
#[get_copy = "pub"]
pub struct Vector3D<V: Copy> {
    x: V,
    y: V,
    z: V,
}

impl<V: Copy> Vector3D<V> {
    pub fn apply<U: Copy>(self, f: impl Fn(V) -> U) -> Vector3D<U> {
        Vector3D::new(f(self.x), f(self.y), f(self.z))
    }
}

impl<V: Copy, O: Copy> Add<Vector3D<O>> for Vector3D<V>
where
    V: Add<O>,
    V::Output: Copy,
{
    type Output = Vector3D<V::Output>;

    fn add(self, rhs: Vector3D<O>) -> Self::Output {
        Vector3D::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<V: Copy, O: Copy> Sub<Vector3D<O>> for Vector3D<V>
where
    V: Sub<O>,
    V::Output: Copy,
{
    type Output = Vector3D<V::Output>;

    fn sub(self, rhs: Vector3D<O>) -> Self::Output {
        Vector3D::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl<V: Copy, O: Copy> Mul<O> for Vector3D<V>
where
    V: Mul<O>,
    V::Output: Copy,
{
    type Output = Vector3D<V::Output>;

    fn mul(self, rhs: O) -> Self::Output {
        Vector3D::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl<V: Copy, O: Copy> Div<O> for Vector3D<V>
where
    V: Div<O>,
    V::Output: Copy,
{
    type Output = Vector3D<V::Output>;

    fn div(self, rhs: O) -> Self::Output {
        Vector3D::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

// ================================================================

#[derive(Debug, PartialEq, Eq, Constructor, CopyGetters)]
#[get_copy = "pub"]
pub struct Position3D<V: Copy> {
    x: V,
    y: V,
    z: V,
}

impl<V: Copy> Position3D<V> {
    pub fn apply<U: Copy>(self, f: impl Fn(V) -> U) -> Position3D<U> {
        Position3D::new(f(self.x), f(self.y), f(self.z))
    }
}

impl<V: Copy, O: Copy> Add<Vector3D<O>> for Position3D<V>
where
    V: Add<O>,
    V::Output: Copy,
{
    type Output = Position3D<V::Output>;

    fn add(self, rhs: Vector3D<O>) -> Self::Output {
        Position3D::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<V: Copy, O: Copy> Sub<Vector3D<O>> for Position3D<V>
where
    V: Sub<O>,
    V::Output: Copy,
{
    type Output = Position3D<V::Output>;

    fn sub(self, rhs: Vector3D<O>) -> Self::Output {
        Position3D::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl<V: Copy, O: Copy> Mul<O> for Position3D<V>
where
    V: Mul<Scalar<O>>,
    V::Output: Into<UnitsMul<O, V, Scalar<O>>>,
    V: From<O>,
    O: Into<Scalar<O>>,
{
    type Output = Position3D<V>;

    fn mul(self, rhs: O) -> Self::Output {
        let s: Scalar<O> = rhs.into();
        let f = |v: V| (v * s).into().scalar();
        Position3D::new(f(self.x), f(self.y), f(self.z))
    }
}

impl<V: Copy, O: Copy> Div<O> for Position3D<V>
where
    V: Div<Scalar<O>>,
    V::Output: Into<UnitsDiv<O, V, Scalar<O>>>,
    V: From<O>,
    O: Into<Scalar<O>>,
{
    type Output = Position3D<V>;

    fn div(self, rhs: O) -> Self::Output {
        let s: Scalar<O> = rhs.into();
        let f = |v: V| (v / s).into().scalar();
        Position3D::new(f(self.x), f(self.y), f(self.z))
    }
}

// ================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accel_to_vector() {
        let a = Accel3D::new(1_f64.into(), 2_f64.into(), 3_f64.into());
        let b: Vector3D<Accel<f64>> = a.into();
        assert_eq!(b.x(), 1.0.into());
        assert_eq!(b.y(), 2.0.into());
        assert_eq!(b.z(), 3.0.into());
    }

    #[test]
    fn vector_add() {
        let a = Vector3D::new(1_f64.meters(), 2_f64.meters(), 3_f64.meters());
        let b = Vector3D::new(10_f64.meters(), 20_f64.meters(), 30_f64.meters());
        let c = a + b;
        assert_eq!(c.x(), 11.0.meters());
        assert_eq!(c.y(), 22.0.meters());
        assert_eq!(c.z(), 33.0.meters());
    }

    #[test]
    fn vector_add_accel() {
        let a = Accel3D::new(1_f64.into(), 2_f64.into(), 3_f64.into());
        let b = Accel3D::new(10_f64.into(), 20_f64.into(), 30_f64.into());
        let x: Vector3D<Accel<f64>> = a.into();
        let y: Vector3D<Accel<f64>> = b.into();
        let r: Accel3D<f64> = (x + y).into();
        assert_eq!(r.x(), 11.0.into());
        assert_eq!(r.y(), 22.0.into());
        assert_eq!(r.z(), 33.0.into());
    }

    #[test]
    fn vector_sub() {
        let a = Vector3D::new(1_f64.meters(), 2_f64.meters(), 3_f64.meters());
        let b = Vector3D::new(10_f64.meters(), 20_f64.meters(), 30_f64.meters());
        let c = a - b;
        assert_eq!(c.x(), (-9.0).meters());
        assert_eq!(c.y(), (-18.0).meters());
        assert_eq!(c.z(), (-27.0).meters());
    }

    #[test]
    fn vector_sub_accel() {
        let a = Accel3D::new(1_f64.into(), 2_f64.into(), 3_f64.into());
        let b = Accel3D::new(10_f64.into(), 20_f64.into(), 30_f64.into());
        let x: Vector3D<Accel<f64>> = a.into();
        let y: Vector3D<Accel<f64>> = b.into();
        let r: Accel3D<f64> = (x - y).into();
        assert_eq!(r.x(), (-9.0).into());
        assert_eq!(r.y(), (-18.0).into());
        assert_eq!(r.z(), (-27.0).into());
    }

    #[test]
    fn vector_mul() {
        let a = Vector3D::new(1_f64.meters(), 2_f64.meters(), 3_f64.meters());
        let b = a * Scalar::from(1.5);
        let r = b.apply(|v| v.scalar());
        assert_eq!(r.x(), 1.5.meters());
        assert_eq!(r.y(), 3.0.meters());
        assert_eq!(r.z(), 4.5.meters());
    }

    #[test]
    fn vector_div() {
        let a = Vector3D::new(1_f64.meters(), 2_f64.meters(), 3_f64.meters());
        let b = a / Scalar::from(2.0);
        let r = b.apply(|v| v.scalar());
        assert_eq!(r.x(), 0.5.meters());
        assert_eq!(r.y(), 1.0.meters());
        assert_eq!(r.z(), 1.5.meters());
    }

    #[test]
    fn position_add() {
        let a = Position3D::new(1_f64.meters(), 2_f64.meters(), 3_f64.meters());
        let b = Vector3D::new(10_f64.meters(), 20_f64.meters(), 30_f64.meters());
        let c = a + b;
        assert_eq!(c.x(), 11.0.meters());
        assert_eq!(c.y(), 22.0.meters());
        assert_eq!(c.z(), 33.0.meters());
    }

    #[test]
    fn position_sub() {
        let a = Position3D::new(1_f64.meters(), 2_f64.meters(), 3_f64.meters());
        let b = Vector3D::new(10_f64.meters(), 20_f64.meters(), 30_f64.meters());
        let c = a - b;
        assert_eq!(c.x(), (-9.0).meters());
        assert_eq!(c.y(), (-18.0).meters());
        assert_eq!(c.z(), (-27.0).meters());
    }

    #[test]
    fn position_mul() {
        let a = Position3D::new(1_f64.meters(), 2_f64.meters(), 3_f64.meters());
        let r = a * 1.5;
        assert_eq!(r.x(), 1.5.meters());
        assert_eq!(r.y(), 3.0.meters());
        assert_eq!(r.z(), 4.5.meters());
    }

    #[test]
    fn position_div() {
        let a = Position3D::new(1_f64.meters(), 2_f64.meters(), 3_f64.meters());
        let r = a / 2.0;
        assert_eq!(r.x(), 0.5.meters());
        assert_eq!(r.y(), 1.0.meters());
        assert_eq!(r.z(), 1.5.meters());
    }

    #[test]
    fn position_convert() {
        let a = Position3D::new(1_f64.meters(), 2_f64.meters(), 3_f64.meters());
        let b: Position3D<Millimeters<f64>> = a.apply(|v| v.into());
        assert_eq!(b.x(), 1000_f64.millimeters());
        assert_eq!(b.y(), 2000_f64.millimeters());
        assert_eq!(b.z(), 3000_f64.millimeters());
    }

    #[test]
    fn gyro_from_info() {
        let info = GyroInfo::new(1_f64, 2_f64, 3_f64);
        let a: Gyro3D<f64> = info.into();
        assert_eq!(a.x(), AngleVelocity::from(1_f64));
        assert_eq!(a.y(), AngleVelocity::from(2_f64));
        assert_eq!(a.z(), AngleVelocity::from(3_f64));
    }

    #[test]
    fn accel_from_info() {
        let info = AccelInfo::new(1_f64, 2_f64, 3_f64);
        let a: Accel3D<f64> = info.into();
        assert_eq!(a.x(), Accel::from(1_f64));
        assert_eq!(a.y(), Accel::from(2_f64));
        assert_eq!(a.z(), Accel::from(3_f64));
    }
}
