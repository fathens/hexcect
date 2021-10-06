use derive_more::Constructor;
use getset::CopyGetters;

#[derive(Debug, Constructor, CopyGetters, PartialEq)]
#[get_copy = "pub"]
pub struct GyroInfo<V: Copy> {
    roll: V,
    pitch: V,
    yaw: V,
}

#[derive(Debug, Constructor, CopyGetters, PartialEq)]
#[get_copy = "pub"]
pub struct AccelInfo<V: Copy> {
    x: V,
    y: V,
    z: V,
}
