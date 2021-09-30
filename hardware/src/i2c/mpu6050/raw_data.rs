use crate::model::sensor::{AccelInfo, GyroInfo};

use derive_more::{From, Into};
use num_derive::FromPrimitive;
use std::ops::{Div, Mul};

pub trait FullScale {
    const RESOLUTION: i32 = 65500;

    fn max(&self) -> i32;

    fn scaled<V>(&self, x: i16, y: i16, z: i16) -> (V, V, V)
    where
        V: Mul<Output = V>,
        V: Div<Output = V>,
        V: From<i32>,
        V: Copy,
    {
        let max: V = (self.max() << 1).into();
        let rsv: V = Self::RESOLUTION.into();
        let rate = max / rsv;

        let scaled = |v| {
            let src: V = (v as i32).into();
            src * rate
        };
        (scaled(x), scaled(y), scaled(z))
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct RawData {
    pub temp: Temperature,
    pub gyro: GyroData,
    pub accel: AccelData,
}

#[derive(Debug, From, Into, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Temperature(i16);

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct GyroData {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

impl GyroData {
    pub fn scale<V>(&self, fs: GyroFullScale) -> GyroInfo<V>
    where
        V: Mul<Output = V>,
        V: Div<Output = V>,
        V: From<i32>,
        V: Copy,
    {
        let (x, y, z) = fs.scaled(self.x, self.y, self.z);
        GyroInfo::new(x, y, z)
    }
}

#[derive(Debug, Clone, Copy, FromPrimitive, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum GyroFullScale {
    Deg250,
    Deg500,
    Deg1000,
    Deg2000,
}

impl FullScale for GyroFullScale {
    fn max(&self) -> i32 {
        250 << (*self as u8)
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct AccelData {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

impl AccelData {
    pub fn scale<V>(&self, fs: AccelFullScale) -> AccelInfo<V>
    where
        V: Mul<Output = V>,
        V: Div<Output = V>,
        V: From<i32>,
        V: Copy,
    {
        let (x, y, z) = fs.scaled(self.x, self.y, self.z);
        AccelInfo::new(x, y, z)
    }
}

#[derive(Debug, Clone, Copy, FromPrimitive, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum AccelFullScale {
    G2,
    G4,
    G8,
    G16,
}

impl FullScale for AccelFullScale {
    fn max(&self) -> i32 {
        2 << (*self as u8)
    }
}

#[derive(Debug, From, Into, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FifoCount(u16);

#[cfg(test)]
mod tests {
    use super::*;
    use approx::*;
    use num_traits::FromPrimitive;
    use rand::prelude::*;

    #[test]
    fn accel_fs_as_u8() {
        assert_eq!(AccelFullScale::G2 as u8, 0);
        assert_eq!(AccelFullScale::G4 as u8, 1);
        assert_eq!(AccelFullScale::G8 as u8, 2);
        assert_eq!(AccelFullScale::G16 as u8, 3);
        for i in 0..4 {
            let a = AccelFullScale::from_u8(i).expect("Must be !");
            assert_eq!(i, a as u8);
        }
    }

    #[test]
    fn gyro_fs_as_u8() {
        assert_eq!(GyroFullScale::Deg250 as u8, 0);
        assert_eq!(GyroFullScale::Deg500 as u8, 1);
        assert_eq!(GyroFullScale::Deg1000 as u8, 2);
        assert_eq!(GyroFullScale::Deg2000 as u8, 3);
        for i in 0..4 {
            let a = GyroFullScale::from_u8(i).expect("Must be !");
            assert_eq!(i, a as u8);
        }
    }

    #[test]
    fn accel_fs_max() {
        assert_eq!(AccelFullScale::G2.max(), 2);
        assert_eq!(AccelFullScale::G4.max(), 4);
        assert_eq!(AccelFullScale::G8.max(), 8);
        assert_eq!(AccelFullScale::G16.max(), 16);
    }

    #[test]
    fn gyro_fs_max() {
        assert_eq!(GyroFullScale::Deg250.max(), 250);
        assert_eq!(GyroFullScale::Deg500.max(), 500);
        assert_eq!(GyroFullScale::Deg1000.max(), 1000);
        assert_eq!(GyroFullScale::Deg2000.max(), 2000);
    }

    #[test]
    fn accel_scaled() {
        let mut rnd = rand::thread_rng();
        use AccelFullScale::*;
        for fs in [G2, G4, G8, G16] {
            for _ in 0..100 {
                let scaled = |v: i16| {
                    let r = (v as f64) * (fs.max() as f64) * 2.0 / 65500.0;
                    (v, r)
                };
                let (a, i) = scaled(rnd.gen());
                let (b, j) = scaled(rnd.gen());
                let (c, k) = scaled(rnd.gen());
                let (x, y, z) = fs.scaled(a, b, c);
                assert_ulps_eq!(i, x);
                assert_ulps_eq!(j, y);
                assert_ulps_eq!(k, z);
            }
        }
    }

    #[test]
    fn gyro_scaled() {
        let mut rnd = rand::thread_rng();
        use GyroFullScale::*;
        for fs in [Deg250, Deg500, Deg1000, Deg2000] {
            for _ in 0..100 {
                let scaled = |v: i16| {
                    let r = (v as f64) * (fs.max() as f64) * 2.0 / 65500.0;
                    (v, r)
                };
                let (a, i) = scaled(rnd.gen());
                let (b, j) = scaled(rnd.gen());
                let (c, k) = scaled(rnd.gen());
                let (x, y, z) = fs.scaled(a, b, c);
                assert_ulps_eq!(i, x);
                assert_ulps_eq!(j, y);
                assert_ulps_eq!(k, z);
            }
        }
    }
}
