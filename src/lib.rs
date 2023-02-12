#![doc = include_str!("../crates-io.md")]
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Fields};

/// [Rand] generates random variants of an enum
///
///
/// [Rand] derives a method with the same signature as this Random trait.
/// ```
/// pub trait Random {
///
///     /// Generates a random variant of the enum it is implemented for
///     fn rand() -> Self;
/// }
/// ```
#[proc_macro_derive(Rand)]
pub fn random_enum(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    // Support only Enums
    let variants = match input.data {
        syn::Data::Enum(data) => data.variants.into_iter().collect::<Vec<_>>(),
        _ => panic!("Only enums with unit-like variants are supported"),
    };

    // Cannot be an empty veriant
    if variants.is_empty() {
        panic!("Enum must have at least one variant defined");
    }

    let variant_generators = variants
        .into_iter()
        .map(|v| {
            let var_ident = v.ident;
            match v.fields {
                Fields::Unit => {
                    quote! {
                            ::std::boxed::Box::new(|| {
                                #name::#var_ident
                        })
                    }
                }
                _ => {
                    let fields = v.fields.iter().collect::<Vec<_>>();
                    quote! {
                            ::std::boxed::Box::new(|| {
                                #name::#var_ident(#(rand::random::<#fields>()),*)
                        })
                    }
                }
            }
        })
        .collect::<Vec<_>>();

    let expanded = quote! {
        impl #name {
            fn rand() -> Self {
                use ::rand::{thread_rng, Rng};

                let mut random_enums: Vec<Box<dyn Fn() -> Self>> = vec![#(#variant_generators),*];
                let mut rng = thread_rng();
                let idx: usize = rng.gen_range(0..random_enums.len());
                (*random_enums.swap_remove(idx))()
            }
        }
    };

    TokenStream::from(expanded)
}
