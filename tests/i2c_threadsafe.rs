use hexcect::hardware::i2c::*;

mod i2c_mock;
use i2c_mock::*;

use rand::prelude::*;

fn sleep_rand() {
    let mut rng = rand::thread_rng();
    let s: f64 = rng.gen();
    let v = (100.0 * s) as u64;
    let dur = std::time::Duration::from_nanos(v);
    std::thread::sleep(dur);
}

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
        let i2c = I2cWithAddr::new(safe_i2c, i.into());

        for offset in 50..100 {
            let mut cloned_i2c = i2c.clone();
            handles.push(std::thread::spawn(move || {
                sleep_rand();
                cloned_i2c
                    .write_register::<MockRegisterA>(mk_reg(offset))
                    .unwrap();
            }));
            let mut cloned_i2c = i2c.clone();
            handles.push(std::thread::spawn(move || {
                sleep_rand();
                cloned_i2c
                    .write_register::<MockRegisterB>(mk_reg(offset))
                    .unwrap();
            }));
            let mut cloned_i2c = i2c.clone();
            handles.push(std::thread::spawn(move || {
                sleep_rand();
                cloned_i2c
                    .write_register::<MockRegisterC>(mk_reg(offset))
                    .unwrap();
            }));
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
