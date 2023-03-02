use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Fields, Ident, Variant};

pub fn expand_derive_rand(input: &mut DeriveInput) -> Result<TokenStream, Vec<syn::Error>> {
    // Support only Enums
    let syn::Data::Enum(ref data) = input.data else { panic!("Only enums are supported") };

    // Cannot be an empty variant
    if data.variants.is_empty() {
        panic!("Enum must have at least one variant defined");
    }

    let variant_gen = |v: &Variant| variant_rand_func_generator(&input.ident, v);
    let var_rand_funcs = data.variants.iter().map(variant_gen);

    let weights = variant_weights_collector(data.variants.iter().cloned().collect::<Vec<_>>());

    let enum_name = &input.ident;
    let expanded = quote! {
        impl ::enum_derived::Rand for #enum_name {
            fn rand() -> Self {
                use ::rand::{thread_rng, Rng, distributions::{WeightedIndex, Distribution}};

                let mut random_enums: Vec<Box<dyn Fn() -> Self>> = vec![#(#var_rand_funcs),*];
                let enum_weights = vec![#(#weights),*];
                let dist = WeightedIndex::new(&enum_weights).unwrap();

                let enum_idx: usize = dist.sample(&mut thread_rng());
                (*random_enums.swap_remove(enum_idx))()
            }
        }
    };

    Ok(TokenStream::from(expanded))
}

// Gets the weight cutoff for a variant
fn variant_weights_collector(variants: Vec<Variant>) -> Vec<proc_macro2::TokenStream> {
    variants
        .iter()
        .map(|v| {
            for attr in v.attrs.iter() {
                if attr.path.get_ident().unwrap() == "weight" {
                    return attr.tokens.clone();
                }
            }

            let default_weight = quote! {
                1
            };
            default_weight
        })
        .collect()
}

/// Creates the rand function for a vartiant
fn variant_rand_func_generator(enum_name: &Ident, variant: &Variant) -> proc_macro2::TokenStream {
    // Check for custom function
    for attr in variant.attrs.iter() {
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

    let var_ident = &variant.ident;

    // Default random function
    let rand_func = match &variant.fields {
        Fields::Unit => {
            quote! {
                    ::std::boxed::Box::new(|| {
                        #enum_name::#var_ident
                })
            }
        }
        Fields::Unnamed(unnamed_fields) => {
            let fields_types = unnamed_fields.unnamed.iter().map(|f| &f.ty);
            quote! {
                    ::std::boxed::Box::new(|| {
                        // #enum_name::#var_ident(#(::rand::random::<#fields_types>()),*)
                        #enum_name::#var_ident(#(<#fields_types as Rand>::rand()),*)
                })
            }
        }
        Fields::Named(named_fields) => {
            let fields_types = named_fields.named.iter().map(|f| &f.ty);
            let fields_ident = named_fields.named.iter().map(|f| f.ident.clone().unwrap());
            quote! {
                    ::std::boxed::Box::new(|| {
                        // #enum_name::#var_ident{ #(#fields_ident: ::rand::random::<#fields_types>()),* }
                        #enum_name::#var_ident{ #(#fields_ident: <#fields_types as Rand>::rand()),* }
                })
            }
        }
    };
    rand_func
}
