extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

/// Rand creates an associated rand method that returns a random variant of the enum when called
#[proc_macro_derive(Rand)]
pub fn random_enum(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let variants = match input.data {
        syn::Data::Enum(data) => data.variants.into_iter().collect::<Vec<_>>(),
        _ => panic!("Only enums are supported"),
    };

    let expanded = quote! {
        impl #name {
            pub fn rand() -> Self {
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
