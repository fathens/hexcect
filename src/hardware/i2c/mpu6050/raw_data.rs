use crate::model::sensor::{AccInfo, GyroInfo};

#[derive(Debug)]
pub struct RawData {
    pub temp: f32,
    pub gyro: GyroData,
    pub acc: AccData,
}

#[derive(Debug)]
pub struct GyroData {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

impl GyroData {
    pub fn from_bytes(data: [u8; 6]) -> Self {
        let (x, y, z) = take2x3(data);
        Self { x, y, z }
    }

    pub fn scale(&self, s: f32) -> GyroInfo {
        todo!()
    }
}

pub enum GyroFullScale {
    Deg250,
    Deg500,
    Deg1000,
    Deg2000,
}

impl GyroFullScale {
    pub fn value(&self) -> u8 {
        match self {
            GyroFullScale::Deg250 => 0,
            GyroFullScale::Deg500 => 1,
            GyroFullScale::Deg1000 => 2,
            GyroFullScale::Deg2000 => 3,
        }
    }

    pub fn max(&self) -> f32 {
        let s = 1 << self.value();
        250.0 * s as f32
    }
}

#[derive(Debug)]
pub struct AccData {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

impl AccData {
    pub fn from_bytes(data: [u8; 6]) -> Self {
        let (x, y, z) = take2x3(data);
        Self { x, y, z }
    }

    pub fn scale(&self, s: f32) -> AccInfo {
        todo!()
    }
}

pub enum AccScale {
    G2,
    G4,
    G8,
    G16,
}

impl AccScale {
    pub fn scale(&self) -> u8 {
        match self {
            AccScale::G2 => 0,
            AccScale::G4 => 1,
            AccScale::G8 => 2,
            AccScale::G16 => 3,
        }
    }

    pub fn max(&self) -> f32 {
        let s = 1 << self.scale();
        1.0 * s as f32
    }
}

fn take2x3(data: [u8; 6]) -> (i16, i16, i16) {
    (
        i16::from_be_bytes([data[0], data[1]]),
        i16::from_be_bytes([data[2], data[3]]),
        i16::from_be_bytes([data[4], data[5]]),
    )
}
