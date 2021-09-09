use super::*;

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
fn pwr_mgmt1_from_u8() {
    for clksel in 0..8 {
        for tempdis in [true, false] {
            for cycle in [true, false] {
                for sleep in [true, false] {
                    for dreset in [true, false] {
                        let mut o = PwrMgmt1(0);
                        o.set_clksel(ClockSel::from_u8(clksel).unwrap());
                        o.set_tempdis(tempdis);
                        o.set_cycle(cycle);
                        o.set_sleep(sleep);
                        o.set_device_reset(dreset);

                        let c: u8 = o.into();
                        assert_eq!(clksel, c.get_with_mask(0b111, 0));
                        assert_eq!(tempdis, c.get(3));
                        assert_eq!(cycle, c.get(5));
                        assert_eq!(sleep, c.get(6));
                        assert_eq!(dreset, c.get(7));

                        let o = PwrMgmt1::from(c);
                        assert_eq!(clksel, o.get_clksel() as u8);
                        assert_eq!(tempdis, o.get_tempdis());
                        assert_eq!(cycle, o.get_cycle());
                        assert_eq!(sleep, o.get_sleep());
                        assert_eq!(dreset, o.get_device_reset());
                    }
                }
            }
        }
    }
}

#[test]
fn user_ctrl_from_u8() {
    for sigcond_reset in [true, false] {
        for i2cmst_reset in [true, false] {
            for fifo_reset in [true, false] {
                for i2cmst_en in [true, false] {
                    for fifo_en in [true, false] {
                        let mut o = UserCtrl(0);
                        o.set_sigcond_reset(sigcond_reset);
                        o.set_i2cmst_reset(i2cmst_reset);
                        o.set_fifo_reset(fifo_reset);
                        o.set_i2cmst_en(i2cmst_en);
                        o.set_fifo_en(fifo_en);

                        let c: u8 = o.into();
                        assert_eq!(sigcond_reset, c.get(0));
                        assert_eq!(i2cmst_reset, c.get(1));
                        assert_eq!(fifo_reset, c.get(2));
                        assert_eq!(i2cmst_en, c.get(5));
                        assert_eq!(fifo_en, c.get(6));

                        let o = UserCtrl::from(c);
                        assert_eq!(sigcond_reset, o.get_sigcond_reset());
                        assert_eq!(i2cmst_reset, o.get_i2cmst_reset());
                        assert_eq!(fifo_reset, o.get_fifo_reset());
                        assert_eq!(i2cmst_en, o.get_i2cmst_en());
                        assert_eq!(fifo_en, o.get_fifo_en());
                    }
                }
            }
        }
    }
}

#[test]
fn int_enable_from_u8() {
    for datardy_en in [true, false] {
        for i2cmst_int_en in [true, false] {
            for fifo_oflow_en in [true, false] {
                for mot_en in [true, false] {
                    let mut o = IntEnable(0);
                    o.set_datardy_en(datardy_en);
                    o.set_i2cmst_int_en(i2cmst_int_en);
                    o.set_fifo_oflow_en(fifo_oflow_en);
                    o.set_mot_en(mot_en);

                    let c: u8 = o.into();
                    assert_eq!(datardy_en, c.get(0));
                    assert_eq!(i2cmst_int_en, c.get(3));
                    assert_eq!(fifo_oflow_en, c.get(4));
                    assert_eq!(mot_en, c.get(6));

                    let o = IntEnable::from(c);
                    assert_eq!(datardy_en, o.get_datardy_en());
                    assert_eq!(i2cmst_int_en, o.get_i2cmst_int_en());
                    assert_eq!(fifo_oflow_en, o.get_fifo_oflow_en());
                    assert_eq!(mot_en, o.get_mot_en());
                }
            }
        }
    }
}

#[test]
fn fifo_enable_from_u8() {
    for c in 0..=255 {
        let o = FifoEnable::from(c);
        assert_eq!(c.get(0), o.get_slv0());
        assert_eq!(c.get(1), o.get_slv1());
        assert_eq!(c.get(2), o.get_slv2());
        assert_eq!(c.get(3), o.get_accel());
        assert_eq!(c.get(4), o.get_zg());
        assert_eq!(c.get(5), o.get_yg());
        assert_eq!(c.get(6), o.get_xg());
        assert_eq!(c.get(7), o.get_temp());

        let mut a = FifoEnable::from(0);
        a.set_slv0(o.get_slv0());
        a.set_slv1(o.get_slv1());
        a.set_slv2(o.get_slv2());
        a.set_accel(o.get_accel());
        a.set_zg(o.get_zg());
        a.set_yg(o.get_yg());
        a.set_xg(o.get_xg());
        a.set_temp(o.get_temp());

        assert_eq!(a, o);
    }
}

#[test]
fn fifo_count_from_buf() {
    let buf = [0x12, 0x34];
    let fc = FifoCount::from(buf);
    assert_eq!(0x1234_u16, fc.into());
}

#[test]
fn raw_data_from_buf() {
    let buf = [0, 1, 0, 2, 0, 3, 1, 0, 1, 1, 1, 2, 1, 3];
    let raw = RawData::from(buf);
    assert_eq!(0x100_i16, raw.temp.into());
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
