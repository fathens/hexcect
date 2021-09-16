use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn float_status(items: TokenStream) -> TokenStream {
    let parsed: DeriveInput = syn::parse2(items).unwrap();
    let name = parsed.ident;
    quote! {
        impl measure_units::FloatStatus for #name {
            fn is_nan(&self) -> bool { self.0.is_nan() }
        }
    }
}
