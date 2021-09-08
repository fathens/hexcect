mod raw_data;
mod register;

use embedded_hal::blocking::i2c::{Write, WriteRead};
use parking_lot::Mutex;
use raw_data::*;
use register::*;
use std::result::Result;
use std::sync::Arc;

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

    pub fn get_infos(&self) -> Result<RawData, Error<T>> {
        let mut buf = [0; 14];
        self.read_registers(AccelData::ADDR, &mut buf)?;
        Ok(RawData::from(buf))
    }

    // ----------------------------------------------------------------
    // ----------------------------------------------------------------

    fn read_registers(&self, reg: RegAddr, res: &mut [u8]) -> Result<(), Error<T>> {
        let mut dev = self.dev.lock();
        dev.write_read(self.address.as_u8(), &[reg.as_u8()], res)
            .map_err(Error::WriteReadError)
    }

    fn read_register(&self, reg: RegAddr) -> Result<u8, Error<T>> {
        let mut buf = [0; 1];
        let mut dev = self.dev.lock();
        dev.write_read(self.address.as_u8(), &[reg.as_u8()], &mut buf)
            .map_err(Error::WriteReadError)?;
        Ok(buf[0])
    }

    fn write_register(&self, reg: RegAddr, v: u8) -> Result<(), Error<T>> {
        let mut dev = self.dev.lock();
        dev.write(self.address.as_u8(), &[reg.as_u8(), v])
            .map_err(Error::WriteError)
    }
}
