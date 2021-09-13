pub mod mpu6050;
pub mod pca9685;
pub mod register_io;
pub mod servo;
pub mod thread_safe;

pub use register_io::*;
pub use thread_safe::*;

use derive_more::{From, Into};
use embedded_hal::blocking::i2c::SevenBitAddress;
use lazy_static::lazy_static;
use linux_embedded_hal::I2cdev;
use parking_lot::Mutex;
use std::collections::HashMap;
use std::io::{ErrorKind, Result};
use backtrace::Backtrace;

type Dev = ThreadSafeI2c<I2cdev>;

lazy_static! {
    static ref DEVICES: Mutex<HashMap<u8, Option<Dev>>> = Mutex::new(HashMap::new());
}

#[derive(Debug, From, Into, Clone, Copy, PartialEq, Eq, Hash)]
pub struct I2cAddr(pub SevenBitAddress);

fn get_connect(path: &str) -> Option<Dev> {
    match I2cdev::new(&path) {
        Ok(dev) => Some(thread_safe::ThreadSafeI2c::new(dev)),
        Err(err) => {
            eprintln!("Could not connect to I2C {}: {}", path, err);
            eprintln!("{:?}", Backtrace::new());
            None
        }
    }
}

pub fn connect(bus: u8) -> Result<Dev> {
    let path = format!("/dev/i2c-{}", bus);

    let result = DEVICES
        .lock()
        .entry(bus)
        .or_insert_with(|| get_connect(&path))
        .clone();

    result.ok_or_else(|| {
        std::io::Error::new(ErrorKind::NotConnected, format!("No I2C device: {}", path))
    })
}
