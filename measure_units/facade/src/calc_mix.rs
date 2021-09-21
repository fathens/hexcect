use std::fmt::Display;
use std::lazy::Lazy;
use std::marker::PhantomData;
use std::ops::{Add, Sub};

#[macro_use]
mod local_macro {
    macro_rules! impl_into {
        ($t:ident, $($v:ident),+) => {
            $(
                impl<A, B> Into<$v> for $t<$v, A, B> {
                    fn into(self) -> $v {
                        self.0
                    }
                }
            )*
        };
    }
}

pub trait CalcMix<V> {
    const UNIT_NAME: Lazy<String>;
}

// ================================================================

#[derive(Clone, Copy)]
pub struct UnitsMul<V, A, B>(V, PhantomData<A>, PhantomData<B>);

impl_into!(UnitsMul, f32, f64, i32, i64);

impl<V, A, B> From<V> for UnitsMul<V, A, B> {
    fn from(a: V) -> Self {
        Self(a, PhantomData, PhantomData)
    }
}

impl<V, A, B> CalcMix<V> for UnitsMul<V, A, B>
where
    A: CalcMix<V>,
    B: CalcMix<V>,
{
    const UNIT_NAME: Lazy<String> = Lazy::new(|| format!("{}{}", *A::UNIT_NAME, *B::UNIT_NAME));
}

// ----------------------------------------------------------------

impl<V, A, B> Display for UnitsMul<V, A, B>
where
    V: Display,
    Self: Copy,
    Self: Into<V>,
    Self: CalcMix<V>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v: V = (*self).into();
        write!(f, "{}{}", v, *Self::UNIT_NAME)
    }
}

impl<V, A, B> Add for UnitsMul<V, A, B>
where
    V: Add,
    Self: Into<V>,
    Self: From<V::Output>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let x: V = self.into();
        let y: V = rhs.into();
        (x + y).into()
    }
}

impl<V, A, B> Sub for UnitsMul<V, A, B>
where
    V: Sub,
    Self: Into<V>,
    Self: From<V::Output>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let x: V = self.into();
        let y: V = rhs.into();
        (x - y).into()
    }
}

// ================================================================

#[derive(Clone, Copy)]
pub struct UnitsDiv<V, A, B>(V, PhantomData<A>, PhantomData<B>);

impl_into!(UnitsDiv, f32, f64, i32, i64);

impl<V, A, B> From<V> for UnitsDiv<V, A, B> {
    fn from(a: V) -> Self {
        Self(a, PhantomData, PhantomData)
    }
}

impl<V, A, B> CalcMix<V> for UnitsDiv<V, A, B>
where
    A: CalcMix<V>,
    B: CalcMix<V>,
{
    const UNIT_NAME: Lazy<String> = Lazy::new(|| format!("{}/{}", *A::UNIT_NAME, *B::UNIT_NAME));
}

// ----------------------------------------------------------------

impl<V, A, B> Display for UnitsDiv<V, A, B>
where
    V: Display,
    Self: Copy,
    Self: Into<V>,
    Self: CalcMix<V>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v: V = (*self).into();
        write!(f, "{}{}", v, *Self::UNIT_NAME)
    }
}

impl<V, A, B> Add for UnitsDiv<V, A, B>
where
    V: Add,
    Self: Into<V>,
    Self: From<V::Output>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let x: V = self.into();
        let y: V = rhs.into();
        (x + y).into()
    }
}

impl<V, A, B> Sub for UnitsDiv<V, A, B>
where
    V: Sub,
    Self: Into<V>,
    Self: From<V::Output>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let x: V = self.into();
        let y: V = rhs.into();
        (x - y).into()
    }
}

// ================================================================
// ================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use derive_more::Into;

    #[derive(Clone, Copy, Into)]
    pub struct Meter(f64);
    impl CalcMix<f64> for Meter {
        const UNIT_NAME: Lazy<String> = Lazy::new(|| "m".to_string());
    }

    #[derive(Clone, Copy, Into)]
    pub struct Second(f64);
    impl CalcMix<f64> for Second {
        const UNIT_NAME: Lazy<String> = Lazy::new(|| "s".to_string());
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
