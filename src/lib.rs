#![recursion_limit="512"]

extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;
extern crate syn;

use quote::quote;
use proc_macro2::{TokenTree, Span, Delimiter, Spacing, Ident, Group, Literal, Punct};

fn error(s: &str, start: Span, end: Span) -> StateMachineError {
    let mut v = Vec::new();
    v.push(respan(Literal::string(&s), Span::call_site()));
    let group = v.into_iter().collect();

    let mut r = Vec::<TokenTree>::new();
    r.push(respan(Ident::new("compile_error", start), start));
    r.push(respan(Punct::new('!', Spacing::Alone), Span::call_site()));
    r.push(respan(Group::new(Delimiter::Brace, group), end));

    StateMachineError::InvalidSyntax(r.into_iter().collect())
}

fn respan<T: Into<TokenTree>>(t: T, span: Span) -> TokenTree {
    let mut t = t.into();
    t.set_span(span);
    t
}

enum StateMachineError {
    NoFurtherTokens,
    InvalidSyntax(proc_macro2::TokenStream),
}

fn parse_event_param(iter: &mut Iterator<Item = TokenTree>, span: Span) -> Result<(Ident, Span), StateMachineError> {
    Ok(span)
        .and_then(|mut span| {
            if let Some(param) = iter.next() {
                span = param.span();
                if let TokenTree::Ident(ident) = param {
                    return Ok((ident, span));
                }
            }

            return Err(error(
                "Expected identifier", span, span,
            ));
        })
        .and_then(|(param, mut span)| {
            if let Some(colon) = iter.next() {
                span = colon.span();
                if let TokenTree::Punct(punct) = colon {
                    if punct.as_char() == ':' {
                        return Ok((param, span));
                    }
                }
            }

            return Err(error(
                "Expected colon ':'", span, span,
            ));
        })
        .and_then(|(param, mut span)| {
            if let Some(reference) = iter.next() {
                span = reference.span();
                if let TokenTree::Punct(punct) = reference {
                    if punct.as_char() == '&' {
                        return Ok((param, span));
                    }
                }
            }

            return Err(error(
                "Expected mutable reference '&mut'", span, span,
            ));
        })
        .and_then(|(param, mut span)| {
            if let Some(mut_kw) = iter.next() {
                span = mut_kw.span();
                if let TokenTree::Ident(ident) = mut_kw {
                    if ident.to_string() == "mut" {
                        return Ok((param, span));
                    }
                }
            }

            return Err(error(
                "Expected &mut reference", span, span,
            ));
        })
        .and_then(|(param, mut span)| {
            if let Some(typename) = iter.next() {
                span = typename.span();
                if let TokenTree::Ident(ident) = typename {
                    if ident.to_string() == "State" {
                        return Ok((param, span));
                    }
                }
            }

            return Err(error(
                "Expected 'State' type", span, span,
            ));
        })
}

fn is_comma(comma_option: Option<TokenTree>, mut span: Span) -> Option<Span> {
    if let Some(comma) = comma_option {
        span = comma.span();
        if let TokenTree::Punct(punct) = comma {
            if punct.as_char() == ',' {
                return None;
            }
        }
    }

    Some(span)
}

