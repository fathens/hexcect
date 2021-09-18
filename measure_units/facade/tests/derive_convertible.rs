use measure_units::*;
use measure_units_derive::*;

use derive_more::From;

#[derive(From, Convertible)]
#[convertible(Meter = 1000)]
#[convertible(Milli ^ 6)]
struct Km(f64);

#[derive(From, Convertible)]
#[convertible(Km ^ -3)]
#[convertible(Milli ^ 3)]
struct Meter(f64);

#[derive(From, Convertible)]
#[convertible(Km ^ -6)]
#[convertible(Meter = 0.001)]
struct Milli(f64);

#[test]
fn derive_convertible() {
    let a_m = Meter::from(1.0);
    let a_km: Km = a_m.convert();
    let a_mm: Milli = a_m.convert();
    assert_eq!(a_m.0, 1.0);
    assert_eq!(a_km.0, 0.001);
    assert_eq!(a_mm.0, 1000.0);

    let b_km = Km::from(1.0);
    let b_m: Meter = b_km.convert();
    let b_mm: Milli = b_km.convert();
    assert_eq!(b_km.0, 1.0);
    assert_eq!(b_m.0, 1000.0);
    assert_eq!(b_mm.0, 1000_000.0);

    let c_mm = Milli::from(1.0);
    let c_m: Meter = c_mm.convert();
    let c_km: Km = c_mm.convert();
    assert_eq!(c_mm.0, 1.0);
    assert_eq!(c_m.0, 0.001);
    assert_eq!(c_km.0, 0.00_0001);
}
