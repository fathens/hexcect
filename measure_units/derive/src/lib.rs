use proc_macro::TokenStream;

#[proc_macro_derive(FloatStatus)]
pub fn derive_float_status(items: TokenStream) -> TokenStream {
    measure_units_derive_internal::float_status(items.into()).into()
}

#[proc_macro_derive(Convertible, attributes(convertible))]
pub fn drive_convertible(items: TokenStream) -> TokenStream {
    measure_units_derive_internal::convertible(items.into()).into()
}
