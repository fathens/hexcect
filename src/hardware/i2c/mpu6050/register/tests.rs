use super::*;
use embedded_time::{duration::*, rate::*};


#[test]
fn singlebyte_get_with_mask() {
    assert_eq!(0b_1010_0110.get_with_mask(0b111, 0), 0b110);
    assert_eq!(0b_1010_0110.get_with_mask(0b111, 1), 0b011);
    assert_eq!(0b_1010_0110.get_with_mask(0b111, 2), 0b001);
    assert_eq!(0b_1010_0110.get_with_mask(0b111, 3), 0b100);
    assert_eq!(0b_1010_0110.get_with_mask(0b111, 4), 0b010);
    assert_eq!(0b_1010_0110.get_with_mask(0b111, 5), 0b101);
    assert_eq!(0b_1010_0110.get_with_mask(0b111, 6), 0b010);
    assert_eq!(0b_1010_0110.get_with_mask(0b111, 7), 0b001);

    assert_eq!(0b_1010_0110.get_with_mask(0b11, 0), 0b10);
    assert_eq!(0b_1010_0110.get_with_mask(0b11, 1), 0b11);
    assert_eq!(0b_1010_0110.get_with_mask(0b11, 2), 0b01);
    assert_eq!(0b_1010_0110.get_with_mask(0b11, 3), 0b00);
    assert_eq!(0b_1010_0110.get_with_mask(0b11, 4), 0b10);
    assert_eq!(0b_1010_0110.get_with_mask(0b11, 5), 0b01);
    assert_eq!(0b_1010_0110.get_with_mask(0b11, 6), 0b10);
    assert_eq!(0b_1010_0110.get_with_mask(0b11, 7), 0b01);
}

#[test]
fn singlebyte_set_with_mask() {
    assert_eq!(0b_1110_0101.set_with_mask(0b111, 0, 0b000), 0b_1110_0000);

    assert_eq!(0b_1110_0101.set_with_mask(0b111, 0, 0b010), 0b_1110_0010);
    assert_eq!(0b_1110_0101.set_with_mask(0b111, 1, 0b010), 0b_1110_0101);
    assert_eq!(0b_1110_0101.set_with_mask(0b111, 2, 0b010), 0b_1110_1001);
    assert_eq!(0b_1110_0101.set_with_mask(0b111, 3, 0b010), 0b_1101_0101);
    assert_eq!(0b_1110_0101.set_with_mask(0b111, 4, 0b010), 0b_1010_0101);
    assert_eq!(0b_1110_0101.set_with_mask(0b111, 5, 0b010), 0b_0100_0101);
    assert_eq!(0b_1110_0101.set_with_mask(0b111, 6, 0b010), 0b_1010_0101);
    assert_eq!(0b_1110_0101.set_with_mask(0b111, 7, 0b010), 0b_0110_0101);

    assert_eq!(0b_1110_0101.set_with_mask(0b111, 1, 0b111), 0b_1110_1111);
    assert_eq!(0b_1110_0101.set_with_mask(0b111, 2, 0b110), 0b_1111_1001);
    assert_eq!(0b_1110_0101.set_with_mask(0b111, 3, 0b110), 0b_1111_0101);
}

#[test]
fn configure_from_u8() {
    for a in 0..8 {
        for b in 0..8 {
            let ext_sync_set = FrameSync::from_u8(a).expect("Must be !");
            let dlpf_cfg = DigitalLowPassFilterCfg::from_u8(b).expect("Must be !");

            let v = (ext_sync_set as u8) * 8 + (dlpf_cfg as u8);
            let config = Configure::from(v);

            assert_eq!(v, config.into());
            assert_eq!(config.get_fsync(), ext_sync_set);
            assert_eq!(config.get_dlpf(), dlpf_cfg);

            let mut config = config;

            let ext_sync_set = FrameSync::from_u8((a + 1) % 8).expect("Must be !");
            config.set_fsync(ext_sync_set);
            assert_eq!(config.get_fsync(), ext_sync_set);
            assert_eq!(config.get_dlpf(), dlpf_cfg);

            let dlpf_cfg = DigitalLowPassFilterCfg::from_u8((b + 1) % 8).expect("Must be !");
            config.set_dlpf(dlpf_cfg);
            assert_eq!(config.get_fsync(), ext_sync_set);
            assert_eq!(config.get_dlpf(), dlpf_cfg);
        }
    }
}

