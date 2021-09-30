use super::*;

use quote::quote;

#[test]
fn with_single() {
    let a = quote! {
        #[calcmix(unit_name = "km".to_string())]
        struct Km(f64);
    };
    let b = quote! {
        impl From<f64> for Km {
            fn from(v: f64) -> Self {
                Self(v)
            }
        }

        impl From<Km> for f64 {
            fn from(a: Km) -> Self {
                a.0
            }
        }

        impl CalcMix<f64> for Km
        where
        {
            fn unit_name() -> std::lazy::Lazy<String> {
                std::lazy::Lazy::new(|| "km".to_string())
            }
        }

        impl std::fmt::Display for Km
        where
            Self: Copy,
            Self: CalcMix<f64>,
        {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let v: f64 = (*self).into();
                write!(f, "{}{}", v, *Self::unit_name())
            }
        }

        impl<O,> std::ops::Add<O> for Km
        where
            f64: std::ops::Add,
            Self: Into<f64>,
            Self: From<<f64 as std::ops::Add>::Output>,
            Self: CalcMix<f64>,
            O: Into<Self>,
        {
            type Output = Self;
            fn add(self, rhs: O) -> Self::Output {
                self.calc_add(rhs.into())
            }
        }

        impl<O,> std::ops::Sub<O> for Km
        where
            f64: std::ops::Sub,
            Self: Into<f64>,
            Self: From<<f64 as std::ops::Sub>::Output>,
            Self: CalcMix<f64>,
            O: Into<Self>,
        {
            type Output = Self;
            fn sub(self, rhs: O) -> Self::Output {
                self.calc_sub(rhs.into())
            }
        }

        impl<O,> std::ops::Mul<O> for Km
        where
            f64: std::ops::Mul,
            O: Into<f64>,
            Self: Into<f64>,
            Self: CalcMix<f64>,
            UnitsMul<f64, Self, O>: From<<f64 as std::ops::Mul>::Output>,
        {
            type Output = UnitsMul<f64, Self, O>;
            fn mul(self, rhs: O) -> Self::Output {
                self.calc_mul(rhs)
            }
        }

        impl<O,> std::ops::Div<O> for Km
        where
            f64: std::ops::Div,
            O: Into<f64>,
            Self: Into<f64>,
            Self: CalcMix<f64>,
            UnitsDiv<f64, Self, O>: From<<f64 as std::ops::Div>::Output>,
        {
            type Output = UnitsDiv<f64, Self, O>;
            fn div(self, rhs: O) -> Self::Output {
                self.calc_div(rhs)
            }
        }
    };
    assert_eq!(derive(a).to_string(), b.to_string());
}

