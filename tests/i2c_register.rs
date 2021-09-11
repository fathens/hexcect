use hexcect::hardware::i2c::register_io::*;

use core::fmt::Debug;
use derive_more::{From, Into};
use embedded_hal::blocking::i2c::{SevenBitAddress, Write, WriteRead};
use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;

type I2cAddr = SevenBitAddress;

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

    assert_eq!(safe_i2c.0.borrow().written.len(), 1);
    assert_eq!(safe_i2c.0.borrow().written[&8], vec![0x01, 0x12, 0x23]);
}

#[test]
fn write_registers() {
    let mock = MockI2c::default();
    let safe_i2c = ClonableI2c::new(mock);

    let mut i2c = I2cWithAddr::new(safe_i2c.clone(), 8.into());

    i2c.write_register(MockRegisterA(0x17)).unwrap();
    i2c.write_register(MockRegisterB(0x78)).unwrap();
    i2c.write_register(MockRegisterC(0x31)).unwrap();

    assert_eq!(safe_i2c.0.borrow().written.len(), 1);
    assert_eq!(
        safe_i2c.0.borrow().written[&8],
        vec![0x01, 0x17, 0x12, 0x78, 0x23, 0x31]
    );
}

#[test]
fn read_bytes() {
    let mut mock = MockI2c::default();
    mock.prepare_data(7, &[0x41, 0x32, 0x23, 0x14]);
    let safe_i2c = ClonableI2c::new(mock);

    let mut i2c = I2cWithAddr::new(safe_i2c.clone(), 7.into());

    let mut buf = [0; 4];
    i2c.read_bytes(RegAddr(0xAC), &mut buf).unwrap();

    assert_eq!(buf, [0x41, 0x32, 0x23, 0x14]);
    assert_eq!(safe_i2c.0.borrow().written.len(), 1);
    assert_eq!(safe_i2c.0.borrow().written[&7], vec![0xAC]);
}

// ----------------------------------------------------------------

#[derive(Default)]
struct MockI2c {
    pub reading: HashMap<I2cAddr, VecDeque<u8>>,
    pub written: HashMap<I2cAddr, VecDeque<u8>>,
}

impl MockI2c {
    pub fn prepare_data(&mut self, address: I2cAddr, data: &[u8]) {
        let reading = self.reading.entry(address).or_insert_with(VecDeque::new);
        for b in data {
            reading.push_back(*b);
        }
    }
}

impl Write for MockI2c {
    type Error = std::io::Error;

    fn write(&mut self, address: I2cAddr, bytes: &[u8]) -> Result<(), Self::Error> {
        let vec = self.written.entry(address).or_insert_with(VecDeque::new);
        for b in bytes {
            vec.push_back(*b);
        }
        Ok(())
    }
}

impl WriteRead for MockI2c {
    type Error = std::io::Error;

    fn write_read(
        &mut self,
        address: I2cAddr,
        bytes: &[u8],
        buf: &mut [u8],
    ) -> Result<(), Self::Error> {
        let writing = self.written.entry(address).or_insert_with(VecDeque::new);
        for b in bytes {
            writing.push_back(*b);
        }
        let reading = self.reading.entry(address).or_insert_with(VecDeque::new);
        for i in 0..buf.len() {
            if let Some(b) = reading.pop_front() {
                buf[i] = b;
            }
        }
        Ok(())
    }
}

// ----------------------------------------------------------------

#[derive(Debug, From, Into, Clone, Copy, PartialEq, Eq)]
struct MockRegisterA(u8);

impl Register for MockRegisterA {
    const ADDR: RegAddr = RegAddr(0x01);
}

#[derive(Debug, From, Into, Clone, Copy, PartialEq, Eq)]
struct MockRegisterB(u8);

impl Register for MockRegisterB {
    const ADDR: RegAddr = RegAddr(0x12);
}
#[derive(Debug, From, Into, Clone, Copy, PartialEq, Eq)]
struct MockRegisterC(u8);

impl Register for MockRegisterC {
    const ADDR: RegAddr = RegAddr(0x23);
}

// ----------------------------------------------------------------

struct ClonableI2c<T>(Rc<RefCell<T>>);

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
