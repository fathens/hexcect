use embedded_hal::blocking::i2c::{AddressMode, Write, WriteRead};
use parking_lot::{Mutex, MutexGuard};
use std::sync::Arc;

pub struct ThreadSafeI2c<T>(Arc<Mutex<T>>);

impl<T> ThreadSafeI2c<T> {
    pub fn new(t: T) -> Self {
        Self(Arc::new(Mutex::new(t)))
    }

    pub fn lock(&self) -> MutexGuard<'_, T> {
        self.0.lock()
    }
}

impl<T> Clone for ThreadSafeI2c<T> {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

impl<A, T> Write<A> for ThreadSafeI2c<T>
where
    A: AddressMode,
    T: Write<A>,
{
    type Error = <T as Write<A>>::Error;

    fn write(&mut self, address: A, bytes: &[u8]) -> Result<(), Self::Error> {
        self.0.lock().write(address, bytes)
    }
}

impl<A, T> WriteRead<A> for ThreadSafeI2c<T>
where
    A: AddressMode,
    T: WriteRead<A>,
{
    type Error = <T as WriteRead<A>>::Error;

    fn write_read(&mut self, address: A, bytes: &[u8], buf: &mut [u8]) -> Result<(), Self::Error> {
        self.0.lock().write_read(address, bytes, buf)
    }
}
