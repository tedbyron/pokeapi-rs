#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![forbid(unsafe_code)]

//! Attribute macro for [`pokeapi-model`](https://docs.rs/pokeapi-model) structs.

use proc_macro::TokenStream;
use quote::quote;
use syn::fold::Fold;
use syn::{
    parse_macro_input, parse_quote, token, Attribute, Data, DataStruct, DeriveInput, Fields,
    FieldsNamed, VisPublic, Visibility,
};

/// Struct for implementing a `Fold` hook.
///
/// [Reference](https://docs.rs/syn/*/syn/fold/index.html)
struct AllFieldsPub;

impl Fold for AllFieldsPub {
    /// Fold `FieldsNamed` and replace every field's visibility with
    /// `Visibility::Public`, unless the field has `#[serde(skip)]`.
    fn fold_fields_named(&mut self, fields: FieldsNamed) -> FieldsNamed {
        let brace_token = fields.brace_token;
        let skip_attr: Attribute = parse_quote!(#[serde(skip)]);
        let named = fields
            .named
            .into_iter()
            .map(|mut field| {
                if field
                    .attrs
                    .iter()
                    .find(|&attr| attr == &skip_attr)
                    .is_none()
                {
                    field.vis = Visibility::Public(VisPublic {
                        pub_token: token::Pub::default(),
                    });
                }
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
/// struct NamedAPIResource<T> {
///     description: String,
///     url: String,
///     #[serde(skip)]
///     _resource_type: std::marker::PhantomData<*const T>,
/// }
///
/// // This attribute will output the `struct` with required derived traits and
/// // visibility:
/// #[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
/// pub struct NamedAPIResource<T> {
///     pub description: String,
///     pub url: String,
///     #[serde(skip)]
///     _resource_type: std::marker::PhantomData<*const T>
/// }
/// ```
#[allow(clippy::doc_markdown)]
#[proc_macro_attribute]
pub fn pokeapi_struct(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Destructure and assign parameter `item` to its corresponding tokens. The
    // field `vis` is not necessary because it will be made `pub`.
    let DeriveInput {
        attrs,
        ident,
        generics,
        data,
        ..
    } = parse_macro_input!(item as DeriveInput);

    // `attrs` must be iterable in order to tokenize.
    let attrs = attrs.iter();

    // Ensure parameter `item` is a `struct` with named fields, and call the
    // `AllFieldsPub.fold_fields_name` hook on the struct's `Field`s.
    let fields = match data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields_named),
            ..
        }) => AllFieldsPub
            .fold_fields_named(fields_named)
            .named
            .into_iter(),
        _ => panic!("Expected a struct with named fields."),
    };

    // Quasi-quote the syntax tree and return as a `TokenStream`.
    TokenStream::from(quote! {
        #[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
        #[non_exhaustive]
        #(#attrs)*
        pub struct #ident #generics {
            #(#fields),*
        }
    })
}
