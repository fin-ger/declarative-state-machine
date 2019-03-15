use syn::parse::Result;
use crate::machine::Machine;

pub fn parse_semantic(machine: Machine) -> Result<Machine> {
    // check if transitions only contain known states
    /*let state_idents = machine.states.defaults.keys()
        .map(|state| state.clone())
        .collect::<Vec<_>>();
    let mut transition_states = machine.transitions.values()
        .flat_map(|transitions| {
            transitions.iter().flat_map(|transition| {
                vec![transition.from.clone(), transition.to.clone()]
            })
        })
        .collect::<Vec<_>>();
    transition_states.sort();
    transition_states.dedup();

    for state in transition_states.clone() {
        if !state_idents.contains(&state) {
            return Err(
                state.span().unwrap()
                    .error("undefined state")
                    .help(format!("add `{}` to the `states` block to resolve", state).as_str())
                    .into()
            );
        }
    }

    // check if transitions only contain known events
    let event_idents = machine.events.iter()
        .map(|event| event.name.clone())
        .collect::<Vec<_>>();
    let mut transition_events = machine.transitions.keys()
        .map(|event| event.clone())
        .collect::<Vec<_>>();
    transition_events.sort();
    transition_events.dedup();

    for event in transition_events.clone() {
        if !event_idents.contains(&event) {
            return Err(
                event.span().unwrap()
                    .error("undefined event")
                    .help(format!("add an event handler for `{}` to resolve", event).as_str())
                    .into()
            );
        }
    }

    // check if each event has at least one transition
    for event in event_idents {
        if !transition_events.contains(&event) {
            event.span()
                .unwrap()
                .warning(
                    "event will always fail as no valid transitions are defined for this event!"
                )
                .help(
                    format!(
                        "add a transition to resolve: `SomeState => OtherState : {}`",
                        event,
                    ).as_str(),
                )
                .emit();
        }
    }

    // check if each state belongs to at least one transition
    for state in state_idents {
        if !transition_states.contains(&state) {
            state.span()
                .unwrap()
                .warning(
                    "state has no transitions and will never be reached or cause deadlock on construction"
                )
                .help(
                    format!(
                        "add a transition to resolve: `{} => OtherState : some_event`",
                        state,
                    ).as_str(),
                )
                .emit();
        }
    }*/

    // TODO: if $event_handle never used, warn that event never gets triggered

    Ok(machine)
}
