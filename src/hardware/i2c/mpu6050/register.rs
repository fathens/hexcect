mod attributes;

#[cfg(test)]
mod tests;

use super::raw_data::*;
use crate::util::SingleByte;
pub use attributes::*;

use core::fmt::Debug;
use derive_more::{From, Into};
use num_traits::FromPrimitive;
use std::convert::TryInto;

#[derive(Debug, From, Into, Clone, Copy, PartialEq, Eq)]
pub struct RegAddr(u8);

pub trait Register: From<u8> + Debug + Copy + Eq {
    const ADDR: RegAddr;
}

/// This register configures the external Frame Synchronization (FSYNC) pin sampling and
/// the Digital Low Pass Filter (DLPF) setting for both the gyroscopes and accelerometers.
#[derive(Debug, From, Into, Clone, Copy, PartialEq, Eq)]
pub struct Configure(u8);

impl Configure {
    pub fn get_fsync(&self) -> FrameSync {
        FrameSync::from_u8(self.0.get_with_mask(0b111, 3))
            .expect("A value of 3 bits must be converted to FSYNC.")
    }

    pub fn get_dlpf(&self) -> DigitalLowPassFilterCfg {
        DigitalLowPassFilterCfg::from_u8(self.0.get_with_mask(0b111, 0))
            .expect("A value of 3 bits must be converted to DLPF_CGF.")
    }

    pub fn set_fsync(&mut self, v: FrameSync) -> () {
        self.0 = self.0.set_with_mask(0b111, 3, v as u8);
    }

    pub fn set_dlpf(&mut self, v: DigitalLowPassFilterCfg) -> () {
        self.0 = self.0.set_with_mask(0b111, 0, v as u8);
    }
}

impl Register for Configure {
    const ADDR: RegAddr = RegAddr(0x1a);
}

/// This register is used to trigger gyroscope self-test and
/// configure the gyroscopes’ full scale range.
#[derive(Debug, From, Into, Clone, Copy, PartialEq, Eq)]
pub struct GyroConfig(u8);

impl GyroConfig {
    pub fn get_xyz(&self) -> FlagsXYZ {
        FlagsXYZ(self.0.get_with_mask(0b111, 5))
    }

    pub fn get_scale(&self) -> GyroFullScale {
        GyroFullScale::from_u8(self.0.get_with_mask(0b11, 3))
            .expect("A value of 2 bits must be converted to GyroFullScale.")
    }

    pub fn set_xyz(&mut self, v: FlagsXYZ) -> () {
        self.0 = self.0.set_with_mask(0b111, 5, v.as_u8());
    }

    pub fn set_scale(&mut self, v: GyroFullScale) -> () {
        self.0 = self.0.set_with_mask(0b11, 3, v as u8);
    }
}

impl Register for GyroConfig {
    const ADDR: RegAddr = RegAddr(0x1b);
}

/// This register is used to trigger accelerometer self test and
/// configure the accelerometer full scale range.
/// This register also configures the Digital High Pass Filter (DHPF).
#[derive(Debug, From, Into, Clone, Copy, PartialEq, Eq)]
pub struct AccelConfig(u8);

impl AccelConfig {
    pub fn get_xyz(&self) -> FlagsXYZ {
        FlagsXYZ(self.0.get_with_mask(0b111, 5))
    }

    pub fn get_scale(&self) -> AccelFullScale {
        AccelFullScale::from_u8(self.0.get_with_mask(0b11, 3))
            .expect("A value of 2 bits must be converted to AccelFullScale.")
    }

    pub fn set_xyz(&mut self, v: FlagsXYZ) -> () {
        self.0 = self.0.set_with_mask(0b111, 5, v.as_u8());
    }

    pub fn set_scale(&mut self, v: AccelFullScale) -> () {
        self.0 = self.0.set_with_mask(0b11, 3, v as u8);
    }
}

impl Register for AccelConfig {
    const ADDR: RegAddr = RegAddr(0x1c);
}

// ----------------------------------------------------------------
// ----------------------------------------------------------------

pub trait RegisterRange {
    const ADDR: RegAddr;
}

impl RegisterRange for AccelData {
    const ADDR: RegAddr = RegAddr(0x3b);
}

impl From<[u8; 6]> for AccelData {
    fn from(data: [u8; 6]) -> Self {
        let (x, y, z) = take2x3(data);
        Self { x, y, z }
    }
}

impl RegisterRange for Temperature {
    const ADDR: RegAddr = RegAddr(0x41);
}

impl From<[u8; 2]> for Temperature {
    fn from(data: [u8; 2]) -> Self {
        Self(i16::from_be_bytes(data))
    }
}

impl RegisterRange for GyroData {
    const ADDR: RegAddr = RegAddr(0x43);
}

impl From<[u8; 6]> for GyroData {
    fn from(data: [u8; 6]) -> Self {
        let (x, y, z) = take2x3(data);
        Self { x, y, z }
    }
}

impl From<[u8; 14]> for RawData {
    fn from(buf: [u8; 14]) -> RawData {
        let array_accel: [u8; 6] = buf[..6].try_into().expect("Accel data must be here");
        let array_temp: [u8; 2] = buf[6..8].try_into().expect("Temperature data must be here");
        let array_gyro: [u8; 6] = buf[8..].try_into().expect("Gyro data must be here");
        RawData {
            accel: AccelData::from(array_accel),
            temp: Temperature::from(array_temp),
            gyro: GyroData::from(array_gyro),
        }
    }
}

fn take2x3(data: [u8; 6]) -> (i16, i16, i16) {
    (
        i16::from_be_bytes([data[0], data[1]]),
        i16::from_be_bytes([data[2], data[3]]),
        i16::from_be_bytes([data[4], data[5]]),
    )
}
