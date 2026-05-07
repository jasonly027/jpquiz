use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

mod dto_to_domain;
mod response_error;

#[proc_macro_derive(EnumDtoToDomain, attributes(dto_to_domain))]
pub fn derive_enum_dto_to_domain(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    dto_to_domain::expand(ast)
}

#[proc_macro_derive(ResponseError, attributes(response))]
pub fn derive_response_error(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    response_error::expand(ast)
}
