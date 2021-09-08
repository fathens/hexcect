extern crate hexcect;

use hexcect::hardware::i2c::connect;
use hexcect::hardware::i2c::mpu6050::{Address, MPU6050};
use parking_lot::Mutex;
use std::io::stdout;
use std::sync::Arc;

use crossterm::{cursor::MoveUp, execute, style::Print};

fn main() {
    let dev = connect(1).unwrap();
    let dev = Arc::new(Mutex::new(dev));
    let mpu = MPU6050::new(dev, Address::LOW).unwrap();

    loop {
        let info = mpu.get_infos().unwrap();

        execute!(stdout(), Print(format!("{:?}\n", info)), MoveUp(1),).unwrap();
    }
}
