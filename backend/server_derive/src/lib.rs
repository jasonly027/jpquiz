use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

mod dto_to_domain;
mod to_response_error;

#[proc_macro_derive(EnumDtoToDomain, attributes(dto_to_domain))]
pub fn derive_enum_dto_to_domain(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    dto_to_domain::expand(ast)
}

#[proc_macro_derive(ToResponseError, attributes(response))]
pub fn derive_to_response_error(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    to_response_error::expand(ast)
}
