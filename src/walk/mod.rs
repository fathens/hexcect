use crate::hardware::i2c::servo::SG90_180;
use crate::hardware::i2c::pca9685::PCA9685;
use pwm_pca9685::Channel;

pub fn pos_a() {
    match PCA9685::new(1, 0x40) {
        Ok(mut pwm) => {
            println!("Get PCA9685");
            pwm.enable().unwrap();
            let servos = [
                SG90_180::new(Channel::C15, 50.0, 0.5, 2.4),
                SG90_180::new(Channel::C12, 50.0, 0.5, 2.4),
            ];
            for servo in &servos {
                servo.set_angle(&mut pwm, 0.0).unwrap();
            }
            for angle in (0..=180).step_by(10) {
                pwm.set_duty_cycle(&[
                    (&servos[0], servos[0].calc_pulse_by_angle(angle as f64)),
                    (&servos[1], servos[1].calc_pulse_by_angle(180.0 - angle as f64)),
                ]).unwrap();
                std::thread::sleep(std::time::Duration::from_millis(500));
            }
            println!("OK")
        }
        Err(err) => println!("Error: {:?}", err),
    }
}
