use super::*;

use darling::*;
use proc_macro2::{Ident, Literal, Punct, TokenStream, TokenTree};
use quote::quote;
use std::{collections::HashMap, slice::Iter};
use syn::{Attribute, DeriveInput};

pub fn convertible(items: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse2(items).unwrap();
    let option = ConOpt::from_derive_input(&ast).unwrap();
    let name = ast.ident;
    let inner_type = newtype_inner(&ast.data).expect(&format!("{} is not newtype struct.", name));

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
    fn parse_tokens(tokens: Vec<TokenTree>) -> HashMap<Ident, ConvRate> {
        fn read_target(token: &TokenTree) -> Ident {
            match token {
                TokenTree::Ident(name) => name.clone(),
                _ => panic!("Can not read target name"),
            }
        }

        fn read_conv(token: &TokenTree) -> impl Fn(&str) -> ConvRate {
            match token {
                TokenTree::Punct(p) => match p.as_char() {
                    '^' => |s: &str| ConvRate::Expo(i8::from_string(s).unwrap()),
                    '=' => |s: &str| ConvRate::Real(f64::from_string(s).unwrap()),
                    c => panic!("Unsupported token: {}", c),
                },
                _ => panic!("Can not read token"),
            }
        }

        fn read_num<'a>(token: &'a TokenTree) -> std::result::Result<&'a Literal, &'a Punct> {
            match token {
                TokenTree::Literal(a) => Ok(a),
                TokenTree::Punct(a) => Err(a),
                _ => panic!("Can not read number"),
            }
        }

        fn read_rate(ts: &mut Iter<TokenTree>) -> String {
            match read_num(ts.next().expect("Can not read number")) {
                Ok(l) => l.to_string(),
                Err(p) => match read_num(ts.next().expect("Can not read number")) {
                    Ok(l) => format!("{}{}", p.as_char(), l.to_string()),
                    Err(_) => panic!("Can not read number"),
                },
            }
        }

        let mut result = HashMap::new();

        let mut ts = tokens.iter();
        while let Some(first) = ts.next() {
            let target = read_target(first);
            let mk_conv = read_conv(ts.next().expect("Can not read token"));
            let rate = read_rate(&mut ts);
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

    fn convert(&self, inner: &Type, src: TokenStream) -> TokenStream {
        match self {
            &ConvRate::Expo(e) => {
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
            &ConvRate::Real(rate) => quote! {
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
    fn from_derive_input(ast: &DeriveInput) -> Result<ConOpt> {
        let mut convertible = HashMap::new();
        ast.attrs
            .iter()
            .filter(|a| a.path.is_ident("convertible"))
            .map(ConOpt::parse_convertible)
            .for_each(|h| convertible.extend(h));
        Ok(ConOpt { convertible })
    }

    fn parse_convertible(attr: &Attribute) -> HashMap<Ident, ConvRate> {
        let mut keys = HashMap::new();
        attr
            .tokens
            .clone()
            .into_iter()
            .for_each(|t| match t {
                TokenTree::Group(g) => {
                    let gr = g.stream().into_iter().collect();
                    let h = ConvRate::parse_tokens(gr);
                    keys.extend(h);
                }
                _ => panic!("Unexpected token: {:?}", t),
            });
        keys
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
}
