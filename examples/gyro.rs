extern crate hexcect;

use hexcect::hardware::i2c::connect;
use hexcect::hardware::i2c::mpu6050::MPU6050;
use std::io::stdout;

use crossterm::{cursor::MoveUp, execute, style::Print};

fn main() {
    let dev = connect(1).unwrap();
    let mut mpu = MPU6050::new(dev).unwrap();

    loop {
        let info = mpu.get_infos().unwrap();

        execute!(stdout(), Print(format!("{:?}\n", info)), MoveUp(1),).unwrap();
    }
}
