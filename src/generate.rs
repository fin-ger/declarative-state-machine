use crate::machine::Machine;
use proc_macro2::{TokenStream, Span};
use syn::{Ident, punctuated::Punctuated, token::Comma, Block, FnArg, Variant};
use quote::quote;

#[derive(Debug)]
struct Prepared {
    name: Ident,

    state_idents: Vec<Ident>,
    state_defaults: Vec<TokenStream>,
    state_initial: Ident,
    state_definitions: Punctuated<Variant, Comma>,
    state_names: Vec<Ident>,

    handler_names: Vec<Ident>,
    handler_params: Vec<Punctuated<FnArg, Comma>>,
    handler_bodies: Vec<Block>,

    event_names: Vec<Ident>,
    event_params: Vec<Punctuated<FnArg, Comma>>,
    event_body: Vec<TokenStream>,
}

fn as_name(ident: &Ident) -> Ident {
    Ident::new(ident.to_string().to_lowercase().as_str(), ident.span())
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

    let handler_names = machine.handlers.iter()
        .map(|handler| Ident::new(&format!("handle_{}", handler.name.to_string()), handler.name.span()))
        .collect::<Vec<_>>();
    let handler_params = machine.handlers.iter()
        .map(|handler| handler.params.clone())
        .collect::<Vec<_>>();
    let handler_bodies = machine.handlers.iter()
        .map(|handler| handler.body.clone())
        .collect::<Vec<_>>();

    let event_names = machine.events.iter()
        .map(|event| event.name.clone())
        .collect::<Vec<_>>();
    let event_params = machine.events.iter()
        .map(|event| event.params.clone())
        .collect::<Vec<_>>();
    let event_body = machine.events.iter()
        .map(|event| {
            let mut from_bodies = Vec::new();
            if let Some(event_transitions) = machine.transitions.get(&event.name) {
                from_bodies = event_transitions
                    .iter()
                    .map(|(from_ident, from_transitions)| {
                        let from_identifiers = from_transitions.iter()
                            .map(|_| from_ident.clone())
                            .collect::<Vec<_>>();
                        let from_names = from_identifiers.iter()
                            .map(as_name)
                            .collect::<Vec<_>>();
                        let to_identifiers = from_transitions.iter()
                            .map(|transition| transition.to_ident.clone())
                            .collect::<Vec<_>>();
                        let from_patterns = from_transitions.iter()
                            .map(|transition| transition.from_pat.clone())
                            .collect::<Vec<_>>();
                        let handlers = from_transitions.iter()
                            .map(|transition| {
                                let event_params = event.params.iter()
                                    .map(|param| {
                                        if let FnArg::Captured(arg) = param {
                                            return arg.pat.clone();
                                        }

                                        return syn::Pat::Ident(syn::PatIdent {
                                            by_ref: None,
                                            mutability: None,
                                            ident: Ident::new("__invalid__", Span::call_site()),
                                            subpat: None,
                                        });
                                    })
                                    .collect::<Vec<_>>();
                                if let Some(handler) = &transition.handler {
                                    let handler_name = Ident::new(
                                        format!("handle_{}", handler).as_str(),
                                        handler.span()
                                    );
                                    let from_name = as_name(from_ident);
                                    let to_name = as_name(&transition.to_ident);
                                    let from;
                                    if *from_ident == transition.to_ident {
                                        from = quote! { None };
                                    } else {
                                        from = quote! { Some(&mut self.#from_name)}
                                    }

                                    return quote! {
                                        Self::#handler_name(
                                            #from,
                                            &mut self.#to_name,
                                            #(#event_params,)*
                                        );
                                    };
                                } else {
                                    return TokenStream::new();
                                }
                            })
                            .collect::<Vec<_>>();
                        let predicates = from_transitions.iter()
                            .map(|transition| {
                                if let Some(predicate) = &transition.predicate {
                                    return quote! {
                                        if (|| -> bool #predicate)()
                                    }
                                } else {
                                    return TokenStream::new();
                                }
                            })
                            .collect::<Vec<_>>();
                        let param_initializers = from_transitions.iter()
                            .map(|transition| {
                                let params = transition.event_params.iter().collect::<Vec<_>>();
                                let event_params = event.params.iter()
                                    .map(|param| {
                                        if let FnArg::Captured(arg) = param {
                                            return arg.pat.clone();
                                        }

                                        return syn::Pat::Ident(syn::PatIdent {
                                            by_ref: None,
                                            mutability: None,
                                            ident: Ident::new("__invalid__", Span::call_site()),
                                            subpat: None,
                                        });
                                    })
                                    .collect::<Vec<_>>();

                                quote! {
                                    #(let #params = #event_params;)*
                                }
                            })
                            .collect::<Vec<_>>();

                        quote! {
                            StateIdentifier::#from_ident => {
                                #({
                                    #param_initializers
                                    if let State::#from_identifiers #from_patterns = self.#from_names {
                                        #predicates {
                                            self.current_state = StateIdentifier::#to_identifiers;
                                            #handlers
                                            return true;
                                        }
                                    }
                                })*
                            }
                        }
                    })
                    .collect::<Vec<_>>();
            }

            return quote! {
                match self.current_state {
                    #(#from_bodies,)*
                    _ => {},
                };

                false
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
        handler_params,
        handler_bodies,
        event_names,
        event_params,
        event_body,
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
        handler_params,
        handler_bodies,
        event_names,
        event_params,
        event_body,
    } = prepare(machine);
    let state_names2 = state_names.clone();

    quote! {
        mod #name {
            #[derive(Debug, Clone)]
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

                #(fn #handler_names(#handler_params) #handler_bodies)*

                #(
                    #[allow(unreachable_code)]
                    pub fn #event_names(&mut self, #event_params) -> bool {
                        #event_body
                    }
                )*
            }
        }
    }
}
