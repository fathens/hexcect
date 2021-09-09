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

impl Register for Configure {
    const ADDR: RegAddr = RegAddr(0x1a);
}

impl Configure {
    pub fn get_fsync(&self) -> FrameSync {
        FrameSync::from_u8(self.0.get_with_mask(0b111, 3))
            .expect("A value of 3 bits must be converted to FSYNC.")
    }

    pub fn get_dlpf(&self) -> DigitalLowPassFilterCfg {
        DigitalLowPassFilterCfg::from_u8(self.0.get_with_mask(0b111, 0))
            .expect("A value of 3 bits must be converted to DLPF_CGF.")
    }

    pub fn set_fsync(&mut self, v: FrameSync) {
        self.0 = self.0.set_with_mask(0b111, 3, v as u8);
    }

    pub fn set_dlpf(&mut self, v: DigitalLowPassFilterCfg) {
        self.0 = self.0.set_with_mask(0b111, 0, v as u8);
    }
}

/// This register is used to trigger gyroscope self-test and
/// configure the gyroscopes’ full scale range.
#[derive(Debug, From, Into, Clone, Copy, PartialEq, Eq)]
pub struct GyroConfig(u8);

impl Register for GyroConfig {
    const ADDR: RegAddr = RegAddr(0x1b);
}

impl GyroConfig {
    pub fn get_xyz(&self) -> FlagsXYZ {
        FlagsXYZ(self.0.get_with_mask(0b111, 5))
    }

    pub fn get_scale(&self) -> GyroFullScale {
        GyroFullScale::from_u8(self.0.get_with_mask(0b11, 3))
            .expect("A value of 2 bits must be converted to GyroFullScale.")
    }

    pub fn set_xyz(&mut self, v: FlagsXYZ) {
        self.0 = self.0.set_with_mask(0b111, 5, v.as_u8());
    }

    pub fn set_scale(&mut self, v: GyroFullScale) {
        self.0 = self.0.set_with_mask(0b11, 3, v as u8);
    }
}

/// This register is used to trigger accelerometer self test and
/// configure the accelerometer full scale range.
/// This register also configures the Digital High Pass Filter (DHPF).
#[derive(Debug, From, Into, Clone, Copy, PartialEq, Eq)]
pub struct AccelConfig(u8);

impl Register for AccelConfig {
    const ADDR: RegAddr = RegAddr(0x1c);
}

impl AccelConfig {
    pub fn get_xyz(&self) -> FlagsXYZ {
        FlagsXYZ(self.0.get_with_mask(0b111, 5))
    }

    pub fn get_scale(&self) -> AccelFullScale {
        AccelFullScale::from_u8(self.0.get_with_mask(0b11, 3))
            .expect("A value of 2 bits must be converted to AccelFullScale.")
    }

    pub fn set_xyz(&mut self, v: FlagsXYZ) {
        self.0 = self.0.set_with_mask(0b111, 5, v.as_u8());
    }

    pub fn set_scale(&mut self, v: AccelFullScale) {
        self.0 = self.0.set_with_mask(0b11, 3, v as u8);
    }
}

/// This register allows the user to configure the power mode and
/// clock source. It also provides a bit for resetting the entire device,
/// and a bit for disabling the temperature sensor.
#[derive(Debug, From, Into, Clone, Copy, PartialEq, Eq)]
pub struct PwrMgmt1(u8);

impl Register for PwrMgmt1 {
    const ADDR: RegAddr = RegAddr(0x6b);
}

impl PwrMgmt1 {
    pub fn get_clksel(&self) -> ClockSel {
        ClockSel::from_u8(self.0.get_with_mask(0b111, 0))
            .expect("A value of 3 bits must be converted to ClockSel.")
    }

    pub fn set_clksel(&mut self, v: ClockSel) {
        self.0 = self.0.set_with_mask(0b111, 0, v as u8);
    }

    /// When set to 1, this bit disables the temperature sensor.
    pub fn get_tempdis(&self) -> bool {
        self.0.get(3)
    }

    pub fn set_tempdis(&mut self, v: bool) {
        self.0 = self.0.set(3, v);
    }

    /// When this bit is set to 1 and SLEEP is disabled,
    /// the MPU-60X0 will cycle between sleep mode and
    /// waking up to take a single sample of data from
    /// active sensors at a rate determined by LP_WAKE_CTRL (register 108).
    pub fn get_cycle(&self) -> bool {
        self.0.get(5)
    }

    pub fn set_cycle(&mut self, v: bool) {
        self.0 = self.0.set(5, v);
    }

