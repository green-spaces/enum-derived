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
/// # Custom Generating Functions
///
/// Once a variant is selected, its generating function is called. By default the rand crate is used to generate values need by the enum variant.
///
/// To overwrite the default generating function, or add one where none exists, the `#[custom_rand(func_name)]` attribute can be used.
///
/// `func_name` must implement `Fn() -> Self` where Self is the enum. It is expected that the function will return a single variant but is not required.
///
/// # Variant Weight
///
/// By default, every variant is equally likely to be randomly generated.
///
/// The probablibility of a variant being crated can be changed using the `#[weight(u64)]` attribute. The default weight for a variant is one.

///
/// # Example
///
///  Custom rand functions are required for non-primitive types, such as a String. Here we wrote `fn rand_string` and use it as the generating function for `Example::RandString`.
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
///
/// # Example
///
/// Changing the frequency that a variant is created at is often needed. In this example we set `Example::NeverHappens` wight to zero, thus preventing it from ever being generated.
///
/// Setting `Example::TwiceAsLikely`'s weight to two means it will be generated twice as ofter as `Example::DefaultWeight`.
///
/// ```rust
/// use enum_derived::Rand;
///
/// #[derive(Rand)]
/// pub enum Example {
///     #[weight(0)]
///     NeverHappens,
///     DefaultWight,
///     #[weight(2)]
///     TwiceAsLikely,
/// }
/// ```
///
#[proc_macro_derive(Rand, attributes(custom_rand, weight))]
pub fn derive_rand(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);

    rand::expand_derive_rand(&mut input).unwrap_or_else(to_compile_errors)
}

fn to_compile_errors(errors: Vec<syn::Error>) -> proc_macro::TokenStream {
    let compile_errors = errors.iter().map(syn::Error::to_compile_error);
    TokenStream::from(quote!(#(#compile_errors)*))
}
