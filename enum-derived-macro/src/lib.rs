// #![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]
extern crate proc_macro;

mod rand;

use proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Rand, attributes(custom_rand, weight))]
pub fn derive_rand(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);

    rand::expand_derive_rand(&mut input).unwrap_or_else(to_compile_errors)
}

fn to_compile_errors(errors: Vec<syn::Error>) -> proc_macro::TokenStream {
    let compile_errors = errors.iter().map(syn::Error::to_compile_error);
    TokenStream::from(quote!(#(#compile_errors)*))
}
