#![warn(
    clippy::all,
    clippy::cargo,
    clippy::nursery,
    clippy::pedantic,
    rust_2018_idioms
)]
#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

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
struct PokeAPIFields;

impl Fold for PokeAPIFields {
    /// Fold `FieldsNamed` and replace every field's visibility with `Visibility::Public`, unless
    /// the field has `#[serde(skip)]`.
    fn fold_fields_named(&mut self, fields: FieldsNamed) -> FieldsNamed {
        let serde_skip: Attribute = parse_quote!(#[serde(skip)]);

        let brace_token = fields.brace_token;
        let named = fields
            .named
            .into_iter()
            .map(|mut field| {
                if let Some(ref ident) = field.ident {
                    if ident.to_string().starts_with('_') {
                        if !field.attrs.iter().any(|attr| attr == &serde_skip) {
                            field.attrs.push(serde_skip.clone());
                        }
                    }
                }

                if !field.attrs.iter().any(|attr| attr == &serde_skip) {
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

/// Attribute macro that generate a `pokeapi-model` struct.
///
/// # Panics
///
/// Panics if the passed `item` is not a valid struct.
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
/// // This attribute will output the struct with required derived traits and visibility:
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
    // Destructure and assign parameter `item` to its corresponding tokens. The field `vis` is not
    // necessary because it will be made `pub`.
    let DeriveInput {
        attrs,
        ident,
        generics,
        data,
        ..
    } = parse_macro_input!(item as DeriveInput);

    let ident_lower = ident.to_string().to_ascii_lowercase();
    let doc_comment = format!(
        "[{url}{ident_lower}]({url}{ident_lower})",
        url = "https://pokeapi.co/docs/v2#"
    );
    let doc_attr: Attribute = parse_quote!(#[doc = #doc_comment]);

    // Ensure parameter `item` is a `struct` with named fields, and call the
    // `PokeAPIFields.fold_fields_name` hook on the struct's `Field`s.
    let fields = match data {
        Data::Struct(DataStruct {
            fields: Fields::Named(named_fields),
            ..
        }) => PokeAPIFields
            .fold_fields_named(named_fields)
            .named
            .into_iter(),
        _ => panic!("This attribute requires a struct with named fields."),
    };

    // Quasi-quote the syntax tree and return as a `TokenStream`.
    TokenStream::from(quote! {
        #doc_attr
        #(#attrs)*
        #[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
        #[non_exhaustive]
        pub struct #ident #generics {
            #(#fields),*
        }
    })
}
