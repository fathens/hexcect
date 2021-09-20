use std::fmt::Display;
use std::lazy::Lazy;
use std::marker::PhantomData;
use std::ops::Add;

pub trait CalcMix {
    const UNIT_NAME: Lazy<String>;
    type Basis;
    fn scalar_value(&self) -> Self::Basis;
}

// ================================================================

#[derive(Clone, Copy)]
pub struct UnitsMul<V: Copy, A, B>(V, PhantomData<A>, PhantomData<B>);

impl<V: Copy, A, B> From<V> for UnitsMul<V, A, B> {
    fn from(a: V) -> Self {
        Self(a, PhantomData, PhantomData)
    }
}

impl<V: Copy, A, B> CalcMix for UnitsMul<V, A, B>
where
    A: CalcMix,
    B: CalcMix,
{
    const UNIT_NAME: Lazy<String> = Lazy::new(|| format!("{}{}", *A::UNIT_NAME, *B::UNIT_NAME));
    type Basis = V;
    fn scalar_value(&self) -> Self::Basis {
        self.0
    }
}

// ----------------------------------------------------------------

impl<V: Copy, A, B> Display for UnitsMul<V, A, B>
where
    A: CalcMix,
    B: CalcMix,
    V: Display,
    Self: CalcMix<Basis = V>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = self.scalar_value();
        write!(f, "{}{}", v, *Self::UNIT_NAME)
    }
}

impl<V: Copy, A, B> Add for UnitsMul<V, A, B>
where
    A: CalcMix,
    B: CalcMix,
    V: Add,
    V: 'static,
    Self: From<<V as Add>::Output>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let x: V = self.scalar_value();
        let y: V = rhs.scalar_value();
        (x + y).into()
    }
}

// ================================================================

#[derive(Clone, Copy)]
pub struct UnitsDiv<V: Copy, A, B>(V, PhantomData<A>, PhantomData<B>);

impl<V: Copy, A, B> From<V> for UnitsDiv<V, A, B> {
    fn from(a: V) -> Self {
        Self(a, PhantomData, PhantomData)
    }
}

impl<V: Copy, A, B> CalcMix for UnitsDiv<V, A, B>
where
    A: CalcMix,
    B: CalcMix,
{
    const UNIT_NAME: Lazy<String> = Lazy::new(|| format!("{}/{}", *A::UNIT_NAME, *B::UNIT_NAME));
    type Basis = V;
    fn scalar_value(&self) -> Self::Basis {
        self.0
    }
}

// ----------------------------------------------------------------

impl<V: Copy, A, B> Display for UnitsDiv<V, A, B>
where
    A: CalcMix,
    B: CalcMix,
    V: Display,
    Self: CalcMix<Basis = V>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = self.scalar_value();
        write!(f, "{}{}", v, *Self::UNIT_NAME)
    }
}

impl<V: Copy, A, B> Add for UnitsDiv<V, A, B>
where
    A: CalcMix,
    B: CalcMix,
    V: Add,
    V: 'static,
    Self: From<<V as Add>::Output>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let x: V = self.scalar_value();
        let y: V = rhs.scalar_value();
        (x + y).into()
    }
}

// ================================================================
// ================================================================

#[cfg(test)]
mod tests {
    use super::*;

    pub struct Meter(f64);
    impl CalcMix for Meter {
        const UNIT_NAME: Lazy<String> = Lazy::new(|| "m".to_string());
        type Basis = f64;
        fn scalar_value(&self) -> Self::Basis {
            self.0
        }
    }

    pub struct Second(f64);
    impl CalcMix for Second {
        const UNIT_NAME: Lazy<String> = Lazy::new(|| "s".to_string());
        type Basis = f64;
        fn scalar_value(&self) -> Self::Basis {
            self.0
        }
    }

    #[test]
    fn simple_add() {
        let a = UnitsDiv::<f64, Meter, Second>::from(1.2);
        let b = UnitsDiv::<f64, Meter, Second>::from(3.4);
        let c = a + b;
        assert_eq!(c.scalar_value(), 4.6);
        assert_eq!(c.to_string(), "4.6m/s");
    }
}
