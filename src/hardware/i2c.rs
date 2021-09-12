pub mod mpu6050;
pub mod pca9685;
pub mod register_io;
pub mod servo;
pub mod thread_safe;

pub use register_io::*;
pub use thread_safe::*;

use derive_more::{From, Into};
use embedded_hal::blocking::i2c::SevenBitAddress;
use linux_embedded_hal::I2cdev;
use std::io::Result;

#[derive(Debug, From, Into, Clone, Copy, PartialEq, Eq, Hash)]
pub struct I2cAddr(pub SevenBitAddress);

pub fn connect(bus: u8) -> Result<ThreadSafeI2c<I2cdev>> {
    let path = format!("/dev/i2c-{}", bus);
    let dev = I2cdev::new(&path)?;
    let safe = thread_safe::ThreadSafeI2c::new(dev);
    Ok(safe)
}
