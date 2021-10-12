use crate::common::*;

use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn derive(items: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse2(items).unwrap();
    let name = ast.ident;
    let generics = &ast.generics;
    let (inner_type, _) = newtype_with_phantoms(&ast.data)
        .unwrap_or_else(|| panic!("{} is not newtype struct.", name));

    if generics.params.is_empty() {
        quote! {
            impl FloatStatus for #name {
                fn abs(&self) -> Self { Self(self.0.abs()) }
                fn is_nan(&self) -> bool { self.0.is_nan() }
                fn is_zero(&self) -> bool { self.0 == 0.0 }
                fn is_normal(&self) -> bool { self.0.is_normal() }
                fn is_subnormal(&self) -> bool { self.0.is_subnormal() }
                fn is_finite(&self) -> bool { self.0.is_finite() }
                fn is_infinite(&self) -> bool { self.0.is_infinite() }
                fn is_sign_positive(&self) -> bool { self.0.is_sign_positive() }
                fn is_sign_negative(&self) -> bool { self.0.is_sign_negative() }
            }
        }
    } else {
        let gparams = clean_generics(&ast.generics);

        quote! {
            impl #generics FloatStatus for #name #gparams
            where
                #inner_type: num_traits::Float,
                #inner_type: Into<Self>
            {
                fn abs(&self) -> Self { self.0.abs().into() }
                fn is_nan(&self) -> bool { self.0.is_nan() }
                fn is_zero(&self) -> bool { self.0.is_zero() }
                fn is_normal(&self) -> bool { self.0.is_normal() }
                fn is_subnormal(&self) -> bool { self.0.classify() == std::num::FpCategory::Subnormal }
                fn is_finite(&self) -> bool { self.0.is_finite() }
                fn is_infinite(&self) -> bool { self.0.is_infinite() }
                fn is_sign_positive(&self) -> bool { self.0.is_sign_positive() }
                fn is_sign_negative(&self) -> bool { self.0.is_sign_negative() }
            }
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
                fn abs(&self) -> Self { Self(self.0.abs()) }
                fn is_nan(&self) -> bool { self.0.is_nan() }
                fn is_zero(&self) -> bool { self.0 == 0.0 }
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
    fn generics_impl() {
        let a = quote! {
            struct MyFloat<V, A>(V, PhantomData<A>);
        };
        let b = quote! {
            impl<V, A> FloatStatus for MyFloat<V, A>
            where
                V: num_traits::Float,
                V: Into<Self>
            {
                fn abs(&self) -> Self { self.0.abs().into() }
                fn is_nan(&self) -> bool { self.0.is_nan() }
                fn is_zero(&self) -> bool { self.0.is_zero() }
                fn is_normal(&self) -> bool { self.0.is_normal() }
                fn is_subnormal(&self) -> bool { self.0.classify() == std::num::FpCategory::Subnormal }
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

    #[test]
    #[should_panic(expected = "Support only `PhantomData`")]
    fn not_phantom_simple_float_status() {
        derive(quote! {
            struct MyAbc(f32, f64);
        });
    }

    #[test]
    #[should_panic(expected = "Support only `PhantomData`")]
    fn not_phantom_generics_float_status() {
        derive(quote! {
            struct MyAbc<A, B>(A, B);
        });
    }

    #[test]
    #[should_panic(expected = "MyAbc is not newtype struct.")]
    fn not_newtype_simple_float_status() {
        derive(quote! {
            struct MyAbc {
                a: f32,
                b: f64,
            }
        });
    }

    #[test]
    #[should_panic(expected = "MyAbc is not newtype struct.")]
    fn not_newtype_unit_float_status() {
        derive(quote! {
            struct MyAbc;
        });
    }

    #[test]
    #[should_panic(expected = "MyAbc is not newtype struct.")]
    fn not_newtype_generics_float_status() {
        derive(quote! {
            struct MyAbc<A, B> {
                a: A,
                b: B,
            }
        });
    }

    #[test]
    #[should_panic(expected = "MyAbc is not newtype struct.")]
    fn not_struct_float_status() {
        derive(quote! {
            enum MyAbc {
                A,
            }
        });
    }
}
