use syn::{Data, Fields, Type};

/// If `data` is a newtype, return the type it's wrapping.
pub fn newtype_inner(data: &Data) -> Option<Type> {
    match *data {
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
    match *data {
        Data::Struct(ref s) => match s.fields {
            Fields::Unnamed(ref fs) => {
                if !fs.unnamed.is_empty() {
                    let mut fields = fs.unnamed.iter();
                    let base_type = fields.next().expect("Should not empty.").ty.clone();
                    let mut ts = Vec::new();
                    for field in fields {
                        match field.ty.clone() {
                            syn::Type::Path(t) => {
                                let phantom = t.path.segments.last().expect("Should not be empty");
                                if phantom.ident == "PhantomData" {
                                    match phantom.arguments.clone() {
                                        syn::PathArguments::AngleBracketed(ab) => {
                                            if ab.args.len() == 1 {
                                                let ga = ab.args[0].clone();
                                                match ga {
                                                    syn::GenericArgument::Type(t) => ts.push(t),
                                                    a => panic!("Unsuppoted args: {:?}", a),
                                                }
                                            } else {
                                                panic!("Only one type arg for `PhantomData`.");
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
                    Some((base_type, ts))
                } else {
                    None
                }
            }
            _ => None,
        },
        _ => None,
    }
}