#[test]
fn with_generics() {
    let a = quote! {
        #[calcmix(unit_name = "m".to_string())]
        struct Meter<V>(V);
    };
    let b = quote! {
        impl<V> From<V> for Meter<V> {
            fn from(v: V) -> Self {
                Self(v)
            }
        }

        impl<> From<Meter<f32,>> for f32 {
            fn from(a: Meter<f32,>) -> Self {
                a.0
            }
        }
        impl<> From<Meter<f64,>> for f64 {
            fn from(a: Meter<f64,>) -> Self {
                a.0
            }
        }

        impl<> From<Meter<i32,>> for i32 {
            fn from(a: Meter<i32,>) -> Self {
                a.0
            }
        }
        impl<> From<Meter<i64,>> for i64 {
            fn from(a: Meter<i64,>) -> Self {
                a.0
            }
        }

        impl<V> CalcMix<V> for Meter<V>
        where
        {
            fn unit_name() -> std::lazy::Lazy<String> {
                std::lazy::Lazy::new(|| "m".to_string())
            }
        }

        impl<V> std::fmt::Display for Meter<V>
        where
            V: std::fmt::Display,
            V: From<Self>,
            Self: Copy,
            Self: CalcMix<V>,
        {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let v: V = (*self).into();
                write!(f, "{}{}", v, *Self::unit_name())
            }
        }

        impl<O, V> std::ops::Add<O> for Meter<V>
        where
            V: std::ops::Add,
            Self: Into<V>,
            Self: From<<V as std::ops::Add>::Output>,
            Self: CalcMix<V>,
            O: Into<Self>,
        {
            type Output = Self;
            fn add(self, rhs: O) -> Self::Output {
                self.calc_add(rhs.into())
            }
        }

        impl<O, V> std::ops::Sub<O> for Meter<V>
        where
            V: std::ops::Sub,
            Self: Into<V>,
            Self: From<<V as std::ops::Sub>::Output>,
            Self: CalcMix<V>,
            O: Into<Self>,
        {
            type Output = Self;
            fn sub(self, rhs: O) -> Self::Output {
                self.calc_sub(rhs.into())
            }
        }

        impl<O, V> std::ops::Mul<O> for Meter<V>
        where
            V: std::ops::Mul,
            O: Into<V>,
            Self: Into<V>,
            Self: CalcMix<V>,
            UnitsMul<V, Self, O>: From<<V as std::ops::Mul>::Output>,
        {
            type Output = UnitsMul<V, Self, O>;
            fn mul(self, rhs: O) -> Self::Output {
                self.calc_mul(rhs)
            }
        }

        impl<O, V> std::ops::Div<O> for Meter<V>
        where
            V: std::ops::Div,
            O: Into<V>,
            Self: Into<V>,
            Self: CalcMix<V>,
            UnitsDiv<V, Self, O>: From<<V as std::ops::Div>::Output>,
        {
            type Output = UnitsDiv<V, Self, O>;
            fn div(self, rhs: O) -> Self::Output {
                self.calc_div(rhs)
            }
        }
    };
    assert_eq!(derive(a).to_string(), b.to_string());
}

#[test]
fn with_generics_bound() {
    let a = quote! {
        #[calcmix(unit_name = "m".to_string())]
        struct Meter<V: FloatConst>(V);
    };
    let b = quote! {
        impl<V: FloatConst> From<V> for Meter<V> {
            fn from(v: V) -> Self {
                Self(v)
            }
        }

        impl<> From<Meter<f32,>> for f32 {
            fn from(a: Meter<f32,>) -> Self {
                a.0
            }
        }
        impl<> From<Meter<f64,>> for f64 {
            fn from(a: Meter<f64,>) -> Self {
                a.0
            }
        }

        impl<> From<Meter<i32,>> for i32 {
            fn from(a: Meter<i32,>) -> Self {
                a.0
            }
        }
        impl<> From<Meter<i64,>> for i64 {
            fn from(a: Meter<i64,>) -> Self {
                a.0
            }
        }

        impl<V: FloatConst> CalcMix<V> for Meter<V>
        where
        {
            fn unit_name() -> std::lazy::Lazy<String> {
                std::lazy::Lazy::new(|| "m".to_string())
            }
        }

        impl<V: FloatConst> std::fmt::Display for Meter<V>
        where
            V: std::fmt::Display,
            V: From<Self>,
            Self: Copy,
            Self: CalcMix<V>,
        {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let v: V = (*self).into();
                write!(f, "{}{}", v, *Self::unit_name())
            }
        }

        impl<O, V: FloatConst> std::ops::Add<O> for Meter<V>
        where
            V: std::ops::Add,
            Self: Into<V>,
            Self: From<<V as std::ops::Add>::Output>,
            Self: CalcMix<V>,
            O: Into<Self>,
        {
            type Output = Self;
            fn add(self, rhs: O) -> Self::Output {
                self.calc_add(rhs.into())
            }
        }

        impl<O, V: FloatConst> std::ops::Sub<O> for Meter<V>
        where
            V: std::ops::Sub,
            Self: Into<V>,
            Self: From<<V as std::ops::Sub>::Output>,
            Self: CalcMix<V>,
            O: Into<Self>,
        {
            type Output = Self;
            fn sub(self, rhs: O) -> Self::Output {
                self.calc_sub(rhs.into())
            }
        }

        impl<O, V: FloatConst> std::ops::Mul<O> for Meter<V>
        where
            V: std::ops::Mul,
            O: Into<V>,
            Self: Into<V>,
            Self: CalcMix<V>,
            UnitsMul<V, Self, O>: From<<V as std::ops::Mul>::Output>,
        {
            type Output = UnitsMul<V, Self, O>;
            fn mul(self, rhs: O) -> Self::Output {
                self.calc_mul(rhs)
            }
        }

        impl<O, V: FloatConst> std::ops::Div<O> for Meter<V>
        where
            V: std::ops::Div,
            O: Into<V>,
            Self: Into<V>,
            Self: CalcMix<V>,
            UnitsDiv<V, Self, O>: From<<V as std::ops::Div>::Output>,
        {
            type Output = UnitsDiv<V, Self, O>;
            fn div(self, rhs: O) -> Self::Output {
                self.calc_div(rhs)
            }
        }
    };
    assert_eq!(derive(a).to_string(), b.to_string());
}

