pub mod mpu6050;
pub mod pca9685;
pub mod register_io;
pub mod servo;
pub mod thread_safe;

pub use register_io::*;
pub use thread_safe::*;

use backtrace::Backtrace;
use derive_more::{From, Into};
use embedded_hal::blocking::i2c::SevenBitAddress;
use lazy_static::lazy_static;
use linux_embedded_hal::I2cdev;
use parking_lot::Mutex;
use std::collections::HashMap;

type Dev = ThreadSafeI2c<I2cdev>;
type ResultDev = Result<Dev, String>;

lazy_static! {
    static ref DEVICES: Mutex<HashMap<u8, ResultDev>> = Mutex::new(HashMap::new());
}

#[derive(Debug, From, Into, Clone, Copy, PartialEq, Eq, Hash)]
pub struct I2cAddr(pub SevenBitAddress);

fn get_connect(path: &str) -> ResultDev {
    I2cdev::new(&path).map(ThreadSafeI2c::new).map_err(|err| {
        eprintln!("Failed to connect I2C device '{}': {}", path, err);
        eprintln!("{:?}", Backtrace::new());
        err.to_string()
    })
}

pub fn connect(bus: u8) -> std::io::Result<Dev> {
    let path = format!("/dev/i2c-{}", bus);

    let result = DEVICES
        .lock()
        .entry(bus)
        .or_insert_with(|| get_connect(&path))
        .clone();

    result.map_err(|msg| std::io::Error::new(std::io::ErrorKind::NotConnected, msg))
}
