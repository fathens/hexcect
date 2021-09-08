#[cfg(test)]
mod tests;

use crate::util::SingleByte;
use core::fmt::Debug;
use derive_more::{From, Into};
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

/// When set to 1, this bit disables the temperature sensor.
#[derive(Debug, From, Into, Clone, Copy, PartialEq, Eq)]
pub struct TempDis(bool);

/// When this bit is set to 1 and SLEEP is disabled,
/// the MPU-60X0 will cycle between sleep mode and
/// waking up to take a single sample of data from
/// active sensors at a rate determined by LP_WAKE_CTRL (register 108).
#[derive(Debug, From, Into, Clone, Copy, PartialEq, Eq)]
pub struct CycleMode(bool);

/// When set to 1, this bit puts the MPU-60X0 into sleep mode.
#[derive(Debug, From, Into, Clone, Copy, PartialEq, Eq)]
pub struct SleepMode(bool);

/// When set to 1, this bit resets all internal registers to their default values.
/// The bit automatically clears to 0 once the reset is done.
/// The default values for each register can be found in Section 3.
#[derive(Debug, From, Into, Clone, Copy, PartialEq, Eq)]
pub struct DeviceReset(bool);

/// When set to 1, this bit resets the signal paths for all sensors
/// (gyroscopes, accelerometers, and temperature sensor).
/// This operation will also clear the sensor registers.
/// This bit automatically clears to 0 after the reset has been triggered.
/// When resetting only the signal path (and not the sensor registers),
/// please use Register 104, SIGNAL_PATH_RESET.
#[derive(Debug, From, Into, Clone, Copy, PartialEq, Eq)]
pub struct SigCondReset(bool);

/// This bit resets the I2C Master when set to 1 while I2C_MST_EN equals 0.
/// This bit automatically clears to 0 after the reset has been triggered.
#[derive(Debug, From, Into, Clone, Copy, PartialEq, Eq)]
pub struct I2cMstRest(bool);

/// This bit resets the FIFO buffer when set to 1 while FIFO_EN equals 0.
/// This bit automatically clears to 0 after the reset has been triggered.
#[derive(Debug, From, Into, Clone, Copy, PartialEq, Eq)]
pub struct FifoReset(bool);

/// When set to 1, this bit enables I2C Master Mode.
/// When this bit is cleared to 0, the auxiliary I2C bus lines
/// (AUX_DA and AUX_CL) are logically driven by the primary I2C bus (SDA and SCL).
#[derive(Debug, From, Into, Clone, Copy, PartialEq, Eq)]
pub struct I2cMstEn(bool);

/// When set to 1, this bit enables FIFO operations.
/// When this bit is cleared to 0, the FIFO buffer is disabled.
/// The FIFO buffer cannot be written to or read from while disabled.
/// The FIFO buffer’s state does not change unless the MPU-60X0 is power cycled.
#[derive(Debug, From, Into, Clone, Copy, PartialEq, Eq)]
pub struct FifoEn(bool);

/// When set to 1, this bit enables the Data Ready interrupt,
/// which occurs each time a write operation to all of the sensor registers has been completed.
#[derive(Debug, From, Into, Clone, Copy, PartialEq, Eq)]
pub struct DataRdyEn(bool);

/// When set to 1, this bit enables any of the I2C Master
/// interrupt sources to generate an interrupt.
#[derive(Debug, From, Into, Clone, Copy, PartialEq, Eq)]
pub struct I2cMstIntEn(bool);

/// When set to 1, this bit enables a FIFO buffer overflow to generate an interrupt.
#[derive(Debug, From, Into, Clone, Copy, PartialEq, Eq)]
pub struct FifoOflowEn(bool);

/// When set to 1, this bit enables Motion detection to generate an interrupt.
#[derive(Debug, From, Into, Clone, Copy, PartialEq, Eq)]
pub struct MotEn(bool);