#[test]
fn with_generics_into() {
    let a = quote! {
        #[calcmix(into = [f32, i32], unit_name = "m".to_string())]
        struct Meter<V>(V);
    };
    let b = quote! {
        impl<V> From<V> for Meter<V> {
            fn from(v: V) -> Self {
                Self(v)
            }
        }

        impl<> From<Meter<f32,>> for f32 {
            fn from(a: Meter<f32,>) -> Self {
                a.0
            }
        }
        impl<> From<Meter<i32,>> for i32 {
            fn from(a: Meter<i32,>) -> Self {
                a.0
            }
        }

        impl<V> CalcMix<V> for Meter<V>
        where
        {
            fn unit_name() -> std::lazy::Lazy<String> {
                std::lazy::Lazy::new(|| "m".to_string())
            }
        }

        impl<V> std::fmt::Display for Meter<V>
        where
            V: std::fmt::Display,
            V: From<Self>,
            Self: Copy,
            Self: CalcMix<V>,
        {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let v: V = (*self).into();
                write!(f, "{}{}", v, *Self::unit_name())
            }
        }

        impl<O, V> std::ops::Add<O> for Meter<V>
        where
            V: std::ops::Add,
            Self: Into<V>,
            Self: From<<V as std::ops::Add>::Output>,
            Self: CalcMix<V>,
            O: Into<Self>,
        {
            type Output = Self;
            fn add(self, rhs: O) -> Self::Output {
                self.calc_add(rhs.into())
            }
        }

        impl<O, V> std::ops::Sub<O> for Meter<V>
        where
            V: std::ops::Sub,
            Self: Into<V>,
            Self: From<<V as std::ops::Sub>::Output>,
            Self: CalcMix<V>,
            O: Into<Self>,
        {
            type Output = Self;
            fn sub(self, rhs: O) -> Self::Output {
                self.calc_sub(rhs.into())
            }
        }

        impl<O, V> std::ops::Mul<O> for Meter<V>
        where
            V: std::ops::Mul,
            O: Into<V>,
            Self: Into<V>,
            Self: CalcMix<V>,
            UnitsMul<V, Self, O>: From<<V as std::ops::Mul>::Output>,
        {
            type Output = UnitsMul<V, Self, O>;
            fn mul(self, rhs: O) -> Self::Output {
                self.calc_mul(rhs)
            }
        }

        impl<O, V> std::ops::Div<O> for Meter<V>
        where
            V: std::ops::Div,
            O: Into<V>,
            Self: Into<V>,
            Self: CalcMix<V>,
            UnitsDiv<V, Self, O>: From<<V as std::ops::Div>::Output>,
        {
            type Output = UnitsDiv<V, Self, O>;
            fn div(self, rhs: O) -> Self::Output {
                self.calc_div(rhs)
            }
        }
    };
    assert_eq!(derive(a).to_string(), b.to_string());
}

