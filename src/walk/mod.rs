use crate::hardware::i2c::servo::ServoMotor;
use crate::hardware::i2c::pca9685::PCA9685;
use pwm_pca9685::Channel;

pub fn pos_a() {
    match PCA9685::new(1, 0x40) {
        Ok(mut pwm) => {
            println!("Get PCA9685");
            pwm.enable().unwrap();
            let servos = [
                ServoMotor::new(Channel::C15, 50.0, 0.5, 2.4),
                ServoMotor::new(Channel::C12, 50.0, 0.5, 2.4),
            ];
            for servo in &servos {
                servo.set_by_rate(&mut pwm, 0.0).unwrap();
            }
            for i in 0..=10 {
                let r = i as f64 / 10.0;
                pwm.set_duty_cycle(&[
                    (&servos[0], servos[0].calc_pulse(r)),
                    (&servos[1], servos[1].calc_pulse(1.0 - r)),
                ]).unwrap();
                std::thread::sleep(std::time::Duration::from_millis(500));
            }
            println!("OK")
        }
        Err(err) => println!("Error: {:?}", err),
    }
}
