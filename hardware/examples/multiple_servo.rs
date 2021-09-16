use hardware::i2c;
use hardware::i2c::pca9685::PCA9685;
use hardware::i2c::servo::SG90_180;
use pwm_pca9685::Channel;
use std::num::ParseIntError;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Args {
    #[structopt(long, default_value = "40", parse(try_from_str = parse_hex))]
    address: u8,

    #[structopt(long, default_value = "0", parse(try_from_str = parse_channel))]
    servo_a: Channel,

    #[structopt(long, default_value = "1", parse(try_from_str = parse_channel))]
    servo_b: Channel,
}

fn main() {
    let args = Args::from_args();
    println!(
        "Using PCA9685({:x}), servo: {:?}, {:?}",
        args.address, args.servo_a, args.servo_b
    );
    let dev = i2c::connect(1).unwrap();
    match PCA9685::new(dev, args.address) {
        Ok(mut pwm) => {
            println!("Get PCA9685");
            pwm.enable().unwrap();
            let servos = [
                SG90_180::new(args.servo_a, 50.0, 0.5, 2.4),
                SG90_180::new(args.servo_b, 50.0, 0.5, 2.4),
            ];
            for servo in &servos {
                servo.set_angle(&mut pwm, 0.0).unwrap();
            }
            for i in (0..=180).step_by(10) {
                let angles = [i, 180 - i];
                let cycles: Vec<_> = servos
                    .iter()
                    .zip(angles)
                    .map(|(servo, v)| (servo, servo.calc_pulse_by_angle(v as f64)))
                    .collect();
                pwm.set_duty_cycle(&cycles).unwrap();
                std::thread::sleep(std::time::Duration::from_millis(500));
            }
            println!("OK")
        }
        Err(err) => println!("Error: {:?}", err),
    }
}

// ----------------------------------------------------------------

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

fn from_u8<T, F>(src: &str, f: F) -> Result<T, String>
where
    F: Fn(u8) -> Result<T, String>,
{
    u8::from_str_radix(src, 10)
        .map_err(|e| e.to_string())
        .and_then(f)
}

fn parse_hex(src: &str) -> Result<u8, ParseIntError> {
    u8::from_str_radix(src, 16)
}
