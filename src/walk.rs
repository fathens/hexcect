use crate::hardware::i2c;
use i2c::pca9685::PCA9685;
use i2c::servo::SG90_180;
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
