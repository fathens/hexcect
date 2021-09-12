use hexcect::hardware::i2c::*;

mod i2c_mock;
use i2c_mock::*;

#[test]
fn write_registers_addresses() {
    let mut mock = MockI2c::default();
    mock.prepare_data(0, &[]);
    let safe_i2c = ThreadSafeI2c::new(mock);

    let mut handles = vec![];

    fn mk_reg<R: Register>(offset: u8) -> R {
        let addr: u8 = R::ADDR.into();
        (offset + addr).into()
    }

    for i in 0_u8..100 {
        let safe_i2c = safe_i2c.clone();
        let h = std::thread::spawn(move || {
            let mut i2c = I2cWithAddr::new(safe_i2c, i.into());
            for r in 0..50 {
                let offset = 50 + r;
                let reg_a: MockRegisterA = mk_reg(offset);
                let reg_b: MockRegisterB = mk_reg(offset);
                let reg_c: MockRegisterC = mk_reg(offset);
                i2c.write_register(reg_a).unwrap();
                i2c.write_register(reg_b).unwrap();
                i2c.write_register(reg_c).unwrap();
            }
        });
        handles.push(h);
    }
    for h in handles {
        h.join().unwrap();
    }

    let written = safe_i2c.lock().written.clone();
    assert_eq!(written.len(), 100);
    for i in 0_u8..100 {
        let data = &written[&i];
        let vec: Vec<_> = data.iter().collect();
        let mut chunks: Vec<_> = vec.chunks(2).collect();
        chunks.sort();
        let chunks: Vec<Vec<u8>> = chunks
            .iter()
            .map(|cs| {
                let v: Vec<_> = (*cs).iter().map(|c| **c).collect();
                v
            })
            .collect();

        let mut expected = vec![];
        for reg_addr in [
            MockRegisterA::ADDR,
            MockRegisterB::ADDR,
            MockRegisterC::ADDR,
        ] {
            for r in 0_u8..50 {
                let a: u8 = reg_addr.into();
                let v = a + r + 50;
                expected.push(vec![a, v]);
            }
        }
        assert_eq!(chunks, expected);
    }
}
