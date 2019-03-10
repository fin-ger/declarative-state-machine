use crate::error::StateMachineError;

use std::collections::HashMap;
use proc_macro2::{TokenTree, Span, Ident};

pub struct Transition {
    pub from: Ident,
    pub to: Ident,
}

pub fn parse_transitions(iter: &mut Iterator<Item = TokenTree>, span: Span) -> Result<HashMap<Ident, Vec<Transition>>, StateMachineError> {
    iter.next();

    let mut transitions = HashMap::new();
    transitions.insert(
        Ident::new("run", Span::call_site()),
        vec![
            Transition {
                from: Ident::new("Stopped", Span::call_site()),
                to: Ident::new("Running", Span::call_site()),
            },
            Transition {
                from: Ident::new("Paused", Span::call_site()),
                to: Ident::new("Running", Span::call_site()),
            },
        ],
    );
    transitions.insert(
        Ident::new("pause", Span::call_site()),
        vec![
            Transition {
                from: Ident::new("Running", Span::call_site()),
                to: Ident::new("Paused", Span::call_site()),
            },
        ],
    );
    transitions.insert(
        Ident::new("stop", Span::call_site()),
        vec![
            Transition {
                from: Ident::new("Running", Span::call_site()),
                to: Ident::new("Stopped", Span::call_site()),
            },
            Transition {
                from: Ident::new("Paused", Span::call_site()),
                to: Ident::new("Stopped", Span::call_site()),
            },
        ],
    );

    Ok(transitions)
}
