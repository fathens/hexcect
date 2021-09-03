use crate::util::group_by;
use i2cdev::linux::LinuxI2CError;
use linux_embedded_hal::I2cdev;
use pwm_pca9685::{Address, Channel, Error, Pca9685};

const OSC: f64 = 25000000.0;
const PULSE_BASE: f64 = 4096.0;
pub type PwmError = Error<LinuxI2CError>;

pub trait HasPrescale {
    fn prescale(&self) -> u8;
}

pub trait HasChannel {
    fn channel(&self) -> Channel;
}

pub struct PCA9685 {
    pub inner: Pca9685<I2cdev>,
    prescale: Option<u8>,
}

impl PCA9685 {
    pub fn collect_frequency(v: f64) -> (f64, u8) {
        let prescale = {
            let v = OSC / (PULSE_BASE * v) - 1.0;
            v.max(3.0).min(255.0)
        };
        let frequency = OSC / (PULSE_BASE * (prescale + 1.0));
        (frequency, prescale as u8)
    }

    pub fn calc_pulse(rate: f64) -> u16 {
        (PULSE_BASE * rate) as u16
    }

    pub fn new(bus: u8, addr: u8) -> Result<PCA9685, PwmError> {
        let path = format!("/dev/i2c-{}", bus);
        let dev = I2cdev::new(&path).map_err(Error::I2C)?;
        let address = Address::from(addr);
        Pca9685::new(dev, address).map(|inner| PCA9685 {
            inner,
            prescale: None,
        })
    }

    pub fn enable(&mut self) -> Result<(), PwmError> {
        self.inner.enable()
    }

    pub fn set_prescale(&mut self, v: u8) -> Result<(), PwmError> {
        match self.prescale {
            Some(prev) if prev == v => Ok(()),
            _ => {
                self.prescale = Some(v);
                self.inner.set_prescale(v)
            }
        }
    }

    pub fn set_one_duty_cycle(&mut self, channel: Channel, rate: f64) -> Result<(), PwmError> {
        let v = PCA9685::calc_pulse(rate);
        self.inner.set_channel_on_off(channel, 0, v)
    }

    pub fn set_duty_cycle<T>(&mut self, rates: &[(&T, f64)]) -> Result<(), PwmError>
    where
        T: HasChannel + HasPrescale,
    {
        let by_prescale = group_by(rates, |e| e.prescale());
        by_prescale.iter().fold(Ok(()), |prev, (ps, values)| {
            prev?;
            let preset = self.set_prescale(*ps);
            values.iter().fold(preset, |prev, (t, rate)| {
                prev?;
                self.set_one_duty_cycle(t.channel(), *rate)
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collect_frequency_50() {
        let (f, p) = PCA9685::collect_frequency(50.0);
        assert_eq!(f.round(), 50.0);
        assert_eq!(p, 121);
    }

    #[test]
    fn collect_frequency_under() {
        for freq in 0..239 {
            let (f, p) = PCA9685::collect_frequency(freq as f64 / 10.0);
            assert_eq!((f * 10.0).round(), 238.0);
            assert_eq!(p, 255);
        }
    }

    #[test]
    fn collect_frequency_upper() {
        for freq in 15259..20000 {
            let (f, p) = PCA9685::collect_frequency(freq as f64 / 10.0);
            assert_eq!((f * 10.0).round(), 15259.0);
            assert_eq!(p, 3);
        }
    }

    #[test]
    fn calc_pulse_min() {
        let pulse = PCA9685::calc_pulse(0.0);
        assert_eq!(pulse, 0);
    }

    #[test]
    fn calc_pulse_max() {
        let pulse = PCA9685::calc_pulse(1.0);
        assert_eq!(pulse, PULSE_BASE as u16);
    }
}
