use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Attribute, Data, DataEnum, DataStruct, DataUnion, DeriveInput, Field, Fields, Ident, Variant,
};

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
    let struct_ident = quote! { #struct_name };
    let rand_struct_generator = build_entity_generator(struct_ident, &data_struct.fields);

    let expanded = quote! {
        impl ::enum_derived::Rand for #struct_name {
            fn rand() -> Self {
                (#rand_struct_generator)()
            }
        }
    };
    Ok(TokenStream::from(expanded))
}

fn expand_derive_rand_enum(
    enum_name: &Ident,
    data_enum: &DataEnum,
) -> Result<TokenStream, Vec<syn::Error>> {
    // Cannot be an empty variant
    // TODO is this necessary?
    if data_enum.variants.is_empty() {
        panic!("Enum must have at least one variant defined");
    }

    let variant_gen = |v: &Variant| variant_generator(enum_name, v);
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
            quote! { 1 }
        })
        .collect()
}

/// Creates the rand function for a vartiant
fn variant_generator(enum_name: &Ident, variant: &Variant) -> proc_macro2::TokenStream {
    let variant_generator = match get_attr_value(&variant.attrs, "custom_rand") {
        Some(f) => f,
        None => {
            let var_ident = &variant.ident;

            let full_variant_ident = quote! { #enum_name::#var_ident };
            build_entity_generator(full_variant_ident, &variant.fields)
        }
    };

    quote! {
        ::std::boxed::Box::new(|| {
            (#variant_generator)()
        })
    }
}

// Returns the generating function
fn build_entity_generator(
    entity_name: proc_macro2::TokenStream,
    fields: &Fields,
) -> proc_macro2::TokenStream {
    match fields {
        Fields::Unit => {
            quote! { || #entity_name }
        }
        Fields::Unnamed(unnamed_fields) => {
            let fields_rand_generators = unnamed_fields.unnamed.iter().map(get_field_generator);
            quote! { || #entity_name(#(#fields_rand_generators()),*) }
        }
        Fields::Named(named_fields) => {
            let fields_ident = named_fields.named.iter().map(|f| f.ident.clone().unwrap());
            let fields_rand_generators = named_fields.named.iter().map(get_field_generator);

            quote! { || #entity_name { #(#fields_ident: #fields_rand_generators()),* } }
        }
    }
}

fn get_field_generator(field: &Field) -> proc_macro2::TokenStream {
    match get_attr_value(&field.attrs, "custom_rand") {
        Some(ts) => ts,
        None => {
            let field_type = &field.ty;
            quote! {
                <#field_type as ::enum_derived::Rand>::rand
            }
        }
    }
}

fn get_attr_value(attrs: &[Attribute], name: &str) -> Option<proc_macro2::TokenStream> {
    for attr in attrs.iter() {
        let Some(ident) = attr.path.get_ident() else {
            continue;
        };
        // Allow for custom over ride functions to be used
        if ident == name {
            let Ok(value_func) = attr.parse_args::<Ident>() else {
                continue;
            };
            return Some(quote! {
                #value_func
            });
        }
    }
    None
}

fn expand_derive_rand_union(
    _union_name: &Ident,
    _data_union: &DataUnion,
) -> Result<TokenStream, Vec<syn::Error>> {
    panic!("Union types are not supported")
}
