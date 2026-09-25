//! Derive macros for oxygraph. Empty scaffold until a derive is actually needed.
use proc_macro::TokenStream;
use quote::quote;
use std::collections::HashMap;
use syn::parse::{Parse, ParseStream};
#[cfg(feature = "serde")]
use syn::parse_quote;
use syn::punctuated::Punctuated;
use syn::{
    GenericParam, Ident, Item, ItemStruct, ItemTrait, Token, TraitItem, TypeParamBound,
    parse_macro_input,
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
        _ => panic!("the macro only supports Struct and Trait items"),
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
    // Bounds go on GraphEdgeStorage's own generics/associated types (the data it holds),
    // never as a `Self: Serialize` supertrait: a storage wrapper like EdgeFilteredView is
    // never itself serializable (it borrows and holds a filter function pointer), even
    // when the data it stores is.
    #[cfg(feature = "serde")]
    let default_bounds: Punctuated<TypeParamBound, Token![+]> =
        parse_quote!(serde::Serialize + serde::de::DeserializeOwned);

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

    // Associated types (e.g. `type Edge;`) get the same treatment as generic params,
    // since a trait can express "one Self-determined type" either way.
    for trait_item in item.items.iter_mut() {
        if let TraitItem::Type(assoc_type) = trait_item {
            if let Some(override_bounds) = overrides.0.get(&assoc_type.ident) {
                assoc_type.bounds.extend(override_bounds.clone());
            } else {
                #[cfg(feature = "serde")]
                assoc_type.bounds.extend(default_bounds.clone());
            }
        }
    }

    quote! {
        #item
    }
    .into()
}
