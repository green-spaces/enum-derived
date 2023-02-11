#![doc = include_str!("../crates-io.md")]
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

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

    // for v in variants.iter() {
    //     match &v.fields {
    //         Fields::Unit => continue,
    //         _e => panic!(
    //             "enums with variants containing fields are not supported.\n{}::{} has a field",
    //             name, v.ident
    //         ),
    //     }
    // }

    let expanded = quote! {
        impl #name {
            fn rand() -> Self {
                use ::rand::{thread_rng, Rng};

                let mut discriminates = vec![#( ::std::mem::discriminant(&#name::#variants)),*];
                let mut rng = thread_rng();
                let idx: usize = rng.gen_range(0..discriminates.len());
                let selected_discriminate = discriminates.swap_remove(idx);
                match selected_discriminate {
                    #( x if x == ::std::mem::discriminant(&#name::#variants) => #name::#variants),* ,
                    _ => panic!("Unreachable"),
                }
            }
        }
    };

    TokenStream::from(expanded)
}
