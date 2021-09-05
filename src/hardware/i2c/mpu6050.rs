use i2cdev::linux::LinuxI2CError;
use linux_embedded_hal::{Delay, I2cdev};
use mpu6050::*;

type MpuError = Mpu6050Error<LinuxI2CError>;

pub struct MPU6050 {
    pub inner: Mpu6050<I2cdev>,
}

#[derive(Debug)]
pub struct MpuInfo {
    temp: f32,
    gyro: String,
    acc: String,
    rp: String,
}

impl MPU6050 {
    pub fn new(dev: I2cdev) -> Result<MPU6050, MpuError> {
        let mut inner = Mpu6050::new(dev);
        let mut delay = Delay;
        inner.init(&mut delay)?;
        Ok(MPU6050 { inner })
    }

    pub fn get_infos(&mut self) -> Result<MpuInfo, MpuError> {
        // get roll and pitch estimate
        let rp = self.inner.get_acc_angles()?.to_string();
        // get temp
        let temp = self.inner.get_temp()?;
        // get gyro data, scaled with sensitivity
        let gyro = self.inner.get_gyro()?.to_string();
        // get accelerometer data, scaled with sensitivity
        let acc = self.inner.get_acc()?.to_string();

        let info = MpuInfo {
            temp,
            rp,
            gyro,
            acc,
        };
        Ok(info)
    }
}
