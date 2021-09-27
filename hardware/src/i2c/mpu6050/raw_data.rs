use crate::model::sensor::{AccelInfo, GyroInfo};
use derive_more::{From, Into};
use num_derive::FromPrimitive;

const RESOLUTION: f64 = 65500.0;

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
    pub fn scale(&self, fs: GyroFullScale) -> GyroInfo<f64> {
        let scaled = |v| v as f64 * fs.max() * 2.0 / RESOLUTION;
        GyroInfo::new(scaled(self.x), scaled(self.y), scaled(self.z))
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

impl GyroFullScale {
    pub fn max(&self) -> f64 {
        let s = 1 << (*self as u8);
        250.0 * s as f64
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct AccelData {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

impl AccelData {
    pub fn scale(&self, fs: AccelFullScale) -> AccelInfo<f64> {
        let scaled = |v| v as f64 * fs.max() * 2.0 / RESOLUTION;
        AccelInfo::new(scaled(self.x), scaled(self.y), scaled(self.z))
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

impl AccelFullScale {
    pub fn max(&self) -> f64 {
        let s = 1 << (*self as u8 + 1);
        s as f64
    }
}

#[derive(Debug, From, Into, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FifoCount(u16);

#[cfg(test)]
mod tests {
    use super::*;
    use num_traits::FromPrimitive;

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
        assert_eq!(AccelFullScale::G2.max(), 2.0);
        assert_eq!(AccelFullScale::G4.max(), 4.0);
        assert_eq!(AccelFullScale::G8.max(), 8.0);
        assert_eq!(AccelFullScale::G16.max(), 16.0);
    }

    #[test]
    fn gyro_fs_max() {
        assert_eq!(GyroFullScale::Deg250.max(), 250.0);
        assert_eq!(GyroFullScale::Deg500.max(), 500.0);
        assert_eq!(GyroFullScale::Deg1000.max(), 1000.0);
        assert_eq!(GyroFullScale::Deg2000.max(), 2000.0);
    }
}
