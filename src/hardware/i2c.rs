pub mod mpu6050;
pub mod pca9685;
pub mod servo;

use linux_embedded_hal::I2cdev;
use i2cdev::linux::LinuxI2CError;

pub fn connect(bus: u8) -> Result<I2cdev, LinuxI2CError> {
    let path = format!("/dev/i2c-{}", bus);
    I2cdev::new(&path)
}
