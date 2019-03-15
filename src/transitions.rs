use std::collections::HashMap;
use proc_macro2::{TokenStream, Group, Delimiter};
use syn::{punctuated::Punctuated, spanned::Spanned, Pat, PatStruct, PatTupleStruct, Ident, Block, Token};
use syn::token::{Brace, Paren, Comma};
use syn::parse::{Parse, ParseStream, Result, Error};
use quote::{ToTokens, TokenStreamExt};

use crate::machine::keywords;

pub type EventIdent = Ident;
pub type TransitionIdent = Ident;
pub type Transitions = HashMap<EventIdent, HashMap<TransitionIdent, Vec<Transition>>>;

#[derive(Debug, Clone)]
pub enum StatePat {
    Struct {
        pat: PatStruct,
    },
    Tuple {
        pat: PatTupleStruct,
    },
    Unit {
        ident: Ident,
    },
}

impl StatePat {
    pub fn ident(&self) -> Result<Ident> {
        match self {
            StatePat::Struct { pat } => {
                let segments = &pat.path.segments;
                if segments.len() != 1 {
                    Err(
                        Error::new(segments.span(), "expected identifier")
                    )
                } else {
                    Ok(segments.first().unwrap().value().ident.clone())
                }
            },
            StatePat::Tuple { pat } => {
                let segments = &pat.path.segments;
                if segments.len() != 1 {
                    Err(
                        Error::new(segments.span(), "expected identifier")
                    )
                } else {
                    Ok(segments.first().unwrap().value().ident.clone())
                }
            },
            StatePat::Unit { ident } => {
                Ok(ident.clone())
            }
        }
    }
}

impl ToTokens for StatePat {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            StatePat::Struct { pat } => {
                let mut s = TokenStream::new();
                pat.fields.to_tokens(&mut s);
                if let Some(t) = pat.dot2_token {
                    t.to_tokens(&mut s);
                }
                tokens.append(Group::new(Delimiter::Brace, s));
            },

            StatePat::Tuple { pat } => {
                pat.pat.to_tokens(tokens);
            },

            StatePat::Unit { .. } => {},
        }
    }
}

impl Parse for StatePat {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek2(Brace) {
            let p = input.parse()?;
            if let Pat::Struct(pat) = p {
                Ok(StatePat::Struct {
                    pat,
                })
            } else {
                Err(
                    Error::new(p.span(), "expected struct pattern '{a, b: y, ..}'")
                )
            }
        } else if input.peek2(Paren) {
            let p = input.parse()?;
            if let Pat::TupleStruct(pat) = p {
                Ok(StatePat::Tuple {
                    pat,
                })
            } else {
                Err(
                    Error::new(p.span(), "expected tuple pattern '(a, _, ..)'")
                )
            }
        } else {
            let ident = input.parse()?;
            Ok(StatePat::Unit {
                ident,
            })
        }
    }
}

#[derive(Debug)]
pub struct Transition {
    pub from_pat: StatePat,
    pub to_ident: Ident,
    pub event_params: Punctuated<Ident, Comma>,
    pub predicate: Option<Block>,
    pub handler: Option<Ident>,
}

pub fn parse_transitions(input: ParseStream) -> Result<Transitions> {
    input.parse::<keywords::transitions>()?;
    let content;
    syn::braced!(content in input);
    let mut events = Transitions::new();

    while !content.is_empty() {
        let from_pat = content.parse::<StatePat>()?;
        let from_ident = from_pat.ident()?;
        content.parse::<Token![=>]>()?;
        let to_ident = content.parse()?;
        content.parse::<Token![:]>()?;
        let event_ident = content.parse()?;
        let params;
        syn::parenthesized!(params in content);
        let event_params = params.parse_terminated(Ident::parse)?;
        let mut predicate = None;
        let mut handler = None;

        if !content.peek(Token![;]) && !content.peek(Token![->]) {
            predicate = Some(content.parse()?);
        }

        if !content.peek(Token![;]) {
            content.parse::<Token![->]>()?;
            handler = Some(content.parse()?);
        }

        content.parse::<Token![;]>()?;

        if !events.contains_key(&event_ident) {
            events.insert(event_ident.clone(), HashMap::new());
        }

        if let Some(froms) = events.get_mut(&event_ident) {
            if !froms.contains_key(&from_ident) {
                froms.insert(from_ident.clone(), Vec::new());
            }

            if let Some(transitions) = froms.get_mut(&from_ident) {
                transitions.push(Transition {
                    from_pat,
                    to_ident,
                    event_params,
                    predicate,
                    handler,
                });
            }
        }
    }

    Ok(events)
}
