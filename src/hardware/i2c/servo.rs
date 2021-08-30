use super::pca9685::{ChannelIndex, HasPrescale, PwmError, PCA9685};
use pwm_pca9685::Channel;

impl HasPrescale for ServoMotor {
    fn prescale(&self) -> u8 {
        self.prescale
    }
}

impl ChannelIndex for ServoMotor {
    fn index(&self) -> Option<u8> {
        self.channel.index()
    }
}

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
