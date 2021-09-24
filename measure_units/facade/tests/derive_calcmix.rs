#![feature(once_cell)]

use measure_units::*;

#[derive(Clone, Copy, CalcMix, Convertible)]
#[calcmix(unit_name="m".to_string())]
#[convertible(Km ^ -3)]
struct Meter(f64);

#[derive(Clone, Copy, CalcMix, Convertible)]
#[calcmix(unit_name="km".to_string())]
#[convertible(Meter ^ 3)]
struct Km(f64);

#[derive(Clone, Copy, CalcMix, Convertible)]
#[calcmix(unit_name="h".to_string())]
#[convertible(Second = 60.0 * 60.0)]
struct Hour(f64);

#[derive(Clone, Copy, CalcMix, Convertible)]
#[calcmix(unit_name="s".to_string())]
#[convertible(Hour = 1.0 / (60.0 * 60.0))]
struct Second(f64);

#[test]
fn div() {
    let a_m: Meter = 1.23.into();
    let b_km: Km = 3.45.into();
    let a_h: Hour = 1.5.into();
    let b_s: Second = 234.56.into();

    let d = a_m + b_km;
    assert_eq!(d.to_string(), "3451.23m");
    let t = a_h + b_s;
    assert_eq!(t.to_string(), "1.5651555555555556h");

    let speed = d / t;
    assert_eq!(speed.to_string(), "2205.039612676056m/h");
}
