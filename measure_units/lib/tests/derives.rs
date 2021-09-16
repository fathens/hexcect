use measure_units::*;
use measure_units_derive::*;

use derive_more::{From, Into};

#[derive(Clone, Copy, From, Into, FloatStatus)]
struct MyUnit(f64);

#[test]
fn derive_float_status() {
    let a = MyUnit::from(1.2);
    let b = MyUnit::from(-0.2);
    let s = MyUnit::from(f64::MIN_POSITIVE / 2.0);
    let z = MyUnit::from(0.0);
    let n = MyUnit::from(0.0 / 0.0);
    let p = MyUnit::from(f64::INFINITY);
    let g = MyUnit::from(f64::NEG_INFINITY);

    assert_eq!(a.is_nan(), false);
    assert_eq!(a.is_normal(), true);
    assert_eq!(a.is_subnormal(), false);
    assert_eq!(a.is_finite(), true);
    assert_eq!(a.is_infinite(), false);
    assert_eq!(a.is_sign_positive(), true);
    assert_eq!(a.is_sign_negative(), false);

    assert_eq!(b.is_nan(), false);
    assert_eq!(b.is_normal(), true);
    assert_eq!(b.is_subnormal(), false);
    assert_eq!(b.is_finite(), true);
    assert_eq!(b.is_infinite(), false);
    assert_eq!(b.is_sign_positive(), false);
    assert_eq!(b.is_sign_negative(), true);

    assert_eq!(s.is_nan(), false);
    assert_eq!(s.is_normal(), false);
    assert_eq!(s.is_subnormal(), true);
    assert_eq!(s.is_finite(), true);
    assert_eq!(s.is_infinite(), false);
    assert_eq!(s.is_sign_positive(), true);
    assert_eq!(s.is_sign_negative(), false);

    assert_eq!(z.is_nan(), false);
    assert_eq!(z.is_normal(), false);
    assert_eq!(z.is_subnormal(), false);
    assert_eq!(z.is_finite(), true);
    assert_eq!(z.is_infinite(), false);
    assert_eq!(z.is_sign_positive(), true);
    assert_eq!(z.is_sign_negative(), false);

    assert_eq!(n.is_nan(), true);
    assert_eq!(n.is_normal(), false);
    assert_eq!(n.is_subnormal(), false);
    assert_eq!(n.is_finite(), false);
    assert_eq!(n.is_infinite(), false);
    assert_eq!(n.is_sign_positive(), true);
    assert_eq!(n.is_sign_negative(), false);

    assert_eq!(p.is_nan(), false);
    assert_eq!(p.is_normal(), false);
    assert_eq!(p.is_subnormal(), false);
    assert_eq!(p.is_finite(), false);
    assert_eq!(p.is_infinite(), true);
    assert_eq!(p.is_sign_positive(), true);
    assert_eq!(p.is_sign_negative(), false);

    assert_eq!(g.is_nan(), false);
    assert_eq!(g.is_normal(), false);
    assert_eq!(g.is_subnormal(), false);
    assert_eq!(g.is_finite(), false);
    assert_eq!(g.is_infinite(), true);
    assert_eq!(g.is_sign_positive(), false);
    assert_eq!(g.is_sign_negative(), true);

    for x in [a, b, s, z, n, p, g] {
        let y: f64 = x.into();

        assert_eq!(x.is_nan(), y.is_nan());
        assert_eq!(x.is_normal(), y.is_normal());
        assert_eq!(x.is_subnormal(), y.is_subnormal());
        assert_eq!(x.is_finite(), y.is_finite());
        assert_eq!(x.is_infinite(), y.is_infinite());
        assert_eq!(x.is_sign_positive(), y.is_sign_positive());
        assert_eq!(x.is_sign_negative(), y.is_sign_negative());
    }
}
