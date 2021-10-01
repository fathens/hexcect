use crate::model::*;

pub fn detect_vertical<V: Copy>(_: Accel3D<V>) -> Vector3D<V>
where
    V: From<Accel<V>>,
{
    todo!()
}

pub fn get_speed<V>(accel: Accel<V>, time: Seconds<V>) -> Speed<V>
where
    V: num_traits::Float,
    V: From<Seconds<V>>,
    V: From<Accel<V>>,
{
    let a = time * accel; // S * ((M / S) / S)
    a.infuse_nmr().reduction_left()
}
