use crate::common::*;

use darling::*;
use proc_macro2::{Delimiter, TokenStream};
use quote::quote;

pub fn derive(items: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse2(items).unwrap();
    let opt = StructOpt::from_derive_input(&ast).unwrap();
    let attr = Attr::read(ast.attrs);
    let name = ast.ident;
    let (inner_type, phantoms) = newtype_with_phantoms(&ast.data)
        .unwrap_or_else(|| panic!("{} is not newtype struct.", name));
    let ginner = generics_inner(&inner_type, &opt.generics);

    let froms = impl_froms(
        &name,
        &inner_type,
        ginner,
        &opt.generics,
        &phantoms,
        attr.into,
    );
    let cmix = impl_calcmix(
        &name,
        &inner_type,
        ginner,
        &opt.generics,
        &phantoms,
        &attr.unit_name,
    );
    let calcs = impl_calcs(&name, &inner_type, &opt.generics);

    TokenStream::from_iter([froms, cmix, calcs])
}

fn is_eq<'a>(gp: &'a syn::GenericParam, ty: &syn::Type) -> Option<&'a syn::TypeParam> {
    match gp {
        syn::GenericParam::Type(g) => match ty {
            syn::Type::Path(t) => match t.path.segments.last() {
                Some(s) if s.ident == g.ident => Some(g),
                _ => None,
            },
            _ => None,
        },
        _ => None,
    }
}

fn generics_inner<'a>(
    inner_type: &syn::Type,
    generics: &'a syn::Generics,
) -> Option<&'a syn::TypeParam> {
    generics
        .params
        .iter()
        .flat_map(|g| is_eq(g, inner_type))
        .next()
}

