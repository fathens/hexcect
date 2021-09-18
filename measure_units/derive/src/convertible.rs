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
        let conv = cr.convert(&inner_type, quote! { self.0 });
        ts.extend(quote! {
            impl Convertible<#target> for #name {
                fn convert(&self) -> #target {
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
        let read_target = |token| match token {
            TokenTree::Ident(name) => name,
            _ => panic!("Can not read target name."),
        };

        let read_conv = |token| match token {
            TokenTree::Punct(p) => match p.as_char() {
                '^' => |s| ConvRate::Expo(s),
                '=' => |s| ConvRate::Real(s),
                c => panic!("Unsupported token: {}", c),
            },
            _ => panic!("Can not read token."),
        };

        let mut ts = tokens.into_iter();

        let target = read_target(ts.next().expect("Can not read target name."));
        let mk_conv = read_conv(ts.next().expect("Can not read token."));

        let mut rate = TokenStream::new();
        rate.extend(ts);
        (target, mk_conv(rate))
    }

    fn convert(&self, inner: &syn::Type, src: TokenStream) -> TokenStream {
        match self {
            ConvRate::Expo(s) => {
                quote! {
                    let e: i8 = #s;
                    let v = if e < 0 {
                        #src / (10u32.pow(e.abs() as u32) as #inner)
                    } else {
                        #src * (10u32.pow(e as u32) as #inner)
                    };
                    v.into()
                }
            }
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
            .flat_map(|a| ConOpt::read_convertible(a).into_iter())
            .collect();
        ConOpt { convertible }
    }

    fn read_convertible(attr: &syn::Attribute) -> HashMap<Ident, ConvRate> {
        attr.tokens
            .clone()
            .into_iter()
            .map(|t| match t {
                TokenTree::Group(g) => {
                    let gr = g.stream().into_iter().collect();
                    ConvRate::read_tokens(gr)
                }
                _ => panic!("Unexpected token: {:?}", t),
            })
            .collect()
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
            impl Convertible<Cm> for Meter {
                fn convert(&self) -> Cm {
                    let r = 100;
                    let v = self.0 * (r as f64);
                    v.into()
                }
            }
            impl Convertible<Km> for Meter {
                fn convert(&self) -> Km {
                    let e: i8 = -3;
                    let v = if e < 0 {
                        self.0 / (10u32.pow(e.abs() as u32) as f64)
                    } else {
                        self.0 * (10u32.pow(e as u32) as f64)
                    };
                    v.into()
                }
            }
            impl Convertible<Milli> for Meter {
                fn convert(&self) -> Milli {
                    let e: i8 = 3;
                    let v = if e < 0 {
                        self.0 / (10u32.pow(e.abs() as u32) as f64)
                    } else {
                        self.0 * (10u32.pow(e as u32) as f64)
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
            impl Convertible<Degree> for Radian {
                fn convert(&self) -> Degree {
                    let r = 180.0 / core::f64::consts::PI;
                    let v = self.0 * (r as f64);
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

        let s = convertible(quote! {
            #[convertible]
            struct MyUnit(u8);
        });
        assert!(s.to_string().is_empty());
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
}
