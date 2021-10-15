#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![forbid(unsafe_code)]

//! Attribute macro for [`pokeapi-rs`](https://docs.rs/pokeapi) structs.

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields};

/// Attribute macro to generate a PokéAPI struct.
#[proc_macro_attribute]
pub fn pub_struct(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse_macro_input!(item);

    let name = &ast.ident;
    let generics = &ast.generics;

    let fields = match &ast.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => &fields.named,
        _ => panic!("expected a struct with named fields"),
    };

    let field_name = fields.iter().map(|field| &field.ident);
    let field_type = fields.iter().map(|field| &field.ty);

    TokenStream::from(quote! {
        #[derive(Debug, Clone, Deserialize)]
        pub struct #name #generics {
            #(
                pub #field_name: #field_type,
            )*
        }
    })
}
