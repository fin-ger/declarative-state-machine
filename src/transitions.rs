use crate::error::{StateMachineError, StateMachineResult};

use std::collections::HashMap;
use proc_macro2::{TokenStream, Span, Group, Delimiter};
use syn::{punctuated::Punctuated, token::Comma, token::Dot2, FieldPat, PatTuple, Ident, Member, Pat, PatIdent, Block};
use quote::{ToTokens, quote, TokenStreamExt};

pub type EventIdent = syn::Ident;
pub type TransitionIdent = syn::Ident;

#[derive(Debug, Clone)]
pub enum StatePat {
    Struct {
        fields: Punctuated<FieldPat, Comma>,
        dot2_token: Option<Dot2>,
    },
    Tuple {
        pat: PatTuple,
    },
    Unit,
}

impl ToTokens for StatePat {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            StatePat::Struct { fields, dot2_token } => {
                let mut s = TokenStream::new();
                fields.to_tokens(&mut s);
                if let Some(t) = dot2_token {
                    t.to_tokens(&mut s);
                }
                tokens.append(Group::new(Delimiter::Brace, s));
            },

            StatePat::Tuple { pat } => {
                pat.to_tokens(tokens);
            },

            StatePat::Unit => {},
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

pub fn get_transitions() -> HashMap<EventIdent, HashMap<TransitionIdent, Vec<Transition>>> {
    let mut events = HashMap::new();

    {
        let mut froms = HashMap::new();

        {
            let mut transitions = Vec::new();

            {
                let mut pat_fields = Punctuated::new();
                pat_fields.push(FieldPat {
                    attrs: Vec::new(),
                    member: Member::Named(Ident::new("remaining", Span::call_site())),
                    colon_token: None,
                    pat: Box::new(Pat::Ident(PatIdent {
                        by_ref: None,
                        mutability: None,
                        ident: Ident::new("remaining", Span::call_site()),
                        subpat: None,
                    })),
                });

                let mut event_params = Punctuated::new();
                event_params.push(Ident::new("volume", Span::call_site()));

                let predicate = syn::parse((quote! {{
                    volume <= remaining
                }}).into()).unwrap();

                transitions.push(Transition {
                    from_pat: StatePat::Struct {
                        fields: pat_fields,
                        dot2_token: None,
                    },
                    to_ident: Ident::new("Filling", Span::call_site()),
                    event_params,
                    predicate: Some(predicate),
                    handler: Some(Ident::new("fill_bottle", Span::call_site())),
                });
            }

            {
                let mut pat_fields = Punctuated::new();
                pat_fields.push(FieldPat {
                    attrs: Vec::new(),
                    member: Member::Named(Ident::new("remaining", Span::call_site())),
                    colon_token: None,
                    pat: Box::new(Pat::Ident(PatIdent {
                        by_ref: None,
                        mutability: None,
                        ident: Ident::new("remaining", Span::call_site()),
                        subpat: None,
                    })),
                });

                let mut event_params = Punctuated::new();
                event_params.push(Ident::new("volume", Span::call_site()));

                let predicate = syn::parse((quote! {{
                    volume > remaining
                }}).into()).unwrap();

                transitions.push(Transition {
                    from_pat: StatePat::Struct {
                        fields: pat_fields,
                        dot2_token: None,
                    },
                    to_ident: Ident::new("Empty", Span::call_site()),
                    event_params,
                    predicate: Some(predicate),
                    handler: None,
                });
            }

            froms.insert(Ident::new("Idle", Span::call_site()), transitions);
        }

        events.insert(Ident::new("fill", Span::call_site()), froms);
    }

    {
        let mut froms = HashMap::new();

        {
            let mut transitions = Vec::new();
            transitions.push(Transition {
                from_pat: StatePat::Struct {
                    fields: Punctuated::new(),
                    dot2_token: Some(Dot2 { spans: [Span::call_site(), Span::call_site()]),
                },
                to_ident: Ident::new("Empty", Span::call_site()),
                event_params: Punctuated::new(),
                predicate: None,
                handler: None,
            });

            froms.insert(Ident::new("Idle", Span::call_site()), transitions);
        }

        events.insert(Ident::new("dump", Span::call_site()), froms);
    }

    {
        let mut froms = HashMap::new();

        {
            let mut transitions = Vec::new();
            transitions.push(Transition {
                from_pat: StatePat::Struct {
                    fields: Punctuated::new(),
                    dot2_token: Some(Dot2 { spans: [Span::call_site(), Span::call_site()]),
                },
                to_ident: Ident::new("Idle", Span::call_site()),
                event_params: Punctuated::new(),
                predicate: None,
                handler: Some(Ident::new("fuel_tank", Span::call_site())),
            });

            froms.insert(Ident::new("Idle", Span::call_site()), transitions);
        }

        {
            let mut transitions = Vec::new();
            transitions.push(Transition {
                from_pat: StatePat::Unit,
                to_ident: Ident::new("Idle", Span::call_site()),
                event_params: Punctuated::new(),
                predicate: None,
                handler: Some(Ident::new("fuel_tank", Span::call_site())),
            });

            froms.insert(Ident::new("Empty", Span::call_site()), transitions);
        }

        events.insert(Ident::new("fuel", Span::call_site()), froms);
    }

    {
        let mut froms = HashMap::new();

        {
            let mut transitions = Vec::new();
            transitions.push(Transition {
                from_pat: StatePat::Tuple {
                    pat: PatTuple {
                        paren_token: syn::token::Paren { span: Span::call_site() },
                        front: Punctuated::new(),
                        dot2_token: Some(Dot2 { spans: [Span::call_site(), Span::call_site()]}),
                        comma_token: None,
                        back: Punctuated::new(),
                    },
                },
                to_ident: Ident::new("Idle", Span::call_site()),
                event_params: Punctuated::new(),
                predicate: None,
                handler: Some(Ident::new("bottle_full", Span::call_site())),
            });

            froms.insert(Ident::new("Filling", Span::call_site()), transitions);
        }

        events.insert(Ident::new("full", Span::call_site()), froms);
    }

    events
}
