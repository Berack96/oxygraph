//! Derive macros for oxygraph. Empty scaffold until a derive is actually needed.
use proc_macro::TokenStream;
use quote::quote;
use std::collections::HashMap;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{
    GenericParam, Ident, Item, ItemStruct, ItemTrait, Token, TypeParamBound, parse_macro_input,
};

struct OverridePair {
    ident: Ident,
    bounds: Punctuated<TypeParamBound, Token![+]>,
}
impl Parse for OverridePair {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident: Ident = input.parse()?;
        input.parse::<Token![=]>()?;
        let bounds = Punctuated::parse_separated_nonempty(input)?;
        Ok(Self { ident, bounds })
    }
}
struct Overrides(HashMap<Ident, Punctuated<TypeParamBound, Token![+]>>);
impl Parse for Overrides {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut map = HashMap::new();
        if !input.is_empty() {
            let pairs = input.parse_terminated(OverridePair::parse, Token![,])?;
            for pair in pairs {
                map.insert(pair.ident, pair.bounds);
            }
        }
        Ok(Self(map))
    }
}

#[proc_macro_attribute]
pub fn serde_feature(args: TokenStream, input: TokenStream) -> TokenStream {
    let overrides = parse_macro_input!(args as Overrides);
    let item = parse_macro_input!(input as Item);

    match item {
        Item::Struct(item_struct) => handle_struct(item_struct),
        Item::Trait(item_trait) => handle_trait(item_trait, &overrides),
        _ => panic!("La macro supporta solo Struct e Trait"),
    }
}

fn handle_struct(item: ItemStruct) -> TokenStream {
    #[cfg(feature = "serde")]
    let derive_attr = quote! { #[derive(serde::Serialize, serde::Deserialize)] };
    #[cfg(not(feature = "serde"))]
    let derive_attr = quote! {};

    quote! {
        #derive_attr
        #item
    }
    .into()
}

fn handle_trait(mut item: ItemTrait, overrides: &Overrides) -> TokenStream {
    #[cfg(feature = "serde")]
    let default_bounds: Punctuated<TypeParamBound, Token![+]> =
        parse_quote!(serde::Serialize + serde::de::DeserializeOwned);

    #[cfg(feature = "serde")]
    item.supertraits.extend(default_bounds.clone());

    for param in item.generics.params.iter_mut() {
        if let GenericParam::Type(type_param) = param {
            if let Some(override_bounds) = overrides.0.get(&type_param.ident) {
                type_param.bounds.extend(override_bounds.clone());
            } else {
                #[cfg(feature = "serde")]
                type_param.bounds.extend(default_bounds.clone());
            }
        }
    }

    quote! {
        #item
    }
    .into()
}
