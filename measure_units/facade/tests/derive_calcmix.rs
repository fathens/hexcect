#![feature(once_cell)]

use measure_units::*;
use num_traits::*;

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

#[test]
fn simplify_mul_reduction() {
    let a = Meter::from(2.0_f64) / Second::from(4.0_f64) * Second::from(2.0_f64);
    let b: Meter = simplify!(a: UnitsMul<f64, UnitsDiv<f64, Meter, Second>, Second>);
    assert_eq!(a.to_string(), "1m/ss");
    assert_eq!(b.to_string(), "1m");
}

#[test]
fn simplify_mul_commutative_reduction() {
    let a = Second::from(2.0_f64) * (Meter::from(3.0_f64) / Second::from(1.0_f64));
    let b: Meter = simplify!(a: UnitsMul<f64, Second, UnitsDiv<f64, Meter, Second>>);
    assert_eq!(a.to_string(), "6sm/s");
    assert_eq!(b.to_string(), "6m");
}

#[test]
fn simplify_mul_scalar() {
    let a = Meter::from(3.0_f64) / Second::from(2.0_f64) * Scalar::from(10.0_f64);
    let b: UnitsDiv<f64, Meter, Second> =
        simplify!(a: UnitsMul<f64, UnitsDiv<f64, Meter, Second>, Scalar<f64>>);
    assert_eq!(a.to_string(), "15m/s");
    assert_eq!(b.to_string(), "15m/s");
}

#[test]
fn simplify_mul_commutative_scalar() {
    let a: UnitsMul<f64, Scalar<f64>, UnitsDiv<f64, Meter, Second>> =
        Scalar::from(10.0_f64) * (Meter::from(6.0_f64) / Second::from(3.0_f64));
    let b: UnitsDiv<f64, Meter, Second> =
        simplify!(a: UnitsMul<f64, Scalar<f64>, UnitsDiv<f64, Meter, Second>>);
    assert_eq!(a.to_string(), "20m/s");
    assert_eq!(b.to_string(), "20m/s");
}

#[test]
fn simplify_div_reduction() {
    let a = Meter::from(5.0_f64) / Meter::from(2.0_f64);
    let b: Scalar<f64> = simplify!(a: UnitsDiv<f64, Meter, Meter>);
    assert_eq!(a.to_string(), "2.5m/m");
    assert_eq!(b.to_string(), "2.5");
}

#[test]
fn simplify_div_scalar() {
    let a = Meter::from(10_f64) / Scalar::from(5_f64);
    let b: Meter = simplify!(a: UnitsDiv<f64, Meter, Scalar<f64>>);
    assert_eq!(a.to_string(), "2m/");
    assert_eq!(b.to_string(), "2m");
}

#[test]
fn simplify_div_reduction_right() {
    let a = Second::from(5_f64) * Meter::from(3_f64) / Meter::from(2_f64);
    let b = simplify!(a: UnitsDiv<f64, UnitsMul<f64, Second, Meter>, Meter>);
    assert_eq!(a.to_string(), "7.5sm/m");
    assert_eq!(b.to_string(), "7.5s");
}

#[test]
fn simplify_div_reduction_left() {
    let a = Meter::from(2_f64) * Second::from(3_f64) / Meter::from(4_f64);
    let b: Second = simplify!(a: UnitsDiv<f64, UnitsMul<f64, Meter, Second>, Meter>);
    assert_eq!(a.to_string(), "1.5ms/m");
    assert_eq!(b.to_string(), "1.5s");
}

#[test]
fn simplify_associative01() {
    let a = (Meter::from(1_f64) * Second::from(1_f64)) * (Km::from(1_f64) / Second::from(1_f64));
    let b: UnitsMul<f64, Meter, Km> =
        simplify!(a: UnitsMul<f64, UnitsMul<f64, Meter, Second>, UnitsDiv<f64, Km, Second>>);
    assert_eq!(a.to_string(), "1mskm/s");
    assert_eq!(b.to_string(), "1mkm");
}

#[test]
fn simplify_associative02() {
    let a = (Second::from(1_f64) * Meter::from(1_f64)) * (Km::from(1_f64) / Second::from(1_f64));
    let b: UnitsMul<f64, Meter, Km> =
        simplify!(a: UnitsMul<f64, UnitsMul<f64, Second, Meter>, UnitsDiv<f64, Km, Second>>);
    assert_eq!(a.to_string(), "1smkm/s");
    assert_eq!(b.to_string(), "1mkm");
}

#[test]
fn simplify_associative03() {
    let a = (Km::from(1_f64) / Second::from(1_f64)) * (Meter::from(1_f64) * Second::from(1_f64));
    let b: UnitsMul<f64, Meter, Km> =
        simplify!(a: UnitsMul<f64, UnitsDiv<f64, Km, Second>, UnitsMul<f64, Meter, Second>>);
    assert_eq!(a.to_string(), "1km/sms");
    assert_eq!(b.to_string(), "1mkm");
}

#[test]
fn simplify_associative04() {
    let a = (Km::from(1_f64) / Second::from(1_f64)) * (Second::from(1_f64) * Meter::from(1_f64));
    let b: UnitsMul<f64, Meter, Km> =
        simplify!(a: UnitsMul<f64, UnitsDiv<f64, Km, Second>, UnitsMul<f64, Second, Meter>>);
    assert_eq!(a.to_string(), "1km/ssm");
    assert_eq!(b.to_string(), "1mkm");
}

#[test]
fn simplify_inner() {
    let a = (Meter::from(1_f64)
        / ((Meter::from(1_f64) * Second::from(1_f64)) / Meter::from(1_f64)))
        * ((((Meter::from(1_f64) * Second::from(1_f64)) / Meter::from(1_f64))
            / (Meter::from(1_f64)
                / (Second::from(1_f64) * (Meter::from(1_f64) / Second::from(1_f64)))))
            * Second::from(1_f64));
    let b: UnitsMul<f64, Meter, Second> = simplify!(
        a: UnitsMul<
            f64,
            UnitsDiv<f64, Meter, UnitsDiv<f64, UnitsMul<f64, Meter, Second>, Meter>>,
            UnitsMul<
                f64,
                UnitsDiv<
                    f64,
                    UnitsDiv<f64, UnitsMul<f64, Meter, Second>, Meter>,
                    UnitsDiv<f64, Meter, UnitsMul<f64, Second, UnitsDiv<f64, Meter, Second>>>,
                >,
                Second,
            >,
        >
    )
    .commutative();
    assert_eq!(a.to_string(), "1m/ms/mms/m/m/sm/ss");
    assert_eq!(b.to_string(), "1ms");
}
