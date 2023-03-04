use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DataEnum, DataStruct, DataUnion, DeriveInput, Fields, Ident, Variant};

pub fn expand_derive_rand(input: &mut DeriveInput) -> Result<TokenStream, Vec<syn::Error>> {
    // println!("{input:#?}");

    match input.data {
        Data::Struct(ref data_struct) => expand_derive_rand_struct(&input.ident, data_struct),
        Data::Enum(ref data_enum) => expand_derive_rand_enum(&input.ident, data_enum),
        Data::Union(ref data_union) => expand_derive_rand_union(&input.ident, data_union),
    }
}

fn expand_derive_rand_struct(
    struct_name: &Ident,
    data_struct: &DataStruct,
) -> Result<TokenStream, Vec<syn::Error>> {
    // Create rand func for struct
    let rand_struct_func = match &data_struct.fields {
        Fields::Unit => {
            quote! {
                    ::std::boxed::Box::new(|| {
                        #struct_name
                })
            }
        }
        Fields::Unnamed(unnamed_fields) => {
            let fields_types = unnamed_fields.unnamed.iter().map(|f| &f.ty);
            quote! {
                    ::std::boxed::Box::new(|| {
                        #struct_name(#(<#fields_types as ::enum_derived::Rand>::rand()),*)
                })
            }
        }
        Fields::Named(named_fields) => {
            // let fields_types = named_fields.named.iter().map(|f| &f.ty);
            let fields_ident = named_fields.named.iter().map(|f| f.ident.clone().unwrap());
            let fields_rand_funcs = named_fields.named.iter().map(|f| {
                // Use custom_rand if available
                for attr in f.attrs.iter() {
                    let Some(ident) = attr.path.get_ident() else { continue; };
                    // Allow for custom over ride functions to be used
                    if ident == "custom_rand" {
                        let Ok(value_func) = attr.parse_args::<Ident>() else {continue};
                        return quote! {
                            #value_func
                        };
                    }
                }

                // Default to Rand impl
                let field_type = &f.ty;
                quote! {
                    <#field_type as ::enum_derived::Rand>::rand
                }
            });
            quote! {
                    ::std::boxed::Box::new(|| {
                        // #struct_name { #(#fields_ident: <#fields_types as ::enum_derived::Rand>::rand()),* }
                        #struct_name { #(#fields_ident: #fields_rand_funcs()),* }
                })
            }
        }
    };

    let expanded = quote! {
        impl ::enum_derived::Rand for #struct_name {
            fn rand() -> Self {
                #rand_struct_func()
            }
        }
    };
    Ok(TokenStream::from(expanded))
}

fn expand_derive_rand_union(
    _union_name: &Ident,
    _data_union: &DataUnion,
) -> Result<TokenStream, Vec<syn::Error>> {
    panic!("Union types are not supported")
}

fn expand_derive_rand_enum(
    enum_name: &Ident,
    data_enum: &DataEnum,
) -> Result<TokenStream, Vec<syn::Error>> {
    // Cannot be an empty variant
    if data_enum.variants.is_empty() {
        panic!("Enum must have at least one variant defined");
    }

    let variant_gen = |v: &Variant| variant_rand_func_generator(enum_name, v);
    let var_rand_funcs = data_enum.variants.iter().map(variant_gen);

    let weights = variant_weights_collector(data_enum.variants.iter().cloned().collect::<Vec<_>>());

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
                        #enum_name::#var_ident(#(<#fields_types as ::enum_derived::Rand>::rand()),*)
                })
            }
        }
        Fields::Named(named_fields) => {
            let fields_types = named_fields.named.iter().map(|f| &f.ty);
            let fields_ident = named_fields.named.iter().map(|f| f.ident.clone().unwrap());
            quote! {
                    ::std::boxed::Box::new(|| {
                        #enum_name::#var_ident{ #(#fields_ident: <#fields_types as ::enum_derived::Rand>::rand()),* }
                })
            }
        }
    };
    rand_func
}
