#[cfg(test)]
mod tests;

use core::fmt::Debug;
use embedded_time::{duration::*, rate::*};
use num_derive::FromPrimitive;
use util::SingleByte;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FlagsXYZ(pub(super) u8);

impl FlagsXYZ {
    pub fn new(x: bool, y: bool, z: bool) -> Self {
        Self(u8::from_bools(&[z, y, x]))
    }

    pub fn x(&self) -> bool {
        self.0.get(2)
    }

    pub fn y(&self) -> bool {
        self.0.get(1)
    }

    pub fn z(&self) -> bool {
        self.0.get(0)
    }

    pub fn as_u8(&self) -> u8 {
        self.0
    }
}

/// Upon power up, the MPU-60X0 clock source defaults to the internal oscillator.
/// However, it is highly recommended that the device be configured to use one of the gyroscopes
/// (or an external clock source) as the clock reference for improved stability.
/// The clock source can be selected according to the following table.
#[derive(Debug, FromPrimitive, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ClockSel {
    Internal8MHz = 0,
    Xgyro = 1,
    Ygyro = 2,
    Zgyro = 3,
    External32768Hz = 4,
    External19200kHz = 5,
    Reserved = 6,
    Stop = 7,
}

#[derive(Debug, Clone, Copy, FromPrimitive, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum FrameSync {
    Disabled = 0,
    TempOutL = 1,
    GyroXoutL = 2,
    GyroYoutL = 3,
    GyroZoutL = 4,
    AccelXoutL = 5,
    AccelYoutL = 6,
    AccelZoutL = 7,
}

#[derive(Debug, Clone, Copy, FromPrimitive, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum DigitalLowPassFilterCfg {
    V0 = 0,
    V1 = 1,
    V2 = 2,
    V3 = 3,
    V4 = 4,
    V5 = 5,
    V6 = 6,
    V7 = 7,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct DlpFilter {
    pub bandwidth: Hertz,
    pub delay: Microseconds<u32>,
    pub fs: Kilohertz,
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
