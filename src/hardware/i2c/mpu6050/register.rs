use super::raw_data::*;

pub trait Register {
    const ADDR: u8;
    const IS_READONLY: bool;

    fn writing_data(&self) -> u8;
}

pub struct GyroConfig {
    pub st_xyz: SelfTestXYZ,
    pub fs_sel: GyroFullScale,
}

impl Register for GyroConfig {
    const ADDR: u8 = 0x1b;
    const IS_READONLY: bool = false;

    fn writing_data(&self) -> u8 {
        let a = self.st_xyz.0 << 5;
        let b = self.fs_sel.value() << 3;
        a + b
    }
}

pub struct SelfTestXYZ(u8);

impl SelfTestXYZ {
    pub fn new(x: bool, y: bool, z: bool) -> Self {
        Self(as_bit(x, 2) + as_bit(y, 1) + as_bit(z, 0))
    }
}

#[inline]
fn as_bit(a: bool, s: u8) -> u8 {
    if a {
        1 << s
    } else {
        0
    }
}
