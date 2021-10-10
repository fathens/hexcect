mod fomula;

use crate::model::*;
use fomula::*;

use derive_more::Constructor;
use getset::Getters;
use measure_units::Scalar;
use nalgebra::RealField;
use num_traits::{Float, FromPrimitive};

#[derive(Debug, Clone, PartialEq, Eq, Constructor, Getters)]
#[get = "pub"]
pub struct Posture<V: Float> {
    gravity: Accel3D<V>,
    angle: Radians3D<V>,
    pos: Position3D<Meters<V>>,
    movement: Vector3D<Speed<V>>,
    prev_accel: Accel3D<V>,
    prev_gyro: Gyro3D<V>,
    timestamp: Timestamp,
}

impl<V: Float> Posture<V> {
    pub fn init(gravity: Accel3D<V>) -> Self
    where
        V: From<Accel<V>>,
    {
        let angle = vector_angle(&gravity);
        Self {
            gravity,
            angle,
            pos: Position3D::init(V::zero().into()),
            movement: Vector3D::init(V::zero().into()),
            prev_accel: Accel3D::init(V::zero().into()),
            prev_gyro: Gyro3D::init(V::zero().into()),
            timestamp: Timestamp::now(),
        }
    }

    pub fn next(self, accel: Accel3D<V>, gyro: Gyro3D<V>) -> Self
    where
        V: Float,
        V: FromPrimitive,
        V: RealField,
        V: From<Scalar<V>>,
        V: From<Degrees<V>>,
        V: From<Radians<V>>,
        V: From<AngleVelocity<V>>,
        V: From<Accel<V>>,
        V: From<Speed<V>>,
        V: From<Meters<V>>,
        V: From<Seconds<V>>,
        V: From<Milliseconds<V>>,
        V: From<Nanoseconds<V>>,
    {
        let (dur, next_timestamp) = self.timestamp.past_dur();

        let rotate_epsilon = self
            .prev_gyro
            .combine(&gyro, |p, n| integral_dur(dur, p, n));
        let angle_delta = gyro_delta(&self.angle, &rotate_epsilon.into());
        let next_gravigy = rotate(&self.gravity, &angle_delta);

        let next_accel = accel - &self.gravity;
        let speed_delta = self
            .prev_accel
            .combine(&next_accel, |p, n| integral_dur(dur, p, n));

        let next_movement = speed_delta + &self.movement;
        let move_delta = self
            .movement
            .combine(&next_movement, |p, n| integral_dur(dur, p, n));

        // TODO 仮の戻り値
        Self {
            gravity: next_gravigy,
            angle: self.angle + &angle_delta,
            pos: self.pos + &move_delta,
            movement: next_movement,
            prev_accel: next_accel,
            prev_gyro: gyro,
            timestamp: next_timestamp,
        }
    }
}