    /// When set to 1, this bit puts the MPU-60X0 into sleep mode.
    pub fn get_sleep(&self) -> bool {
        self.0.get(6)
    }

    pub fn set_sleep(&mut self, v: bool) {
        self.0 = self.0.set(6, v);
    }

    /// When set to 1, this bit resets all internal registers to their default values.
    /// The bit automatically clears to 0 once the reset is done.
    /// The default values for each register can be found in Section 3.
    pub fn get_device_reset(&self) -> bool {
        self.0.get(7)
    }

    pub fn set_device_reset(&mut self, v: bool) {
        self.0 = self.0.set(7, v);
    }
}

/// This register specifies the divider from the gyroscope output rate
/// used to generate the Sample Rate for the MPU-60X0.
/// The sensor register output, FIFO output, DMP sampling and Motion detection are
/// all based on the Sample Rate.
/// The Sample Rate is generated by dividing the gyroscope output rate by SMPLRT_DIV:
///     Sample Rate = Gyroscope Output Rate / (1 + SMPLRT_DIV)
/// where Gyroscope Output Rate = 8kHz when the DLPF is disabled (DLPF_CFG = 0 or 7),
/// and 1kHz when the DLPF is enabled (see Register 26).
///
/// Note:
/// The accelerometer output rate is 1kHz.
/// This means that for a Sample Rate greater than 1kHz,
/// the same accelerometer sample may be output to the FIFO, DMP, and sensor registers more than once.
/// For a diagram of the gyroscope and accelerometer signal paths,
/// see Section 8 of the MPU6000/MPU-6050 Product Specification document.
#[derive(Debug, From, Into, Clone, Copy, PartialEq, Eq)]
pub struct SampleRateDivider(u8);

impl Register for SampleRateDivider {
    const ADDR: RegAddr = RegAddr(0x19);
}

impl SampleRateDivider {
    pub fn get_value(&self) -> u8 {
        self.0
    }

    pub fn set_value(&mut self, v: u8) {
        self.0 = v;
    }
}

/// This register allows the user to enable and disable the FIFO buffer,
/// I2C Master Mode, and primary I2C interface.
/// The FIFO buffer, I2C Master, sensor signal paths and sensor registers
/// can also be reset using this register.
#[derive(Debug, From, Into, Clone, Copy, PartialEq, Eq)]
pub struct UserCtrl(u8);

impl Register for UserCtrl {
    const ADDR: RegAddr = RegAddr(0x6a);
}

impl UserCtrl {
    /// When set to 1, this bit resets the signal paths for all sensors
    /// (gyroscopes, accelerometers, and temperature sensor).
    /// This operation will also clear the sensor registers.
    /// This bit automatically clears to 0 after the reset has been triggered.
    /// When resetting only the signal path (and not the sensor registers),
    /// please use Register 104, SIGNAL_PATH_RESET.
    pub fn get_sigcond_reset(&self) -> bool {
        self.0.get(0)
    }

    pub fn set_sigcond_reset(&mut self, v: bool) {
        self.0 = self.0.set(0, v);
    }

    /// This bit resets the I2C Master when set to 1 while I2C_MST_EN equals 0.
    /// This bit automatically clears to 0 after the reset has been triggered.
    pub fn get_i2cmst_reset(&self) -> bool {
        self.0.get(1)
    }

    pub fn set_i2cmst_reset(&mut self, v: bool) {
        self.0 = self.0.set(1, v);
    }

    /// This bit resets the FIFO buffer when set to 1 while FIFO_EN equals 0.
    /// This bit automatically clears to 0 after the reset has been triggered.
    pub fn get_fifo_reset(&self) -> bool {
        self.0.get(2)
    }

    pub fn set_fifo_reset(&mut self, v: bool) {
        self.0 = self.0.set(2, v);
    }

    /// When set to 1, this bit enables I2C Master Mode.
    /// When this bit is cleared to 0, the auxiliary I2C bus lines
    /// (AUX_DA and AUX_CL) are logically driven by the primary I2C bus (SDA and SCL).
    pub fn get_i2cmst_en(&self) -> bool {
        self.0.get(5)
    }

    pub fn set_i2cmst_en(&mut self, v: bool) {
        self.0 = self.0.set(5, v);
    }

    /// When set to 1, this bit enables FIFO operations.
    /// When this bit is cleared to 0, the FIFO buffer is disabled.
    /// The FIFO buffer cannot be written to or read from while disabled.
    /// The FIFO buffer’s state does not change unless the MPU-60X0 is power cycled.
    pub fn get_fifo_en(&self) -> bool {
        self.0.get(6)
    }

