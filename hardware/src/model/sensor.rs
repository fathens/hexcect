use getset::Getters;

#[derive(Debug, Getters, PartialEq)]
#[get = "pub"]
pub struct GyroInfo<V> {
    x: V,
    y: V,
    z: V,
}

impl<V> GyroInfo<V> {
    pub fn new(x: V, y: V, z: V) -> Self {
        Self { x, y, z }
    }
}

#[derive(Debug, Getters, PartialEq)]
#[get = "pub"]
pub struct AccelInfo<V> {
    x: V,
    y: V,
    z: V,
}

impl<V> AccelInfo<V> {
    pub fn new(x: V, y: V, z: V) -> Self {
        Self { x, y, z }
    }
}
