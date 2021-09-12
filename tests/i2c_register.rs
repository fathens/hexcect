use hexcect::hardware::i2c::*;

mod i2c_mock;
use i2c_mock::*;

#[test]
fn read_registers() {
    let mut mock = MockI2c::default();
    mock.prepare_data(8, &[0x17, 0x78, 0x31]);
    let safe_i2c = ClonableI2c::new(mock);

    let mut i2c = I2cWithAddr::new(safe_i2c.clone(), 8.into());

    let reg_a: MockRegisterA = i2c.read_register().unwrap();
    let reg_b: MockRegisterB = i2c.read_register().unwrap();
    let reg_c: MockRegisterC = i2c.read_register().unwrap();

    assert_eq!(0x17_u8, reg_a.into());
    assert_eq!(0x78_u8, reg_b.into());
    assert_eq!(0x31_u8, reg_c.into());

    let written = safe_i2c.0.borrow().written.clone();
    assert_eq!(written.len(), 1);
    assert_eq!(written[&8], vec![0x01, 0x12, 0x23]);
}

#[test]
fn write_registers() {
    let mock = MockI2c::default();
    let safe_i2c = ClonableI2c::new(mock);

    let mut i2c = I2cWithAddr::new(safe_i2c.clone(), 8.into());

    i2c.write_register(MockRegisterA(0x17)).unwrap();
    i2c.write_register(MockRegisterB(0x78)).unwrap();
    i2c.write_register(MockRegisterC(0x31)).unwrap();

    let written = safe_i2c.0.borrow().written.clone();
    assert_eq!(written.len(), 1);
    assert_eq!(written[&8], vec![0x01, 0x17, 0x12, 0x78, 0x23, 0x31]);
}

#[test]
fn read_bytes() {
    let mut mock = MockI2c::default();
    mock.prepare_data(7, &[0x41, 0x32, 0x23, 0x14]);
    let safe_i2c = ClonableI2c::new(mock);

    let mut i2c = I2cWithAddr::new(safe_i2c.clone(), 7.into());

    let mut buf = [0; 4];
    i2c.read_bytes(RegAddr(0xAC), &mut buf).unwrap();

    let written = safe_i2c.0.borrow().written.clone();
    assert_eq!(buf, [0x41, 0x32, 0x23, 0x14]);
    assert_eq!(written.len(), 1);
    assert_eq!(written[&7], vec![0xAC]);
}

// ----------------------------------------------------------------
use embedded_hal::blocking::i2c::{SevenBitAddress, Write, WriteRead};
use std::cell::RefCell;
use std::rc::Rc;
type I2cAddr = SevenBitAddress;

pub struct ClonableI2c<T>(pub Rc<RefCell<T>>);

impl<T> ClonableI2c<T> {
    pub fn new(t: T) -> Self {
        Self(Rc::new(RefCell::new(t)))
    }
}

impl<T> Clone for ClonableI2c<T> {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}

impl<T: Write> Write for ClonableI2c<T> {
    type Error = <T as Write>::Error;

    fn write(&mut self, address: I2cAddr, bytes: &[u8]) -> Result<(), Self::Error> {
        self.0.borrow_mut().write(address, bytes)
    }
}

impl<T: WriteRead> WriteRead for ClonableI2c<T> {
    type Error = <T as WriteRead>::Error;

    fn write_read(
        &mut self,
        address: I2cAddr,
        bytes: &[u8],
        buf: &mut [u8],
    ) -> Result<(), Self::Error> {
        self.0.borrow_mut().write_read(address, bytes, buf)
    }
}
