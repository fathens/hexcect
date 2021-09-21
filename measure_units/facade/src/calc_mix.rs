use std::fmt::Display;
use std::lazy::Lazy;
use std::marker::PhantomData;
use std::ops::{Add, Div, Mul, Sub};

#[macro_use]
mod local_macro {
    macro_rules! impl_froms {
        ($t:ident<$ga:ident, $($g:ident),+> -> $vs:tt) => {
            impl_froms!(@inner $t ($ga $(,$g)*) $vs);
        };
        ($t:ident<$ga:ident> -> $vs:tt) => {
            impl_froms!(@inner $t ($ga) $vs);
        };
        ($t:ident -> $vs:tt) => {
            impl_froms!(@inner $t () $vs);
        };
        (@inner $t:ident $gs:tt ($($v:ident),+)) => {
            impl_froms!(@from $t $gs);
            $(impl_froms!(@into $t $v $gs);)*
        };
        (@from $t:ident ($($g:ident),*)) => {
            impl<V$(, $g)*> From<V> for $t<V$(, $g)*> {
                fn from(v: V) -> Self {
                    Self(v, $(PhantomData::<$g>,)*)
                }
            }
        };
        (@into $t:ident $v:ident ($($g:ident),*)) => {
            impl<$($g, )*> From<$t<$v, $($g,)*>> for $v {
                fn from(a: $t<$v, $($g,)*>) -> Self {
                    a.0
                }
            }
        };
    }

    macro_rules! impl_calcs {
        ($t:ident<$($g:ident),*>, $u:expr) => {
            impl_calcs!(@inner $t, $u $(,$g)*);
        };
        ($t:ident, $u:expr) => {
            impl_calcs!(@inner $t, $u, );
        };
        (@inner $t:ident, $u:expr, $($g:ident),*) => {
            impl<V$(, $g)*> CalcMix<V> for $t<V$(, $g)*>
            where
                $($g: CalcMix<V>,)*
            {
                fn unit_name() -> Lazy<String> {
                    Lazy::new(|| $u)
                }
            }

            impl<V$(, $g)*> Display for $t<V$(, $g)*>
            where
                V: Display,
                V: From<Self>,
                Self: Copy,
                Self: CalcMix<V>,
            {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    let v: V = (*self).into();
                    write!(f, "{}{}", v, *Self::unit_name())
                }
            }

            impl<V$(, $g)*, O> Add<O> for $t<V$(, $g)*>
            where
                V: Add,
                Self: Into<V>,
                Self: From<V::Output>,
                Self: CalcMix<V>,
                O: Into<Self>,
            {
                type Output = Self;

                fn add(self, rhs: O) -> Self::Output {
                    self.calc_add(rhs.into())
                }
            }

            impl<V$(, $g)*, O> Sub<O> for $t<V$(, $g)*>
            where
                V: Sub,
                Self: Into<V>,
                Self: From<V::Output>,
                Self: CalcMix<V>,
                O: Into<Self>,
            {
                type Output = Self;

                fn sub(self, rhs: O) -> Self::Output {
                    self.calc_sub(rhs.into())
                }
            }

            impl<V$(, $g)*, O> Mul<O> for $t<V$(, $g)*>
            where
                V: Mul,
                O: Into<V>,
                Self: Into<V>,
                Self: CalcMix<V>,
                UnitsMul<V, Self, O>: From<V::Output>,
            {
                type Output = UnitsMul<V, Self, O>;

                fn mul(self, rhs: O) -> Self::Output {
                    self.calc_mul(rhs)
                }
            }

            impl<V$(, $g)*, O> Div<O> for $t<V$(, $g)*>
            where
                V: Div,
                O: Into<V>,
                Self: Into<V>,
                Self: CalcMix<V>,
                UnitsDiv<V, Self, O>: From<V::Output>,
            {
                type Output = UnitsDiv<V, Self, O>;

                fn div(self, rhs: O) -> Self::Output {
                    self.calc_div(rhs)
                }
            }
        };
    }

    macro_rules! impl_calcmix {
        ($t:ident<$ga:ident, $($g:ident),+>, $vs:tt, $u:expr) => {
            impl_froms!($t<$ga $(,$g)*> -> $vs);
            impl_calcs!($t<$ga $(,$g)*>, $u);
        };
        ($t:ident<$ga:ident>, $vs:tt, $u:expr) => {
            impl_froms!($t<$ga> -> $vs);
            impl_calcs!($t<$ga>, $u);
        };
        ($t:ident, $vs:tt, $u:expr) => {
            impl_froms!($t -> $vs);
            impl_calcs!($t, $u);
        };
    }
}

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

#[derive(Clone, Copy)]
pub struct Scalar<V>(V);
impl_calcmix!(Scalar, (f32, f64, i32, i64), "scalar_value".to_string());

#[derive(Clone, Copy)]
pub struct UnitsMul<V, A, B>(V, PhantomData<A>, PhantomData<B>);
impl_calcmix!(
    UnitsMul<A, B>,
    (f32, f64, i32, i64),
    format!("{}{}", *A::unit_name(), *B::unit_name())
);

#[derive(Clone, Copy)]
pub struct UnitsDiv<V, A, B>(V, PhantomData<A>, PhantomData<B>);
impl_calcmix!(
    UnitsDiv<A, B>,
    (f32, f64, i32, i64),
    format!("{}/{}", *A::unit_name(), *B::unit_name())
);

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
