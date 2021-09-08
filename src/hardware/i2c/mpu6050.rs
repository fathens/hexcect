mod raw_data;
mod register;

use embedded_hal::blocking::i2c::{Write, WriteRead};
use parking_lot::Mutex;
use raw_data::*;
use register::*;
use std::result::Result;
use std::sync::Arc;

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

#[derive(Debug)]
pub enum Error<T>
where
    T: Write + WriteRead,
    <T as Write>::Error: core::fmt::Debug,
    <T as WriteRead>::Error: core::fmt::Debug,
{
    WriteError(<T as Write>::Error),
    WriteReadError(<T as WriteRead>::Error),
}

pub struct MPU6050<T> {
    dev: Arc<Mutex<T>>,
    address: Address,
}

impl<T> MPU6050<T>
where
    T: Write + WriteRead,
    <T as Write>::Error: core::fmt::Debug,
    <T as WriteRead>::Error: core::fmt::Debug,
{
    pub fn new(dev: Arc<Mutex<T>>, address: Address) -> Result<MPU6050<T>, Error<T>> {
        Ok(MPU6050 { dev, address })
    }

    fn read_bytes(&self, reg: RegAddr, res: &mut [u8]) -> Result<(), Error<T>> {
        let mut dev = self.dev.lock();
        dev.write_read(self.address.as_u8(), &[reg.as_u8()], res)
            .map_err(Error::WriteReadError)
    }

    fn read_byte(&self, reg: RegAddr) -> Result<u8, Error<T>> {
        let mut buf = [0; 1];
        self.read_bytes(reg, &mut buf)?;
        Ok(buf[0])
    }

    fn write_byte(&self, reg: RegAddr, v: u8) -> Result<(), Error<T>> {
        let mut dev = self.dev.lock();
        dev.write(self.address.as_u8(), &[reg.as_u8(), v])
            .map_err(Error::WriteError)
    }

    // ----------------------------------------------------------------

    fn read_register<R: Register>(&self) -> Result<R, Error<T>> {
        let byte = self.read_byte(R::ADDR)?;
        Ok(R::from(byte))
    }

    fn write_register<R: Register>(&self, reg_value: R) -> Result<(), Error<T>> {
        self.write_byte(R::ADDR, reg_value.as_u8())
    }

    // ----------------------------------------------------------------
    // ----------------------------------------------------------------

    pub fn set_digital_lowpass_filter(
        &self,
        filter: DigitalLowPassFilterCfg,
    ) -> Result<(), Error<T>> {
        let mut value: Configure = self.read_register()?;
        value.dlpf_cfg = filter;
        self.write_register(value)
    }

    pub fn set_accel_full_scale(&self, scale: AccelFullScale) -> Result<(), Error<T>> {
        let mut value: AccelConfig = self.read_register()?;
        value.afs_sel = scale;
        self.write_register(value)
    }

    pub fn set_gyro_full_scale(&self, scale: GyroFullScale) -> Result<(), Error<T>> {
        let mut value: GyroConfig = self.read_register()?;
        value.fs_sel = scale;
        self.write_register(value)
    }

    pub fn get_infos(&self) -> Result<RawData, Error<T>> {
        let mut buf = [0; 14];
        self.read_bytes(AccelData::ADDR, &mut buf)?;
        Ok(RawData::from(buf))
    }
}
