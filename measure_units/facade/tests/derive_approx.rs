use std::marker::PhantomData;

use approx::{assert_relative_eq, assert_relative_ne, assert_ulps_eq, assert_ulps_ne};
use measure_units::*;

use derive_more::{From, Into};

#[derive(Debug, Clone, Copy, From, Into, PartialEq, Approx)]
struct MyUnit(f64);

#[derive(Debug, Clone, Copy, PartialEq, Approx)]
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
fn derive_approx() {
    let a = MyUnit::from(1.2);
    let b = MyUnit::from(-0.2);
    let c = MyUnit::from(1.2);

    assert_relative_ne!(a, b);
    assert_relative_eq!(a, c);

    assert_ulps_ne!(a, b);
    assert_ulps_eq!(a, c);
}

#[test]
fn derive_approx_with_generics() {
    let a = MyGenerics::from(1.2);
    let b = MyGenerics::from(-0.2);
    let c = MyGenerics::from(1.2);

    assert_relative_ne!(a, b);
    assert_relative_eq!(a, c);

    assert_ulps_ne!(a, b);
    assert_ulps_eq!(a, c);
}