#[test]
fn gyro_cfg_from_u8() {
    for a in 0..8 {
        for b in 0..4 {
            let st_xyz = FlagsXYZ(a);
            let fs_sel = GyroFullScale::from_u8(b).expect("Must be !");

            let v = st_xyz.as_u8() * 32 + (fs_sel as u8) * 8;
            let c = GyroConfig::from(v);

            assert_eq!(v, c.into());
            assert_eq!(c.get_xyz(), st_xyz);
            assert_eq!(c.get_scale(), fs_sel);

            let mut c = c;

            let st_xyz = FlagsXYZ((a + 1) % 8);
            c.set_xyz(st_xyz);
            assert_eq!(c.get_xyz(), st_xyz);
            assert_eq!(c.get_scale(), fs_sel);

            let fs_sel = GyroFullScale::from_u8((b + 1) % 4).expect("Must be !");
            c.set_scale(fs_sel);
            assert_eq!(c.get_xyz(), st_xyz);
            assert_eq!(c.get_scale(), fs_sel);
        }
    }
}

#[test]
fn accel_cfg_from_u8() {
    for a in 0..8 {
        for b in 0..4 {
            let st_xyz = FlagsXYZ(a);
            let afs_sel = AccelFullScale::from_u8(b).expect("Must be !");

            let v = st_xyz.as_u8() * 32 + (afs_sel as u8) * 8;
            let c = AccelConfig::from(v);

            assert_eq!(v, c.into());
            assert_eq!(c.get_xyz(), st_xyz);
            assert_eq!(c.get_scale(), afs_sel);

            let mut c = c;

            let st_xyz = FlagsXYZ((a + 1) % 8);
            c.set_xyz(st_xyz);
            assert_eq!(c.get_xyz(), st_xyz);
            assert_eq!(c.get_scale(), afs_sel);

            let afs_sel = AccelFullScale::from_u8((b + 1) % 4).expect("Must be !");
            c.set_scale(afs_sel);
            assert_eq!(c.get_xyz(), st_xyz);
            assert_eq!(c.get_scale(), afs_sel);
        }
    }
}

#[test]
fn raw_data_from_buf() {
    let buf = [0, 1, 0, 2, 0, 3, 1, 0, 1, 1, 1, 2, 1, 3];
    let raw = RawData::from(buf);
    assert_eq!(raw.temp.0, 0x100);
    assert_eq!(
        raw.accel,
        AccelData {
            x: 0x01,
            y: 0x02,
            z: 0x03
        }
    );
    assert_eq!(
        raw.gyro,
        GyroData {
            x: 0x101,
            y: 0x102,
            z: 0x103
        }
    )
}

#[test]
fn flags_xyz_from_booleans() {
    assert_eq!(FlagsXYZ::new(false, false, false), FlagsXYZ(0));
    assert_eq!(FlagsXYZ::new(false, false, true), FlagsXYZ(1));
    assert_eq!(FlagsXYZ::new(false, true, false), FlagsXYZ(2));
    assert_eq!(FlagsXYZ::new(false, true, true), FlagsXYZ(3));
    assert_eq!(FlagsXYZ::new(true, false, false), FlagsXYZ(4));
    assert_eq!(FlagsXYZ::new(true, false, true), FlagsXYZ(5));
    assert_eq!(FlagsXYZ::new(true, true, false), FlagsXYZ(6));
    assert_eq!(FlagsXYZ::new(true, true, true), FlagsXYZ(7));
    for i in 0..8 {
        let a = FlagsXYZ(i);
        let b = FlagsXYZ::new(a.x(), a.y(), a.z());
        assert_eq!(a, b);
    }
}

