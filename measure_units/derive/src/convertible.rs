use crate::common::*;

use proc_macro2::{Ident, TokenStream, TokenTree};
use quote::quote;
use std::{collections::HashMap, iter::Peekable};

pub fn derive(items: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse2(items).unwrap();
    let name = ast.ident;
    let inner_type =
        newtype_inner(&ast.data).unwrap_or_else(|| panic!("{} is not newtype struct.", name));
    let gs = &ast.generics;
    let option = ConOpt::read_from_derive_input(ast.attrs);

    TokenStream::from_iter(option.convertible_sorted().into_iter().map(|(target, cr)| {
        let conv = cr.convert(&inner_type, quote! { src.0 });
        quote! {
            impl #gs From<#name #gs> for #target #gs
            where
                #inner_type: std::ops::Mul,
                #inner_type: std::ops::Div,
                #inner_type: From<f64>,
                #target #gs: From<<#inner_type as std::ops::Mul>::Output>,
                #target #gs: From<<#inner_type as std::ops::Div>::Output>,
            {
                fn from(src: #name #gs) -> #target #gs {
                    #conv
                }
            }
        }
    }))
}

#[derive(Debug)]
enum ConvRate {
    Expo(TokenStream),
    Real(TokenStream),
}

impl ConvRate {
    fn read_tokens<I>(ts: &mut Peekable<I>) -> (Ident, ConvRate)
    where
        I: Iterator<Item = TokenTree>,
    {
        let (target, c, tokens) = read_expr(ts, None).unwrap_or_else(|| {
            panic!("Can not read target name.");
        });
        let rate = TokenStream::from_iter(tokens);
        let conv = match c {
            '^' => ConvRate::Expo(rate),
            '=' => ConvRate::Real(rate),
            c => panic!("Unsupported token: {}", c),
        };
        (target, conv)
    }

    fn convert(&self, inner: &syn::Type, src: TokenStream) -> TokenStream {
        match self {
            ConvRate::Expo(s) => quote! {
                let s: i8 = #s;
                let s_f: f64 = 10u32.pow(s.abs() as u32).into();
                let r: #inner = s_f.into();
                let a: #inner = #src;
                let v = if s.is_negative() { a / r } else { a * r };
                v.into()
            },
            ConvRate::Real(s) => quote! {
                let s = #s;
                let s_f: f64 = s.into();
                if s_f == 0.0 {
                    panic!("Using Zero as a rate !");
                }
                if !(s_f < 0.0) && !(0.0 < s_f) {
                    panic!("Using NaN as a rate !");
                }
                let r: #inner = s_f.into();
                let a: #inner = #src;
                let v = a * r;
                v.into()
            },
        }
    }
}

#[derive(Debug)]
struct ConOpt {
    convertible: HashMap<Ident, ConvRate>,
}

impl ConOpt {
    fn read_from_derive_input(attrs: Vec<syn::Attribute>) -> ConOpt {
        let convertible = attrs
            .into_iter()
            .filter(|a| a.path.is_ident("convertible"))
            .map(ConOpt::read_convertible)
            .collect();
        ConOpt { convertible }
    }

    fn read_convertible(attr: syn::Attribute) -> (Ident, ConvRate) {
        let mut ts = read_attr_args(attr).peekable();
        ConvRate::read_tokens(&mut ts)
    }

