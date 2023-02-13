#![doc = include_str!("../README.md")]
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Fields, Ident};

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
/// A custom random function, using the `#[custom_rand(func_name)]` can be used for overwrite the default behavior or extend the functionality to types without a default implementation.
///
/// The custom function must have a signature of the form `Fn() -> Self` where Self is the enum. It is expected that the custom function will return a single variant but not required.
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
pub fn random_enum(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    // Support only Enums
    let variants = match input.data {
        syn::Data::Enum(data) => data.variants.into_iter().collect::<Vec<_>>(),
        _ => panic!("Only enums with unit-like variants are supported"),
    };

    // Cannot be an empty variant
    if variants.is_empty() {
        panic!("Enum must have at least one variant defined");
    }

    let variant_generators = variants
        .into_iter()
        .map(|v| {
            let var_ident = v.ident;

            for attr in v.attrs.iter() {
                let Some(ident) = attr.path.get_ident() else { continue; };
                // Allow for custom over ride functions to be used 
                if ident == "custom_rand" {
                    let Ok(func_name) = attr.parse_args::<Ident>() else {continue};
                    return quote! {
                        ::std::boxed::Box::new(|| {
                            #func_name()
                    })
                };
                }
            }

            match v.fields {
                Fields::Unit => {
                    quote! {
                            ::std::boxed::Box::new(|| {
                                #name::#var_ident
                        })
                    }
                }
                Fields::Unnamed(unnamed_fields) => {
                    let fields_types = unnamed_fields.unnamed.iter().map(|f| f.ty.clone()).collect::<Vec<_>>();
                    quote! {
                            ::std::boxed::Box::new(|| {
                                #name::#var_ident(#(::rand::random::<#fields_types>()),*)
                        })
                    }
                }
                Fields::Named(named_fields) => {
                    let fields_types = named_fields.named.iter().map(|f| f.ty.clone()).collect::<Vec<_>>();
                    let fields_ident = named_fields.named.iter().map(|f| f.ident.clone().unwrap()).collect::<Vec<_>>();
                    quote! {
                            ::std::boxed::Box::new(|| {
                                #name::#var_ident{ #(#fields_ident: ::rand::random::<#fields_types>()),* }
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
