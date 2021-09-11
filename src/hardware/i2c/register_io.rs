use core::fmt::Debug;
use core::fmt::Formatter;
use derive_more::{From, Into};
use embedded_hal::blocking::i2c::{Write, WriteRead};

#[derive(Debug, From, Into, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RegAddr(pub u8);

#[derive(Debug, From, Into, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Address(pub u8);

pub trait Register: From<u8> + Debug + Copy + Eq {
    const ADDR: RegAddr;
}

pub trait RegisterIO<T>
where
    T: Write + WriteRead,
    <T as Write>::Error: Debug,
    <T as WriteRead>::Error: Debug,
{
    fn i2c_device(&mut self) -> &mut T;
    fn address(&self) -> Address;

    fn read_bytes(&mut self, reg: RegAddr, res: &mut [u8]) -> Result<(), Error<T>> {
        let addr = self.address();
        self.i2c_device()
            .write_read(addr.into(), &[reg.into()], res)
            .map_err(Error::WriteReadError)
    }

    fn read_byte(&mut self, reg: RegAddr) -> Result<u8, Error<T>> {
        let mut buf = [0; 1];
        self.read_bytes(reg, &mut buf)?;
        Ok(buf[0])
    }

    fn write_byte(&mut self, reg: RegAddr, v: u8) -> Result<(), Error<T>> {
        let addr = self.address();
        self.i2c_device()
            .write(addr.into(), &[reg.into(), v])
            .map_err(Error::WriteError)
    }

    fn read_register<R: Register>(&mut self) -> Result<R, Error<T>> {
        let byte = self.read_byte(R::ADDR)?;
        Ok(R::from(byte))
    }

    fn write_register<R: Register>(&mut self, reg_value: R) -> Result<(), Error<T>>
    where
        u8: From<R>,
    {
        self.write_byte(R::ADDR, reg_value.into())
    }
}

pub struct I2cWithAddr<T> {
    dev: T,
    address: Address,
}

impl<T> I2cWithAddr<T> {
    pub fn new(dev: T, address: Address) -> Self {
        Self { dev, address }
    }
}

impl<T> RegisterIO<T> for I2cWithAddr<T>
where
    T: Write + WriteRead,
    <T as Write>::Error: Debug,
    <T as WriteRead>::Error: Debug,
{
    fn i2c_device(&mut self) -> &mut T {
        &mut self.dev
    }

    fn address(&self) -> Address {
        self.address
    }
}

// ----------------------------------------------------------------

pub enum Error<T>
where
    T: Write + WriteRead,
    <T as Write>::Error: Debug,
    <T as WriteRead>::Error: Debug,
{
    WriteError(<T as Write>::Error),
    WriteReadError(<T as WriteRead>::Error),
}

impl<I2c> Debug for Error<I2c>
where
    I2c: WriteRead + Write,
    <I2c as WriteRead>::Error: Debug,
    <I2c as Write>::Error: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), core::fmt::Error> {
        match self {
            Error::WriteReadError(e) => f.debug_tuple("WriteReadError").field(e).finish(),
            Error::WriteError(e) => f.debug_tuple("WriteError").field(e).finish(),
        }
    }
}
