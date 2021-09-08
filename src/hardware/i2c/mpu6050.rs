pub mod error;
mod raw_data;
mod register;

use embedded_hal::blocking::i2c::{Write, WriteRead};
use error::Error;
use raw_data::*;
use register::*;
use std::result::Result;

#[derive(Debug, PartialEq, Eq)]
pub enum Address {
    LOW,
    HIGH,
    Custom(u8),
}

impl Address {
    fn as_u8(&self) -> u8 {
        match *self {
            Address::LOW => 0x68,
            Address::HIGH => 0x69,
            Address::Custom(v) => v,
        }
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
        Ok(MPU6050 { dev, address })
    }

    fn read_bytes(&mut self, reg: RegAddr, res: &mut [u8]) -> Result<(), Error<T>> {
        self.dev
            .write_read(self.address.as_u8(), &[reg.into()], res)
            .map_err(Error::WriteReadError)
    }

    fn read_byte(&mut self, reg: RegAddr) -> Result<u8, Error<T>> {
        let mut buf = [0; 1];
        self.read_bytes(reg, &mut buf)?;
        Ok(buf[0])
    }

    fn write_byte(&mut self, reg: RegAddr, v: u8) -> Result<(), Error<T>> {
        self.dev
            .write(self.address.as_u8(), &[reg.into(), v])
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
