use hexcect::hardware::i2c::register_io::*;

use core::fmt::Debug;
use derive_more::{From, Into};
use embedded_hal::blocking::i2c::{SevenBitAddress, Write, WriteRead};
use std::collections::{HashMap, VecDeque};

type I2cAddr = SevenBitAddress;

#[derive(Default)]
pub struct MockI2c {
    pub reading: HashMap<I2cAddr, VecDeque<u8>>,
    pub written: HashMap<I2cAddr, VecDeque<u8>>,
    current_addr: I2cAddr,
}

impl MockI2c {
    pub fn prepare_data(&mut self, address: I2cAddr, data: &[u8]) {
        let reading = self.reading.entry(address).or_insert_with(VecDeque::new);
        for b in data {
            reading.push_back(*b);
        }
    }

    fn write_current(&mut self, bytes: &[u8]) -> Result<(), std::io::Error> {
        let vec = self.written.entry(self.current_addr).or_insert_with(VecDeque::new);
        for b in bytes {
            vec.push_back(*b);
        }
        Ok(())
    }

    fn read_current(&mut self, buf: &mut [u8]) -> Result<(), std::io::Error> {
        let vec = self.reading.entry(self.current_addr).or_insert_with(VecDeque::new);
        for i in 0..buf.len() {
            if let Some(b) = vec.pop_front() {
                buf[i] = b;
            }
        }
        Ok(())
    }
}

impl Write for MockI2c {
    type Error = std::io::Error;

    fn write(&mut self, address: I2cAddr, bytes: &[u8]) -> Result<(), Self::Error> {
        self.current_addr = address;
        self.write_current(bytes)
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
        self.current_addr = address;
        self.write_current(bytes)?;
        self.read_current(buf)
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
