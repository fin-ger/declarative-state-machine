use crate::syntax::Machine;
use proc_macro2::{Ident, TokenStream};
use quote::quote;

struct Prepared {
    name: Ident,

    state_idents: Vec<Ident>,
    state_defaults: Vec<TokenStream>,
    state_initial: Ident,
    state_definitions: TokenStream,
    state_names: Vec<Ident>,

    handler_names: Vec<Ident>,
    handler_old_param_names: Vec<Ident>,
    handler_new_param_names: Vec<Ident>,
    handler_bodies: Vec<TokenStream>,

    event_names: Vec<Ident>,
    event_handlers: Vec<Ident>,
    event_transitions: Vec<TokenStream>,
}

fn prepare<'a>(machine: Machine) -> Prepared {
    // The ordering of the vectors here is crucial as the nth item of a state_ prefixed
    // vector corresponds to the nth item of another state_ prefixed vector!
    // This holds for all vectors with the same prefix!

    let (state_idents, state_defaults): (Vec<_>, Vec<_>) = machine.states.defaults.iter()
        .map(|(ident, default)| (ident.clone(), default.clone()))
        .unzip();
    let state_initial = machine.states.initial.clone();
    let state_definitions = machine.states.definition.clone();
    let state_names = state_idents
        .iter()
        .map(|name| Ident::new(name.to_string().to_lowercase().as_str(), name.span()))
        .collect::<Vec<_>>();

    let handler_names = machine.events.iter()
        .map(|event| Ident::new(&format!("handle_{}", event.name.to_string()), event.name.span()))
        .collect::<Vec<_>>();
    let handler_old_param_names = machine.events.iter()
        .map(|event| event.old_param_name.clone())
        .collect::<Vec<_>>();
    let handler_new_param_names = machine.events.iter()
        .map(|event| event.new_param_name.clone())
        .collect::<Vec<_>>();
    let handler_bodies = machine.events.iter()
        .map(|event| event.body.clone())
        .collect::<Vec<_>>();

    let event_names = machine.events.iter()
        .map(|event| event.name.clone())
        .collect::<Vec<_>>();
    let event_handlers = handler_names.clone();
    let event_transitions = event_names.iter()
        .map(|name| {
            let mut from_identifiers = Vec::new();
            let mut from_names = Vec::new();
            let mut to_identifiers = Vec::new();
            let mut to_names = Vec::new();

            if let Some(trns) = machine.transitions.get(name) {
                from_identifiers = trns.iter()
                    .map(|transition| transition.from.clone())
                    .collect::<Vec<_>>();
                from_names = from_identifiers.iter()
                    .map(|from| Ident::new(from.to_string().to_lowercase().as_str(), from.span()))
                    .collect::<Vec<_>>();
                to_identifiers = trns.iter()
                    .map(|transition| transition.to.clone())
                    .collect::<Vec<_>>();
                to_names = to_identifiers.iter()
                    .map(|to| Ident::new(to.to_string().to_lowercase().as_str(), to.span()))
                    .collect::<Vec<_>>();
            }

            return quote! {
                let (from, to, ident) = match self.current_state {
                    #(
                        StateIdentifier::#from_identifiers => (
                            &mut self.#from_names,
                            &mut self.#to_names,
                            StateIdentifier::#to_identifiers,
                        ),
                    )*
                    _ => return false,
                };
            };
        })
        .collect::<Vec<_>>();

    Prepared {
        name: machine.name,
        state_idents,
        state_defaults,
        state_initial,
        state_definitions,
        state_names,
        handler_names,
        handler_old_param_names,
        handler_new_param_names,
        handler_bodies,
        event_names,
        event_handlers,
        event_transitions,
    }
}

pub fn generate(machine: Machine) -> TokenStream {
    let Prepared {
        name,
        state_idents,
        state_defaults,
        state_initial,
        state_definitions,
        state_names,
        handler_names,
        handler_old_param_names,
        handler_new_param_names,
        handler_bodies,
        event_names,
        event_handlers,
        event_transitions,
    } = prepare(machine);
    let state_names2 = state_names.clone();

    quote! {
        mod #name {
            #[derive(Debug, Clone, PartialEq, Eq, Hash)]
            pub enum State {
                #state_definitions
            }

            #[allow(dead_code)]
            enum StateIdentifier {
                #(#state_idents,)*
            }

            pub struct Machine {
                current_state: StateIdentifier,
                #(#state_names: State,)*
            }

            impl Machine {
                pub fn new() -> Self {
                    Self {
                        current_state: StateIdentifier::#state_initial,
                        #(#state_names2: #state_defaults,)*
                    }
                }

                #(
                    fn #handler_names(
                        #handler_old_param_names: &mut State,
                        #handler_new_param_names: &mut State,
                    ) {
                        #handler_bodies
                    }
                )*

                #(
                    #[allow(unreachable_code)]
                    pub fn #event_names(&mut self) -> bool {
                        #event_transitions

                        Self::#event_handlers(from, to);
                        self.current_state = ident;

                        true
                    }
                )*
            }
        }
    }
}