fn parse_event(iter: &mut Iterator<Item = TokenTree>, span: Span) -> Result<(Ident, Ident, Ident, proc_macro2::TokenStream), StateMachineError> {
    iter.next()
        .ok_or(error("Expected event name", span, span))
        .and_then(|next| {
            let span = next.span();
            if let TokenTree::Ident(name) = next {
                return iter.next()
                    .ok_or(error("Missing event signature", span, span))
                    .map(|next| (name, next));
            } else {
                return Err(error("Expected event name", span, span));
            }
        })
        .and_then(|(name, next)| {
            let mut span = next.span();
            if let TokenTree::Group(group) = next {
                span = group.span();
                if let Delimiter::Parenthesis = group.delimiter() {
                    let mut param_iter = group.stream().into_iter();

                    let (old_state, span) = parse_event_param(&mut param_iter, span)?;

                    if let Some(span) = is_comma(param_iter.next(), span) {
                        return Err(error(
                            "Expected second parameter", span, span,
                        ));
                    }

                    let (new_state, mut span) = parse_event_param(&mut param_iter, span)?;

                    let result = iter.next()
                        .ok_or(error("Missing event body", span, span))
                        .map(|next| (name, old_state, new_state, next));

                    if let Some(comma) = param_iter.next() {
                        span = comma.span();
                        if let TokenTree::Punct(punct) = comma {
                            if punct.as_char() == ',' {
                                return result;
                            }
                        } else {
                            return Err(error(
                                "Only two arguments expected", span, span,
                            ));
                        }
                    }

                    return result;
                }
            }

            return Err(error(
                "Expected event signature '(old: &mut State, new: &mut State)'",
                span,
                span,
            ));
        })
        .and_then(|(name, old_state, new_state, next)| {
            let mut span = next.span();
            if let TokenTree::Group(group) = next {
                span = group.span();
                if let Delimiter::Brace = group.delimiter() {
                    return Ok((name, old_state, new_state, group.stream()));
                }
            }

            return Err(error(
                "Expected event body", span, span,
            ));
        })
}

fn parse_states(iter: &mut Iterator<Item = TokenTree>, span: Span) -> Result<(Option<Ident>, Vec<Ident>, Vec<proc_macro2::TokenStream>, Option<proc_macro2::TokenStream>), StateMachineError> {
    iter.next();

    Ok((
        Some(Ident::new("stopped", Span::call_site())),
        vec![
            Ident::new("stopped", Span::call_site()),
            Ident::new("running", Span::call_site()),
            Ident::new("paused", Span::call_site()),
        ],
        vec![
            (quote! {State::Stopped()}).into(),
            (quote! {State::Running(String::default())}).into(),
            (quote! {State::Paused()}).into(),
        ],
        Some((quote! {
            Stopped(),
            Running(String),
            Paused(),
        }).into()),
    ))
}

fn parse_transitions(iter: &mut Iterator<Item = TokenTree>, span: Span) -> Result<(Vec<Ident>, Vec<Ident>, Vec<Ident>), StateMachineError> {
    iter.next();

    Ok((
        vec![
            Ident::new("stopped", Span::call_site()),
            Ident::new("paused", Span::call_site()),
            Ident::new("running", Span::call_site()),
            Ident::new("running", Span::call_site()),
            Ident::new("paused", Span::call_site()),
        ],
        vec![
            Ident::new("transitions_run", Span::call_site()),
            Ident::new("transitions_run", Span::call_site()),
            Ident::new("transitions_pause", Span::call_site()),
            Ident::new("transitions_stop", Span::call_site()),
            Ident::new("transitions_stop", Span::call_site()),
        ],
        vec![
            Ident::new("running", Span::call_site()),
            Ident::new("running", Span::call_site()),
            Ident::new("paused", Span::call_site()),
            Ident::new("stopped", Span::call_site()),
            Ident::new("stopped", Span::call_site()),
        ],
    ))
}

