use hardware::i2c::connect;
use hardware::i2c::mpu6050::raw_data::{AccelFullScale, GyroFullScale};
use hardware::i2c::mpu6050::{ADDRESS_LOW, MPU6050};
use hardware::i2c::pca9685::PCA9685;
use hardware::i2c::register_io::I2cWithAddr;
use hardware::i2c::servo::SG90_180;
use hardware::model::sensor::{AccelInfo, GyroInfo};
use linux_embedded_hal::Delay;
use num_traits::FromPrimitive;
use pwm_pca9685::Channel;
use std::io::Result as IOResult;
use std::result::Result;
use std::sync::mpsc;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Args {
    #[structopt(long, default_value = "0", parse(try_from_str = parse_accel))]
    accel_fs: AccelFullScale,

    #[structopt(long, default_value = "0", parse(try_from_str = parse_gyro))]
    gyro_fs: GyroFullScale,

    #[structopt(long, default_value = "100")]
    accel_solt: f64,

    #[structopt(long, default_value = "100")]
    gyro_solt: f64,

    #[structopt(long, default_value = "100")]
    accel_filter: f64,

    #[structopt(long, default_value = "100")]
    gyro_filter: f64,

    #[structopt(long, default_value = "0", parse(try_from_str = parse_channel))]
    servo_accel_x: Channel,

    #[structopt(long, default_value = "1", parse(try_from_str = parse_channel))]
    servo_accel_y: Channel,

    #[structopt(long, default_value = "2", parse(try_from_str = parse_channel))]
    servo_accel_z: Channel,

    #[structopt(long, default_value = "3", parse(try_from_str = parse_channel))]
    servo_gyro_x: Channel,

    #[structopt(long, default_value = "4", parse(try_from_str = parse_channel))]
    servo_gyro_y: Channel,

    #[structopt(long, default_value = "6", parse(try_from_str = parse_channel))]
    servo_gyro_z: Channel,
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

fn parse_channel(src: &str) -> Result<Channel, String> {
    from_u8(src, |v| match v {
        0 => Ok(Channel::C0),
        1 => Ok(Channel::C1),
        2 => Ok(Channel::C2),
        3 => Ok(Channel::C3),
        4 => Ok(Channel::C4),
        5 => Ok(Channel::C5),
        6 => Ok(Channel::C6),
        7 => Ok(Channel::C7),
        8 => Ok(Channel::C8),
        9 => Ok(Channel::C9),
        10 => Ok(Channel::C10),
        11 => Ok(Channel::C11),
        12 => Ok(Channel::C12),
        13 => Ok(Channel::C13),
        14 => Ok(Channel::C14),
        15 => Ok(Channel::C15),
        _ => Err(format!("Channel number is out of range [0-15]: {}", v)),
    })
}

struct ServeAccel {
    x: Channel,
    y: Channel,
    z: Channel,
}

struct ServeGyro {
    x: Channel,
    y: Channel,
    z: Channel,
}

fn main() -> IOResult<()> {
    let args = Args::from_args();
    let accel_servoes = ServeAccel {
        x: args.servo_accel_x,
        y: args.servo_accel_y,
        z: args.servo_accel_z,
    };
    let gyro_servoes = ServeGyro {
        x: args.servo_gyro_x,
        y: args.servo_gyro_y,
        z: args.servo_gyro_z,
    };

    let for_pca9685 = start_pca9685()?;

    let for_accel = start_accel(
        for_pca9685.clone(),
        args.accel_fs,
        accel_servoes,
        args.accel_solt,
        args.accel_filter,
    )?;
    let for_gyro = start_gyro(
        for_pca9685.clone(),
        args.gyro_fs,
        gyro_servoes,
        args.gyro_solt,
        args.gyro_filter,
    )?;

    start_mpu6050(args.accel_fs, args.gyro_fs, for_accel, for_gyro)
}

fn start_mpu6050(
    accel_fs: AccelFullScale,
    gyro_fs: GyroFullScale,
    for_accel: mpsc::Sender<AccelInfo>,
    for_gyro: mpsc::Sender<GyroInfo>,
) -> IOResult<()> {
    let dev = connect(1)?;
    let mut mpu = MPU6050::new(I2cWithAddr::new(dev, ADDRESS_LOW)).unwrap();
    mpu.normal_setup(&mut Delay).unwrap();
    mpu.set_gyro_full_scale(gyro_fs).unwrap();
    mpu.set_accel_full_scale(accel_fs).unwrap();

    loop {
        let info = mpu.get_infos().unwrap();
        let accel_info = info.accel.scale(accel_fs);
        let gyro_info = info.gyro.scale(gyro_fs);

        for_accel.send(accel_info).unwrap();
        for_gyro.send(gyro_info).unwrap();
    }
}

fn start_pca9685() -> IOResult<mpsc::Sender<(Channel, f64)>> {
    let (tx, rx) = mpsc::channel();

    std::thread::spawn(move || {
        let dev = connect(1).unwrap();
        let mut pca9685 = PCA9685::new(dev, 0x40).unwrap();

        for (channel, rate) in rx {
            pca9685.set_one_duty_cycle(channel, rate).unwrap();
        }
    });

    Ok(tx)
}

fn start_accel(
    for_pca9685: mpsc::Sender<(Channel, f64)>,
    scale: AccelFullScale,
    channels: ServeAccel,
    solt: f64,
    filter: f64,
) -> IOResult<mpsc::Sender<AccelInfo>> {
    let (tx, rx) = mpsc::channel::<AccelInfo>();

    std::thread::spawn(move || {
        let mut values = ValueXYZ::new(
            solt * scale.max() as f64,
            filter,
            [channels.x, channels.y, channels.z],
        );

        for info in rx {
            for (channel, rate) in values.update([info.x(), info.y(), info.z()]) {
                for_pca9685.send((channel, rate)).unwrap();
            }
        }
    });

    Ok(tx)
}

fn start_gyro(
    for_pca9685: mpsc::Sender<(Channel, f64)>,
    scale: GyroFullScale,
    channels: ServeGyro,
    solt: f64,
    filter: f64,
) -> IOResult<mpsc::Sender<GyroInfo>> {
    let (tx, rx) = mpsc::channel::<GyroInfo>();

    std::thread::spawn(move || {
        let mut values = ValueXYZ::new(
            solt * scale.max() as f64,
            filter,
            [channels.x, channels.y, channels.z],
        );
        values.init(90.0);

        for info in rx {
            for (channel, rate) in values.update([info.x(), info.y(), info.z()]) {
                for_pca9685.send((channel, rate)).unwrap();
            }
        }
    });

    Ok(tx)
}

struct ValueXYZ {
    scale: f64,
    filter: f64,
    current_values: [(Channel, f64); 3],
}

impl ValueXYZ {
    pub fn new(scale: f64, filter: f64, channels: [Channel; 3]) -> Self {
        Self {
            scale,
            filter,
            current_values: [(channels[0], 0.0), (channels[1], 0.0), (channels[2], 0.0)],
        }
    }

    pub fn init(&mut self, angle: f64) -> Vec<(Channel, f64)> {
        let mut vs = vec![];
        for i in 0..self.current_values.len() {
            let (channel, _) = self.current_values[i];
            self.current_values[i] = (channel, angle);
            let rate = SG90_180::calc_angle_rate(angle);
            vs.push((channel, rate));
        }
        vs
    }

    pub fn update(&mut self, values: [f32; 3]) -> Vec<(Channel, f64)> {
        let values_f64: Vec<_> = values.iter().map(|v| *v as f64).collect();
        let cloned = self.current_values;
        let vs: Vec<_> = cloned.iter().zip(values_f64.clone()).collect();

        let mut sorted: Vec<_> = values_f64.iter().map(|v| v.abs()).collect();
        sorted.sort_by(|a, b| a.partial_cmp(&b).unwrap());
        let under = sorted.first().unwrap() / self.filter;

        let mut filtered: Vec<(Channel, f64)> = vec![];
        for (i, (&(channel, current), v)) in vs.iter().enumerate() {
            if 0.0 < *v && under <= *v {
                let mut next = current + *v / self.scale;
                if next < 0.0 {
                    next = 0.0;
                }
                if next > 180.0 {
                    next = 180.0;
                }
                let rate = SG90_180::calc_angle_rate(next);
                filtered.push((channel, rate));
                self.current_values[i] = (channel, next);
            }
        }
        filtered
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
