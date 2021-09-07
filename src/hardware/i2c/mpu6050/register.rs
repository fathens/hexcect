use super::raw_data::*;
use embedded_time::{duration::*, rate::*};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::convert::TryInto;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RegAddr(u8);

impl RegAddr {
    pub fn as_u8(&self) -> u8 {
        self.0
    }
}

pub trait Register: From<u8> {
    const ADDR: RegAddr;

    fn as_u8(&self) -> u8;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Configure {
    pub ext_sync_set: FrameSync,
    pub dlpf_cfg: DigitalLowPassFilterCfg,
}

impl Configure {
    const ADDR: RegAddr = RegAddr(0x1a);

    fn as_u8(&self) -> u8 {
        let a = (self.ext_sync_set as u8) << 3;
        let b = self.dlpf_cfg as u8;
        a + b
    }
}

impl From<u8> for Configure {
    fn from(v: u8) -> Self {
        Self {
            ext_sync_set: FrameSync::from_u8((v >> 3) & 7).unwrap(),
            dlpf_cfg: DigitalLowPassFilterCfg::from_u8(v & 7).unwrap(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GyroConfig {
    pub st_xyz: FlagsXYZ,
    pub fs_sel: GyroFullScale,
}

impl Register for GyroConfig {
    const ADDR: RegAddr = RegAddr(0x1b);

    fn as_u8(&self) -> u8 {
        let a = self.st_xyz.as_u8() << 5;
        let b = (self.fs_sel as u8) << 3;
        a + b
    }
}

impl From<u8> for GyroConfig {
    fn from(v: u8) -> Self {
        Self {
            st_xyz: FlagsXYZ((v >> 5) & 7),
            fs_sel: GyroFullScale::from_u8((v >> 3) & 3)
                .expect("A value of 2 bits must be converted to GyroFullScale."),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AccelConfig {
    pub st_xyz: FlagsXYZ,
    pub afs_sel: AccelFullScale,
}

impl Register for AccelConfig {
    const ADDR: RegAddr = RegAddr(0x1c);

    fn as_u8(&self) -> u8 {
        let a = self.st_xyz.as_u8() << 5;
        let b = (self.afs_sel as u8) << 3;
        a + b
    }
}

impl From<u8> for AccelConfig {
    fn from(v: u8) -> Self {
        Self {
            st_xyz: FlagsXYZ((v >> 5) & 7),
            afs_sel: AccelFullScale::from_u8((v >> 3) & 3)
                .expect("A value of 2 bits must be converted to AccelFullScale."),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FlagsXYZ(u8);

impl FlagsXYZ {
    pub fn new(x: bool, y: bool, z: bool) -> Self {
        Self(u8::from_bits(&[z, y, x]))
    }

    pub fn x(&self) -> bool {
        self.0.at(2)
    }

    pub fn y(&self) -> bool {
        self.0.at(1)
    }

    pub fn z(&self) -> bool {
        self.0.at(0)
    }

    pub fn as_u8(&self) -> u8 {
        self.0
    }
}

trait SingleByte {
    fn from_bits(bs: &[bool]) -> u8 {
        assert!(bs.len() <= 8);
        bs.iter()
            .enumerate()
            .map(|(i, b)| if *b { 1 << i } else { 0 })
            .sum()
    }

    fn at(&self, i: u8) -> bool {
        self.value() & (1 << i) != 0
    }

    fn value(&self) -> u8;
}

impl SingleByte for u8 {
    fn value(&self) -> u8 {
        *self
    }
}

#[derive(Debug, Clone, Copy, FromPrimitive, PartialEq, Eq)]
#[repr(u8)]
pub enum FrameSync {
    Disabled,
    TempOutL,
    GyroXoutL,
    GyroYoutL,
    GyroZoutL,
    AccelXoutL,
    AccelYoutL,
    AccelZoutL,
}

#[derive(Debug, Clone, Copy, FromPrimitive, PartialEq, Eq)]
#[repr(u8)]
pub enum DigitalLowPassFilterCfg {
    V0,
    V1,
    V2,
    V3,
    V4,
    V5,
    V6,
    V7,
}

#[derive(Debug, PartialEq, Eq)]
pub struct DlpFilter {
    bandwidth: Hertz,
    delay: Microseconds<u32>,
    fs: Kilohertz,
}

impl DigitalLowPassFilterCfg {
    pub fn accel(&self) -> DlpFilter {
        match self {
            DigitalLowPassFilterCfg::V0 => DlpFilter {
                bandwidth: 260_u32.Hz(),
                delay: 0.microseconds(),
                fs: 1_u32.kHz(),
            },
            DigitalLowPassFilterCfg::V1 => DlpFilter {
                bandwidth: 184_u32.Hz(),
                delay: 2000.microseconds(),
                fs: 1_u32.kHz(),
            },
            DigitalLowPassFilterCfg::V2 => DlpFilter {
                bandwidth: 94_u32.Hz(),
                delay: 3000.microseconds(),
                fs: 1_u32.kHz(),
            },
            DigitalLowPassFilterCfg::V3 => DlpFilter {
                bandwidth: 44_u32.Hz(),
                delay: 4900.microseconds(),
                fs: 1_u32.kHz(),
            },
            DigitalLowPassFilterCfg::V4 => DlpFilter {
                bandwidth: 21_u32.Hz(),
                delay: 8500.microseconds(),
                fs: 1_u32.kHz(),
            },
            DigitalLowPassFilterCfg::V5 => DlpFilter {
                bandwidth: 10_u32.Hz(),
                delay: 13800.microseconds(),
                fs: 1_u32.kHz(),
            },
            DigitalLowPassFilterCfg::V6 => DlpFilter {
                bandwidth: 5_u32.Hz(),
                delay: 19000.microseconds(),
                fs: 1_u32.kHz(),
            },
            DigitalLowPassFilterCfg::V7 => DlpFilter {
                bandwidth: 0_u32.Hz(),
                delay: 0.microseconds(),
                fs: 0_u32.kHz(),
            },
        }
    }

    pub fn gyro(&self) -> DlpFilter {
        match self {
            DigitalLowPassFilterCfg::V0 => DlpFilter {
                bandwidth: 256_u32.Hz(),
                delay: 980.microseconds(),
                fs: 8_u32.kHz(),
            },
            DigitalLowPassFilterCfg::V1 => DlpFilter {
                bandwidth: 188_u32.Hz(),
                delay: 1900.microseconds(),
                fs: 1_u32.kHz(),
            },
            DigitalLowPassFilterCfg::V2 => DlpFilter {
                bandwidth: 98_u32.Hz(),
                delay: 2800.microseconds(),
                fs: 1_u32.kHz(),
            },
            DigitalLowPassFilterCfg::V3 => DlpFilter {
                bandwidth: 42_u32.Hz(),
                delay: 4800.microseconds(),
                fs: 1_u32.kHz(),
            },
            DigitalLowPassFilterCfg::V4 => DlpFilter {
                bandwidth: 20_u32.Hz(),
                delay: 8300.microseconds(),
                fs: 1_u32.kHz(),
            },
            DigitalLowPassFilterCfg::V5 => DlpFilter {
                bandwidth: 10_u32.Hz(),
                delay: 13400.microseconds(),
                fs: 1_u32.kHz(),
            },
            DigitalLowPassFilterCfg::V6 => DlpFilter {
                bandwidth: 5_u32.Hz(),
                delay: 18600.microseconds(),
                fs: 1_u32.kHz(),
            },
            DigitalLowPassFilterCfg::V7 => DlpFilter {
                bandwidth: 0_u32.Hz(),
                delay: 0.microseconds(),
                fs: 8_u32.kHz(),
            },
        }
    }
}

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

#[cfg(test)]
mod tests;
