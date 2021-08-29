use linux_embedded_hal::I2cdev;
use pwm_pca9685::{Address, Pca9685};
use std::io;

pub fn init_pca9685(bus: u8, addr: u8) -> Result<Pca9685<I2cdev>, io::Error> {
    let path = format!("/dev/i2c-{}", bus);
    let dev = I2cdev::new(&path)?;
    let address = Address::from(addr);
    Pca9685::new(dev, address).map_err(|err| {
        io::Error::new(
            io::ErrorKind::NotConnected,
            format!("{}({:?}): {:?}", path, address, err),
        )
    })
}
