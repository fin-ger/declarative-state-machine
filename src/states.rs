use crate::machine::keywords;
use std::collections::HashMap;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Variant, Fields, Ident, punctuated::Punctuated, token::Comma};
use syn::parse::{Parse, ParseStream, Result};

#[derive(Debug)]
pub struct States {
    pub initial: Ident,
    pub definition: Punctuated<Variant, Comma>,
    pub defaults: HashMap<Ident, TokenStream>,
}

impl Parse for States {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<keywords::states>()?;
        let content;
        syn::braced!(content in input);
        let initial = content.fork().parse()?;
        let definition = content.parse_terminated(Variant::parse)?;
        let defaults = definition.clone().into_iter()
            .map(|variant| {
                let name = variant.ident;
                let default = match variant.fields {
                    Fields::Named(fields) => {
                        let (idents, types): (Vec<_>, Vec<_>) = fields.named.into_iter()
                            .map(|field| (
                                field.ident.unwrap(),
                                field.ty,
                            )).unzip();

                        quote! {
                            State::#name{
                                #(#idents: <#types as core::default::Default>::default(),)*
                            }
                        }
                    },
                    Fields::Unnamed(fields) => {
                        let types = fields.unnamed.into_iter()
                            .map(|field| field.ty)
                            .collect::<Vec<_>>();

                        quote! {
                            State::#name(
                                #(<#types as core::default::Default>::default(),)*
                            )
                        }
                    },
                    Fields::Unit => {
                        quote! {
                            State::#name
                        }
                    },
                };

                (name, default)
            })
            .collect();

        Ok(Self {
            initial,
            definition,
            defaults,
        })
    }
}
