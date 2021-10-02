use crate::model::*;

use derive_more::Constructor;
use getset::Getters;
use num_traits::{FloatConst, NumCast, Zero};
use std::time::Instant;

pub fn get_speed<V>(accel: Accel<V>, time: Seconds<V>) -> Speed<V>
where
    V: num_traits::Float,
    V: From<Seconds<V>>,
    V: From<Accel<V>>,
{
    let a = time * accel; // S * ((M / S) / S)
    a.infuse_nmr().reduction_left()
}

#[derive(Debug, Clone, PartialEq, Eq, Constructor, Getters)]
#[get = "pub"]
pub struct Posture<V: Copy> {
    /// direction to bottom
    bottom: Vector3D<V>,
    pos: Position3D<Millimeters<V>>,
    movement: Vector3D<Speed<V>>,
    prev_accel: Accel3D<V>,
    timestamp: Instant,
}

impl<V: Copy> Posture<V> {
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
            timestamp: Instant::now(),
        }
    }

    pub fn next(self, accel: Accel3D<V>, gyro: Gyro3D<V>) -> Self
    where
        V: FloatConst,
        V: NumCast,
    {
        let dur = {
            let d = Instant::now() - self.timestamp;
            let v = V::from(d.as_nanos()).unwrap();
            Nanoseconds::from(v)
        };
        todo!()
    }
}
