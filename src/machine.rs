use syn::{Ident, braced};
use syn::parse::{Parse, ParseStream, Result};
use crate::transitions::{Transitions, parse_transitions};
use crate::events::Event;
use crate::handlers::Handler;
use crate::states::States;

#[derive(Debug)]
pub struct Machine {
    pub name: Ident,
    pub events: Vec<Event>,
    pub handlers: Vec<Handler>,
    pub transitions: Transitions,
    pub states: States,
}

pub mod keywords {
    syn::custom_keyword!(machine);
    syn::custom_keyword!(event);
    syn::custom_keyword!(handler);
    syn::custom_keyword!(states);
    syn::custom_keyword!(transitions);
}

impl Parse for Machine {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<keywords::machine>()?;
        let name = input.parse::<Ident>()?;
        let content;
        braced!(content in input);

        let mut events = Vec::new();
        let mut handlers = Vec::new();
        let mut states = None;
        let mut transitions = None;

        while !content.is_empty() {
            if content.peek(keywords::event) {
                events.push(content.parse()?);
            } else if content.peek(keywords::handler) {
                handlers.push(content.parse()?);
            } else if content.peek(keywords::states) {
                states = Some(content.parse()?);
            } else if content.peek(keywords::transitions) {
                transitions = Some(parse_transitions(&content)?);
            } else {
                return Err(
                    content.error("expected 'event', 'handler', 'states', or 'transition' keyword")
                );
            }
        }

        if states.is_none() {
            return Err(content.error("expected at least one 'states' block!"));
        }

        if transitions.is_none() {
            return Err(content.error("expected at least one 'transitions' block!"));
        }

        Ok(Self {
            name,
            events,
            handlers,
            transitions: transitions.unwrap(),
            states: states.unwrap(),
        })
    }
}
