use derive_more::Constructor;
use getset::CopyGetters;
use measure_units::*;

#[derive(Debug, Clone, Copy, PartialEq, CalcMix)]
#[calcmix(unit_name = "m".to_string())]
pub struct Meters<V>(V);

#[derive(Debug, Clone, Copy, PartialEq, CalcMix)]
#[calcmix(unit_name = "s".to_string())]
pub struct Seconds<V>(V);

pub type Accel<V> = UnitsDiv<V, Meters<V>, UnitsMul<V, Seconds<V>, Seconds<V>>>;

pub type Speed<V> = UnitsDiv<V, Meters<V>, Seconds<V>>;

#[derive(Debug, Constructor, CopyGetters)]
#[get_copy = "pub"]
pub struct Gyro3D<V: Copy> {
    x: V,
    y: V,
    z: V,
}

#[derive(Debug, Constructor, CopyGetters)]
#[get_copy = "pub"]
pub struct Accel3D<V: Copy> {
    x: Accel<V>,
    y: Accel<V>,
    z: Accel<V>,
}
