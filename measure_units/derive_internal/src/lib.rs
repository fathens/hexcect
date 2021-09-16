mod float_status;

pub use float_status::*;

// ----------------------------------------------------------------
use syn::{Data, Fields, Type};

/// If `data` is a newtype, return the type it's wrapping.
fn newtype_inner(data: &Data) -> Option<Type> {
    match *data {
        Data::Struct(ref s) => {
            match s.fields {
                Fields::Unnamed(ref fs) => {
                    if fs.unnamed.len() == 1 {
                        Some(fs.unnamed[0].ty.clone())
                    } else {
                        None
                    }
                }
                Fields::Named(ref fs) => {
                    if fs.named.len() == 1 {
                        panic!("num-derive doesn't know how to handle newtypes with named fields yet. \
                                Please use a tuple-style newtype, or submit a PR!");
                    }
                    None
                }
                _ => None,
            }
        }
        _ => None,
    }
}
