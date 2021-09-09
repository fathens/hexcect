pub mod error;
mod raw_data;
mod register;

use error::Error;
use raw_data::*;
use register::*;

use derive_more::{From, Into};
use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::blocking::i2c::{Write, WriteRead};
use std::result::Result;

#[derive(Debug, From, Into, Clone, Copy, PartialEq, Eq)]
pub struct Address(u8);

impl Address {
    pub const LOW: Address = Address(0x68);
    pub const HIGH: Address = Address(0x69);
}

impl Default for Address {
    fn default() -> Self {
        Address::LOW
    }
}

pub struct MPU6050<T> {
    dev: T,
    address: Address,
}

impl<T> MPU6050<T>
where
    T: Write + WriteRead,
    <T as Write>::Error: core::fmt::Debug,
    <T as WriteRead>::Error: core::fmt::Debug,
{
    pub fn new(dev: T, address: Address) -> Result<MPU6050<T>, Error<T>> {
        let o = MPU6050 { dev, address };
        // ここで何かすることになるかもしれないので Result 型にしている。
        Ok(o)
    }

    pub fn normal_setup<D: DelayMs<u8>>(&mut self, d: &mut D) -> Result<(), Error<T>> {
        self.reset(d)?;
        self.set_sleep_enabled(false)?;
        self.disable_all_interrupts()?;
        self.set_clock_source(ClockSel::Xgyro)?;
        self.set_accel_full_scale(AccelFullScale::G2)?;
        self.set_gyro_full_scale(GyroFullScale::Deg2000)?;
        self.set_sample_rate_divider(4.into())?;
        Ok(())
    }

    // ----------------------------------------------------------------
    // ----------------------------------------------------------------

    fn read_bytes(&mut self, reg: RegAddr, res: &mut [u8]) -> Result<(), Error<T>> {
        self.dev
            .write_read(self.address.into(), &[reg.into()], res)
            .map_err(Error::WriteReadError)
    }

    fn read_byte(&mut self, reg: RegAddr) -> Result<u8, Error<T>> {
        let mut buf = [0; 1];
        self.read_bytes(reg, &mut buf)?;
        Ok(buf[0])
    }

    fn write_byte(&mut self, reg: RegAddr, v: u8) -> Result<(), Error<T>> {
        self.dev
            .write(self.address.into(), &[reg.into(), v])
            .map_err(Error::WriteError)
    }

    // ----------------------------------------------------------------

    fn read_register<R: Register>(&mut self) -> Result<R, Error<T>> {
        let byte = self.read_byte(R::ADDR)?;
        Ok(R::from(byte))
    }

    fn write_register<R: Register>(&mut self, reg_value: R) -> Result<(), Error<T>>
    where
        u8: From<R>,
    {
        self.write_byte(R::ADDR, reg_value.into())
    }

    // ----------------------------------------------------------------
    // ----------------------------------------------------------------

    pub fn reset<D: DelayMs<u8>>(&mut self, d: &mut D) -> Result<(), Error<T>> {
        let mut value: PwrMgmt1 = self.read_register()?;
        value.set_device_reset(true);
        self.write_register(value)?;
        d.delay_ms(200);
        Ok(())
    }

    pub fn set_sleep_enabled(&mut self, v: bool) -> Result<(), Error<T>> {
        let mut value: PwrMgmt1 = self.read_register()?;
        value.set_sleep(v);
        self.write_register(value)
    }

    pub fn reset_signal_path<D: DelayMs<u8>>(&mut self, d: &mut D) -> Result<(), Error<T>> {
        let mut value: UserCtrl = self.read_register()?;
        value.set_sigcond_reset(true);
        self.write_register(value)?;
        d.delay_ms(200);
        Ok(())
    }

    pub fn disable_all_interrupts(&mut self) -> Result<(), Error<T>> {
        self.write_register(IntEnable::from(0))
    }

    pub fn set_clock_source(&mut self, v: ClockSel) -> Result<(), Error<T>> {
        let mut value: PwrMgmt1 = self.read_register()?;
        value.set_clksel(v);
        self.write_register(value)
    }

    pub fn set_sample_rate_divider(&mut self, v: SampleRateDivider) -> Result<(), Error<T>> {
        self.write_register(v)
    }

    pub fn set_digital_lowpass_filter(
        &mut self,
        filter: DigitalLowPassFilterCfg,
    ) -> Result<(), Error<T>> {
        let mut value: Configure = self.read_register()?;
        value.set_dlpf(filter);
        self.write_register(value)
    }

    pub fn set_accel_full_scale(&mut self, scale: AccelFullScale) -> Result<(), Error<T>> {
        let mut value: AccelConfig = self.read_register()?;
        value.set_scale(scale);
        self.write_register(value)
    }

    pub fn set_gyro_full_scale(&mut self, scale: GyroFullScale) -> Result<(), Error<T>> {
        let mut value: GyroConfig = self.read_register()?;
        value.set_scale(scale);
        self.write_register(value)
    }

    pub fn get_infos(&mut self) -> Result<RawData, Error<T>> {
        let mut buf = [0; 14];
        self.read_bytes(AccelData::ADDR, &mut buf)?;
        Ok(RawData::from(buf))
    }
}