#[test]
fn with_mix() {
    let a = quote! {
        #[calcmix(into=[f32, f64], unit_name = format!("{}/{}", *A::unit_name(), *B::unit_name()))]
        struct UnitsDiv<V, A, B>(V, PhantomData<A>, PhantomData<B>);
    };
    let b = quote! {
        impl<V, A, B> From<V> for UnitsDiv<V, A, B> {
            fn from(v: V) -> Self {
                Self(v, std::marker::PhantomData::<A>, std::marker::PhantomData::<B>)
            }
        }

        impl<A, B,> From<UnitsDiv<f32, A, B,>> for f32 {
            fn from(a: UnitsDiv<f32, A, B,>) -> Self {
                a.0
            }
        }
        impl<A, B,> From<UnitsDiv<f64, A, B,>> for f64 {
            fn from(a: UnitsDiv<f64, A, B,>) -> Self {
                a.0
            }
        }

        impl<V, A, B> CalcMix<V> for UnitsDiv<V, A, B>
        where
            A: CalcMix<V>,
            B: CalcMix<V>,
        {
            fn unit_name() -> std::lazy::Lazy<String> {
                std::lazy::Lazy::new(|| format!("{}/{}", *A::unit_name(), *B::unit_name()))
            }
        }

        impl<V, A, B> std::fmt::Display for UnitsDiv<V, A, B>
        where
            V: std::fmt::Display,
            V: From<Self>,
            Self: Copy,
            Self: CalcMix<V>,
        {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let v: V = (*self).into();
                write!(f, "{}{}", v, *Self::unit_name())
            }
        }

        impl<O, V, A, B> std::ops::Add<O> for UnitsDiv<V, A, B>
        where
            V: std::ops::Add,
            Self: Into<V>,
            Self: From<<V as std::ops::Add>::Output>,
            Self: CalcMix<V>,
            O: Into<Self>,
        {
            type Output = Self;
            fn add(self, rhs: O) -> Self::Output {
                self.calc_add(rhs.into())
            }
        }

        impl<O, V, A, B> std::ops::Sub<O> for UnitsDiv<V, A, B>
        where
            V: std::ops::Sub,
            Self: Into<V>,
            Self: From<<V as std::ops::Sub>::Output>,
            Self: CalcMix<V>,
            O: Into<Self>,
        {
            type Output = Self;
            fn sub(self, rhs: O) -> Self::Output {
                self.calc_sub(rhs.into())
            }
        }

        impl<O, V, A, B> std::ops::Mul<O> for UnitsDiv<V, A, B>
        where
            V: std::ops::Mul,
            O: Into<V>,
            Self: Into<V>,
            Self: CalcMix<V>,
            UnitsMul<V, Self, O>: From<<V as std::ops::Mul>::Output>,
        {
            type Output = UnitsMul<V, Self, O>;
            fn mul(self, rhs: O) -> Self::Output {
                self.calc_mul(rhs)
            }
        }

        impl<O, V, A, B> std::ops::Div<O> for UnitsDiv<V, A, B>
        where
            V: std::ops::Div,
            O: Into<V>,
            Self: Into<V>,
            Self: CalcMix<V>,
            UnitsDiv<V, Self, O>: From<<V as std::ops::Div>::Output>,
        {
            type Output = UnitsDiv<V, Self, O>;
            fn div(self, rhs: O) -> Self::Output {
                self.calc_div(rhs)
            }
        }
    };
    assert_eq!(derive(a).to_string(), b.to_string());
}

