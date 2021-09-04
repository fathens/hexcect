pub mod mpu6050;
pub mod pca9685;
pub mod servo;

use i2cdev::linux::LinuxI2CError;
use linux_embedded_hal::I2cdev;

pub fn connect(bus: u8) -> Result<I2cdev, LinuxI2CError> {
    let path = format!("/dev/i2c-{}", bus);
    I2cdev::new(&path)
}
