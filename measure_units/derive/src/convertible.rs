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
    let clean_gs = clean_generics(gs);
    let option = ConOpt::read_from_derive_input(ast.attrs);

    TokenStream::from_iter(option.convertible_sorted().into_iter().map(|(target, cr)| {
        let conv = cr.convert(&inner_type);
        quote! {
            impl #gs From<#name #clean_gs> for #target #clean_gs
            where
                #inner_type: num_traits::Float,
                #inner_type: num_traits::FromPrimitive,
                #inner_type: From<#name #clean_gs>,
                #inner_type: Into<#target #clean_gs>,
            {
                fn from(src: #name #clean_gs) -> #target #clean_gs {
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
    Func(TokenStream),
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
            '*' => ConvRate::Real(rate),
            '=' => ConvRate::Func(rate),
            c => panic!("Unsupported token: {}", c),
        };
        (target, conv)
    }

    fn convert(&self, inner: &syn::Type) -> TokenStream {
        match self {
            ConvRate::Expo(s) => quote! {
                let s: i8 = #s;
                let p = 10u32.pow(s.abs() as u32);
                let r = #inner::from_u32(p).unwrap();
                let a: #inner = src.into();
                let v = if s.is_negative() { a / r } else { a * r };
                v.into()
            },
            ConvRate::Real(s) => quote! {
                let r: #inner = #s;
                if r.is_zero() { panic!("Using Zero as a rate !"); }
                if r.is_nan() { panic!("Using NaN as a rate !"); }
                let a: #inner = src.into();
                let v = a * r;
                v.into()
            },
            ConvRate::Func(s) => quote! {
                let f = |v: #inner| #s;
                let a: #inner = src.into();
                let v = f(a);
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
            #[convertible(Cm * 100)]
            struct Meter(f64);
        };
        let b = quote! {
            impl From<Meter> for Cm
            where
                f64: num_traits::Float,
                f64: num_traits::FromPrimitive,
                f64: From<Meter>,
                f64: Into<Cm>,
            {
                fn from(src: Meter) -> Cm {
                    let r: f64 = 100;
                    if r.is_zero() { panic!("Using Zero as a rate !"); }
                    if r.is_nan() { panic!("Using NaN as a rate !"); }
                    let a: f64 = src.into();
                    let v = a * r;
                    v.into()
                }
            }
            impl From<Meter> for Km
            where
                f64: num_traits::Float,
                f64: num_traits::FromPrimitive,
                f64: From<Meter>,
                f64: Into<Km>,
            {
                fn from(src: Meter) -> Km {
                    let s: i8 = -3;
                    let p = 10u32.pow(s.abs() as u32);
                    let r = f64::from_u32(p).unwrap();
                    let a: f64 = src.into();
                    let v = if s.is_negative() { a / r } else { a * r };
                    v.into()
                }
            }
            impl From<Meter> for Milli
            where
                f64: num_traits::Float,
                f64: num_traits::FromPrimitive,
                f64: From<Meter>,
                f64: Into<Milli>,
            {
                fn from(src: Meter) -> Milli {
                    let s: i8 = 3;
                    let p = 10u32.pow(s.abs() as u32);
                    let r = f64::from_u32(p).unwrap();
                    let a: f64 = src.into();
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
            #[convertible(Degree * 180.0 / core::f64::consts::PI)]
            struct Radian(f64);
        };
        let b = quote! {
            impl From<Radian> for Degree
            where
                f64: num_traits::Float,
                f64: num_traits::FromPrimitive,
                f64: From<Radian>,
                f64: Into<Degree>,
            {
                fn from(src: Radian) -> Degree {
                    let r: f64 = 180.0 / core::f64::consts::PI;
                    if r.is_zero() { panic!("Using Zero as a rate !"); }
                    if r.is_nan() { panic!("Using NaN as a rate !"); }
                    let a: f64 = src.into();
                    let v = a * r;
                    v.into()
                }
            }
        };
        assert_eq!(derive(a).to_string(), b.to_string());
    }

    #[test]
    fn functions() {
        let a = quote! {
            #[convertible(Degree = v.to_degrees())]
            struct Radian(f64);
        };
        let b = quote! {
            impl From<Radian> for Degree
            where
                f64: num_traits::Float,
                f64: num_traits::FromPrimitive,
                f64: From<Radian>,
                f64: Into<Degree>,
            {
                fn from(src: Radian) -> Degree {
                    let f = |v: f64| v.to_degrees();
                    let a: f64 = src.into();
                    let v = f(a);
                    v.into()
                }
            }
        };
        assert_eq!(derive(a).to_string(), b.to_string());
    }

    #[test]
    fn generics() {
        let a = quote! {
            #[convertible(Minute * 1.0/60.0)]
            struct Second<V>(V);
        };
        let b = quote! {
            impl <V> From<Second <V> > for Minute<V>
            where
                V: num_traits::Float,
                V: num_traits::FromPrimitive,
                V: From< Second<V> >,
                V: Into< Minute<V> >,
            {
                fn from(src: Second<V>) -> Minute<V> {
                    let r: V = 1.0/60.0;
                    if r.is_zero() { panic!("Using Zero as a rate !"); }
                    if r.is_nan() { panic!("Using NaN as a rate !"); }
                    let a: V = src.into();
                    let v = a * r;
                    v.into()
                }
            }
        };
        assert_eq!(derive(a).to_string(), b.to_string());
    }

    #[test]
    fn generics_bound() {
        let a = quote! {
            #[convertible(Minute * 1.0/60.0)]
            struct Second<V: FloatConst>(V);
        };
        let b = quote! {
            impl <V: FloatConst> From<Second <V> > for Minute<V>
            where
                V: num_traits::Float,
                V: num_traits::FromPrimitive,
                V: From< Second<V> >,
                V: Into< Minute<V> >,
            {
                fn from(src: Second<V>) -> Minute<V> {
                    let r: V = 1.0/60.0;
                    if r.is_zero() { panic!("Using Zero as a rate !"); }
                    if r.is_nan() { panic!("Using NaN as a rate !"); }
                    let a: V = src.into();
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

    #[test]
    #[should_panic(expected = "MyAbc is not newtype struct.")]
    fn not_newtype_simple00_convertible() {
        derive(quote! {
            struct MyAbc;
        });
    }

    #[test]
    #[should_panic(expected = "MyAbc is not newtype struct.")]
    fn not_newtype_simple01_convertible() {
        derive(quote! {
            struct MyAbc(f32, f64);
        });
    }

    #[test]
    #[should_panic(expected = "MyAbc is not newtype struct.")]
    fn not_newtype_simple02_convertible() {
        derive(quote! {
            struct MyAbc {
                a: f32,
            }
        });
    }

    #[test]
    #[should_panic(expected = "MyAbc is not newtype struct.")]
    fn not_struct_convertible() {
        derive(quote! {
            enum MyAbc {
                A,
            }
        });
    }
}
