pub mod mpu6050;
pub mod pca9685;
pub mod register_io;
pub mod servo;
pub mod thread_safe;

pub use register_io::*;
pub use thread_safe::*;

use derive_more::{From, Into};
use embedded_hal::blocking::i2c::SevenBitAddress;
use i2cdev::linux::LinuxI2CError;
use linux_embedded_hal::I2cdev;

#[derive(Debug, From, Into, Clone, Copy, PartialEq, Eq, Hash)]
pub struct I2cAddr(pub SevenBitAddress);

pub fn connect(bus: u8) -> Result<I2cdev, LinuxI2CError> {
    let path = format!("/dev/i2c-{}", bus);
    I2cdev::new(&path)
}
