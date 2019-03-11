use crate::error::{StateMachineError, StateMachineResult};

use std::collections::HashMap;
use proc_macro2::{TokenTree, Span, Ident, Delimiter, Spacing};

#[derive(Debug)]
pub struct Transition {
    pub from: Ident,
    pub to: Ident,
}

fn parse_transition(
    iter: &mut Iterator<Item = TokenTree>,
    mut span: Span,
) -> StateMachineResult<(Ident, Transition)> {
    iter.next()
        .ok_or(StateMachineError::NoFurtherTokens)
        .and_then(|next| {
            span = next.span();
            if let TokenTree::Ident(ident) = next {
                return Ok(ident);
            }

            Err(span.unwrap().error(
                "expected state identifier for transition source state"
            ).into())
        })
        .and_then(|from| {
            if let Some(next) = iter.next() {
                span = next.span();
                if let TokenTree::Punct(equal_sign) = next {
                    if let Spacing::Joint = equal_sign.spacing() {
                        if let Some(other) = iter.next() {
                            span = other.span();
                            if let TokenTree::Punct(arrow) = other {
                                if let Spacing::Alone = arrow.spacing() {
                                    if equal_sign.as_char() == '=' && arrow.as_char() == '>' {
                                        return Ok(from);
                                    }
                                }
                            }
                        }
                    }
                }
            }

            Err(span.unwrap().error("expected '=>' event transition operator").into())
        })
        .and_then(|from| {
            if let Some(next) = iter.next() {
                span = next.span();
                if let TokenTree::Ident(ident) = next {
                    return Ok((from, ident));
                }
            }

            Err(span.unwrap().error(
                "expected state identifier for transition destination state",
            ).into())
        })
        .and_then(|tuple| {
            if let Some(next) = iter.next() {
                span = next.span();
                if let TokenTree::Punct(punct) = next {
                    if let Spacing::Alone = punct.spacing() {
                        if punct.as_char() == ':' {
                            return Ok(tuple);
                        }
                    }
                }
            }

            Err(span.unwrap().error(
                "expected colon ':' to specify associated event handler"
            ).into())
        })
        .and_then(|(from, to)| {
            if let Some(next) = iter.next() {
                span = next.span();
                if let TokenTree::Ident(ident) = next {
                    return Ok((ident, Transition {
                        from: from,
                        to: to,
                    }));
                }
            }

            Err(span.unwrap().error("expected event handler name").into())
        })
        .and_then(|tuple| {
            if let Some(next) = iter.next() {
                if let TokenTree::Punct(punct) = next {
                    if let Spacing::Alone = punct.spacing() {
                        if punct.as_char() == ';' {
                            return Ok(tuple);
                        }
                    }
                }
            }

            Err(span.unwrap().error("expected semicolon ';' at end of transition").into())
        })
}

pub fn parse_transitions(
    iter: &mut Iterator<Item = TokenTree>,
    mut span: Span,
) -> StateMachineResult<HashMap<Ident, Vec<Transition>>> {
    let mut transitions: HashMap<Ident, Vec<Transition>> = HashMap::new();

    if let Some(next) = iter.next() {
        span = next.span();
        if let TokenTree::Group(group) = next {
            if let Delimiter::Brace = group.delimiter() {
                let mut iter = group.stream().into_iter();
                loop {
                    match parse_transition(&mut iter, span) {
                        Err(StateMachineError::NoFurtherTokens) => {
                            return Ok(transitions);
                        },
                        Err(err) => {
                            return Err(err);
                        },
                        Ok((event, transition)) => {
                            if let Some(value) = transitions.get_mut(&event) {
                                value.push(transition);
                            } else {
                                transitions.insert(event, vec![transition]);
                            }
                        }
                    }
                }
            }
        }
    }

    Err(span.unwrap().error("expected transitions body 'transitions { ... }'").into())
}