    pub fn set_fifo_en(&mut self, v: bool) {
        self.0 = self.0.set(6, v.into());
    }
}

/// This register enables interrupt generation by interrupt sources.
/// For information regarding the interrupt status for each interrupt generation source,
/// please refer to Register 58.
/// Further information regarding I2C Master interrupt generation can be found in Register 54.
#[derive(Debug, From, Into, Clone, Copy, PartialEq, Eq)]
pub struct IntEnable(u8);

impl Register for IntEnable {
    const ADDR: RegAddr = RegAddr(0x38);
}

impl IntEnable {
    /// When set to 1, this bit enables the Data Ready interrupt,
    /// which occurs each time a write operation to all of the sensor registers has been completed.
    pub fn get_datardy_en(&self) -> bool {
        self.0.get(0)
    }

    pub fn set_datardy_en(&mut self, v: bool) {
        self.0 = self.0.set(0, v.into());
    }

    /// When set to 1, this bit enables any of the I2C Master
    /// interrupt sources to generate an interrupt.
    pub fn get_i2cmst_int_en(&self) -> bool {
        self.0.get(3)
    }

    pub fn set_i2cmst_int_en(&mut self, v: bool) {
        self.0 = self.0.set(3, v.into());
    }

    /// When set to 1, this bit enables a FIFO buffer overflow to generate an interrupt.
    pub fn get_fifo_oflow_en(&self) -> bool {
        self.0.get(4)
    }

    pub fn set_fifo_oflow_en(&mut self, v: bool) {
        self.0 = self.0.set(4, v);
    }

    /// When set to 1, this bit enables Motion detection to generate an interrupt.
    pub fn get_mot_en(&self) -> bool {
        self.0.get(6)
    }

    pub fn set_mot_en(&mut self, v: bool) {
        self.0 = self.0.set(6, v);
    }
}

/// This register determines which sensor measurements are loaded into the FIFO buffer.
#[derive(Debug, From, Into, Clone, Copy, PartialEq, Eq)]
pub struct FifoEnable(u8);

impl Register for FifoEnable {
    const ADDR: RegAddr = RegAddr(0x23);
}

impl FifoEnable {
    pub fn get_slv0(&self) -> bool {
        self.0.get(0)
    }

    pub fn set_slv0(&mut self, v: bool) {
        self.0 = self.0.set(0, v);
    }

    pub fn get_slv1(&self) -> bool {
        self.0.get(1)
    }

    pub fn set_slv1(&mut self, v: bool) {
        self.0 = self.0.set(1, v);
    }

    pub fn get_slv2(&self) -> bool {
        self.0.get(2)
    }

    pub fn set_slv2(&mut self, v: bool) {
        self.0 = self.0.set(2, v);
    }

    pub fn get_accel(&self) -> bool {
        self.0.get(3)
    }

    pub fn set_accel(&mut self, v: bool) {
        self.0 = self.0.set(3, v);
    }

    pub fn get_zg(&self) -> bool {
        self.0.get(4)
    }

    pub fn set_zg(&mut self, v: bool) {
        self.0 = self.0.set(4, v);
    }

    pub fn get_yg(&self) -> bool {
        self.0.get(5)
    }

    pub fn set_yg(&mut self, v: bool) {
        self.0 = self.0.set(5, v);
    }

    pub fn get_xg(&self) -> bool {
        self.0.get(6)
    }

    pub fn set_xg(&mut self, v: bool) {
        self.0 = self.0.set(6, v);
    }

    pub fn get_temp(&self) -> bool {
        self.0.get(7)
    }

    pub fn set_temp(&mut self, v: bool) {
        self.0 = self.0.set(7, v);
    }
}

/// This register is used to read and write data from the FIFO buffer.
#[derive(Debug, From, Into, Clone, Copy, PartialEq, Eq)]
pub struct FifoData(u8);

impl Register for FifoData {
    const ADDR: RegAddr = RegAddr(0x74);
}

// ----------------------------------------------------------------
// ----------------------------------------------------------------

pub trait RegisterRange {
    const ADDR: RegAddr;
}

impl RegisterRange for FifoCount {
    const ADDR: RegAddr = RegAddr(0x72);
}

impl From<[u8; 2]> for FifoCount {
    fn from(buf: [u8; 2]) -> Self {
        Self::from(u16::from_be_bytes(buf))
    }
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
        Self::from(i16::from_be_bytes(data))
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
    fn from(buf: [u8; 14]) -> Self {
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
