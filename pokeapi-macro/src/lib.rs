#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![forbid(unsafe_code)]

//! Attribute macro for [`pokeapi-model`](https://docs.rs/pokeapi-model) structs.

use proc_macro::TokenStream;
use quote::quote;
use syn::fold::Fold;
use syn::{
    parse_macro_input, token, Data, DataStruct, DeriveInput, Fields, FieldsNamed, VisPublic,
    Visibility,
};

/// Struct for implementing a `Fold` hook.
/// [Reference](https://docs.rs/syn/*/syn/fold/index.html)
struct AllFieldsPub;

impl Fold for AllFieldsPub {
    /// Fold `FieldsNamed` and replace all `field.vis` with `Visibility::Public`.
    fn fold_fields_named(&mut self, fields: syn::FieldsNamed) -> syn::FieldsNamed {
        let brace_token = fields.brace_token;
        let named = fields
            .named
            .into_iter()
            .map(|mut field| {
                field.vis = Visibility::Public(VisPublic {
                    pub_token: token::Pub::default(),
                });
                field
            })
            .collect();

        FieldsNamed { brace_token, named }
    }
}

/// Attribute macro to generate a PokéAPI struct.
///
/// # Panics
///
/// Panics if the passed `item` is not a valid `struct`.
///
/// # Examples
///
/// ```ignore
/// // Consider the following example:
/// use pokeapi_macro::pokeapi_struct;
///
/// #[pokeapi_struct]
/// struct NamedAPIResource {
///   description: String,
///   url: String,
/// }
///
/// // This attribute will output the `struct` with required derived traits and
/// // visibility:
/// #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// pub struct NamedAPIResource {
///   pub description: String,
///   pub url: String,
/// }
/// ```
#[allow(clippy::doc_markdown)]
#[proc_macro_attribute]
pub fn pokeapi_struct(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Destructure and assign `item` to its corresponding tokens. The field `vis` is
    // not necessary because the item's visibility will be made `pub`.
    let DeriveInput {
        attrs,
        vis: _,
        ident,
        generics,
        data,
    } = parse_macro_input!(item as DeriveInput);

    // `attrs` must be iterable in order to tokenize.
    let attrs = attrs.iter();
    // Ensure `item` is a `struct` with named fields, and change field visibility to
    // `pub`.
    let fields = match data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields_named),
            ..
        }) => AllFieldsPub
            .fold_fields_named(fields_named)
            .named
            .into_iter(),
        _ => panic!("Expected a struct with named fields"),
    };

    // Tokenize a syntax tree and return as a `TokenStream`.
    TokenStream::from(quote! {
        #[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
        #[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
        #[non_exhaustive]
        #(#attrs)*
        pub struct #ident #generics {
            #(#fields),*
        }
    })
}
