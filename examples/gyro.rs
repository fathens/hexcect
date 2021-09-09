extern crate hexcect;

use hexcect::hardware::i2c::connect;
use hexcect::hardware::i2c::mpu6050::{ADDRESS_LOW, MPU6050};
use hexcect::hardware::i2c::register_io::I2cWithAddr;
use linux_embedded_hal::Delay;
use std::io::stdout;

use crossterm::{cursor::MoveUp, execute, style::Print};

fn main() {
    let dev = connect(1).unwrap();
    let mut mpu = MPU6050::new(I2cWithAddr::new(dev, ADDRESS_LOW)).unwrap();
    mpu.normal_setup(&mut Delay).unwrap();

    loop {
        let info = mpu.get_infos().unwrap();

        execute!(stdout(), Print(format!("{:?}\n", info)), MoveUp(1),).unwrap();
    }
}
