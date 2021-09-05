use i2cdev::linux::LinuxI2CError;
use linux_embedded_hal::{Delay, I2cdev};
use mpu6050::*;

type MpuError = Mpu6050Error<LinuxI2CError>;

pub struct MPU6050 {
    pub inner: Mpu6050<I2cdev>,
}

#[derive(Debug)]
pub struct XYZ {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Debug)]
pub struct MpuInfo {
    temp: f32,
    gyro: XYZ,
    acc: XYZ,
}

impl MPU6050 {
    pub fn new(dev: I2cdev) -> Result<MPU6050, MpuError> {
        let mut inner = Mpu6050::new(dev);
        let mut delay = Delay;
        inner.init(&mut delay)?;
        Ok(MPU6050 { inner })
    }

    pub fn get_infos(&mut self) -> Result<MpuInfo, MpuError> {
        let temp = self.inner.get_temp()?;
        let gyro_vec = self.inner.get_gyro()?;
        let acc_vec = self.inner.get_acc()?;

        let gyro = gyro_vec.as_slice();
        let acc = acc_vec.as_slice();

        let info = MpuInfo {
            temp,
            gyro: XYZ {
                x: gyro[0],
                y: gyro[1],
                z: gyro[2],
            },
            acc: XYZ {
                x: acc[0],
                y: acc[1],
                z: acc[2],
            },
        };
        Ok(info)
    }
}