#[test]
fn with_mix_conc1() {
    let a = quote! {
        #[calcmix(into=[f32, f64], unit_name = format!("{}!", *A::unit_name()))]
        struct UnitsPat<V, A>(V, PhantomData<A>, PhantomData<char>);
    };
    let b = quote! {
        impl<V, A> From<V> for UnitsPat<V, A> {
            fn from(v: V) -> Self {
                Self(v, std::marker::PhantomData::<A>, std::marker::PhantomData::<char>)
            }
        }

        impl<A,> From<UnitsPat<f32, A,>> for f32 {
            fn from(a: UnitsPat<f32, A,>) -> Self {
                a.0
            }
        }
        impl<A,> From<UnitsPat<f64, A,>> for f64 {
            fn from(a: UnitsPat<f64, A,>) -> Self {
                a.0
            }
        }

        impl<V, A> CalcMix<V> for UnitsPat<V, A>
        where
            A: CalcMix<V>,
        {
            fn unit_name() -> std::lazy::Lazy<String> {
                std::lazy::Lazy::new(|| format!("{}!", *A::unit_name()))
            }
        }

        impl<V, A> std::fmt::Display for UnitsPat<V, A>
        where
            V: std::fmt::Display,
            V: From<Self>,
            Self: Copy,
            Self: CalcMix<V>,
        {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let v: V = (*self).into();
                write!(f, "{}{}", v, *Self::unit_name())
            }
        }

        impl<O, V, A> std::ops::Add<O> for UnitsPat<V, A>
        where
            V: std::ops::Add,
            Self: Into<V>,
            Self: From<<V as std::ops::Add>::Output>,
            Self: CalcMix<V>,
            O: Into<Self>,
        {
            type Output = Self;
            fn add(self, rhs: O) -> Self::Output {
                self.calc_add(rhs.into())
            }
        }

        impl<O, V, A> std::ops::Sub<O> for UnitsPat<V, A>
        where
            V: std::ops::Sub,
            Self: Into<V>,
            Self: From<<V as std::ops::Sub>::Output>,
            Self: CalcMix<V>,
            O: Into<Self>,
        {
            type Output = Self;
            fn sub(self, rhs: O) -> Self::Output {
                self.calc_sub(rhs.into())
            }
        }

        impl<O, V, A> std::ops::Mul<O> for UnitsPat<V, A>
        where
            V: std::ops::Mul,
            O: Into<V>,
            Self: Into<V>,
            Self: CalcMix<V>,
            UnitsMul<V, Self, O>: From<<V as std::ops::Mul>::Output>,
        {
            type Output = UnitsMul<V, Self, O>;
            fn mul(self, rhs: O) -> Self::Output {
                self.calc_mul(rhs)
            }
        }

        impl<O, V, A> std::ops::Div<O> for UnitsPat<V, A>
        where
            V: std::ops::Div,
            O: Into<V>,
            Self: Into<V>,
            Self: CalcMix<V>,
            UnitsDiv<V, Self, O>: From<<V as std::ops::Div>::Output>,
        {
            type Output = UnitsDiv<V, Self, O>;
            fn div(self, rhs: O) -> Self::Output {
                self.calc_div(rhs)
            }
        }
    };
    assert_eq!(derive(a).to_string(), b.to_string());
}

