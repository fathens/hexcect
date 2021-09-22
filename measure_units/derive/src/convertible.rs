use crate::common::*;

use proc_macro2::{Ident, TokenStream, TokenTree};
use quote::quote;
use std::collections::HashMap;

pub fn convertible(items: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse2(items).unwrap();
    let option = ConOpt::read_from_derive_input(&ast);
    let name = ast.ident;
    let inner_type =
        newtype_inner(&ast.data).unwrap_or_else(|| panic!("{} is not newtype struct.", name));

    let mut ts = TokenStream::new();
    for (target, cr) in option.convertible_sorted() {
        let conv = cr.convert(&inner_type, quote! { src.0 });
        ts.extend(quote! {
            impl From<#name> for #target {
                fn from(src: #name) -> #target {
                    #conv
                }
            }
        });
    }
    ts
}

#[derive(Debug)]
enum ConvRate {
    Expo(TokenStream),
    Real(TokenStream),
}

impl ConvRate {
    fn read_tokens(tokens: Vec<TokenTree>) -> (Ident, ConvRate) {
        let mut ts = tokens.into_iter();

        let target = match ts.next() {
            Some(TokenTree::Ident(name)) => name,
            _ => panic!("Can not read target name."),
        };

        let mk_conv = match ts.next() {
            Some(TokenTree::Punct(p)) => match p.as_char() {
                '^' => ConvRate::Expo,
                '=' => ConvRate::Real,
                c => panic!("Unsupported token: {}", c),
            },
            _ => panic!("Can not read token."),
        };

        let mut rate = TokenStream::new();
        rate.extend(ts);

        (target, mk_conv(rate))
    }

    fn convert(&self, inner: &syn::Type, src: TokenStream) -> TokenStream {
        match self {
            ConvRate::Expo(s) => quote! {
                let e: i8 = #s;
                let v = if e < 0 {
                    #src / (10u32.pow(e.abs() as u32) as #inner)
                } else {
                    #src * (10u32.pow(e as u32) as #inner)
                };
                v.into()
            },
            ConvRate::Real(s) => quote! {
                let r = #s;
                let v = #src * (r as #inner);
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
    fn read_from_derive_input(ast: &syn::DeriveInput) -> ConOpt {
        let convertible = ast
            .attrs
            .iter()
            .filter(|a| a.path.is_ident("convertible"))
            .map(ConOpt::read_convertible)
            .collect();
        ConOpt { convertible }
    }

    fn read_convertible(attr: &syn::Attribute) -> (Ident, ConvRate) {
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
            let gr = g.stream().into_iter().collect();
            ConvRate::read_tokens(gr)
        } else {
            panic!("An argument must be supplied.");
        }
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
            impl From<Meter> for Cm {
                fn from(src: Meter) -> Cm {
                    let r = 100;
                    let v = src.0 * (r as f64);
                    v.into()
                }
            }
            impl From<Meter> for Km {
                fn from(src: Meter) -> Km {
                    let e: i8 = -3;
                    let v = if e < 0 {
                        src.0 / (10u32.pow(e.abs() as u32) as f64)
                    } else {
                        src.0 * (10u32.pow(e as u32) as f64)
                    };
                    v.into()
                }
            }
            impl From<Meter> for Milli {
                fn from(src: Meter) -> Milli {
                    let e: i8 = 3;
                    let v = if e < 0 {
                        src.0 / (10u32.pow(e.abs() as u32) as f64)
                    } else {
                        src.0 * (10u32.pow(e as u32) as f64)
                    };
                    v.into()
                }
            }
        };
        assert_eq!(convertible(a).to_string(), b.to_string());
    }

    #[test]
    fn expressions() {
        let a = quote! {
            #[convertible(Degree = 180.0 / core::f64::consts::PI)]
            struct Radian(f64);
        };
        let b = quote! {
            impl From<Radian> for Degree {
                fn from(src: Radian) -> Degree {
                    let r = 180.0 / core::f64::consts::PI;
                    let v = src.0 * (r as f64);
                    v.into()
                }
            }
        };
        assert_eq!(convertible(a).to_string(), b.to_string());
    }

    #[test]
    fn write_empty01() {
        let s = convertible(quote! {
            struct MyUnit(u8);
        });
        assert!(s.to_string().is_empty());
    }

    #[test]
    #[should_panic(expected = "An argument must be supplied.")]
    fn write_empty02() {
        convertible(quote! {
            #[convertible]
            struct MyUnit(u8);
        });
    }

    #[test]
    #[should_panic(expected = "Can not read target name.")]
    fn write_empty03() {
        convertible(quote! {
            #[convertible()]
            struct MyUnit(u8);
        });
    }

    #[test]
    #[should_panic(expected = "MyUnit is not newtype struct.")]
    fn error_non_newtype() {
        convertible(quote! {
            struct MyUnit(u8, u8);
        });
    }

    #[test]
    #[should_panic(expected = "Can not read token.")]
    fn error_bad_syntax() {
        convertible(quote! {
            #[convertible(a)]
            struct MyUnit(u8);
        });
    }

    #[test]
    #[should_panic(expected = "Can not read token.")]
    fn error_bad_token01() {
        convertible(quote! {
            #[convertible(a b)]
            struct MyUnit(u8);
        });
    }

    #[test]
    #[should_panic(expected = "Unsupported token: -")]
    fn error_bad_token02() {
        convertible(quote! {
            #[convertible(a - b)]
            struct MyUnit(u8);
        });
    }

    #[test]
    #[should_panic(expected = "Can not read target name.")]
    fn error_bad_list01() {
        convertible(quote! {
            #[convertible(,)]
            struct MyUnit(u8);
        });
    }

    #[test]
    #[should_panic(expected = "Can not read target name.")]
    fn error_bad_list02() {
        convertible(quote! {
            #[convertible(,a = 2)]
            struct MyUnit(u8);
        });
    }

    #[test]
    #[should_panic(expected = "An argument must be supplied.")]
    fn error_bad_list03() {
        convertible(quote! {
            #[convertible(a = 2)(b = 3)]
            struct MyUnit(u8);
        });
    }
}
