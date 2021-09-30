use crate::common::*;

use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn derive(items: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse2(items).unwrap();
    let name = ast.ident;
    if newtype_inner(&ast.data).is_none() {
        panic!("{} is not newtype struct.", name);
    }
    quote! {
        impl FloatStatus for #name {
            fn is_nan(&self) -> bool { self.0.is_nan() }
            fn is_normal(&self) -> bool { self.0.is_normal() }
            fn is_subnormal(&self) -> bool { self.0.is_subnormal() }
            fn is_finite(&self) -> bool { self.0.is_finite() }
            fn is_infinite(&self) -> bool { self.0.is_infinite() }
            fn is_sign_positive(&self) -> bool { self.0.is_sign_positive() }
            fn is_sign_negative(&self) -> bool { self.0.is_sign_negative() }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_impl() {
        let a = quote! {
            struct MyFloat(f64);
        };
        let b = quote! {
            impl FloatStatus for MyFloat {
                fn is_nan(&self) -> bool { self.0.is_nan() }
                fn is_normal(&self) -> bool { self.0.is_normal() }
                fn is_subnormal(&self) -> bool { self.0.is_subnormal() }
                fn is_finite(&self) -> bool { self.0.is_finite() }
                fn is_infinite(&self) -> bool { self.0.is_infinite() }
                fn is_sign_positive(&self) -> bool { self.0.is_sign_positive() }
                fn is_sign_negative(&self) -> bool { self.0.is_sign_negative() }
            }
        };
        assert_eq!(derive(a).to_string(), b.to_string());
    }

    #[test]
    fn check_float_specs_of_abnormals() {
        use core::cmp::Ordering;

        let v: f64 = 1.23;
        let z: f64 = 0.0;
        assert_eq!(v.is_normal(), true);
        assert_eq!(z.is_normal(), false);

        assert_eq!((v / v).is_normal(), true);
        assert_eq!((v / v).is_finite(), true);
        assert_eq!((v / v).is_infinite(), false);
        assert_eq!((v / v).is_nan(), false);

        assert_eq!((v / z).is_normal(), false);
        assert_eq!((v / z).is_finite(), false);
        assert_eq!((v / z).is_infinite(), true);
        assert_eq!((v / z).is_nan(), false);
        assert_eq!((v / z), f64::INFINITY);
        assert_eq!((-v / z), f64::NEG_INFINITY);

        assert_eq!(
            f64::INFINITY.partial_cmp(&f64::INFINITY),
            Some(Ordering::Equal)
        );
        assert_eq!(
            f64::INFINITY.partial_cmp(&f64::NEG_INFINITY),
            Some(Ordering::Greater)
        );
        assert_eq!(
            f64::NEG_INFINITY.partial_cmp(&f64::INFINITY),
            Some(Ordering::Less)
        );
        assert_eq!(
            f64::NEG_INFINITY.partial_cmp(&f64::NEG_INFINITY),
            Some(Ordering::Equal)
        );

        assert_eq!((z / z).is_normal(), false);
        assert_eq!((z / z).is_finite(), false);
        assert_eq!((z / z).is_infinite(), false);
        assert_eq!((z / z).is_nan(), true);
        assert_eq!((z / z) == (z / z), false);
        assert_eq!((z / z) == f64::NAN, false);
        assert_eq!(f64::NAN == f64::NAN, false);

        let m: f64 = f64::MIN_POSITIVE;
        let h: f64 = m / 2.0;
        let h2: f64 = m / 2.0;
        let t: f64 = m / 3.0;

        assert_eq!(m.is_normal(), true);
        assert_eq!(m.is_subnormal(), false);
        assert_eq!(m.is_sign_positive(), true);

        assert_eq!(h.is_normal(), false);
        assert_eq!(h.is_subnormal(), true);
        assert_eq!(h.is_sign_positive(), true);

        assert_eq!(h.partial_cmp(&t), Some(Ordering::Greater));
        assert_eq!(t.partial_cmp(&h), Some(Ordering::Less));
        assert_eq!(h2.partial_cmp(&h), Some(Ordering::Equal));
    }
}
