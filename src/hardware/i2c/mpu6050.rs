mod raw_data;
mod register;

use i2cdev::linux::LinuxI2CError;
use linux_embedded_hal::I2cdev;
use raw_data::*;
use register::*;
use std::io::Result;

pub enum Address {
    LOW,
    HIGH,
    Custom(u8),
}

impl From<Address> for u8 {
    fn from(a: Address) -> u8 {
        match a {
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

trait DevI2c {
    fn write() -> Result<()>;
}

impl MPU6050 {
    pub fn new(dev: I2cdev, address: Address) -> Result<MPU6050> {
        Ok(MPU6050 { dev, address })
    }

    pub fn get_infos(&mut self) -> Result<RawData> {
        let info = RawData {
            temp: 0.0,
            gyro: GyroData { x: 0, y: 0, z: 0 },
            acc: AccData { x: 0, y: 0, z: 0 },
        };
        Ok(info)
    }
}
