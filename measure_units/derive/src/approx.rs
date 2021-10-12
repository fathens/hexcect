use crate::common::*;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Generics, Ident, Type};

pub fn derive(items: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse2(items).unwrap();
    let name = ast.ident;
    let (inner_type, phantoms) = newtype_with_phantoms(&ast.data)
        .unwrap_or_else(|| panic!("{} is not newtype struct.", name));

    let gp = if ast.generics.params.is_empty() {
        None
    } else {
        Some(clean_generics(&ast.generics))
    };

    let params = Params {
        name,
        inner_type,
        phantoms,
        generics: ast.generics,
        gparams: gp,
    };

    let absdiff = impl_absdiff(&params);
    let relative = impl_relative(&params);
    let ulps = impl_ulps(&params);

    TokenStream::from_iter([absdiff, relative, ulps])
}

struct Params {
    name: Ident,
    inner_type: Type,
    phantoms: Vec<Type>,
    generics: Generics,
    gparams: Option<TokenStream>,
}

impl Params {
    pub fn all_ref(&self) -> (&Ident, &Type, &Vec<Type>, &Generics) {
        (&self.name, &self.inner_type, &self.phantoms, &self.generics)
    }
}

fn impl_absdiff(params: &Params) -> TokenStream {
    let (name, inner_type, phantoms, generics) = params.all_ref();

    let body = quote! {
        type Epsilon = <#inner_type as approx::AbsDiffEq>::Epsilon;

        fn default_epsilon() -> Self::Epsilon {
            #inner_type::default_epsilon()
        }

        fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
            self.0.abs_diff_eq(&other.0, epsilon)
        }
    };

    if let Some(gparams) = &params.gparams {
        let partial_eqs = TokenStream::from_iter(phantoms.iter().map(|p| {
            quote! {
                #p: core::cmp::PartialEq,
            }
        }));
        quote! {
            impl #generics approx::AbsDiffEq for #name #gparams
            where
                #inner_type: approx::AbsDiffEq,
                #partial_eqs
            {
                #body
            }
        }
    } else {
        quote! {
            impl approx::AbsDiffEq for #name {
                #body
            }
        }
    }
}

fn impl_relative(params: &Params) -> TokenStream {
    let (name, inner_type, phantoms, generics) = params.all_ref();

    let body = quote! {
        fn default_max_relative() -> <#inner_type as approx::AbsDiffEq>::Epsilon {
            #inner_type::default_max_relative()
        }

        fn relative_eq(
            &self,
            other: &Self,
            epsilon: <#inner_type as approx::AbsDiffEq>::Epsilon,
            max_relative: <#inner_type as approx::AbsDiffEq>::Epsilon,
        ) -> bool {
            self.0.relative_eq(&other.0, epsilon, max_relative)
        }
    };

    if let Some(gparams) = &params.gparams {
        let partial_eqs = TokenStream::from_iter(phantoms.iter().map(|p| {
            quote! {
                #p: core::cmp::PartialEq,
            }
        }));
        quote! {
            impl #generics approx::RelativeEq for #name #gparams
            where
                #inner_type: approx::RelativeEq,
                #partial_eqs
            {
                #body
            }
        }
    } else {
        quote! {
            impl approx::RelativeEq for #name {
                #body
            }
        }
    }
}

