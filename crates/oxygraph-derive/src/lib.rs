//! Derive macros for oxygraph. Empty scaffold until a derive is actually needed.
use proc_macro::TokenStream;
use quote::quote;
use syn::{GenericParam, Item, parse_macro_input, parse_quote};

/// Basic Clone + Default + Debug derive to use for all the types.
/// This macros is used to avoid having to write the same derive for all the types
/// and to avoid having duplicate/inconsistent derives in the codebase.
#[proc_macro_attribute]
pub fn base_model(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as Item);

    let expanded = match &mut input {
        // Se è applicato a un trait, iniettiamo i supertraits
        Item::Trait(t) => {
            // 1. Supertraits del trait stesso
            t.supertraits.push(parse_quote!(Clone));
            t.supertraits.push(parse_quote!(Default));
            t.supertraits.push(parse_quote!(std::fmt::Debug));

            // 2. Bound standard (Clone, Default, Debug) su TUTTI i tipi generici definiti
            let mut base_where = t.generics.make_where_clause().clone();
            for param in t.generics.params.iter() {
                if let GenericParam::Type(type_param) = param {
                    let ident = &type_param.ident;
                    base_where.predicates.push(parse_quote!(
                        #ident: Clone + Default + std::fmt::Debug
                    ));
                }
            }

            // 3. Clone per la variante Serde
            let mut serde_trait = t.clone();
            serde_trait.supertraits.push(parse_quote!(serde::Serialize));
            serde_trait
                .supertraits
                .push(parse_quote!(for<'de> serde::Deserialize<'de>));
            let mut serde_where = serde_trait.generics.make_where_clause().clone();
            for param in serde_trait.generics.params.iter() {
                if let GenericParam::Type(type_param) = param {
                    let ident = &type_param.ident;
                    serde_where.predicates.push(parse_quote!(
                        #ident: serde::Serialize + for<'de> serde::Deserialize<'de>
                    ));
                }
            }

            quote! {
                #[cfg(feature = "serde")]
                #serde_trait

                #[cfg(not(feature = "serde"))]
                #t
            }
        }
        // Se è applicato a struct, enum o union, aggiungiamo i derive
        _ => {
            quote! {
                #[derive(Clone, Default, Debug)]
                #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
                #input
            }
        }
    };

    TokenStream::from(expanded)
}
