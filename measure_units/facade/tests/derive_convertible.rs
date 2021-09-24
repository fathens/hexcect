use measure_units::*;

use derive_more::From;

#[derive(Clone, Copy, From, Convertible)]
#[convertible(Meter = 1000)]
#[convertible(Milli ^ 6)]
struct Km(f64);

#[derive(Clone, Copy, From, Convertible)]
#[convertible(Km ^ -3)]
#[convertible(Milli ^ 3)]
struct Meter(f64);

#[derive(Clone, Copy, From, Convertible)]
#[convertible(Km ^ -6)]
#[convertible(Meter = 0.001)]
struct Milli(f64);

#[derive(Clone, Copy, From, Convertible)]
#[convertible(Minute = 1.0/60.0)]
struct Second<V>(V);

#[derive(Clone, Copy, From, Convertible)]
#[convertible(Second = 60.0)]
struct Minute<V>(V);

#[derive(Clone, Copy, From, Convertible)]
#[convertible(Minute = 0.0/0.0)]
#[convertible(Second = 1/(60 * 60))]
struct Hour<V>(V);

#[test]
fn derive_convertible() {
    let a_m = Meter::from(1.0);
    let a_km: Km = a_m.into();
    let a_mm: Milli = a_m.into();
    assert_eq!(a_m.0, 1.0);
    assert_eq!(a_km.0, 0.001);
    assert_eq!(a_mm.0, 1000.0);

    let b_km = Km::from(1.0);
    let b_m: Meter = b_km.into();
    let b_mm: Milli = b_km.into();
    assert_eq!(b_km.0, 1.0);
    assert_eq!(b_m.0, 1000.0);
    assert_eq!(b_mm.0, 1000_000.0);

    let c_mm = Milli::from(1.0);
    let c_m: Meter = c_mm.into();
    let c_km: Km = c_mm.into();
    assert_eq!(c_mm.0, 1.0);
    assert_eq!(c_m.0, 0.001);
    assert_eq!(c_km.0, 0.00_0001);
}

#[test]
fn derive_convertible_with_generics() {
    let a_s = Second::from(60.0_f64);
    let a_m: Minute<f64> = a_s.into();
    assert_eq!(a_s.0, 60.0);
    assert_eq!(a_m.0, 1.0);

    let b_s: Second<f64> = a_m.into();
    assert_eq!(b_s.0, 60.0);
}

#[test]
#[should_panic(expected = "NaN")]
fn derive_convertible_nan() {
    let a_h = Hour::from(3.0_f64);
    let _: Minute<f64> = a_h.into();
}

#[test]
#[should_panic(expected = "Zero")]
fn derive_convertible_zero() {
    let a_h = Hour::from(1.0_f64);
    let _: Second<f64> = a_h.into();
}
