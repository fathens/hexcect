use super::*;

use derive_more::Constructor;
use getset::CopyGetters;
use num_traits::FloatConst;

#[derive(Debug, Constructor, CopyGetters)]
#[get_copy = "pub"]
pub struct Gyro3D<V: Copy + FloatConst> {
    x: AngleVelocity<V>,
    y: AngleVelocity<V>,
    z: AngleVelocity<V>,
}

#[derive(Debug, Constructor, CopyGetters)]
#[get_copy = "pub"]
pub struct Accel3D<V: Copy> {
    x: Accel<V>,
    y: Accel<V>,
    z: Accel<V>,
}
