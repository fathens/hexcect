use hardware::i2c::connect;
use hardware::i2c::mpu6050::raw_data::{AccelFullScale, GyroFullScale};
use hardware::i2c::mpu6050::{ADDRESS_LOW, MPU6050};
use hardware::i2c::register_io::I2cWithAddr;
use linux_embedded_hal::Delay;
use num_traits::FromPrimitive;
use std::io::{stdout, Result as IOResult};
use std::result::Result;

use crossterm::*;
use ctrlc;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Args {
    #[structopt(long, default_value = "0", parse(try_from_str = parse_accel))]
    accel: AccelFullScale,

    #[structopt(long, default_value = "0", parse(try_from_str = parse_gyro))]
    gyro: GyroFullScale,
}

fn parse_accel(src: &str) -> Result<AccelFullScale, String> {
    from_u8(src, |v| {
        AccelFullScale::from_u8(v)
            .ok_or_else(|| format!("Accel Full Scale Selector is out of range [0-3]: {}", v))
    })
}

fn parse_gyro(src: &str) -> Result<GyroFullScale, String> {
    from_u8(src, |v| {
        GyroFullScale::from_u8(v)
            .ok_or_else(|| format!("Gyro Full Scale Selector is out of range [0-3]: {}", v))
    })
}

fn main() {
    let args = Args::from_args();
    run_loop(args.accel, args.gyro).unwrap();
}

fn run_loop(accel_fs: AccelFullScale, gyro_fs: GyroFullScale) -> IOResult<()> {
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
        let accel_info = info.accel.scale::<f64>(accel_fs);
        let gyro_info = info.gyro.scale::<f64>(gyro_fs);

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

// ----------------------------------------------------------------

fn from_u8<T, F>(src: &str, f: F) -> Result<T, String>
where
    F: Fn(u8) -> Result<T, String>,
{
    u8::from_str_radix(src, 10)
        .map_err(|e| e.to_string())
        .and_then(f)
}