fn impl_ulps(params: &Params) -> TokenStream {
    let (name, inner_type, phantoms, generics) = params.all_ref();

    let body = quote! {
        fn default_max_ulps() -> u32 {
            #inner_type::default_max_ulps()
        }

        fn ulps_eq(
            &self,
            other: &Self,
            epsilon: <#inner_type as approx::AbsDiffEq>::Epsilon,
            max_ulps: u32,
        ) -> bool {
            self.0.ulps_eq(&other.0, epsilon, max_ulps)
        }
    };

    if let Some(gparams) = &params.gparams {
        let partial_eqs = TokenStream::from_iter(phantoms.iter().map(|p| {
            quote! {
                #p: core::cmp::PartialEq,
            }
        }));
        quote! {
            impl #generics approx::UlpsEq for #name #gparams
            where
                #inner_type: approx::UlpsEq,
                #partial_eqs
            {
                #body
            }
        }
    } else {
        quote! {
            impl approx::UlpsEq for #name {
                #body
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_approx() {
        let a = quote! {
            struct MyUnit(f64);
        };

        let b = quote! {
            impl approx::AbsDiffEq for MyUnit {
                type Epsilon = <f64 as approx::AbsDiffEq>::Epsilon;
                fn default_epsilon() -> Self::Epsilon {
                    f64::default_epsilon()
                }
                fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
                    self.0.abs_diff_eq(&other.0, epsilon)
                }
            }
            impl approx::RelativeEq for MyUnit {
                fn default_max_relative() -> <f64 as approx::AbsDiffEq>::Epsilon {
                    f64::default_max_relative()
                }
                fn relative_eq(
                    &self,
                    other: &Self,
                    epsilon: <f64 as approx::AbsDiffEq>::Epsilon,
                    max_relative: <f64 as approx::AbsDiffEq>::Epsilon,
                ) -> bool {
                    self.0.relative_eq(&other.0, epsilon, max_relative)
                }
            }
            impl approx::UlpsEq for MyUnit {
                fn default_max_ulps() -> u32 {
                    f64::default_max_ulps()
                }
                fn ulps_eq(
                    &self,
                    other: &Self,
                    epsilon: <f64 as approx::AbsDiffEq>::Epsilon,
                    max_ulps: u32,
                ) -> bool {
                    self.0.ulps_eq(&other.0, epsilon, max_ulps)
                }
            }
        };
        assert_eq!(derive(a).to_string(), b.to_string());
    }

    #[test]
    fn generics_approx() {
        let a = quote! {
            struct MyGenerics<V, A>(V, PhantomData<A>);
        };

        let b = quote! {
            impl<V, A> approx::AbsDiffEq for MyGenerics<V, A>
            where
                V: approx::AbsDiffEq,
                A: core::cmp::PartialEq,
            {
                type Epsilon = <V as approx::AbsDiffEq>::Epsilon;
                fn default_epsilon() -> Self::Epsilon {
                    V::default_epsilon()
                }
                fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
                    self.0.abs_diff_eq(&other.0, epsilon)
                }
            }
            impl<V, A> approx::RelativeEq for MyGenerics<V, A>
            where
                V: approx::RelativeEq,
                A: core::cmp::PartialEq,
            {
                fn default_max_relative() -> <V as approx::AbsDiffEq>::Epsilon {
                    V::default_max_relative()
                }
                fn relative_eq(
                    &self,
                    other: &Self,
                    epsilon: <V as approx::AbsDiffEq>::Epsilon,
                    max_relative: <V as approx::AbsDiffEq>::Epsilon,
                ) -> bool {
                    self.0.relative_eq(&other.0, epsilon, max_relative)
                }
            }
            impl<V, A> approx::UlpsEq for MyGenerics<V, A>
            where
                V: approx::UlpsEq,
                A: core::cmp::PartialEq,
            {
                fn default_max_ulps() -> u32 {
                    V::default_max_ulps()
                }
                fn ulps_eq(
                    &self,
                    other: &Self,
                    epsilon: <V as approx::AbsDiffEq>::Epsilon,
                    max_ulps: u32,
                ) -> bool {
                    self.0.ulps_eq(&other.0, epsilon, max_ulps)
                }
            }
        };
        assert_eq!(derive(a).to_string(), b.to_string());
    }

    #[test]
    #[should_panic(expected = "Support only `PhantomData`")]
    fn not_phantom_simple_approx() {
        derive(quote! {
            struct MyAbc(f32, f64);
        });
    }

    #[test]
    #[should_panic(expected = "Support only `PhantomData`")]
    fn not_phantom_generics_approx() {
        derive(quote! {
            struct MyAbc<A, B>(A, B);
        });
    }

    #[test]
    #[should_panic(expected = "MyAbc is not newtype struct.")]
    fn not_newtype_simple_approx() {
        derive(quote! {
            struct MyAbc {
                a: f32,
                b: f64,
            }
        });
    }

    #[test]
    #[should_panic(expected = "MyAbc is not newtype struct.")]
    fn not_newtype_unit_approx() {
        derive(quote! {
            struct MyAbc;
        });
    }

    #[test]
    #[should_panic(expected = "MyAbc is not newtype struct.")]
    fn not_newtype_generics_approx() {
        derive(quote! {
            struct MyAbc<A, B> {
                a: A,
                b: B,
            }
        });
    }

    #[test]
    #[should_panic(expected = "MyAbc is not newtype struct.")]
    fn not_struct_approx() {
        derive(quote! {
            enum MyAbc {
                A,
            }
        });
    }
}
