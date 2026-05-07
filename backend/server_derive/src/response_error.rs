use darling::{
    FromDeriveInput, FromVariant,
    ast::{Data, Fields},
    util::Ignored,
};
use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

#[derive(FromDeriveInput)]
struct Opts {
    ident: syn::Ident,
    data: Data<VariantOpts, Ignored>,
}

#[derive(FromVariant)]
#[darling(attributes(response))]
struct VariantOpts {
    ident: syn::Ident,
    fields: Fields<Ignored>,
    status: syn::Ident,
    log: bool,
}

pub fn expand(ast: DeriveInput) -> TokenStream {
    let opts = match Opts::from_derive_input(&ast) {
        Ok(v) => v,
        Err(e) => return e.write_errors().into(),
    };

    let ident = opts.ident;

    let variants = match &opts.data {
        darling::ast::Data::Enum(variants) => variants,
        _ => panic!("ResponseError only supports enums"),
    };

    let arms = variants.into_iter().map(|v| {
        let vident = &v.ident;

        let pattern = if v.fields.is_empty() {
            quote! { #ident::#vident }
        } else {
            quote! { #ident::#vident(..) }
        };

        let status = &v.status;

        let body = if v.status == "INTERNAL_SERVER_ERROR" {
            quote! { "Something went wrong".to_string() }
        } else {
            quote! { self.to_string() }
        };

        let err = if v.log {
            quote! { Some(self) }
        } else {
            quote! { None::<Self> }
        };

        quote! {
            #pattern => (
                ::http::StatusCode::#status,
                #body,
                #err
            )
        }
    });

    quote! {
        impl ::axum::response::IntoResponse for #ident {
            fn into_response(self) -> ::axum::response::Response {
                let (status, msg, err) = match self {
                    #(#arms),*
                };

                let mut response = (status, msg).into_response();
                if let Some(err) = err {
                    response
                        .extensions_mut()
                        .insert(::std::sync::Arc::new(::anyhow::Error::new(err)));
                }

                response
            }
        }
    }
    .into()
}