fn impl_calcmix(
    name: &syn::Ident,
    inner_type: &syn::Type,
    ginner: Option<&syn::TypeParam>,
    generics: &syn::Generics,
    phantoms: &[syn::Type],
    unit_name: &TokenStream,
) -> TokenStream {
    let gv = match ginner {
        Some(gi) => quote! {
            #gi: std::fmt::Display,
            #gi: From<Self>,
        },
        None => quote! {},
    };

    let gs = TokenStream::from_iter(
        generics
            .params
            .iter()
            .filter(|g| phantoms.iter().any(|a| is_eq(g, a).is_some()))
            .map(|g| quote! { #g: CalcMix<#inner_type>, }),
    );

    quote! {
        impl #generics CalcMix<#inner_type> for #name #generics
        where
            #gs
        {
            fn unit_name() -> std::lazy::Lazy<String> {
                std::lazy::Lazy::new(|| #unit_name)
            }
        }

        impl #generics std::fmt::Display for #name #generics
        where
            #gv
            Self: Copy,
            Self: CalcMix<#inner_type>,
        {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let v: #inner_type = (*self).into();
                write!(f, "{}{}", v, *Self::unit_name())
            }
        }
    }
}

fn impl_calcs(name: &syn::Ident, inner_type: &syn::Type, generics: &syn::Generics) -> TokenStream {
    let gp = &generics.params;
    quote! {
        impl<O, #gp> std::ops::Add<O> for #name #generics
        where
            #inner_type: std::ops::Add,
            Self: Into<#inner_type>,
            Self: From<<#inner_type as std::ops::Add>::Output>,
            Self: CalcMix<#inner_type>,
            O: Into<Self>,
        {
            type Output = Self;

            fn add(self, rhs: O) -> Self::Output {
                self.calc_add(rhs.into())
            }
        }

        impl<O, #gp> std::ops::Sub<O> for #name #generics
        where
            #inner_type: std::ops::Sub,
            Self: Into<#inner_type>,
            Self: From<<#inner_type as std::ops::Sub>::Output>,
            Self: CalcMix<#inner_type>,
            O: Into<Self>,
        {
            type Output = Self;

            fn sub(self, rhs: O) -> Self::Output {
                self.calc_sub(rhs.into())
            }
        }

        impl<O, #gp> std::ops::Mul<O> for #name #generics
        where
            #inner_type: std::ops::Mul,
            O: Into<#inner_type>,
            Self: Into<#inner_type>,
            Self: CalcMix<#inner_type>,
            UnitsMul<#inner_type, Self, O>: From<<#inner_type as std::ops::Mul>::Output>,
        {
            type Output = UnitsMul<#inner_type, Self, O>;

            fn mul(self, rhs: O) -> Self::Output {
                self.calc_mul(rhs)
            }
        }

        impl<O, #gp> std::ops::Div<O> for #name #generics
        where
            #inner_type: std::ops::Div,
            O: Into<#inner_type>,
            Self: Into<#inner_type>,
            Self: CalcMix<#inner_type>,
            UnitsDiv<#inner_type, Self, O>: From<<#inner_type as std::ops::Div>::Output>,
        {
            type Output = UnitsDiv<#inner_type, Self, O>;

            fn div(self, rhs: O) -> Self::Output {
                self.calc_div(rhs)
            }
        }
    }
}

fn impl_froms(
    name: &syn::Ident,
    inner_type: &syn::Type,
    ginner: Option<&syn::TypeParam>,
    generics: &syn::Generics,
    phantoms: &[syn::Type],
    into_types: Option<Vec<syn::Ident>>,
) -> TokenStream {
    let args = {
        let mut q = quote! { v };
        q.extend(phantoms.iter().map(|p| {
            quote! {
                , std::marker::PhantomData::<#p>
            }
        }));
        q
    };
    let mut base = quote! {
        impl #generics From<#inner_type> for #name #generics {
            fn from(v: #inner_type) -> Self {
                Self(#args)
            }
        }
    };

    if let Some(gi) = ginner {
        let target_types = into_types.unwrap_or_else(|| {
            vec!["f32", "f64", "i32", "i64"]
                .iter()
                .map(|a| syn::Ident::from_string(a).unwrap())
                .collect()
        });
        let gs = TokenStream::from_iter(
            generics
                .params
                .iter()
                .filter(|g| match g {
                    syn::GenericParam::Type(t) => t.ident != gi.ident,
                    _ => false,
                })
                .map(|g| quote! { #g, }),
        );
        for ty in target_types {
            base.extend(quote! {
                impl<#gs> From<#name<#ty, #gs>> for #ty {
                    fn from(a: #name<#ty, #gs>) -> Self {
                        a.0
                    }
                }
            });
        }
    } else {
        if into_types.is_some() {
            panic!("Unable to specify types for non-generic typed struct.");
        }
        base.extend(quote! {
            impl From<#name> for #inner_type {
                fn from(a: #name) -> Self {
                    a.0
                }
            }
        });
    }
    base
}

#[derive(Debug, FromDeriveInput)]
struct StructOpt {
    generics: syn::Generics,
}

#[derive(Debug)]
struct Attr {
    into: Option<Vec<syn::Ident>>,
    unit_name: TokenStream,
}

impl Attr {
    fn read(attrs: Vec<syn::Attribute>) -> Attr {
        let mut ats = attrs.into_iter().filter(|a| a.path.is_ident("calcmix"));

        if let Some(a) = ats.next() {
            if ats.next().is_some() {
                panic!("Only one attribute 'calcmix' must be supplied.");
            }
            Attr::read_attrs(a)
        } else {
            panic!("Least one attribute 'calcmix' must be supplied.");
        }
    }

    fn read_attrs(attr: syn::Attribute) -> Attr {
        let mut ts = read_attr_args(attr).peekable();

        let into = read_agroup("into", &mut ts).map(|g| {
            skip_comma(&mut ts);
            if g.delimiter() != Delimiter::Bracket {
                panic!("Expect '[' and ']' but {}", g);
            }
            read_array(&mut g.stream().into_iter())
        });

        let unit_name = TokenStream::from_iter({
            let (_, c, tokens) =
                read_expr(&mut ts, Some("unit_name")).expect("`unit_name` is required.");
            if c != '=' {
                panic!("Expect '=' but {:?}", c);
            }
            tokens
        });
        Attr { into, unit_name }
    }
}

#[cfg(test)]
mod tests;
