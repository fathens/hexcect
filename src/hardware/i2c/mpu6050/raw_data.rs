use crate::model::sensor::{AccInfo, GyroInfo};
use embedded_time::TimeInt;
use embedded_time::{duration::*, rate::*};
use num_derive::FromPrimitive;

#[derive(Debug)]
pub struct RawData {
    pub temp: Temperature,
    pub gyro: GyroData,
    pub accel: AccelData,
}

#[derive(Debug)]
pub struct Temperature(pub i16);

#[derive(Debug)]
pub struct GyroData {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

impl GyroData {
    pub fn scale(&self, _: f32) -> GyroInfo {
        todo!()
    }
}

#[derive(Clone, Copy, FromPrimitive)]
#[repr(u8)]
pub enum GyroFullScale {
    Deg250,
    Deg500,
    Deg1000,
    Deg2000,
}

impl GyroFullScale {
    pub fn max(&self) -> f32 {
        let s = 1 << (*self as u8);
        250.0 * s as f32
    }
}

#[derive(Debug)]
pub struct AccelData {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

impl AccelData {
    pub fn scale(&self, _: f32) -> AccInfo {
        todo!()
    }
}

#[derive(Clone, Copy, FromPrimitive)]
#[repr(u8)]
pub enum AccelFullScale {
    G2,
    G4,
    G8,
    G16,
}

impl AccelFullScale {
    pub fn max(&self) -> f32 {
        let s = 1 << (*self as u8);
        2.0 * s as f32
    }
}

#[derive(Clone, Copy, FromPrimitive)]
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

#[derive(Clone, Copy, FromPrimitive)]
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
                delay: 0.0.milliseconds(),
                fs: 1_u32.kHz(),
            },
            DigitalLowPassFilterCfg::V1 => DlpFilter {
                bandwidth: 184_u32.Hz(),
                delay: 2.0.milliseconds(),
                fs: 1_u32.kHz(),
            },
            DigitalLowPassFilterCfg::V2 => DlpFilter {
                bandwidth: 94_u32.Hz(),
                delay: 3.0.milliseconds(),
                fs: 1_u32.kHz(),
            },
            DigitalLowPassFilterCfg::V3 => DlpFilter {
                bandwidth: 44_u32.Hz(),
                delay: 4.9.milliseconds(),
                fs: 1_u32.kHz(),
            },
            DigitalLowPassFilterCfg::V4 => DlpFilter {
                bandwidth: 21_u32.Hz(),
                delay: 8.5.milliseconds(),
                fs: 1_u32.kHz(),
            },
            DigitalLowPassFilterCfg::V5 => DlpFilter {
                bandwidth: 10_u32.Hz(),
                delay: 13.8.milliseconds(),
                fs: 1_u32.kHz(),
            },
            DigitalLowPassFilterCfg::V6 => DlpFilter {
                bandwidth: 5_u32.Hz(),
                delay: 19.0.milliseconds(),
                fs: 1_u32.kHz(),
            },
            DigitalLowPassFilterCfg::V7 => DlpFilter {
                bandwidth: 0_u32.Hz(),
                delay: 0.0.milliseconds(),
                fs: 0_u32.kHz(),
            },
        }
    }

    pub fn gyro(&self) -> DlpFilter {
        match self {
            DigitalLowPassFilterCfg::V0 => DlpFilter {
                bandwidth: 256_u32.Hz(),
                delay: 0.98.milliseconds(),
                fs: 8_u32.kHz(),
            },
            DigitalLowPassFilterCfg::V1 => DlpFilter {
                bandwidth: 188_u32.Hz(),
                delay: 1.9.milliseconds(),
                fs: 1_u32.kHz(),
            },
            DigitalLowPassFilterCfg::V2 => DlpFilter {
                bandwidth: 98_u32.Hz(),
                delay: 2.8.milliseconds(),
                fs: 1_u32.kHz(),
            },
            DigitalLowPassFilterCfg::V3 => DlpFilter {
                bandwidth: 42_u32.Hz(),
                delay: 4.8.milliseconds(),
                fs: 1_u32.kHz(),
            },
            DigitalLowPassFilterCfg::V4 => DlpFilter {
                bandwidth: 20_u32.Hz(),
                delay: 8.3.milliseconds(),
                fs: 1_u32.kHz(),
            },
            DigitalLowPassFilterCfg::V5 => DlpFilter {
                bandwidth: 10_u32.Hz(),
                delay: 13.4.milliseconds(),
                fs: 1_u32.kHz(),
            },
            DigitalLowPassFilterCfg::V6 => DlpFilter {
                bandwidth: 5_u32.Hz(),
                delay: 18.6.milliseconds(),
                fs: 1_u32.kHz(),
            },
            DigitalLowPassFilterCfg::V7 => DlpFilter {
                bandwidth: 0_u32.Hz(),
                delay: 0.0.milliseconds(),
                fs: 8_u32.kHz(),
            },
        }
    }
}

/// Float で計算して端数やオーバーフローは切り捨てる変換を実装
trait FloatDuration<T: TimeInt> {
    fn milliseconds(self) -> Microseconds<T>;
}

impl FloatDuration<u32> for f32 {
    fn milliseconds(self) -> Microseconds<u32> {
        ((self * 1000.0) as u32).microseconds()
    }
}
