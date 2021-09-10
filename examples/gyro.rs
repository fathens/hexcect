extern crate hexcect;

use hexcect::hardware::i2c::connect;
use hexcect::hardware::i2c::mpu6050::{ADDRESS_LOW, MPU6050};
use hexcect::hardware::i2c::register_io::I2cWithAddr;
use linux_embedded_hal::Delay;
use std::io::{stdout, Result};

use crossterm::*;
use ctrlc;

fn main() {
    run_loop().unwrap();
}

fn run_loop() -> Result<()> {
    let dev = connect(1)?;
    let mut mpu = MPU6050::new(I2cWithAddr::new(dev, ADDRESS_LOW)).unwrap();
    mpu.normal_setup(&mut Delay).unwrap();

    ctrlc::set_handler(|| {
        execute!(
            stdout(),
            terminal::Clear(terminal::ClearType::All),
            cursor::Show,
            cursor::MoveTo(0, 0),
        )
        .unwrap();
        std::process::exit(0);
    })
    .unwrap();

    let (cols, _rows) = terminal::size()?;
    execute!(
        stdout(),
        cursor::Hide,
        terminal::Clear(terminal::ClearType::All),
    )?;

    let mut accel = ViewXYZ::new("Accel", cols, 0)?;
    let mut gyro = ViewXYZ::new("Gyro", cols, 5)?;
    loop {
        let info = mpu.get_infos().unwrap();
        accel.update(info.accel.x, info.accel.y, info.accel.z)?;
        gyro.update(info.gyro.x, info.gyro.y, info.gyro.z)?;
    }
}

struct ViewXYZ<'a> {
    title: &'a str,
    width: u16,
    pos: u16,
    x: HLine,
    y: HLine,
    z: HLine,
}

impl<'a> ViewXYZ<'a> {
    pub fn new(title: &'a str, width: u16, pos: u16) -> Result<Self> {
        let o = Self {
            title,
            width,
            pos,
            x: HLine::new('|', 'X', width, pos + 1),
            y: HLine::new('|', 'Y', width, pos + 2),
            z: HLine::new('|', 'Z', width, pos + 3),
        };
        o.draw_title()?;
        Ok(o)
    }

    pub fn draw_title(&self) -> Result<()> {
        let x = (self.width as usize - self.title.len()) / 2;
        execute!(
            stdout(),
            cursor::MoveTo(x as u16, self.pos),
            style::Print(self.title)
        )
    }

    pub fn update(&mut self, x: i16, y: i16, z: i16) -> Result<()> {
        self.x.update(x)?;
        self.y.update(y)?;
        self.z.update(z)?;
        Ok(())
    }
}

struct HLine {
    vc: char,
    key: char,
    width: u16,
    pos: u16,
    previous_value: Option<i16>,
}

impl HLine {
    pub fn new(vc: char, key: char, width: u16, pos: u16) -> HLine {
        Self {
            vc,
            key,
            width,
            pos,
            previous_value: None,
        }
    }

    fn calc(&self, value: i16) -> (u16, usize) {
        let center = self.width as i32 / 2;

        let x_pos = {
            let scale = self.width as f64 / (i16::MAX as f64 - i16::MIN as f64);
            (value as f64 * scale).round() as i32
        };

        let len = x_pos.abs();
        let mut start = center.min(x_pos + center);
        if start == center {
            start = center + 1;
        }

        (start as u16, len as usize)
    }

    pub fn update(&mut self, value: i16) -> Result<()> {
        let center = self.width / 2;
        let (start, len) = self.calc(value);

        match self.previous_value {
            None => {
                execute!(
                    stdout(),
                    cursor::MoveTo(center, self.pos),
                    style::Print(self.key),
                    cursor::MoveTo(start, self.pos),
                    style::Print(format!("{}", self.vc).repeat(len)),
                )?;
            }
            Some(prev_value) => {
                let (prev_start, prev_len) = self.calc(prev_value);

                let diff_start = start as isize - prev_start as isize;
                if diff_start < 0 {
                    let size = len.min(diff_start.abs() as usize);
                    execute!(
                        stdout(),
                        cursor::MoveTo(start, self.pos),
                        style::Print(format!("{}", self.vc).repeat(size)),
                    )?;
                } else if diff_start > 0 {
                    let size = prev_len.min(diff_start as usize);
                    execute!(
                        stdout(),
                        cursor::MoveTo(prev_start, self.pos),
                        style::Print(" ".repeat(size)),
                    )?;
                }

                let end = start + len as u16;
                let prev_end = prev_start + prev_len as u16;
                let diff_end = end as isize - prev_end as isize;
                if diff_end < 0 {
                    let p = end.max(center + 1);
                    let size = prev_len.min(diff_end.abs() as usize);
                    execute!(
                        stdout(),
                        cursor::MoveTo(p, self.pos),
                        style::Print(" ".repeat(size)),
                    )?;
                } else if diff_end > 0 {
                    let p = prev_end.max(center + 1);
                    let size = len.min(diff_end.abs() as usize);
                    execute!(
                        stdout(),
                        cursor::MoveTo(p, self.pos),
                        style::Print(format!("{}", self.vc).repeat(size)),
                    )?;
                }
            }
        }
        self.previous_value = Some(value);
        Ok(())
    }
}
