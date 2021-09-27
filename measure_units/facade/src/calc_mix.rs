use measure_units_derive::*;

use std::lazy::Lazy;
use std::marker::PhantomData;
use std::ops::{Add, Div, Mul, Sub};

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
#[calcmix(unit_name = "".to_string())]
pub struct Scalar<V>(V);

#[derive(Clone, Copy, CalcMix)]
#[calcmix(unit_name = format!("{}{}", *A::unit_name(), *B::unit_name()))]
pub struct UnitsMul<V, A, B>(V, PhantomData<A>, PhantomData<B>);

impl<V, A, B> UnitsMul<V, A, B> {
    pub fn inner_right<C>(self, f: impl Fn(B) -> C) -> UnitsMul<V, A, C>
    where
        V: Copy,
        B: From<V>,
    {
        let _c: C = f(self.0.into());
        self.0.into()
    }

    pub fn inner_left<C>(self, f: impl Fn(A) -> C) -> UnitsMul<V, C, B>
    where
        V: Copy,
        A: From<V>,
    {
        let _c: C = f(self.0.into());
        self.0.into()
    }

    /// A * B = B * A
    pub fn commutative(self) -> UnitsMul<V, B, A> {
        self.0.into()
    }
}

impl<V, A, B, C> UnitsMul<V, UnitsMul<V, A, B>, C>
where
    A: From<V>,
{
    /// (A * B) * C = A * (B * C)
    pub fn associative(self) -> UnitsMul<V, A, UnitsMul<V, B, C>> {
        self.0.into()
    }
}

impl<V, A> UnitsMul<V, A, Scalar<V>>
where
    A: From<V>,
{
    /// A * Scalar = A
    pub fn scalar(self) -> A {
        self.0.into()
    }
}

impl<V, A, B> UnitsMul<V, UnitsDiv<V, A, B>, B>
where
    A: From<V>,
{
    /// A/B * B = A
    pub fn reduction(self) -> A {
        self.0.into()
    }
}

#[derive(Clone, Copy, CalcMix)]
#[calcmix(unit_name = format!("{}/{}", *A::unit_name(), *B::unit_name()))]
pub struct UnitsDiv<V, A, B>(V, PhantomData<A>, PhantomData<B>);

impl<V, A, B> UnitsDiv<V, A, B> {
    pub fn inner_right<C>(self, f: impl Fn(B) -> C) -> UnitsDiv<V, A, C>
    where
        V: Copy,
        B: From<V>,
    {
        let _c: C = f(self.0.into());
        self.0.into()
    }

    pub fn inner_left<C>(self, f: impl Fn(A) -> C) -> UnitsDiv<V, C, B>
    where
        V: Copy,
        A: From<V>,
    {
        let _c: C = f(self.0.into());
        self.0.into()
    }
}

impl<V, A> UnitsDiv<V, A, A> {
    /// A / A = Scalar
    pub fn reduction(self) -> Scalar<V> {
        self.0.into()
    }
}

impl<V, A> UnitsDiv<V, A, Scalar<V>>
where
    A: From<V>,
{
    /// A / Scalar = A
    pub fn scalar(self) -> A {
        self.0.into()
    }
}

impl<V, A, B> UnitsDiv<V, UnitsMul<V, A, B>, B>
where
    A: From<V>,
{
    /// A * B / B = A
    pub fn reduction_right(self) -> A {
        self.0.into()
    }
}

impl<V, A, B> UnitsDiv<V, UnitsMul<V, A, B>, A>
where
    B: From<V>,
{
    /// A * B / A = B
    pub fn reduction_left(self) -> B {
        self.0.into()
    }
}

