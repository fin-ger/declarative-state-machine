use syn::Ident;
use proc_macro2::Span;
use std::collections::HashMap;
use crate::transitions::{Transition, EventIdent, TransitionIdent};
use crate::events::Event;
use crate::handlers::Handler;
use crate::states::States;

pub struct Machine {
    pub name: Ident,
    pub events: Vec<Event>,
    pub handlers: Vec<Handler>,
    pub transitions: HashMap<EventIdent, HashMap<TransitionIdent, Vec<Transition>>>,
    pub states: States,
}

impl Default for Machine {
    fn default() -> Self {
        Self {
            name: Ident::new("__invalid__", Span::call_site()),
            events: Vec::new(),
            handlers: Vec::new(),
            transitions: HashMap::new(),
            states: States::default(),
        }
    }
}
