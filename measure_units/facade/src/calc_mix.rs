use measure_units_derive::*;

use std::lazy::Lazy;
use std::marker::PhantomData;
use std::ops::{Add, Div, Mul, Sub};

#[macro_use]
mod local_macro {}

// ================================================================

pub trait CalcMix<V> {
    fn unit_name() -> Lazy<String>;

    fn calc_add(self, o: Self) -> Self
    where
        V: Add,
        Self: Into<V>,
        Self: From<V::Output>,
    {
        let x: V = self.into();
        let y: V = o.into();
        (x + y).into()
    }

    fn calc_sub(self, o: Self) -> Self
    where
        V: Sub,
        Self: Into<V>,
        Self: From<V::Output>,
    {
        let x: V = self.into();
        let y: V = o.into();
        (x - y).into()
    }

    fn calc_mul<O>(self, o: O) -> UnitsMul<V, Self, O>
    where
        V: Mul,
        O: Into<V>,
        Self: Into<V>,
        UnitsMul<V, Self, O>: From<V::Output>,
    {
        let x: V = self.into();
        let y: V = o.into();
        (x * y).into()
    }

    fn calc_div<O>(self, o: O) -> UnitsDiv<V, Self, O>
    where
        V: Div,
        O: Into<V>,
        Self: Into<V>,
        UnitsDiv<V, Self, O>: From<V::Output>,
    {
        let x: V = self.into();
        let y: V = o.into();
        (x / y).into()
    }
}

// ================================================================

#[derive(Clone, Copy, CalcMix)]
#[calcmix(unit_name = "scalar_value".to_string())]
pub struct Scalar<V>(V);

#[derive(Clone, Copy, CalcMix)]
#[calcmix(unit_name = format!("{}{}", *A::unit_name(), *B::unit_name()))]
pub struct UnitsMul<V, A, B>(V, PhantomData<A>, PhantomData<B>);

#[derive(Clone, Copy, CalcMix)]
#[calcmix(unit_name = format!("{}/{}", *A::unit_name(), *B::unit_name()))]
pub struct UnitsDiv<V, A, B>(V, PhantomData<A>, PhantomData<B>);

// ================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use derive_more::Into;

    #[derive(Clone, Copy, Into)]
    pub struct Meter(f64);
    impl CalcMix<f64> for Meter {
        fn unit_name() -> Lazy<String> {
            Lazy::new(|| "m".to_string())
        }
    }

    #[derive(Clone, Copy, Into)]
    pub struct Second(f64);
    impl CalcMix<f64> for Second {
        fn unit_name() -> Lazy<String> {
            Lazy::new(|| "s".to_string())
        }
    }

    #[test]
    fn simple_add() {
        let a = UnitsDiv::<f64, Meter, Second>::from(1.2);
        let b = UnitsDiv::<f64, Meter, Second>::from(3.4);
        let c = a + b;
        assert_eq!(c.0, 4.6);
        assert_eq!(c.to_string(), "4.6m/s");
    }

    #[test]
    fn simple_sub() {
        let a = UnitsDiv::<f64, Meter, Second>::from(1.2);
        let b = UnitsDiv::<f64, Meter, Second>::from(3.5);
        let c = a - b;
        assert_eq!(c.0, -2.3);
        assert_eq!(c.to_string(), "-2.3m/s");
    }
}
