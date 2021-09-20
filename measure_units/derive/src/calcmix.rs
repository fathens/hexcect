use crate::common::*;

use proc_macro2::TokenStream;

pub fn derive(items: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse2(items).unwrap();
    let name = ast.ident;
    let _inner_type =
        newtype_inner(&ast.data).unwrap_or_else(|| panic!("{} is not newtype struct.", name));
    TokenStream::new()
}
