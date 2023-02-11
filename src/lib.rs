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

    let variants = match input.data {
        syn::Data::Enum(data) => data.variants.into_iter().collect::<Vec<_>>(),
        _ => panic!("Only enums with unit-like variants are supported"),
    };

    if variants.is_empty() {
        panic!("Enum must have at least one variant defined");
    }

    for v in variants.iter() {
        match &v.fields {
            Fields::Unit => continue,
            _e => panic!(
                "enums with variants containing fields are not supported.\n{}::{} has a field",
                name, v.ident
            ),
        }
    }

    let expanded = quote! {
        impl #name {
            fn rand() -> Self {
                use #name::{#(#variants),*};
                use ::rand::{thread_rng, seq::SliceRandom};

                let mut samples = vec![#(#variants),*];
                let mut rng = thread_rng();
                samples.shuffle(&mut rng);
                samples.pop().unwrap()
            }
        }
    };
    TokenStream::from(expanded)
}
