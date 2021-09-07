mod raw_data;
mod register;

use embedded_hal::blocking::i2c::{Write, WriteRead};
use i2cdev::linux::LinuxI2CError;
use linux_embedded_hal::I2cdev;
use raw_data::*;
use register::*;

type Result<T> = std::result::Result<T, LinuxI2CError>;

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

pub struct MPU6050 {
    dev: I2cdev,
    address: Address,
}

impl MPU6050 {
    pub fn new(dev: I2cdev, address: Address) -> Result<MPU6050> {
        Ok(MPU6050 { dev, address })
    }

    pub fn get_infos(&mut self) -> Result<RawData> {
        let mut buf = [0; 14];
        self.read_registers(AccelData::ADDR, &mut buf)?;
        Ok(RawData::from(buf))
    }

    fn read_registers(&mut self, reg: RegAddr, res: &mut [u8]) -> Result<()> {
        self.dev
            .write_read(self.address.as_u8(), &[reg.as_u8()], res)
    }

    fn read_register(&mut self, reg: RegAddr) -> Result<u8> {
        let mut buf = [0; 1];
        self.dev
            .write_read(self.address.as_u8(), &[reg.as_u8()], &mut buf)?;
        Ok(buf[0])
    }

    fn write_register(&mut self, reg: RegAddr, v: u8) -> Result<()> {
        self.dev.write(self.address.as_u8(), &[reg.as_u8(), v])
    }
}