    fn convertible_sorted(&self) -> Vec<(&Ident, &ConvRate)> {
        let mut vs: Vec<_> = self.convertible.iter().collect();
        vs.sort_by_key(|(i, _)| i.to_string());
        vs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_impl() {
        let a = quote! {
            #[convertible(Km ^ -3)]
            #[convertible(Milli ^ 3)]
            #[convertible(Cm = 100)]
            struct Meter(f64);
        };
        let b = quote! {
            impl From<Meter> for Cm
            where
                f64: std::ops::Mul,
                f64: std::ops::Div,
                f64: From<f64>,
                Cm: From<<f64 as std::ops::Mul>::Output>,
                Cm: From<<f64 as std::ops::Div>::Output>,
            {
                fn from(src: Meter) -> Cm {
                    let s = 100;
                    let s_f: f64 = s.into();
                    if s_f == 0.0 {
                        panic!("Using Zero as a rate !");
                    }
                    if !(s_f < 0.0) && !(0.0 < s_f) {
                        panic!("Using NaN as a rate !");
                    }
                    let r: f64 = s_f.into();
                    let a: f64 = src.0;
                    let v = a * r;
                    v.into()
                }
            }
            impl From<Meter> for Km
            where
                f64: std::ops::Mul,
                f64: std::ops::Div,
                f64: From<f64>,
                Km: From<<f64 as std::ops::Mul>::Output>,
                Km: From<<f64 as std::ops::Div>::Output>,
            {
                fn from(src: Meter) -> Km {
                    let s: i8 = -3;
                    let s_f: f64 = 10u32.pow(s.abs() as u32).into();
                    let r: f64 = s_f.into();
                    let a: f64 = src.0;
                    let v = if s.is_negative() { a / r } else { a * r };
                    v.into()
                }
            }
            impl From<Meter> for Milli
            where
                f64: std::ops::Mul,
                f64: std::ops::Div,
                f64: From<f64>,
                Milli: From<<f64 as std::ops::Mul>::Output>,
                Milli: From<<f64 as std::ops::Div>::Output>,
            {
                fn from(src: Meter) -> Milli {
                    let s: i8 = 3;
                    let s_f: f64 = 10u32.pow(s.abs() as u32).into();
                    let r: f64 = s_f.into();
                    let a: f64 = src.0;
                    let v = if s.is_negative() { a / r } else { a * r };
                    v.into()
                }
            }
        };
        assert_eq!(derive(a).to_string(), b.to_string());
    }

    #[test]
    fn expressions() {
        let a = quote! {
            #[convertible(Degree = 180.0 / core::f64::consts::PI)]
            struct Radian(f64);
        };
        let b = quote! {
            impl From<Radian> for Degree
            where
                f64: std::ops::Mul,
                f64: std::ops::Div,
                f64: From<f64>,
                Degree: From<<f64 as std::ops::Mul>::Output>,
                Degree: From<<f64 as std::ops::Div>::Output>,
            {
                fn from(src: Radian) -> Degree {
                    let s = 180.0 / core::f64::consts::PI;
                    let s_f: f64 = s.into();
                    if s_f == 0.0 {
                        panic!("Using Zero as a rate !");
                    }
                    if !(s_f < 0.0) && !(0.0 < s_f) {
                        panic!("Using NaN as a rate !");
                    }
                    let r: f64 = s_f.into();
                    let a: f64 = src.0;
                    let v = a * r;
                    v.into()
                }
            }
        };
        assert_eq!(derive(a).to_string(), b.to_string());
    }

    #[test]
    fn generics() {
        let a = quote! {
            #[convertible(Minute = 1.0/60.0)]
            struct Second<V>(V);
        };
        let b = quote! {
            impl <V> From<Second <V> > for Minute<V>
            where
                V: std::ops::Mul,
                V: std::ops::Div,
                V: From<f64>,
                Minute<V>: From<<V as std::ops::Mul>::Output>,
                Minute<V>: From<<V as std::ops::Div>::Output>,
            {
                fn from(src: Second<V>) -> Minute<V> {
                    let s = 1.0/60.0;
                    let s_f: f64 = s.into();
                    if s_f == 0.0 {
                        panic!("Using Zero as a rate !");
                    }
                    if !(s_f < 0.0) && !(0.0 < s_f) {
                        panic!("Using NaN as a rate !");
                    }
                    let r: V = s_f.into();
                    let a: V = src.0;
                    let v = a * r;
                    v.into()
                }
            }
        };
        assert_eq!(derive(a).to_string(), b.to_string());
    }

    #[test]
    fn write_empty01() {
        let s = derive(quote! {
            struct MyUnit(u8);
        });
        assert!(s.to_string().is_empty());
    }

    #[test]
    #[should_panic(expected = "Least one argument must be supplied.")]
    fn write_empty02() {
        derive(quote! {
            #[convertible]
            struct MyUnit(u8);
        });
    }

    #[test]
    #[should_panic(expected = "Can not read target name.")]
    fn write_empty03() {
        derive(quote! {
            #[convertible()]
            struct MyUnit(u8);
        });
    }

    #[test]
    #[should_panic(expected = "MyUnit is not newtype struct.")]
    fn error_non_newtype() {
        derive(quote! {
            struct MyUnit(u8, u8);
        });
    }

    #[test]
    #[should_panic(expected = "Unsupported token ")]
    fn error_bad_syntax() {
        derive(quote! {
            #[convertible(a)]
            struct MyUnit(u8);
        });
    }

    #[test]
    #[should_panic(expected = "Unsupported token ")]
    fn error_bad_token01() {
        derive(quote! {
            #[convertible(a b)]
            struct MyUnit(u8);
        });
    }

    #[test]
    #[should_panic(expected = "Unsupported token: -")]
    fn error_bad_token02() {
        derive(quote! {
            #[convertible(a - b)]
            struct MyUnit(u8);
        });
    }

    #[test]
    #[should_panic(expected = "Can not read target name.")]
    fn error_bad_list01() {
        derive(quote! {
            #[convertible(,)]
            struct MyUnit(u8);
        });
    }

    #[test]
    #[should_panic(expected = "Can not read target name.")]
    fn error_bad_list02() {
        derive(quote! {
            #[convertible(,a = 2)]
            struct MyUnit(u8);
        });
    }

    #[test]
    #[should_panic(expected = "Only one argument must be supplied.")]
    fn error_bad_list03() {
        derive(quote! {
            #[convertible(a = 2)(b = 3)]
            struct MyUnit(u8);
        });
    }
}
