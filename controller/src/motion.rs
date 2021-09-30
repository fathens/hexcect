use crate::model::*;

pub fn detect_vertical<V: Copy>(sensor: Accel3D<V>) -> Vector3D<V>
where
    V: From<Accel<V>>,
{
    sensor.into()
}
