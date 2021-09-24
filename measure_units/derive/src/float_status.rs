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
}
