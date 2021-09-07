use super::raw_data::*;
use num_traits::FromPrimitive;

pub struct RegAddr(u8);

impl RegAddr {
    pub fn as_u8(&self) -> u8 {
        self.0
    }
}

pub trait Register: From<u8> {
    const ADDR: RegAddr;

    fn as_u8(&self) -> u8;
}

pub struct Configure {
    pub ext_sync_set: FrameSync,
    pub dlpf_cfg: DigitalLowPassFilterCfg,
}

impl Configure {
    const ADDR: RegAddr = RegAddr(0x1a);

    fn as_u8(&self) -> u8 {
        let a = (self.ext_sync_set as u8) << 3;
        let b = self.dlpf_cfg as u8;
        a + b
    }
}

impl From<u8> for Configure {
    fn from(v: u8) -> Self {
        Self {
            ext_sync_set: FrameSync::from_u8((v >> 3) & 7).unwrap(),
            dlpf_cfg: DigitalLowPassFilterCfg::from_u8(v & 7).unwrap(),
        }
    }
}

pub struct GyroConfig {
    pub st_xyz: FlagsXYZ,
    pub fs_sel: GyroFullScale,
}

impl Register for GyroConfig {
    const ADDR: RegAddr = RegAddr(0x1b);

    fn as_u8(&self) -> u8 {
        let a = self.st_xyz.as_u8() << 5;
        let b = (self.fs_sel as u8) << 3;
        a + b
    }
}

impl From<u8> for GyroConfig {
    fn from(v: u8) -> Self {
        Self {
            st_xyz: FlagsXYZ((v >> 5) & 7),
            fs_sel: GyroFullScale::from_u8((v >> 3) & 2).unwrap(),
        }
    }
}

pub struct AccelConfig {
    pub st_xyz: FlagsXYZ,
    pub afs_sel: AccelFullScale,
}

impl Register for AccelConfig {
    const ADDR: RegAddr = RegAddr(0x1c);

    fn as_u8(&self) -> u8 {
        let a = self.st_xyz.as_u8() << 5;
        let b = (self.afs_sel as u8) << 3;
        a + b
    }
}

impl From<u8> for AccelConfig {
    fn from(v: u8) -> Self {
        Self {
            st_xyz: FlagsXYZ((v >> 5) & 7),
            afs_sel: AccelFullScale::from_u8((v >> 3) & 2).unwrap(),
        }
    }
}

#[derive(Clone, Copy)]
pub struct FlagsXYZ(u8);

impl FlagsXYZ {
    pub fn new(x: bool, y: bool, z: bool) -> Self {
        Self(u8::from_bits(&[z, y, x]))
    }

    pub fn x(&self) -> bool {
        self.0.at(2)
    }

    pub fn y(&self) -> bool {
        self.0.at(1)
    }

    pub fn z(&self) -> bool {
        self.0.at(0)
    }

    pub fn as_u8(&self) -> u8 {
        self.0
    }
}

trait SingleByte {
    fn from_bits(bs: &[bool]) -> u8 {
        assert!(bs.len() <= 8);
        bs.iter()
            .enumerate()
            .map(|(i, b)| if *b { 1 << i } else { 0 })
            .sum()
    }

    fn at(&self, i: u8) -> bool {
        self.value() & (1 << i) == 1
    }

    fn value(&self) -> u8;
}

impl SingleByte for u8 {
    fn value(&self) -> u8 {
        *self
    }
}

pub trait RegisterRange {
    const ADDR: RegAddr;
}

impl RegisterRange for AccelData {
    const ADDR: RegAddr = RegAddr(0x3b);
}

impl From<[u8; 6]> for AccelData {
    fn from(data: [u8; 6]) -> Self {
        let (x, y, z) = take2x3(data);
        Self { x, y, z }
    }
}

impl RegisterRange for Temperature {
    const ADDR: RegAddr = RegAddr(0x41);
}

impl From<[u8; 2]> for Temperature {
    fn from(data: [u8; 2]) -> Self {
        Self(i16::from_be_bytes(data))
    }
}

impl RegisterRange for GyroData {
    const ADDR: RegAddr = RegAddr(0x43);
}

impl From<[u8; 6]> for GyroData {
    fn from(data: [u8; 6]) -> Self {
        let (x, y, z) = take2x3(data);
        Self { x, y, z }
    }
}

fn take2x3(data: [u8; 6]) -> (i16, i16, i16) {
    (
        i16::from_be_bytes([data[0], data[1]]),
        i16::from_be_bytes([data[2], data[3]]),
        i16::from_be_bytes([data[4], data[5]]),
    )
}
