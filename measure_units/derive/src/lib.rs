use proc_macro::TokenStream;

#[proc_macro_derive(FloatStatus)]
pub fn measure_calc(items: TokenStream) -> TokenStream {
    measure_units_derive_internal::float_status(items.into()).into()
}
