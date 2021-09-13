use hexcect::hardware::i2c::*;

mod i2c_mock;
use i2c_mock::*;

fn sleep_rand() {
    let v = (100.0 * rand::random::<f64>()) as u64;
    let dur = std::time::Duration::from_nanos(v);
    std::thread::sleep(dur);
}

fn run_write<R: Register>(
    mut i2c: I2cWithAddr<ThreadSafeI2c<MockI2c>>,
    offset: u8,
) -> std::thread::JoinHandle<()> {
    std::thread::spawn(move || {
        sleep_rand();
        let addr: u8 = R::ADDR.into();
        let reg: R = (offset + addr).into();
        i2c.write_register(reg).unwrap();
    })
}

#[test]
fn write_registers_addresses() {
    let mut mock = MockI2c::default();
    mock.prepare_data(0, &[]);
    let safe_i2c = ThreadSafeI2c::new(mock);

    let mut handles = vec![];

    for i in 0_u8..100 {
        let i2c = I2cWithAddr::new(safe_i2c.clone(), i.into());

        for offset in 50..100 {
            handles.push(run_write::<MockRegisterA>(i2c.clone(), offset));
            handles.push(run_write::<MockRegisterB>(i2c.clone(), offset));
            handles.push(run_write::<MockRegisterC>(i2c.clone(), offset));
        }
    }
    for h in handles {
        h.join().unwrap();
    }

    let written = safe_i2c.lock().written.clone();
    assert_eq!(written.len(), 100);
    for i in 0_u8..100 {
        let data = &written[&i];
        let vec: Vec<_> = data.iter().map(|a| *a).collect();
        let mut chunks: Vec<_> = vec.chunks(2).map(|c| [c[0], c[1]]).collect();
        chunks.sort();

        let mut expected = vec![];
        for reg_addr in [
            MockRegisterA::ADDR,
            MockRegisterB::ADDR,
            MockRegisterC::ADDR,
        ] {
            for offset in 50_u8..100 {
                let a: u8 = reg_addr.into();
                let v = a + offset;
                expected.push([a, v]);
            }
        }
        assert_eq!(chunks, expected);
    }
}
