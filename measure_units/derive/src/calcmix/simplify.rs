use proc_macro2::{Ident, TokenStream, TokenTree};
use quote::quote;

pub fn simplify(items: TokenStream) -> TokenStream {
    let ast: TokenStream = syn::parse2(items).unwrap();
    let mut tokens = ast.into_iter();
    let src = take_src(&mut tokens);
    let g = parse_type(tokens);
    let mixed = Mixed::parse(g);
    let (_, ts) = mixed.simplify();

    quote! {
        #src #ts
    }
}

fn take_src<T>(ts: &mut T) -> Ident
where
    T: Iterator<Item = TokenTree>,
{
    if let Some(TokenTree::Ident(name)) = ts.next() {
        match ts.next() {
            Some(TokenTree::Punct(p)) if p.as_char() == ':' => name,
            a => panic!("Unexpected token: {:?}", a),
        }
    } else {
        panic!("Ident required first.");
    }
}

fn parse_type<T>(ts: T) -> syn::TypePath
where
    T: Iterator<Item = TokenTree>,
{
    let st = TokenStream::from_iter(ts);
    match syn::parse2(st).unwrap() {
        syn::Type::Path(ty) => ty,
        a => panic!("Unsuported type: {:?}", a),
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Mixed {
    Scalar,
    Mul(Box<Mixed>, Box<Mixed>),
    Div(Box<Mixed>, Box<Mixed>),
    Single(Ident),
}

impl Mixed {
    fn parse(root: syn::TypePath) -> Mixed {
        Mixed::parse_internal(root, None)
    }

    fn parse_internal(root: syn::TypePath, varg: Option<&syn::TypePath>) -> Mixed {
        fn only_typepath<T>(a: syn::GenericArgument, f: impl Fn(syn::TypePath) -> T) -> T {
            match a {
                syn::GenericArgument::Type(t) => match t {
                    syn::Type::Path(a) => f(a),
                    a => panic!("Unsupported type: {:?}", a),
                },
                a => panic!("Unsupported arg: {:?}", a),
            }
        }

        let get_args = |pa| match pa {
            syn::PathArguments::AngleBracketed(ab) => {
                let mut args = ab.args.into_iter();

                let v = &only_typepath(args.next().expect("Any type args not found."), |actual| {
                    if let Some(expected) = varg {
                        assert_eq!(expected, &actual);
                    }
                    actual
                });

                args.next().map(|a| {
                    let b = args.next().expect("Specify type B.");

                    let rec_parse = |t| Box::new(Mixed::parse_internal(t, Some(v)));
                    (only_typepath(a, rec_parse), only_typepath(b, rec_parse))
                })
            }
            a => panic!("Unsupported arg: {:?}", a),
        };

        let mut segs = root.path.segments.into_iter();
        let seg = segs.next().expect("Empty type not accepted.");
        if segs.next().is_some() {
            panic!("Support only single segment types.");
        }

        match seg.ident.to_string().as_str() {
            "Scalar" => {
                if get_args(seg.arguments).is_none() {
                    Mixed::Scalar
                } else {
                    panic!("Scalar has no more type args.");
                }
            }
            "UnitsMul" => {
                let (a, b) = get_args(seg.arguments).expect("Specify type A, B");
                Mixed::Mul(a, b)
            }
            "UnitsDiv" => {
                let (a, b) = get_args(seg.arguments).expect("Specify type A, B");
                Mixed::Div(a, b)
            }
            _ => Mixed::Single(seg.ident),
        }
    }

    fn simplify(self) -> (Mixed, TokenStream) {
        match self {
            Mixed::Mul(a, b) => {
                if *b == Mixed::Scalar {
                    let mut ts = quote! { .scalar() };
                    let (next, more) = a.simplify();
                    ts.extend(more);
                    (next, ts)
                } else if *a == Mixed::Scalar {
                    let mut ts = quote! { .commutative() };
                    let next = Mixed::Mul(b, a);
                    let (next, more) = next.simplify();
                    ts.extend(more);
                    (next, ts)
                } else {
                    match (*a, *b) {
                        (Mixed::Div(x, y), b) if *y == b => {
                            let mut ts = quote! { .reduction() };
                            let (next, more) = x.simplify();
                            ts.extend(more);
                            (next, ts)
                        }
                        (a, Mixed::Div(x, y)) if *y == a => {
                            let mut ts = quote! { .commutative() };
                            let next = Mixed::Mul(Box::new(Mixed::Div(x, y)), Box::new(a));
                            let (next, more) = next.simplify();
                            ts.extend(more);
                            (next, ts)
                        }
                        (left, right) => {
                            let (next_left, more_left) = left.simplify();
                            let (next_right, more_right) = right.simplify();

                            let mut ts = TokenStream::new();
                            let next = Mixed::Mul(Box::new(next_left), Box::new(next_right));

                            if more_left.is_empty() && more_right.is_empty() {
                                (next, ts)
                            } else {
                                if !more_left.is_empty() {
                                    ts.extend(quote! {
                                        .inner_left(|a| a #more_left)
                                    });
                                }
                                if !more_right.is_empty() {
                                    ts.extend(quote! {
                                        .inner_right(|a| a #more_right)
                                    });
                                }
                                let (next, more) = next.simplify();
                                ts.extend(more);
                                (next, ts)
                            }
                        }
                    }
                }
            }
            Mixed::Div(a, b) => {
                if a == b {
                    (Mixed::Scalar, quote! { .reduction() })
                } else if *b == Mixed::Scalar {
                    let mut ts = quote! { .scalar() };
                    let (next, more) = a.simplify();
                    ts.extend(more);
                    (next, ts)
                } else {
                    match *a {
                        Mixed::Mul(x, y) if y == b => {
                            let mut ts = quote! { .reduction_right() };
                            let (next, more) = x.simplify();
                            ts.extend(more);
                            (next, ts)
                        }
                        Mixed::Mul(x, y) if x == b => {
                            let mut ts = quote! { .reduction_left() };
                            let (next, more) = y.simplify();
                            ts.extend(more);
                            (next, ts)
                        }
                        left => {
                            let (next_left, more_left) = left.simplify();
                            let (next_right, more_right) = (*b).simplify();

                            let mut ts = TokenStream::new();
                            let next = Mixed::Div(Box::new(next_left), Box::new(next_right));

                            if more_left.is_empty() && more_right.is_empty() {
                                (next, ts)
                            } else {
                                if !more_left.is_empty() {
                                    ts.extend(quote! {
                                        .inner_left(|a| a #more_left)
                                    });
                                }
                                if !more_right.is_empty() {
                                    ts.extend(quote! {
                                        .inner_right(|a| a #more_right)
                                    });
                                }
                                let (next, more) = next.simplify();
                                ts.extend(more);
                                (next, ts)
                            }
                        }
                    }
                }
            }
            Mixed::Single(a) => (Mixed::Single(a), TokenStream::new()),
            Mixed::Scalar => (Mixed::Scalar, TokenStream::new()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mul_reduction() {
        let a = quote! {
            src: UnitsMul<f64, UnitsDiv<f64, Meter, Second>, Second>
        };
        let b = quote! {
            src.reduction()
        };
        assert_eq!(simplify(a).to_string(), b.to_string());
    }

    #[test]
    fn mul_commutative_reduction() {
        let a = quote! {
            src: UnitsMul<f64, Second, UnitsDiv<f64, Meter, Second>>
        };
        let b = quote! {
            src.commutative().reduction()
        };
        assert_eq!(simplify(a).to_string(), b.to_string());
    }

    #[test]
    fn mul_scalar() {
        let a = quote! {
            src: UnitsMul<f64, UnitsDiv<f64, Meter, Second>, Scalar<f64>>
        };
        let b = quote! {
            src.scalar()
        };
        assert_eq!(simplify(a).to_string(), b.to_string());
    }

    #[test]
    fn mul_commutative_scalar() {
        let a = quote! {
            src: UnitsMul<f64, Scalar<f64>, UnitsDiv<f64, Meter, Second>>
        };
        let b = quote! {
            src.commutative().scalar()
        };
        assert_eq!(simplify(a).to_string(), b.to_string());
    }

    #[test]
    fn div_reduction() {
        let a = quote! {
            src: UnitsDiv<f64, Meter, Meter>
        };
        let b = quote! {
            src.reduction()
        };
        assert_eq!(simplify(a).to_string(), b.to_string());
    }

    #[test]
    fn div_scalar() {
        let a = quote! {
            src: UnitsDiv<f64, Meter, Scalar<f64>>
        };
        let b = quote! {
            src.scalar()
        };
        assert_eq!(simplify(a).to_string(), b.to_string());
    }

    #[test]
    fn div_reduction_right() {
        let a = quote! {
            src: UnitsDiv<f64, UnitsMul<f64, Second, Meter>, Meter>
        };
        let b = quote! {
            src.reduction_right()
        };
        assert_eq!(simplify(a).to_string(), b.to_string());
    }

    #[test]
    fn div_reduction_left() {
        let a = quote! {
            src: UnitsDiv<f64, UnitsMul<f64, Meter, Second>, Meter>
        };
        let b = quote! {
            src.reduction_left()
        };
        assert_eq!(simplify(a).to_string(), b.to_string());
    }

    #[test]
    fn mul_inner_right_div_reduction_left() {
        let a = quote! {
            src: UnitsMul<f64, Second, UnitsDiv<f64, UnitsMul<f64, Meter, Second>, Meter>>
        };
        let b = quote! {
            src.inner_right(|a| a.reduction_left())
        };
        assert_eq!(simplify(a).to_string(), b.to_string());
    }

    #[test]
    fn mul_inner_left_div_reduction_left() {
        let a = quote! {
            src: UnitsMul<f64, UnitsDiv<f64, UnitsMul<f64, Meter, Second>, Meter>, Second>
        };
        let b = quote! {
            src.inner_left(|a| a.reduction_left())
        };
        assert_eq!(simplify(a).to_string(), b.to_string());
    }

    #[test]
    fn div_inner_right_div_reduction_left() {
        let a = quote! {
            src: UnitsDiv<f64, Meter, UnitsDiv<f64, UnitsMul<f64, Meter, Second>, Meter>>
        };
        let b = quote! {
            src.inner_right(|a| a.reduction_left())
        };
        assert_eq!(simplify(a).to_string(), b.to_string());
    }

    #[test]
    fn div_inner_left_div_reduction_left() {
        let a = quote! {
            src: UnitsDiv<f64, UnitsDiv<f64, UnitsMul<f64, Meter, Second>, Meter>, Meter>
        };
        let b = quote! {
            src.inner_left(|a| a.reduction_left())
        };
        assert_eq!(simplify(a).to_string(), b.to_string());
    }

    #[test]
    fn nested_inner() {
        let a = quote! {
            src: UnitsMul<f64,
                          UnitsMul<f64,
                                   UnitsDiv<f64,
                                            UnitsDiv<f64,
                                                     UnitsMul<f64,
                                                              Meter,
                                                              Second>,
                                                     Meter>,
                                            UnitsDiv<f64,
                                                     Meter,
                                                     UnitsMul<f64,
                                                              Second,
                                                              UnitsDiv<f64,
                                                                       Meter,
                                                                       Second>>>>,
                                           Second>,
                                  UnitsDiv<f64,
                                           Meter,
                                           UnitsDiv<f64,
                                                    UnitsMul<f64,
                                                             Meter,
                                                             Second>,
                                                    Meter>>>
        };
        let b = quote! {
            src.inner_left(|a|
                           a.inner_left (|a|
                                         a.inner_left (|a|
                                                       a.reduction_left ()
                                         ).inner_right(|a|
                                                       a.inner_right (|a|
                                                                      a.commutative().reduction()
                                                       ).reduction()
                                         ).scalar ()
                           )
            ).inner_right(|a|
                          a.inner_right (|a|
                                         a.reduction_left ()
                          )
            )
        };
        assert_eq!(simplify(a).to_string(), b.to_string());
    }
}
