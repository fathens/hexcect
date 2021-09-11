pub mod error;
pub mod raw_data;
mod register;

use crate::hardware::i2c::register_io::*;
use raw_data::*;
use register::*;

use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::blocking::i2c::{Write, WriteRead};
use std::result::Result;

pub const ADDRESS_LOW: I2cAddr = I2cAddr(0x68);
pub const ADDRESS_HIGH: I2cAddr = I2cAddr(0x69);

pub struct MPU6050<T> {
    dev: I2cWithAddr<T>,
}

impl<T> MPU6050<T>
where
    T: Write + WriteRead,
    <T as Write>::Error: core::fmt::Debug,
    <T as WriteRead>::Error: core::fmt::Debug,
{
    pub fn new(dev: I2cWithAddr<T>) -> Result<MPU6050<T>, Error<T>> {
        let o = MPU6050 { dev };
        // ここで何かすることになるかもしれないので Result 型にしている。
        Ok(o)
    }

    pub fn normal_setup(&mut self, d: &mut impl DelayMs<u8>) -> Result<(), Error<T>> {
        self.reset(d)?;
        self.set_sleep_enabled(false)?;
        self.disable_all_interrupts()?;
        self.set_clock_source(ClockSel::Xgyro)?;
        self.set_accel_full_scale(AccelFullScale::G2)?;
        self.set_gyro_full_scale(GyroFullScale::Deg2000)?;
        self.set_sample_rate_divider(4.into())?;
        Ok(())
    }

    pub fn reset(&mut self, d: &mut impl DelayMs<u8>) -> Result<(), Error<T>> {
        let mut value: PwrMgmt1 = self.dev.read_register()?;
        value.set_device_reset(true);
        self.dev.write_register(value)?;
        d.delay_ms(200);
        Ok(())
    }

    pub fn set_sleep_enabled(&mut self, v: bool) -> Result<(), Error<T>> {
        let mut value: PwrMgmt1 = self.dev.read_register()?;
        value.set_sleep(v);
        self.dev.write_register(value)
    }

    pub fn reset_signal_path(&mut self, d: &mut impl DelayMs<u8>) -> Result<(), Error<T>> {
        let mut value: UserCtrl = self.dev.read_register()?;
        value.set_sigcond_reset(true);
        self.dev.write_register(value)?;
        d.delay_ms(200);
        Ok(())
    }

    pub fn disable_all_interrupts(&mut self) -> Result<(), Error<T>> {
        self.dev.write_register(IntEnable::from(0))
    }

    pub fn set_clock_source(&mut self, v: ClockSel) -> Result<(), Error<T>> {
        let mut value: PwrMgmt1 = self.dev.read_register()?;
        value.set_clksel(v);
        self.dev.write_register(value)
    }

    pub fn set_sample_rate_divider(&mut self, v: SampleRateDivider) -> Result<(), Error<T>> {
        self.dev.write_register(v)
    }

    pub fn set_digital_lowpass_filter(
        &mut self,
        filter: DigitalLowPassFilterCfg,
    ) -> Result<(), Error<T>> {
        let mut value: Configure = self.dev.read_register()?;
        value.set_dlpf(filter);
        self.dev.write_register(value)
    }

    pub fn set_accel_full_scale(&mut self, scale: AccelFullScale) -> Result<(), Error<T>> {
        let mut value: AccelConfig = self.dev.read_register()?;
        value.set_scale(scale);
        self.dev.write_register(value)
    }

    pub fn set_gyro_full_scale(&mut self, scale: GyroFullScale) -> Result<(), Error<T>> {
        let mut value: GyroConfig = self.dev.read_register()?;
        value.set_scale(scale);
        self.dev.write_register(value)
    }

    pub fn get_infos(&mut self) -> Result<RawData, Error<T>> {
        let mut buf = [0; 14];
        self.dev.read_bytes(AccelData::ADDR, &mut buf)?;
        Ok(RawData::from(&buf))
    }
}