#[test]
fn frame_sync_as_u8() {
    assert_eq!(FrameSync::Disabled as u8, 0);
    assert_eq!(FrameSync::TempOutL as u8, 1);
    assert_eq!(FrameSync::GyroXoutL as u8, 2);
    assert_eq!(FrameSync::GyroYoutL as u8, 3);
    assert_eq!(FrameSync::GyroZoutL as u8, 4);
    assert_eq!(FrameSync::AccelXoutL as u8, 5);
    assert_eq!(FrameSync::AccelYoutL as u8, 6);
    assert_eq!(FrameSync::AccelZoutL as u8, 7);
    for i in 0..8 {
        let a = FrameSync::from_u8(i).expect("Must be !");
        assert_eq!(i, a as u8);
    }
}

#[test]
fn dlpf_as_u8() {
    assert_eq!(DigitalLowPassFilterCfg::V0 as u8, 0);
    assert_eq!(DigitalLowPassFilterCfg::V1 as u8, 1);
    assert_eq!(DigitalLowPassFilterCfg::V2 as u8, 2);
    assert_eq!(DigitalLowPassFilterCfg::V3 as u8, 3);
    assert_eq!(DigitalLowPassFilterCfg::V4 as u8, 4);
    assert_eq!(DigitalLowPassFilterCfg::V5 as u8, 5);
    assert_eq!(DigitalLowPassFilterCfg::V6 as u8, 6);
    assert_eq!(DigitalLowPassFilterCfg::V7 as u8, 7);
    for i in 0..8 {
        let a = DigitalLowPassFilterCfg::from_u8(i).expect("Must be !");
        assert_eq!(i, a as u8);
    }
}

#[test]
fn dlpf_cfg() {
    fn as_tuple(
        c: DigitalLowPassFilterCfg,
    ) -> (
        Hertz,
        Microseconds,
        Kilohertz,
        Hertz,
        Microseconds,
        Kilohertz,
    ) {
        let a = c.accel();
        let g = c.gyro();
        (a.bandwidth, a.delay, a.fs, g.bandwidth, g.delay, g.fs)
    }

    assert_eq!(
        as_tuple(DigitalLowPassFilterCfg::V0),
        (
            260.Hz(),
            0.0.milliseconds(),
            1.kHz(),
            256.Hz(),
            0.98.milliseconds(),
            8.kHz()
        )
    );
    assert_eq!(
        as_tuple(DigitalLowPassFilterCfg::V1),
        (
            184.Hz(),
            2.0.milliseconds(),
            1.kHz(),
            188.Hz(),
            1.9.milliseconds(),
            1.kHz()
        )
    );
    assert_eq!(
        as_tuple(DigitalLowPassFilterCfg::V2),
        (
            94.Hz(),
            3.0.milliseconds(),
            1.kHz(),
            98.Hz(),
            2.8.milliseconds(),
            1.kHz()
        )
    );
    assert_eq!(
        as_tuple(DigitalLowPassFilterCfg::V3),
        (
            44.Hz(),
            4.9.milliseconds(),
            1.kHz(),
            42.Hz(),
            4.8.milliseconds(),
            1.kHz()
        )
    );
    assert_eq!(
        as_tuple(DigitalLowPassFilterCfg::V4),
        (
            21.Hz(),
            8.5.milliseconds(),
            1.kHz(),
            20.Hz(),
            8.3.milliseconds(),
            1.kHz()
        )
    );
    assert_eq!(
        as_tuple(DigitalLowPassFilterCfg::V5),
        (
            10.Hz(),
            13.8.milliseconds(),
            1.kHz(),
            10.Hz(),
            13.4.milliseconds(),
            1.kHz()
        )
    );
    assert_eq!(
        as_tuple(DigitalLowPassFilterCfg::V6),
        (
            5.Hz(),
            19.0.milliseconds(),
            1.kHz(),
            5.Hz(),
            18.6.milliseconds(),
            1.kHz()
        )
    );
    assert_eq!(
        as_tuple(DigitalLowPassFilterCfg::V7),
        (
            0.Hz(),
            0.0.milliseconds(),
            0.kHz(),
            0.Hz(),
            0.0.milliseconds(),
            8.kHz()
        )
    );
}

/// Float で計算して端数やオーバーフローは切り捨てる変換を実装
trait FloatDuration<T: embedded_time::TimeInt> {
    fn milliseconds(self) -> Microseconds<T>;
}

impl FloatDuration<u32> for f32 {
    fn milliseconds(self) -> Microseconds<u32> {
        ((self * 1000.0) as u32).microseconds()
    }
}
