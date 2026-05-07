use darling::{FromDeriveInput, FromVariant, ast::Data, util::Ignored};
use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

#[derive(FromDeriveInput)]
#[darling(attributes(dto_to_domain))]
struct Opts {
    target: syn::Path,
    data: Data<VariantOpts, Ignored>,
}

#[derive(FromVariant)]
struct VariantOpts {
    ident: syn::Ident,
}

pub fn expand(ast: DeriveInput) -> TokenStream {
    let opts = match Opts::from_derive_input(&ast) {
        Ok(v) => v,
        Err(e) => return e.write_errors().into(),
    };

    let source = ast.ident;
    let target = opts.target;

    let variants = match &opts.data {
        darling::ast::Data::Enum(variants) => variants,
        _ => panic!("DtoToDomain only supports enums"),
    };

    let arms_to_target = variants.iter().map(|v| {
        let vident = &v.ident;
        quote! {
            #source::#vident => #target::#vident
        }
    });

    let arms_from_target = variants.iter().map(|v| {
        let vident = &v.ident;
        quote! {
            #target::#vident => #source::#vident
        }
    });

    quote! {
        impl Into<#target> for #source {
            fn into(self) -> #target {
                match self {
                    #(#arms_to_target),*
                }
            }
        }

        impl Into<#source> for #target {
            fn into(self) -> #source {
                match self {
                    #(#arms_from_target),*
                }
            }
        }
    }
    .into()
}
