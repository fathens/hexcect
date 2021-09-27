use proc_macro2::{Group, TokenTree};
use std::iter::Peekable;
use syn::{Data, Fields, Type};

/// If `data` is a newtype, return the type it's wrapping.
pub fn newtype_inner(data: &Data) -> Option<Type> {
    match data {
        Data::Struct(ref s) => match s.fields {
            Fields::Unnamed(ref fs) => {
                if fs.unnamed.len() == 1 {
                    Some(fs.unnamed[0].ty.clone())
                } else {
                    None
                }
            }
            _ => None,
        },
        _ => None,
    }
}

/// `data` が newtype に PhantomData が付いているなら、それらの型を返す。
pub fn newtype_with_phantoms(data: &Data) -> Option<(Type, Vec<Type>)> {
    fn only_phantom(ty: &Type) -> Option<&Type> {
        match ty {
            Type::Path(t) => {
                let phantom = t.path.segments.last().expect("Should not be empty");
                if phantom.ident == "PhantomData" {
                    match &phantom.arguments {
                        syn::PathArguments::AngleBracketed(ab) => {
                            match ab
                                .args
                                .first()
                                .expect("PhantomData should have a generic type.")
                            {
                                syn::GenericArgument::Type(t) => Some(t),
                                a => panic!("Unsuppoted args: {:?}", a),
                            }
                        }
                        a => panic!("Unsuppoted args: {:?}", a),
                    }
                } else {
                    panic!("Suppor only `PhantomData`");
                }
            }
            t => panic!("Unsupported type: {:?}", t),
        }
    }
    match data {
        Data::Struct(s) => match &s.fields {
            Fields::Unnamed(fs) => {
                let mut types = fs.unnamed.iter().map(|f| &f.ty);
                types.next().map(|inner_type| {
                    let ts = types.flat_map(only_phantom);
                    (inner_type.clone(), ts.cloned().collect())
                })
            }
            _ => None,
        },
        _ => None,
    }
}

pub fn read_attr_args(attr: syn::Attribute) -> impl Iterator<Item = TokenTree> {
    let mut gs = attr.tokens.into_iter().map(|t| match t {
        TokenTree::Group(g) => g,
        _ => panic!("Unexpected token: {:?}", t),
    });

    if let Some(g) = gs.next() {
        if gs.next().is_some() {
            panic!("Only one argument must be supplied.");
        }
        g.stream().into_iter()
    } else {
        panic!("Least one argument must be supplied.");
    }
}

pub fn skip_comma<I>(ts: &mut I)
where
    I: Iterator<Item = TokenTree>,
{
    match ts.next() {
        Some(TokenTree::Punct(p)) if p.as_char() == ',' => (),
        Some(a) => panic!("Unexpected token: {:?}", a),
        None => (),
    }
}

pub fn read_array<I>(ts: &mut I) -> Vec<syn::Ident>
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

pub fn read_agroup<I>(key: &str, ts: &mut Peekable<I>) -> Option<Group>
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

pub fn read_expr<I>(
    ts: &mut Peekable<I>,
    key: Option<&str>,
) -> Option<(syn::Ident, char, Vec<TokenTree>)>
where
    I: Iterator<Item = TokenTree>,
{
    match ts.peek() {
        Some(TokenTree::Ident(me)) if key.map(|k| me == k).unwrap_or(true) => {
            if let Some(TokenTree::Ident(name)) = ts.next() {
                match ts.next() {
                    Some(TokenTree::Punct(p)) => Some((name, p.as_char(), ts.collect())),
                    a => panic!("Unsupported token {:?}", a),
                }
            } else {
                panic!("Should not come here.");
            }
        }
        _ => None,
    }
}
