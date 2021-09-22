mod calcmix;
mod common;
mod convertible;
mod float_status;

use proc_macro::TokenStream;

#[proc_macro_derive(FloatStatus)]
pub fn derive_float_status(items: TokenStream) -> TokenStream {
    float_status::float_status(items.into()).into()
}

#[proc_macro_derive(Convertible, attributes(convertible))]
pub fn drive_convertible(items: TokenStream) -> TokenStream {
    convertible::convertible(items.into()).into()
}

#[proc_macro_derive(CalcMix, attributes(calcmix))]
pub fn derive_calc_mix(items: TokenStream) -> TokenStream {
    calcmix::derive(items.into()).into()
}
