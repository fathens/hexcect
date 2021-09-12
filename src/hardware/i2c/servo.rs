use super::pca9685::{collect_frequency, HasChannel, HasPrescale, PwmError, PCA9685};
use embedded_hal::blocking::i2c::{Write, WriteRead};
use i2cdev::linux::LinuxI2CError;
use pwm_pca9685::Channel;

pub struct ServoMotor {
    channel: Channel,
    frequency: f64,
    prescale: u8,
    min_width: f64,
    max_width: f64,
}

impl ServoMotor {
    pub fn new(channel: Channel, frequency: f64, min_width: f64, max_width: f64) -> ServoMotor {
        let (frequency, prescale) = collect_frequency(frequency);
        ServoMotor {
            channel,
            frequency,
            prescale,
            min_width,
            max_width,
        }
    }

    pub fn calc_pulse(&self, v: f64) -> f64 {
        let unit = 1000.0 / self.frequency;
        let pulse = self.min_width + (self.max_width - self.min_width) * v;
        pulse / unit
    }

    pub fn set_by_rate<D>(&self, pwm: &mut PCA9685<D>, v: f64) -> Result<(), PwmError>
    where
        D: Write<Error = LinuxI2CError> + WriteRead<Error = LinuxI2CError>,
    {
        let rate = self.calc_pulse(v);
        pwm.set_prescale(self.prescale)?;
        pwm.set_one_duty_cycle(self.channel, rate)
    }
}

impl HasPrescale for ServoMotor {
    fn prescale(&self) -> u8 {
        self.prescale
    }
}

impl HasChannel for ServoMotor {
    fn channel(&self) -> Channel {
        self.channel
    }
}

pub struct SG90_180 {
    servo: ServoMotor,
}

impl SG90_180 {
    pub fn new(channel: Channel, frequency: f64, min_width: f64, max_width: f64) -> SG90_180 {
        let servo = ServoMotor::new(channel, frequency, min_width, max_width);
        SG90_180 { servo }
    }

    fn calc_angle_rate(angle: f64) -> f64 {
        angle.max(0.0).min(180.0) / 180.0
    }

    pub fn calc_pulse_by_angle(&self, angle: f64) -> f64 {
        let v = SG90_180::calc_angle_rate(angle);
        self.servo.calc_pulse(v)
    }

    pub fn set_angle<D>(&self, pwm: &mut PCA9685<D>, angle: f64) -> Result<(), PwmError>
    where
        D: Write<Error = LinuxI2CError> + WriteRead<Error = LinuxI2CError>,
    {
        let v = SG90_180::calc_angle_rate(angle);
        self.servo.set_by_rate(pwm, v)
    }
}

impl HasPrescale for SG90_180 {
    fn prescale(&self) -> u8 {
        self.servo.prescale()
    }
}

impl HasChannel for SG90_180 {
    fn channel(&self) -> Channel {
        self.servo.channel()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calc_pulse_min() {
        let servo = ServoMotor::new(Channel::C0, 50.0, 0.5, 2.4);
        let pulse = servo.calc_pulse(0.0);
        assert_eq!((pulse * 1000.0).round(), 0.5 * 50.0);
    }

    #[test]
    fn calc_pulse_max() {
        let servo = ServoMotor::new(Channel::C0, 50.0, 0.5, 2.4);
        let pulse = servo.calc_pulse(1.0);
        assert_eq!((pulse * 1000.0).round(), 2.4 * 50.0);
    }

    #[test]
    fn calc_pulse_half() {
        let servo = ServoMotor::new(Channel::C0, 50.0, 0.5, 2.4);
        let pulse = servo.calc_pulse(0.5);
        assert_eq!(
            (pulse * 1000.0 * 10.0).round(),
            (0.5 + 2.4) / 2.0 * 50.0 * 10.0
        );
    }

    #[test]
    fn calc_angle_rate_half() {
        let angle = 90.0;
        let rate = SG90_180::calc_angle_rate(angle);
        assert_eq!(rate, 0.5);
    }

    #[test]
    fn calc_angle_rate_all() {
        for angle in 0..=180 {
            let rate = SG90_180::calc_angle_rate(angle as f64);
            let revert = rate * 180.0;
            assert_eq!(angle, revert.round() as u8);
        }
    }

    #[test]
    fn calc_angle_rate_under() {
        for angle in 0..100 {
            let rate = SG90_180::calc_angle_rate(-angle as f64);
            assert_eq!(rate, 0.0);
        }
    }

    #[test]
    fn calc_angle_rate_over() {
        for angle in 180..200 {
            let rate = SG90_180::calc_angle_rate(angle as f64);
            assert_eq!(rate, 1.0);
        }
    }
}
