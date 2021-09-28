use getset::CopyGetters;

#[derive(Debug, CopyGetters, PartialEq)]
#[get_copy = "pub"]
pub struct GyroInfo<V: Copy> {
    x: V,
    y: V,
    z: V,
}

impl<V: Copy> GyroInfo<V> {
    pub fn new(x: V, y: V, z: V) -> Self {
        Self { x, y, z }
    }
}

#[derive(Debug, CopyGetters, PartialEq)]
#[get_copy = "pub"]
pub struct AccelInfo<V: Copy> {
    x: V,
    y: V,
    z: V,
}

impl<V: Copy> AccelInfo<V> {
    pub fn new(x: V, y: V, z: V) -> Self {
        Self { x, y, z }
    }
}
