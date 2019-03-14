use crate::error::{StateMachineError, StateMachineResult};
use crate::events;
use crate::transitions;
use crate::states;
use crate::machine::Machine;

use proc_macro2::{TokenStream, Span};
use syn::{Ident};

use crate::events::get_events;
use crate::handlers::get_handlers;
use crate::states::get_states;
use crate::transitions::get_transitions;

pub fn parse_syntax(machine: TokenStream) -> StateMachineResult<Machine> {
    let events = get_events();
    let handlers = get_handlers();
    let states = get_states();
    let transitions = get_transitions();

    Ok(Machine {
        name: Ident::new("bottle_filler", Span::call_site()),
        events,
        handlers,
        states,
        transitions,
    })
}
