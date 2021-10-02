use std::marker::PhantomData;

use measure_units::*;

use derive_more::{From, Into};

#[derive(Clone, Copy, From, Into, FloatStatus)]
struct MyUnit(f64);

#[derive(Clone, Copy, FloatStatus)]
struct MyGenerics<V, A>(V, PhantomData<A>);

impl From<f64> for MyGenerics<f64, i8> {
    fn from(s: f64) -> Self {
        MyGenerics(s, PhantomData::<i8>)
    }
}

impl From<MyGenerics<f64, i8>> for f64 {
    fn from(s: MyGenerics<f64, i8>) -> Self {
        s.0
    }
}

#[test]
fn derive_float_status() {
    let a = MyUnit::from(1.2);
    let b = MyUnit::from(-0.2);
    let s = MyUnit::from(f64::MIN_POSITIVE / 2.0);
    let m = MyUnit::from(-1.0 * f64::MIN_POSITIVE / 2.0);
    let z = MyUnit::from(0.0);
    let n = MyUnit::from(0.0 / 0.0);
    let p = MyUnit::from(f64::INFINITY);
    let g = MyUnit::from(f64::NEG_INFINITY);

    assert_eq!(a.abs().0, 1.2);
    assert_eq!(a.is_nan(), false);
    assert_eq!(a.is_zero(), false);
    assert_eq!(a.is_normal(), true);
    assert_eq!(a.is_subnormal(), false);
    assert_eq!(a.is_finite(), true);
    assert_eq!(a.is_infinite(), false);
    assert_eq!(a.is_sign_positive(), true);
    assert_eq!(a.is_sign_negative(), false);

    assert_eq!(b.abs().0, 0.2);
    assert_eq!(b.is_nan(), false);
    assert_eq!(b.is_zero(), false);
    assert_eq!(b.is_normal(), true);
    assert_eq!(b.is_subnormal(), false);
    assert_eq!(b.is_finite(), true);
    assert_eq!(b.is_infinite(), false);
    assert_eq!(b.is_sign_positive(), false);
    assert_eq!(b.is_sign_negative(), true);

    assert_eq!(s.abs().0, f64::MIN_POSITIVE / 2.0);
    assert_eq!(s.is_nan(), false);
    assert_eq!(s.is_zero(), false);
    assert_eq!(s.is_normal(), false);
    assert_eq!(s.is_subnormal(), true);
    assert_eq!(s.is_finite(), true);
    assert_eq!(s.is_infinite(), false);
    assert_eq!(s.is_sign_positive(), true);
    assert_eq!(s.is_sign_negative(), false);

    assert_eq!(m.abs().0, f64::MIN_POSITIVE / 2.0);
    assert_eq!(m.is_nan(), false);
    assert_eq!(m.is_zero(), false);
    assert_eq!(m.is_normal(), false);
    assert_eq!(m.is_subnormal(), true);
    assert_eq!(m.is_finite(), true);
    assert_eq!(m.is_infinite(), false);
    assert_eq!(m.is_sign_positive(), false);
    assert_eq!(m.is_sign_negative(), true);

    assert_eq!(z.abs().0, 0.0);
    assert_eq!(z.is_nan(), false);
    assert_eq!(z.is_zero(), true);
    assert_eq!(z.is_normal(), false);
    assert_eq!(z.is_subnormal(), false);
    assert_eq!(z.is_finite(), true);
    assert_eq!(z.is_infinite(), false);
    assert_eq!(z.is_sign_positive(), true);
    assert_eq!(z.is_sign_negative(), false);

    assert!(n.abs().0.is_nan());
    assert_eq!(n.is_nan(), true);
    assert_eq!(n.is_zero(), false);
    assert_eq!(n.is_normal(), false);
    assert_eq!(n.is_subnormal(), false);
    assert_eq!(n.is_finite(), false);
    assert_eq!(n.is_infinite(), false);
    assert_eq!(n.is_sign_positive(), true);
    assert_eq!(n.is_sign_negative(), false);

    assert_eq!(p.abs().0, f64::INFINITY);
    assert_eq!(p.is_nan(), false);
    assert_eq!(p.is_zero(), false);
    assert_eq!(p.is_normal(), false);
    assert_eq!(p.is_subnormal(), false);
    assert_eq!(p.is_finite(), false);
    assert_eq!(p.is_infinite(), true);
    assert_eq!(p.is_sign_positive(), true);
    assert_eq!(p.is_sign_negative(), false);

    assert_eq!(g.abs().0, f64::INFINITY);
    assert_eq!(g.is_nan(), false);
    assert_eq!(p.is_zero(), false);
    assert_eq!(g.is_normal(), false);
    assert_eq!(g.is_subnormal(), false);
    assert_eq!(g.is_finite(), false);
    assert_eq!(g.is_infinite(), true);
    assert_eq!(g.is_sign_positive(), false);
    assert_eq!(g.is_sign_negative(), true);

    for x in [a, b, s, m, z, n, p, g] {
        let y: f64 = x.into();

        if !y.is_nan() {
            assert_eq!(x.abs().0, y.abs());
        }
        assert_eq!(x.is_nan(), y.is_nan());
        assert_eq!(x.is_zero(), y == 0.0);
        assert_eq!(x.is_normal(), y.is_normal());
        assert_eq!(x.is_subnormal(), y.is_subnormal());
        assert_eq!(x.is_finite(), y.is_finite());
        assert_eq!(x.is_infinite(), y.is_infinite());
        assert_eq!(x.is_sign_positive(), y.is_sign_positive());
        assert_eq!(x.is_sign_negative(), y.is_sign_negative());
    }
}

#[test]
fn derive_float_status_with_generics() {
    let a = MyGenerics::from(1.2);
    let b = MyGenerics::from(-0.2);
    let s = MyGenerics::from(f64::MIN_POSITIVE / 2.0);
    let m = MyGenerics::from(-1.0 * f64::MIN_POSITIVE / 2.0);
    let z = MyGenerics::from(0.0);
    let n = MyGenerics::from(0.0 / 0.0);
    let p = MyGenerics::from(f64::INFINITY);
    let g = MyGenerics::from(f64::NEG_INFINITY);

    for x in [a, b, s, m, z, n, p, g] {
        let y: f64 = x.into();

        if !y.is_nan() {
            assert_eq!(x.abs().0, y.abs());
        }
        assert_eq!(x.is_nan(), y.is_nan());
        assert_eq!(x.is_zero(), y == 0.0);
        assert_eq!(x.is_normal(), y.is_normal());
        assert_eq!(x.is_subnormal(), y.is_subnormal());
        assert_eq!(x.is_finite(), y.is_finite());
        assert_eq!(x.is_infinite(), y.is_infinite());
        assert_eq!(x.is_sign_positive(), y.is_sign_positive());
        assert_eq!(x.is_sign_negative(), y.is_sign_negative());
    }
}