#[proc_macro]
pub fn state_machine(machine: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut iter = proc_macro2::TokenStream::from(machine).into_iter();

    let mut name = None;
    let mut initial = None;
    let mut state_names = Vec::new();
    let mut state_defaults = Vec::new();
    let mut states = None;
    let mut event_names = Vec::new();
    let mut event_olds = Vec::new();
    let mut event_news = Vec::new();
    let mut event_handlers = Vec::new();
    let mut transitions_from = Vec::new();
    let mut transitions_event = Vec::new();
    let mut transitions_to = Vec::new();

    loop {
        let result = iter.next()
            .ok_or(StateMachineError::NoFurtherTokens)
            .and_then(|next| {
                let span = next.span();
                if let TokenTree::Ident(ident) = next {
                    if ident.to_string() == "machine" {
                        return iter.next()
                            .ok_or(error("Missing machine name", span, span));
                    } else {
                        return Err(error("Invalid identifier, expected 'machine'", span, span));
                    }
                } else {
                    return Err(error("Unexpected token, expected 'machine'", span, span));
                }
            })
            .and_then(|next| {
                let span = next.span();
                if let TokenTree::Ident(ident) = next {
                    name = Some(ident);
                    return iter.next()
                        .ok_or(error("Missing machine definition", span, span));
                } else {
                    return Err(error("Invalid machine name", span, span));
                }
            })
            .and_then(|next| {
                let span = next.span();
                if let TokenTree::Group(group) = next {
                    if let Delimiter::Brace = group.delimiter() {
                        return Ok(group.stream().into_iter());
                    } else {
                        return Err(error("Expected braces '{ ... }'", span, span));
                    }
                } else {
                    return Err(error("Expected state machine definition", span, span));
                }
            })
            .and_then(|mut group_iter| {
                while let Some(next) = group_iter.next() {
                    let span = next.span();
                    if let TokenTree::Ident(ident) = next {
                        match ident.to_string().as_str() {
                            "event" => {
                                let (name, old_state, new_state, handler) =
                                    parse_event(&mut group_iter, span)?;
                                event_names.push(name);
                                event_olds.push(old_state);
                                event_news.push(new_state);
                                event_handlers.push(handler);
                            },
                            "states" => {
                                let result = parse_states(&mut group_iter, span)?;

                                initial = result.0;
                                state_names = result.1;
                                state_defaults = result.2;
                                states = result.3;
                            },
                            "transitions" => {
                                let result = parse_transitions(&mut group_iter, span)?;

                                transitions_from = result.0;
                                transitions_event = result.1;
                                transitions_to = result.2;
                            },
                            _ => {
                                return Err(error(
                                    "Expected 'event', 'states', or 'transitions'",
                                    span,
                                    span,
                                ));
                            }
                        }
                    } else {
                        return Err(error("Expected definition of machine", span, span));
                    }
                }

                Ok(())
            });

        match result {
            Err(StateMachineError::NoFurtherTokens) => {
                break;
            },
            Err(StateMachineError::InvalidSyntax(error)) => {
                return error.into();
            },
            Ok(_) => {
                continue;
            }
        }
    }

    // trust me, I'm an engineer!
    let state_names2 = state_names.clone();
    let state_names3 = state_names.clone();
    let state_names4 = state_names.clone();
    let event_transitions = event_names
        .iter()
        .map(|event| Ident::new(&format!("transitions_{}", event.to_string()), event.span()))
        .collect::<Vec<Ident>>();
    let event_transitions2 = event_transitions.clone();
    let event_transitions3 = event_transitions.clone();
    let event_transitions4 = event_transitions.clone();
    let event_transitions5 = event_transitions.clone();
    // xD

    (quote! {
        mod #name {
            #[derive(Debug, Clone, PartialEq, Eq, Hash)]
            pub enum State {
                #states
            }

            pub struct Machine<'a> {
                current_state: &'a mut State,
                #(#state_names: State,)*
                #(#event_transitions: std::collections::HashMap<&'a State, &'a mut State>,)*
            }

            impl<'a> Machine<'a> {
                pub fn new() -> Self {
                    #(let #state_names2 = #state_defaults;)*;

                    #(let mut #event_transitions2 = std::collections::HashMap::new();)*
                    #(#transitions_event.insert(&#transitions_from, &mut #transitions_to);)*

                    Machine {
                        #(#event_transitions3: #event_transitions4,)*
                        current_state: &mut #initial,
                        #(#state_names3: #state_names4,)*
                    }
                }

                #(
                    pub fn #event_names(&mut self) -> bool {
                        let from = self.current_state;
                        let to_option = self.#event_transitions5.get(self.current_state);

                        if let Some(to) = to_option {
                            {
                                let #event_olds: &mut State = from;
                                let #event_news: &mut State = to;

                                #event_handlers
                            }

                            self.current_state = to;
                            return true;
                        }

                        return false;
                    }
                )*
            }
        }
    }).into()
}
