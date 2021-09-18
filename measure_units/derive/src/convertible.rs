use crate::common::*;

use darling::FromMeta;
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
    Expo(i8),
    Real(f64),
}

impl ConvRate {
    fn read_tokens(tokens: Vec<TokenTree>) -> HashMap<Ident, ConvRate> {
        let read_target = |token| match token {
            TokenTree::Ident(name) => name,
            _ => panic!("Can not read target name"),
        };

        let read_conv = |token| match token {
            TokenTree::Punct(p) => match p.as_char() {
                '^' => |s: &str| ConvRate::Expo(i8::from_string(s).unwrap()),
                '=' => |s: &str| ConvRate::Real(f64::from_string(s).unwrap()),
                c => panic!("Unsupported token: {}", c),
            },
            _ => panic!("Can not read token"),
        };

        let read_num = |token| match token {
            TokenTree::Literal(a) => Ok(a),
            TokenTree::Punct(a) => Err(a),
            _ => panic!("Can not read number"),
        };

        let mut result = HashMap::new();

        let mut ts = tokens.into_iter();
        while let Some(first) = ts.next() {
            let target = read_target(first);
            let mk_conv = read_conv(ts.next().expect("Can not read token"));
            let rate = match read_num(ts.next().expect("Can not read number")) {
                Ok(l) => l.to_string(),
                Err(p) => match read_num(ts.next().expect("Can not read number")) {
                    Ok(l) => format!("{}{}", p.as_char(), l.to_string()),
                    Err(_) => panic!("Can not read number"),
                },
            };
            result.insert(target, mk_conv(&rate));

            if let Some(c) = ts.next() {
                match c {
                    TokenTree::Punct(a) if a.as_char() == ',' => (),
                    _ => panic!("Unrecognized token: {}", c),
                }
            }
        }
        result
    }

    fn convert(&self, inner: &syn::Type, src: TokenStream) -> TokenStream {
        match *self {
            ConvRate::Expo(e) => {
                if e < 0 {
                    quote! {
                        let v = #src * (10u32.pow((#e).abs() as u32) as #inner);
                        v.into()
                    }
                } else {
                    quote! {
                        let v = #src / (10u32.pow(#e as u32) as #inner);
                        v.into()
                    }
                }
            }
            ConvRate::Real(rate) => quote! {
                let v = #src / (#rate as #inner);
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
            .flat_map(|t| match t {
                TokenTree::Group(g) => {
                    let gr = g.stream().into_iter().collect();
                    ConvRate::read_tokens(gr).into_iter()
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
            #[convertible(Km ^ 3, Milli ^ -3, Cm = 0.01)]
            struct Meter(f64);
        };
        let b = quote! {
            impl Convertible<Cm> for Meter {
                fn convert(&self) -> Cm {
                    let v = self.0 / (0.01f64 as f64);
                    v.into()
                }
            }
            impl Convertible<Km> for Meter {
                fn convert(&self) -> Km {
                    let v = self.0 / (10u32.pow(3i8 as u32) as f64);
                    v.into()
                }
            }
            impl Convertible<Milli> for Meter {
                fn convert(&self) -> Milli {
                    let v = self.0 * (10u32.pow((-3i8).abs() as u32) as f64);
                    v.into()
                }
            }
        };
        assert_eq!(convertible(a).to_string(), b.to_string());
    }

    #[test]
    fn write_empty() {
        let a = quote! {
            struct MyUnit(f64);
        };
        let b = quote! {
            #[convertible]
            struct MyUnit(f64);
        };
        let c = quote! {
            #[convertible()]
            struct MyUnit(f64);
        };

        assert!(convertible(a).to_string().is_empty());
        assert!(convertible(b).to_string().is_empty());
        assert!(convertible(c).to_string().is_empty());
    }
}