// ================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Copy, CalcMix)]
    #[calcmix(unit_name = "m".to_string())]
    pub struct Meter(f64);

    #[derive(Clone, Copy, CalcMix)]
    #[calcmix(unit_name = "s".to_string())]
    pub struct Second(f64);

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

    #[test]
    fn commutative() {
        let d = Meter::from(10.0);
        let t = Second::from(2.0);
        let g = d * t;
        assert_eq!(g.0, 20.0);
        assert_eq!(g.to_string(), "20ms");

        let r = g.commutative();
        assert_eq!(r.0, 20.0);
        assert_eq!(r.to_string(), "20sm");
    }

    #[test]
    fn associative() {
        let d = Meter::from(10.0);
        let t = Second::from(2.0);
        let o = Meter::from(3.0);
        let g: UnitsMul<f64, UnitsMul<f64, Meter, Second>, Meter> = d * t * o;
        assert_eq!(g.0, 60.0);
        assert_eq!(g.to_string(), "60msm");

        let r: UnitsMul<f64, Meter, UnitsMul<f64, Second, Meter>> = g.associative();
        assert_eq!(r.0, 60.0);
        assert_eq!(r.to_string(), "60msm");
    }

    #[test]
    fn reduction_mul() {
        let distance = Meter::from(10.0);
        let time = Second::from(2.0);
        let takes = Second::from(3.0);

        let speed = distance / time;
        let goal = speed * takes;
        assert_eq!(goal.0, 15.0);
        assert_eq!(goal.to_string(), "15m/ss");

        let s = goal.reduction();
        assert_eq!(s.0, 15.0);
        assert_eq!(s.to_string(), "15m");
    }

    #[test]
    fn reduction_div() {
        let a = Second::from(3.0);
        let b = Second::from(1.5);

        let c = a / b;
        assert_eq!(c.0, 2.0);
        assert_eq!(c.to_string(), "2s/s");

        let s = c.reduction();
        assert_eq!(s.0, 2.0);
        assert_eq!(s.to_string(), "2");
    }

    #[test]
    fn reduction_right() {
        let distance = Meter::from(10.0);
        let time = Second::from(2.0);
        let takes = Second::from(3.0);

        let a = distance * takes;
        let b = a / time;
        assert_eq!(b.0, 15.0);
        assert_eq!(b.to_string(), "15ms/s");

        let s = b.reduction_right();
        assert_eq!(s.0, 15.0);
        assert_eq!(s.to_string(), "15m");
    }

    #[test]
    fn reduction_left() {
        let distance = Meter::from(10.0);
        let time = Second::from(2.0);
        let takes = Second::from(3.0);

        let a = takes * distance;
        let b = a / time;
        assert_eq!(b.0, 15.0);
        assert_eq!(b.to_string(), "15sm/s");

        let s = b.reduction_left();
        assert_eq!(s.0, 15.0);
        assert_eq!(s.to_string(), "15m");
    }

    #[test]
    fn mul_inner_right_reduction_left() {
        let distance = Meter::from(10.0);
        let time = Second::from(2.0);
        let takes = Second::from(3.0);

        let a = takes * distance;
        let b = a / time;
        let c = Second::from(5.0) * b;
        assert_eq!(c.0, 75.0);
        assert_eq!(c.to_string(), "75ssm/s");

        let s = c.inner_right(|a| a.reduction_left());
        assert_eq!(s.0, 75.0);
        assert_eq!(s.to_string(), "75sm");
    }

    #[test]
    fn mul_inner_left_reduction_left() {
        let distance = Meter::from(10.0);
        let time = Second::from(2.0);
        let takes = Second::from(3.0);

        let a = takes * distance;
        let b = a / time;
        let c = b * Second::from(5.0);
        assert_eq!(c.0, 75.0);
        assert_eq!(c.to_string(), "75sm/ss");

        let s = c.inner_left(|a| a.reduction_left());
        assert_eq!(s.0, 75.0);
        assert_eq!(s.to_string(), "75ms");
    }

    #[test]
    fn div_inner_right_reduction_left() {
        let distance = Meter::from(10.0);
        let time = Second::from(2.0);
        let takes = Second::from(3.0);

        let a = takes * distance;
        let b = a / time;
        let c = Second::from(30.0) / b;
        assert_eq!(c.0, 2.0);
        assert_eq!(c.to_string(), "2s/sm/s");

        let s = c.inner_right(|a| a.reduction_left());
        assert_eq!(s.0, 2.0);
        assert_eq!(s.to_string(), "2s/m");
    }

    #[test]
    fn div_inner_left_reduction_left() {
        let distance = Meter::from(10.0);
        let time = Second::from(2.0);
        let takes = Second::from(3.0);

        let a = takes * distance;
        let b = a / time;
        let c = b / Second::from(5.0);
        assert_eq!(c.0, 3.0);
        assert_eq!(c.to_string(), "3sm/s/s");

        let s = c.inner_left(|a| a.reduction_left());
        assert_eq!(s.0, 3.0);
        assert_eq!(s.to_string(), "3m/s");
    }
}
