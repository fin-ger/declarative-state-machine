use crate::error::StateMachineResult;
use proc_macro2::{Ident, Span, Delimiter, TokenTree, TokenStream};

fn parse_event_param(
    iter: &mut Iterator<Item = TokenTree>,
    span: Span,
) -> StateMachineResult<(Ident, Span)> {
    Ok(span)
        .and_then(|mut span| {
            if let Some(param) = iter.next() {
                span = param.span();
                if let TokenTree::Ident(ident) = param {
                    return Ok((ident, span));
                }
            }

            return Err(span.unwrap().error("expected identifier").into());
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

            return Err(span.unwrap().error("expected colon ':'").into());
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

            return Err(span.unwrap().error("expected mutable reference '&mut'").into());
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

            return Err(span.unwrap().error("expected mutable reference '&mut'").into());
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

            return Err(span.unwrap().error("expected 'State' type").into());
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

pub struct Event {
    pub name: Ident,
    pub old_param_name: Ident,
    pub new_param_name: Ident,
    pub body: TokenStream,
}

pub fn parse_event(iter: &mut Iterator<Item = TokenTree>, span: Span) -> StateMachineResult<Event> {
    iter.next()
        .ok_or(span.unwrap().error("expected event name").into())
        .and_then(|next| {
            let span = next.span();
            if let TokenTree::Ident(name) = next {
                return iter.next()
                    .ok_or(span.unwrap().error("missing event signature").into())
                    .map(|next| (name, next));
            } else {
                return Err(span.unwrap().error("expected event name").into());
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
                        return Err(span.unwrap().error("expected second parameter").into());
                    }

                    let (new_state, mut span) = parse_event_param(&mut param_iter, span)?;

                    let result = iter.next()
                        .ok_or(span.unwrap().error("missing event body").into())
                        .map(|next| (name, old_state, new_state, next));

                    if let Some(comma) = param_iter.next() {
                        span = comma.span();
                        if let TokenTree::Punct(punct) = comma {
                            if punct.as_char() == ',' {
                                return result;
                            }
                        } else {
                            return Err(span.unwrap().error("only two arguments expected").into());
                        }
                    }

                    return result;
                }
            }

            return Err(span.unwrap().error(
                "expected event signature '(old: &mut State, new: &mut State)'"
            ).into());
        })
        .and_then(|(name, old_state, new_state, next)| {
            let mut span = next.span();
            if let TokenTree::Group(group) = next {
                span = group.span();
                if let Delimiter::Brace = group.delimiter() {
                    return Ok(Event {
                        name: name,
                        old_param_name: old_state,
                        new_param_name: new_state,
                        body: group.stream(),
                    });
                }
            }

            return Err(span.unwrap().error("expected event body").into());
        })
}
