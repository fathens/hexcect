#[cfg(test)]
mod tests;

use crate::util::SingleByte;
use core::fmt::Debug;
use embedded_time::{duration::*, rate::*};
use num_derive::FromPrimitive;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

/// Upon power up, the MPU-60X0 clock source defaults to the internal oscillator.
/// However, it is highly recommended that the device be configured to use one of the gyroscopes
/// (or an external clock source) as the clock reference for improved stability.
/// The clock source can be selected according to the following table.
#[derive(Debug, FromPrimitive, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ClockSel {
    /// Internal 8MHz oscillator
    V0,
    /// PLL with X axis gyroscope reference
    V1,
    /// PLL with Y axis gyroscope reference
    V2,
    /// PLL with Z axis gyroscope reference
    V3,
    /// PLL with external 32.768kHz reference
    V4,
    /// PLL with external 19.2MHz reference
    V5,
    /// Reserved
    V6,
    /// Stops the clock and keeps the timing generator in reset
    V7,
}
