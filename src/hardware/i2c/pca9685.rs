use i2cdev::linux::LinuxI2CError;
use linux_embedded_hal::I2cdev;
use pwm_pca9685::{Address, Channel, Error, Pca9685};

const OSC: f64 = 25000000.0;
const PULSE_BASE: f64 = 4096.0;
pub type PwmError = Error<LinuxI2CError>;

pub trait ChannelIndex {
    fn index(&self) -> Option<u8>;
}

impl ChannelIndex for Channel {
    fn index(&self) -> Option<u8> {
        match self {
            Channel::All => None,
            Channel::C0 => Some(0),
            Channel::C1 => Some(1),
            Channel::C2 => Some(2),
            Channel::C3 => Some(3),
            Channel::C4 => Some(4),
            Channel::C5 => Some(5),
            Channel::C6 => Some(6),
            Channel::C7 => Some(7),
            Channel::C8 => Some(8),
            Channel::C9 => Some(9),
            Channel::C10 => Some(10),
            Channel::C11 => Some(11),
            Channel::C12 => Some(12),
            Channel::C13 => Some(13),
            Channel::C14 => Some(14),
            Channel::C15 => Some(15),
        }
    }
}

pub struct PCA9685 {
    pub inner: Pca9685<I2cdev>,
    prescale: Option<u8>,
}

impl PCA9685 {
    pub fn collect_frequency(v: f64) -> (f64, u8) {
        let prescale = {
            let v = OSC / (PULSE_BASE * v) - 1.0;
            v.max(3.0).min(255.0) as u8
        };
        let frequency = OSC / (PULSE_BASE * (prescale as f64 + 1.0));
        (frequency, prescale)
    }

    pub fn new(bus: u8, addr: u8) -> Result<PCA9685, PwmError> {
        let path = format!("/dev/i2c-{}", bus);
        let dev = I2cdev::new(&path).map_err(|err| Error::I2C(err))?;
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
        let steps = (PULSE_BASE * rate) as u16;
        self.inner.set_channel_on_off(channel, 0, steps)
    }

    pub fn set_duty_cycle<T>(&mut self, rates: &[(&T, f64)]) -> Result<(), PwmError>
    where
        T: ChannelIndex
    {
        let mut values = [0u16; 16];
        for (t, rate) in rates {
            let v = (PULSE_BASE * rate) as u16;
            match t.index() {
                Some(i) => values[i as usize] = v,
                None => for i in 0..values.len() {
                    values[i] = v
                }
            }
        }
        self.inner.set_all_on_off(&[0; 16], &values)
    }
}
