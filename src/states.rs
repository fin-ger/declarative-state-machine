use crate::error::{StateMachineResult, StateMachineError};

use std::collections::HashMap;
use proc_macro2::{TokenTree, Span, Ident, TokenStream, Delimiter, Spacing};
use quote::quote;
use syn::{Variant, Fields};

pub struct States {
    pub initial: Ident,
    pub definition: TokenStream,
    pub defaults: HashMap<Ident, TokenStream>,
}

impl Default for States {
    fn default() -> Self {
        Self {
            initial: Ident::new("__invalid__", Span::call_site()),
            definition: TokenStream::new(),
            defaults: HashMap::new(),
        }
    }
}

struct State {
    name: Ident,
    default: TokenStream,
}

fn parse_state(
    iter: &mut Iterator<Item = TokenTree>,
) -> StateMachineResult<State> {
    let enum_item: Variant = syn::parse(iter.collect::<TokenStream>().into())
        .map_err::<StateMachineError, _>(
            |err| err.span().unwrap().error(format!("{}", err)).into()
        )?;
    let name = enum_item.ident;

    let default = match enum_item.fields {
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

    Ok(State {
        name,
        default,
    })
}

pub fn parse_states(
    iter: &mut Iterator<Item = TokenTree>,
    mut span: Span,
) -> StateMachineResult<States> {
    let mut initial = Ident::new("__invalid__", span);
    let mut defaults = HashMap::new();
    let mut definition;

    if let Some(next) = iter.next() {
        span = next.span();
        if let TokenTree::Group(group) = next {
            if let Delimiter::Brace = group.delimiter() {
                let mut state = Vec::new();
                definition = group.stream();

                for token in group.stream() {
                    if let TokenTree::Punct(punct) = token.clone() {
                        if let Spacing::Alone = punct.spacing() {
                            if punct.as_char() == ',' {
                                let mut iter = state.clone().into_iter();
                                match parse_state(&mut iter) {
                                    Err(err) => {
                                        return Err(err);
                                    },
                                    Ok(state) => {
                                        if initial.to_string() == "__invalid__" {
                                            initial = state.name.clone();
                                        }
                                        defaults.insert(state.name, state.default);
                                    }
                                }

                                state.clear();
                                continue;
                            }
                        }
                    }

                    state.push(token);
                }

                if !state.is_empty() {
                    let mut iter = state.clone().into_iter();
                    match parse_state(&mut iter) {
                        Err(err) => {
                            return Err(err);
                        },
                        Ok(state) => {
                            if initial.to_string() == "__invalid__" {
                                initial = state.name.clone();
                            }
                            defaults.insert(state.name, state.default);
                        }
                    }
                }

                return Ok(States {
                    initial,
                    definition,
                    defaults,
                });
            }
        }
    }

    Err(span.unwrap().error("expected states body `states { ... }`").into())
}