#[test]
fn with_mix_conc2() {
    let a = quote! {
        #[calcmix(into=[f32, f64], unit_name = format!("{}${}", *A::unit_name(), *B::unit_name()))]
        struct UnitsPat<V, A, B>(V, PhantomData<A>, PhantomData<B>, PhantomData<char>);
    };
    let b = quote! {
        impl<V, A, B> From<V> for UnitsPat<V, A, B> {
            fn from(v: V) -> Self {
                Self(v, std::marker::PhantomData::<A>, std::marker::PhantomData::<B>, std::marker::PhantomData::<char>)
            }
        }

        impl<A, B,> From<UnitsPat<f32, A, B,>> for f32 {
            fn from(a: UnitsPat<f32, A, B,>) -> Self {
                a.0
            }
        }
        impl<A, B,> From<UnitsPat<f64, A, B,>> for f64 {
            fn from(a: UnitsPat<f64, A, B,>) -> Self {
                a.0
            }
        }

        impl<V, A, B> CalcMix<V> for UnitsPat<V, A, B>
        where
            A: CalcMix<V>,
            B: CalcMix<V>,
        {
            fn unit_name() -> std::lazy::Lazy<String> {
                std::lazy::Lazy::new(|| format!("{}${}", *A::unit_name(), *B::unit_name()))
            }
        }

        impl<V, A, B> std::fmt::Display for UnitsPat<V, A, B>
        where
            V: std::fmt::Display,
            V: From<Self>,
            Self: Copy,
            Self: CalcMix<V>,
        {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let v: V = (*self).into();
                write!(f, "{}{}", v, *Self::unit_name())
            }
        }

        impl<O, V, A, B> std::ops::Add<O> for UnitsPat<V, A, B>
        where
            V: std::ops::Add,
            Self: Into<V>,
            Self: From<<V as std::ops::Add>::Output>,
            Self: CalcMix<V>,
            O: Into<Self>,
        {
            type Output = Self;
            fn add(self, rhs: O) -> Self::Output {
                self.calc_add(rhs.into())
            }
        }

        impl<O, V, A, B> std::ops::Sub<O> for UnitsPat<V, A, B>
        where
            V: std::ops::Sub,
            Self: Into<V>,
            Self: From<<V as std::ops::Sub>::Output>,
            Self: CalcMix<V>,
            O: Into<Self>,
        {
            type Output = Self;
            fn sub(self, rhs: O) -> Self::Output {
                self.calc_sub(rhs.into())
            }
        }

        impl<O, V, A, B> std::ops::Mul<O> for UnitsPat<V, A, B>
        where
            V: std::ops::Mul,
            O: Into<V>,
            Self: Into<V>,
            Self: CalcMix<V>,
            UnitsMul<V, Self, O>: From<<V as std::ops::Mul>::Output>,
        {
            type Output = UnitsMul<V, Self, O>;
            fn mul(self, rhs: O) -> Self::Output {
                self.calc_mul(rhs)
            }
        }

        impl<O, V, A, B> std::ops::Div<O> for UnitsPat<V, A, B>
        where
            V: std::ops::Div,
            O: Into<V>,
            Self: Into<V>,
            Self: CalcMix<V>,
            UnitsDiv<V, Self, O>: From<<V as std::ops::Div>::Output>,
        {
            type Output = UnitsDiv<V, Self, O>;
            fn div(self, rhs: O) -> Self::Output {
                self.calc_div(rhs)
            }
        }
    };
    assert_eq!(derive(a).to_string(), b.to_string());
}

#[test]
fn with_mix_conc2_bound() {
    let a = quote! {
        #[calcmix(into=[f32, f64], unit_name = format!("{}${}", *A::unit_name(), *B::unit_name()))]
        struct UnitsPat<V: Float, A, B>(V, PhantomData<A>, PhantomData<B>, PhantomData<char>);
    };
    let b = quote! {
        impl<V: Float, A, B> From<V> for UnitsPat<V, A, B> {
            fn from(v: V) -> Self {
                Self(v, std::marker::PhantomData::<A>, std::marker::PhantomData::<B>, std::marker::PhantomData::<char>)
            }
        }

        impl<A, B,> From<UnitsPat<f32, A, B,>> for f32 {
            fn from(a: UnitsPat<f32, A, B,>) -> Self {
                a.0
            }
        }
        impl<A, B,> From<UnitsPat<f64, A, B,>> for f64 {
            fn from(a: UnitsPat<f64, A, B,>) -> Self {
                a.0
            }
        }

        impl<V: Float, A, B> CalcMix<V> for UnitsPat<V, A, B>
        where
            A: CalcMix<V>,
            B: CalcMix<V>,
        {
            fn unit_name() -> std::lazy::Lazy<String> {
                std::lazy::Lazy::new(|| format!("{}${}", *A::unit_name(), *B::unit_name()))
            }
        }

        impl<V: Float, A, B> std::fmt::Display for UnitsPat<V, A, B>
        where
            V: std::fmt::Display,
            V: From<Self>,
            Self: Copy,
            Self: CalcMix<V>,
        {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let v: V = (*self).into();
                write!(f, "{}{}", v, *Self::unit_name())
            }
        }

        impl<O, V: Float, A, B> std::ops::Add<O> for UnitsPat<V, A, B>
        where
            V: std::ops::Add,
            Self: Into<V>,
            Self: From<<V as std::ops::Add>::Output>,
            Self: CalcMix<V>,
            O: Into<Self>,
        {
            type Output = Self;
            fn add(self, rhs: O) -> Self::Output {
                self.calc_add(rhs.into())
            }
        }

        impl<O, V: Float, A, B> std::ops::Sub<O> for UnitsPat<V, A, B>
        where
            V: std::ops::Sub,
            Self: Into<V>,
            Self: From<<V as std::ops::Sub>::Output>,
            Self: CalcMix<V>,
            O: Into<Self>,
        {
            type Output = Self;
            fn sub(self, rhs: O) -> Self::Output {
                self.calc_sub(rhs.into())
            }
        }

        impl<O, V: Float, A, B> std::ops::Mul<O> for UnitsPat<V, A, B>
        where
            V: std::ops::Mul,
            O: Into<V>,
            Self: Into<V>,
            Self: CalcMix<V>,
            UnitsMul<V, Self, O>: From<<V as std::ops::Mul>::Output>,
        {
            type Output = UnitsMul<V, Self, O>;
            fn mul(self, rhs: O) -> Self::Output {
                self.calc_mul(rhs)
            }
        }

        impl<O, V: Float, A, B> std::ops::Div<O> for UnitsPat<V, A, B>
        where
            V: std::ops::Div,
            O: Into<V>,
            Self: Into<V>,
            Self: CalcMix<V>,
            UnitsDiv<V, Self, O>: From<<V as std::ops::Div>::Output>,
        {
            type Output = UnitsDiv<V, Self, O>;
            fn div(self, rhs: O) -> Self::Output {
                self.calc_div(rhs)
            }
        }
    };
    assert_eq!(derive(a).to_string(), b.to_string());
}

