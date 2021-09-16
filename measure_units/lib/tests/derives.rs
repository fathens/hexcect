use measure_units::*;
use measure_units_derive::*;

#[derive(FloatStatus)]
struct MyUnit(f64);

#[test]
fn derive_float_status() {
    let z = MyUnit(0.0);
    let n = MyUnit(0.0 / 0.0);
    assert_eq!(z.is_nan(), false);
    assert_eq!(n.is_nan(), true);
}
