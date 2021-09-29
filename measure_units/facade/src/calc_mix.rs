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

#[derive(Debug, Clone, Copy, CalcMix)]
#[calcmix(unit_name = "".to_string())]
pub struct Scalar<V>(V);

#[derive(Debug, Clone, Copy, CalcMix)]
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

#[derive(Debug, Clone, Copy, CalcMix)]
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

#[cfg(test)]
mod tests;
