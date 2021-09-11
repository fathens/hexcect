use hexcect::hardware::i2c::register_io::*;

use core::fmt::Debug;
use derive_more::{From, Into};
use embedded_hal::blocking::i2c::{SevenBitAddress, Write, WriteRead};
use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;

type I2cAddr = SevenBitAddress;

#[derive(Default)]
pub struct MockI2c {
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
pub struct MockRegisterA(pub u8);

impl Register for MockRegisterA {
    const ADDR: RegAddr = RegAddr(0x01);
}

#[derive(Debug, From, Into, Clone, Copy, PartialEq, Eq)]
pub struct MockRegisterB(pub u8);

impl Register for MockRegisterB {
    const ADDR: RegAddr = RegAddr(0x12);
}
#[derive(Debug, From, Into, Clone, Copy, PartialEq, Eq)]
pub struct MockRegisterC(pub u8);

impl Register for MockRegisterC {
    const ADDR: RegAddr = RegAddr(0x23);
}

// ----------------------------------------------------------------

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
