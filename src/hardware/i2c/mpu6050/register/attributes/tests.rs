use super::*;
use embedded_time::{duration::*, rate::*};
use num_traits::FromPrimitive;

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
