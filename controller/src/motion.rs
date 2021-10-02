mod fomula;

use crate::model::*;
use fomula::*;

use derive_more::Constructor;
use getset::Getters;
use measure_units::Scalar;
use num_traits::{Float, FloatConst, FromPrimitive, Zero};
use std::time::Instant;

#[derive(Debug, Clone, PartialEq, Eq, Constructor, Getters)]
#[get = "pub"]
pub struct Posture<V: Copy + FloatConst> {
    /// direction to bottom
    bottom: Vector3D<V>,
    pos: Position3D<Millimeters<V>>,
    movement: Vector3D<Speed<V>>,
    prev_accel: Accel3D<V>,
    prev_gyro: Gyro3D<V>,
    timestamp: Instant,
}

impl<V: Copy + FloatConst> Posture<V> {
    pub fn init(accel: Accel3D<V>) -> Self
    where
        V: Zero,
        V: From<Accel<V>>,
    {
        Self {
            bottom: accel.apply(|v| v.into()),
            pos: Position3D::init(V::zero().into()),
            movement: Vector3D::init(V::zero().into()),
            prev_accel: Accel3D::init(V::zero().into()),
            prev_gyro: Gyro3D::init(V::zero().into()),
            timestamp: Instant::now(),
        }
    }

    pub fn next(self, accel: Accel3D<V>, _gyro: Gyro3D<V>) -> Self
    where
        V: Float,
        V: FromPrimitive,
        V: From<Scalar<V>>,
        V: From<Accel<V>>,
        V: From<Speed<V>>,
        V: From<Seconds<V>>,
        V: From<Milliseconds<V>>,
        V: From<Nanoseconds<V>>,
    {
        let dur: Milliseconds<V> = (Instant::now() - self.timestamp).into();
        let _speed = self
            .prev_accel
            .combine(accel, |p, n| integral_accel(dur, p, n));
        todo!()
    }
}
