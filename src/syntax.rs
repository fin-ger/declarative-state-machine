use crate::error::{StateMachineError, StateMachineResult};
use crate::events;
use crate::events::Event;
use crate::transitions;
use crate::transitions::Transition;
use crate::states;
use crate::states::States;

use std::collections::HashMap;
use proc_macro2::{TokenStream, TokenTree, Ident, Delimiter, Span};

pub struct Machine {
    pub name: Ident,
    pub events: Vec<Event>,
    pub transitions: HashMap<Ident, Vec<Transition>>,
    pub states: States,
}

impl Default for Machine {
    fn default() -> Self {
        Self {
            name: Ident::new("__invalid__", Span::call_site()),
            events: Vec::new(),
            transitions: HashMap::new(),
            states: States::default(),
        }
    }
}

pub fn parse_syntax(machine: TokenStream) -> StateMachineResult<Machine> {
    let mut iter = machine.into_iter();
    let mut machine = Machine::default();

    loop {
        let result = iter.next()
            .ok_or(StateMachineError::NoFurtherTokens)
            .and_then(|next| {
                let span = next.span();
                if let TokenTree::Ident(ident) = next {
                    if ident.to_string() == "machine" {
                        return iter.next()
                            .ok_or(span.unwrap().error("missing machine name").into());
                    } else {
                        return Err(span.unwrap().error(
                            "invalid identifier, expected 'machine'"
                        ).into());
                    }
                } else {
                    return Err(span.unwrap().error("unexpected token, expected 'machine'").into());
                }
            })
            .and_then(|next| {
                let span = next.span();
                if let TokenTree::Ident(ident) = next {
                    machine.name = ident;
                    return iter.next()
                        .ok_or(span.unwrap().error("missing machine definition").into());
                } else {
                    return Err(span.unwrap().error("invalid machine name").into());
                }
            })
            .and_then(|next| {
                let span = next.span();
                if let TokenTree::Group(group) = next {
                    if let Delimiter::Brace = group.delimiter() {
                        return Ok(group.stream().into_iter());
                    } else {
                        return Err(span.unwrap().error("expected braces '{ ... }'").into());
                    }
                } else {
                    return Err(span.unwrap().error("expected state machine definition").into());
                }
            })
            .and_then(|mut group_iter| {
                while let Some(next) = group_iter.next() {
                    let span = next.span();
                    if let TokenTree::Ident(ident) = next {
                        match ident.to_string().as_str() {
                            "event" => {
                                machine.events.push(events::parse_event(&mut group_iter, span)?);
                            },
                            "states" => {
                                machine.states = states::parse_states(&mut group_iter, span)?;
                            },
                            "transitions" => {
                                machine.transitions = transitions::parse_transitions(
                                    &mut group_iter,
                                    span,
                                )?;
                            },
                            _ => {
                                return Err(span.unwrap().error(
                                    "expected 'event', 'states', or 'transitions' keyword"
                                ).into());
                            }
                        }
                    } else {
                        return Err(span.unwrap().error("expected definition of machine").into());
                    }
                }

                Ok(())
            });

        match result {
            Err(StateMachineError::NoFurtherTokens) => {
                return Ok(machine);
            },
            Err(StateMachineError::CompilationFailure(diagnostics)) => {
                return Err(StateMachineError::CompilationFailure(diagnostics));
            },
            Ok(()) => {
                continue;
            }
        }
    }
}
