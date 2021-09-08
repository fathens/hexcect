pub mod mpu6050;
pub mod pca9685;
pub mod servo;

use i2cdev::linux::LinuxI2CError;
use lazy_static::*;
use linux_embedded_hal::I2cdev;
use parking_lot::Mutex;
use std::collections::HashMap;
use std::sync::Arc;

type Result<T> = std::result::Result<T, LinuxI2CError>;
type ThreadSafeI2c = Arc<Mutex<I2cdev>>;

lazy_static! {
    static ref I2C_DEVICES: Mutex<HashMap<u8, ThreadSafeI2c>> = Mutex::new(HashMap::new());
}

/// 指定された bus で既に作られていればそれを使い、無ければ作成する。
/// 作成した I2cdev は Arc と Mutex でラップして、返すときは常に clone して返す。
pub fn connect(bus: u8) -> Result<ThreadSafeI2c> {
    let mut devices = I2C_DEVICES.lock();
    let result = match devices.get(&bus) {
        Some(a) => a,
        None => {
            let path = format!("/dev/i2c-{}", bus);
            let dev = I2cdev::new(&path)?;
            let v = Arc::new(Mutex::new(dev));
            devices.insert(bus, v);
            devices.get(&bus).unwrap()
        }
    };
    Ok(Arc::clone(result))
}
