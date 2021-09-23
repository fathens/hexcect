use crate::common::*;

use darling::*;
use proc_macro2::{Group, TokenStream, TokenTree};
use quote::quote;
use std::iter::Peekable;

pub fn derive(items: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse2(items).unwrap();
    let opt = StructOpt::from_derive_input(&ast).unwrap();
    let attr = Attr::read(&ast.attrs);
    let name = ast.ident;
    let (inner_type, phantoms) = newtype_with_phantoms(&ast.data)
        .unwrap_or_else(|| panic!("{} is not newtype struct.", name));
    let ginner = generics_inner(&inner_type, &opt.generics);
    let mut ts = TokenStream::new();
    ts.extend(impl_froms(
        &name,
        &inner_type,
        &ginner,
        &opt.generics,
        &phantoms,
        &attr.into,
    ));
    ts.extend(impl_calcmix(
        &name,
        &inner_type,
        &ginner,
        &opt.generics,
        phantoms,
        attr.unit_name,
    ));
    ts.extend(impl_calcs(&name, &inner_type, &opt.generics));
    ts
}

fn is_eq(a: &syn::TypeParam, b: &syn::Type) -> bool {
    a.to_token_stream().to_string() == b.to_token_stream().to_string()
}

fn generics_inner(inner_type: &syn::Type, generics: &syn::Generics) -> Option<syn::TypeParam> {
    generics
        .params
        .iter()
        .flat_map(|g| match g {
            syn::GenericParam::Type(t) if is_eq(t, inner_type) => Some(t),
            _ => None,
        })
        .next()
        .cloned()
}

fn impl_calcmix(
    name: &syn::Ident,
    inner_type: &syn::Type,
    ginner: &Option<syn::TypeParam>,
    generics: &syn::Generics,
    mut phantoms: Vec<syn::Type>,
    unit_name: TokenStream,
) -> TokenStream {
    let gv = match ginner {
        Some(gi) => quote! {
            #gi: std::fmt::Display,
            #gi: From<Self>,
        },
        None => quote! {},
    };

    let gs = {
        let subs = generics.params.iter().filter(|g| match g {
            syn::GenericParam::Type(t) => phantoms.iter_mut().any(|a| is_eq(t, a)),
            _ => false,
        });
        let mut t = TokenStream::new();
        for g in subs {
            t.extend(quote! {
                #g: CalcMix<#inner_type>,
            });
        }
        t
    };

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
    ginner: &Option<syn::TypeParam>,
    generics: &syn::Generics,
    phantoms: &[syn::Type],
    into_types: &[syn::Ident],
) -> TokenStream {
    let args = {
        let mut q = quote! { v };
        for p in phantoms {
            q.extend(quote! { , std::marker::PhantomData::<#p> });
        }
        q
    };
    let mut base = quote! {
        impl #generics From<#inner_type> for #name #generics {
            fn from(v: #inner_type) -> Self {
                Self(#args)
            }
        }
    };
    for gi in ginner {
        let gs = {
            let qs: Vec<_> = generics
                .params
                .iter()
                .filter(|g| match g {
                    syn::GenericParam::Type(t) => t.ident != gi.ident,
                    _ => false,
                })
                .map(|g| quote! { #g, })
                .collect();
            let mut t = TokenStream::new();
            if !qs.is_empty() {
                t.extend(qs);
            }
            t
        };
        for ty in into_types {
            base.extend(quote! {
                impl<#gs> From<#name<#ty, #gs>> for #ty {
                    fn from(a: #name<#ty, #gs>) -> Self {
                        a.0
                    }
                }
            });
        }
    }
    base
}

#[derive(Debug, FromDeriveInput)]
struct StructOpt {
    generics: syn::Generics,
}

#[derive(Debug)]
struct Attr {
    into: Vec<syn::Ident>,
    unit_name: TokenStream,
}

impl Attr {
    fn read(attrs: &[syn::Attribute]) -> Attr {
        if let Some(attr) = attrs.iter().find(|a| a.path.is_ident("calcmix")) {
            Attr::read_attrs(attr)
        } else {
            panic!("`unit_name` must be specified.");
        }
    }

    fn read_attrs(attr: &syn::Attribute) -> Attr {
        let gs: Vec<_> = attr
            .tokens
            .clone()
            .into_iter()
            .map(|t| match t {
                TokenTree::Group(g) => g,
                _ => panic!("Unexpected token: {:?}", t),
            })
            .collect();

        if let [g] = &gs[..] {
            let mut ts = g.stream().into_iter().peekable();

            let into = match read_agroup("into", &mut ts) {
                Some(g) => {
                    skip_comma(&mut ts);
                    read_array(&mut g.stream().into_iter())
                }
                None => vec!["f32", "f64", "i32", "i64"]
                    .iter()
                    .map(|a| syn::Ident::from_string(a).unwrap())
                    .collect(),
            };

            let mut unit_name = TokenStream::new();
            unit_name.extend(read_expr("unit_name", &mut ts).expect("`unit_name` is required."));
            Attr { into, unit_name }
        } else {
            panic!("An argument must be supplied.");
        }
    }
}

fn skip_comma<I>(ts: &mut I)
where
    I: Iterator<Item = TokenTree>,
{
    match ts.next() {
        Some(TokenTree::Punct(p)) if p.as_char() == ',' => (),
        None => (),
        a => panic!("Unexpected token: {:?}", a),
    }
}

fn read_array<I>(ts: &mut I) -> Vec<syn::Ident>
where
    I: Iterator<Item = TokenTree>,
{
    let mut result = Vec::new();
    while let Some(TokenTree::Ident(ty)) = ts.next() {
        result.push(ty);
        skip_comma(ts);
    }
    result
}

fn read_agroup<I>(key: &str, ts: &mut Peekable<I>) -> Option<Group>
where
    I: Iterator<Item = TokenTree>,
{
    match ts.peek() {
        Some(TokenTree::Ident(name)) if name == key => {
            ts.next();
            match ts.next() {
                Some(TokenTree::Punct(p)) if p.as_char() == '=' => match ts.next() {
                    Some(TokenTree::Group(g)) => Some(g),
                    a => panic!("Expect a group but {:?}", a),
                },
                a => panic!("Expect `=` but {:?}", a),
            }
        }
        _ => None,
    }
}

fn read_expr<I>(key: &str, ts: &mut Peekable<I>) -> Option<Vec<TokenTree>>
where
    I: Iterator<Item = TokenTree>,
{
    match ts.peek() {
        Some(TokenTree::Ident(name)) if name == key => {
            ts.next();
            match ts.next() {
                Some(TokenTree::Punct(p)) if p.as_char() == '=' => Some(ts.collect()),
                a => panic!("Expect `=` but {:?}", a),
            }
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests;
