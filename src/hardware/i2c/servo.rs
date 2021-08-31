use super::pca9685::{HasChannel, HasPrescale, PwmError, PCA9685};
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
        let (frequency, prescale) = PCA9685::collect_frequency(frequency);
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

    pub fn set_by_rate(&self, pwm: &mut PCA9685, v: f64) -> Result<(), PwmError> {
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

    pub fn set_angle(&self, pwm: &mut PCA9685, angle: f64) -> Result<(), PwmError> {
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