#[test]
#[should_panic(expected = "Expect '[' and ']' but ")]
fn error_bad_list01() {
    derive(quote! {
        #[calcmix(into = (f32))]
        struct Bad<V>(V);
    });
}

#[test]
#[should_panic(expected = "`unit_name` is required.")]
fn error_ok_list02() {
    derive(quote! {
        #[calcmix(into = [])]
        struct Bad<V>(V);
    });
}

#[test]
#[should_panic(expected = "Unable to specify types")]
fn error_bad_into01() {
    derive(quote! {
        #[calcmix(into = [f32], unit_name = "a".to_string())]
        struct Bad(f32);
    });
}

#[test]
#[should_panic(expected = "Unable to specify types")]
fn error_bad_into02() {
    derive(quote! {
        #[calcmix(into = [f32], unit_name = "a".to_string())]
        struct Bad(f64);
    });
}

#[test]
#[should_panic(expected = "Unexpected token: ")]
fn error_no_token() {
    derive(quote! {
        #[calcmix(into = [] unit_name = "a".to_string())]
        struct Bad<V>(V);
    });
}

#[test]
#[should_panic(expected = "Unexpected token: ")]
fn error_bad_token() {
    derive(quote! {
        #[calcmix(into = []; unit_name = "a".to_string())]
        struct Bad<V>(V);
    });
}

#[test]
#[should_panic(expected = "Least one argument must be supplied.")]
fn error_no_arg() {
    derive(quote! {
        #[calcmix]
        struct Bad<V>(V);
    });
}

#[test]
#[should_panic(expected = "Least one attribute 'calcmix' must be supplied.")]
fn error_no_attr() {
    derive(quote! {
        struct Bad<V>(V);
    });
}

#[test]
#[should_panic(expected = "Only one argument must be supplied.")]
fn error_bad_args() {
    derive(quote! {
        #[calcmix(into = [])(unit_name = "a".to_string())]
        struct Bad<V>(V);
    });
}

#[test]
#[should_panic(expected = "Only one attribute 'calcmix' must be supplied.")]
fn error_bad_attrs() {
    derive(quote! {
        #[calcmix(into = [])]
        #[calcmix(unit_name = "a".to_string())]
        struct Bad<V>(V);
    });
}
