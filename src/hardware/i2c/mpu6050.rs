mod raw_data;
mod register;

use embedded_hal::blocking::i2c::{Write, WriteRead};
use i2cdev::linux::LinuxI2CError;
use linux_embedded_hal::I2cdev;
use raw_data::*;
use register::*;
use std::convert::TryInto;

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
        self.read_register(AccelData::ADDR, &mut buf)?;
        Ok(buf_into_raw_data(buf))
    }

    fn read_register(&mut self, reg: RegAddr, res: &mut [u8]) -> Result<()> {
        self.dev.write_read(self.address.as_u8(), &[reg.as_u8()], res)
    }

    fn write_register(&mut self, reg: RegAddr, v: u8) -> Result<()> {
        self.dev.write(self.address.as_u8(), &[reg.as_u8(), v])
    }
}

fn buf_into_raw_data(buf: [u8; 14]) -> RawData {
    let array_accel: [u8; 6] = buf[..6].try_into().expect("Accel data must be here");
    let array_temp: [u8; 2] = buf[6..8].try_into().expect("Temperature data must be here");
    let array_gyro: [u8; 6] = buf[8..].try_into().expect("Gyro data must be here");
    RawData {
        accel: AccelData::from(array_accel),
        temp: Temperature::from(array_temp),
        gyro: GyroData::from(array_gyro),
    }
}
