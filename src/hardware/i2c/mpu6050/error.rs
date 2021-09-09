use core::fmt::Formatter;
use embedded_hal::blocking::i2c::{Write, WriteRead};

pub enum Error<T>
where
    T: Write + WriteRead,
    <T as Write>::Error: core::fmt::Debug,
    <T as WriteRead>::Error: core::fmt::Debug,
{
    WriteError(<T as Write>::Error),
    WriteReadError(<T as WriteRead>::Error),
}

impl<I2c> core::fmt::Debug for Error<I2c>
where
    I2c: WriteRead + Write,
    <I2c as WriteRead>::Error: core::fmt::Debug,
    <I2c as Write>::Error: core::fmt::Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::result::Result<(), core::fmt::Error> {
        match self {
            Error::WriteReadError(e) => f.debug_tuple("WriteReadError").field(e).finish(),
            Error::WriteError(e) => f.debug_tuple("WriteError").field(e).finish(),
        }
    }
}
