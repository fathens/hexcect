use derive_more::Constructor;
use getset::CopyGetters;

#[derive(Debug, Constructor, CopyGetters, PartialEq)]
#[get_copy = "pub"]
pub struct GyroInfo<V: Copy> {
    x: V,
    y: V,
    z: V,
}

#[derive(Debug, Constructor, CopyGetters, PartialEq)]
#[get_copy = "pub"]
pub struct AccelInfo<V: Copy> {
    x: V,
    y: V,
    z: V,
}
