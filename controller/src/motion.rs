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
    gravity: Accel3D<V>,
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
            gravity: accel,
            pos: Position3D::init(V::zero().into()),
            movement: Vector3D::init(V::zero().into()),
            prev_accel: Accel3D::init(V::zero().into()),
            prev_gyro: Gyro3D::init(V::zero().into()),
            timestamp: Instant::now(),
        }
    }

    pub fn next(self, accel: Accel3D<V>, gyro: Gyro3D<V>) -> Self
    where
        V: Float,
        V: FromPrimitive,
        V: From<Scalar<V>>,
        V: From<Degrees<V>>,
        V: From<AngleVelocity<V>>,
        V: From<Accel<V>>,
        V: From<Speed<V>>,
        V: From<Seconds<V>>,
        V: From<Milliseconds<V>>,
        V: From<Nanoseconds<V>>,
    {
        let dur: Seconds<V> = (Instant::now() - self.timestamp).into();

        let rotation = self
            .prev_gyro
            .combine(&gyro, |p, n| integral_dur(dur, p, n));
        let next_gravigy = rotate(&self.gravity, &rotation);

        let next_accel = accel - &self.gravity;
        let speed = self
            .prev_accel
            .combine(&next_accel, |p, n| integral_dur(dur, p, n));

        // TODO 仮の戻り値
        Self {
            gravity: next_gravigy,
            pos: self.pos,
            movement: self.movement + &speed,
            prev_accel: next_accel,
            prev_gyro: gyro,
            timestamp: Instant::now(),
        }
    }
}
