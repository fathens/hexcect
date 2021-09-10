extern crate hexcect;

use hexcect::hardware::i2c::connect;
use hexcect::hardware::i2c::mpu6050::raw_data::{AccelFullScale, GyroFullScale};
use hexcect::hardware::i2c::mpu6050::{ADDRESS_LOW, MPU6050};
use hexcect::hardware::i2c::register_io::I2cWithAddr;
use linux_embedded_hal::Delay;
use num_traits::FromPrimitive;
use std::io::{stdout, Result};

use crossterm::*;
use ctrlc;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Args {
    #[structopt(long, default_value = "0")]
    accel: u8,

    #[structopt(long, default_value = "0")]
    gyro: u8,
}

fn main() {
    let args = Args::from_args();
    let accel_fs = AccelFullScale::from_u8(args.accel).unwrap_or_else(|| {
        eprintln!(
            "Accel Full Scale Selector is out of range [0-3]: {}",
            args.accel
        );
        std::process::exit(1);
    });
    let gyro_fs = GyroFullScale::from_u8(args.gyro).unwrap_or_else(|| {
        eprintln!(
            "Gyro Full Scale Selector is out of range [0-3]: {}",
            args.gyro
        );
        std::process::exit(1);
    });
    run_loop(accel_fs, gyro_fs).unwrap();
}

fn run_loop(accel_fs: AccelFullScale, gyro_fs: GyroFullScale) -> Result<()> {
    let dev = connect(1)?;
    let mut mpu = MPU6050::new(I2cWithAddr::new(dev, ADDRESS_LOW)).unwrap();
    mpu.normal_setup(&mut Delay).unwrap();
    mpu.set_gyro_full_scale(gyro_fs).unwrap();
    mpu.set_accel_full_scale(accel_fs).unwrap();

    ctrlc::set_handler(|| {
        execute!(
            stdout(),
            terminal::Clear(terminal::ClearType::All),
            cursor::Show,
            cursor::MoveTo(0, 0),
        )
        .unwrap();
        std::process::exit(0);
    })
    .unwrap();

    execute!(
        stdout(),
        cursor::Hide,
        terminal::Clear(terminal::ClearType::All),
    )?;

    loop {
        let info = mpu.get_infos().unwrap();
        let accel_info = info.accel.scale(accel_fs);
        let gyro_info = info.gyro.scale(gyro_fs);

        execute!(
            stdout(),
            style::Print(format!("{:?}\n", info.accel)),
            style::Print(format!("{:?}\n", info.gyro)),
            cursor::MoveDown(1),
            style::Print(format!("{:?}\n", accel_info)),
            style::Print(format!("{:?}\n", gyro_info)),
            cursor::MoveUp(5),
        )?;

        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}
