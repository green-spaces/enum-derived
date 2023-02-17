#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]
extern crate proc_macro;

mod rand;

use proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input, DeriveInput};

/// [Rand] derives a `Self::rand` method with the same signature as this Random trait.
///
/// ```
/// pub trait Random {
///
///     /// Generates a random variant of the enum it is implemented for
///     fn rand() -> Self;
/// }
///
/// ```
///
/// A custom function, using the `#[custom_rand(func_name)]` attribute, can be used for overwrite the default behavior or extend the functionality to types without a default implementation.
///
/// `func_name` must implement `Fn() -> Self` where Self is the enum. It is expected that the function will return a single variant but is not required.
///
/// # Example
///
/// ```rust
/// use enum_derived::Rand;
///
/// #[derive(Rand)]
/// pub enum Example {
///     Empty,
///     Boolean(bool),
///     #[custom_rand(rand_string)]
///     RandString(String),
/// }
///
/// /// A custom function used for generating the RandString variant
/// fn rand_string() -> Example {
///     let unique_str = format!("{:?}", std::time::SystemTime::now());
///     Example::RandString(unique_str)
/// }
/// ```
#[proc_macro_derive(Rand, attributes(custom_rand))]
pub fn derive_rand(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);

    rand::expand_derive_rand(&mut input).unwrap_or_else(to_compile_errors)
}

fn to_compile_errors(errors: Vec<syn::Error>) -> proc_macro::TokenStream {
    let compile_errors = errors.iter().map(syn::Error::to_compile_error);
    TokenStream::from(quote!(#(#compile_errors)*))
}
