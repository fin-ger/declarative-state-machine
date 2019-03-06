extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;
extern crate syn;

use quote::quote;
use proc_macro2::{TokenTree, Span, Delimiter, Spacing, Ident, Group, Literal, Punct};

fn error(s: &str, start: Span, end: Span) -> StateMachineError {
    let mut v = Vec::new();
    v.push(respan(Literal::string(&s), Span::call_site()));
    let group = v.into_iter().collect();

    let mut r = Vec::<TokenTree>::new();
    r.push(respan(Ident::new("compile_error", start), start));
    r.push(respan(Punct::new('!', Spacing::Alone), Span::call_site()));
    r.push(respan(Group::new(Delimiter::Brace, group), end));

    StateMachineError::InvalidSyntax(r.into_iter().collect())
}

fn respan<T: Into<TokenTree>>(t: T, span: Span) -> TokenTree {
    let mut t = t.into();
    t.set_span(span);
    t
}

enum StateMachineError {
    NoFurtherTokens,
    InvalidSyntax(proc_macro2::TokenStream),
}

#[proc_macro]
pub fn state_machine(machine: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut iter = proc_macro2::TokenStream::from(machine).into_iter();

    let mut name = None;

    loop {
        let result = iter.next()
            .ok_or(StateMachineError::NoFurtherTokens)
            .and_then(|next| {
                let span = next.span();
                if let TokenTree::Ident(ident) = next {
                    if ident.to_string() == "machine" {
                        return iter.next()
                            .ok_or(error("Missing machine name", span, span));
                    } else {
                        return Err(error("Invalid identifier, expected 'machine'", span, span));
                    }
                } else {
                    return Err(error("Unexpected token, expected 'machine'", span, span));
                }
            })
            .and_then(|next| {
                let span = next.span();
                if let TokenTree::Ident(ident) = next {
                    name = Some(ident);
                    return iter.next()
                        .ok_or(error("Missing machine definition", span, span));
                } else {
                    return Err(error("Invalid machine name", span, span));
                }
            })
            .and_then(|next| {
                let span = next.span();
                if let TokenTree::Group(group) = next {
                    //println!("Found state machine definition {:?}", group);
                    return Ok(());
                } else {
                    return Err(error("Expected state machine definition", span, span));
                }
            });

        match result {
            Err(StateMachineError::NoFurtherTokens) => {
                break;
            },
            Err(StateMachineError::InvalidSyntax(error)) => {
                return error.into();
            },
            Ok(_) => {
                continue;
            }
        }
    }

    (quote! {
        mod #name {
            
        }
    }).into()

    /*quote! {
        mod #name {
            enum State {
                #(#states),*
            }

            pub struct Machine<'a> {
                current_state: &'a State,
                #(#state_names: State),*
            }

            impl<'a> Machine<'a> {
                pub fn new() -> Self {
                    #(
                        let #state_names = #state_defaults;
                    )*

                    Machine {
                        current_state: #initial,
                        #(#state_names: #state_names),*
                    }
                }

                #(
                    pub fn #event_names(&mut self) -> bool {
                        let from = self.#events_from;
                        let to = self.#events_to;

                        if self.current_state == from {
                            #handlers
                            self.current_state = to;
                            return true;
                        }

                        return false;
                    }
                )*
            }
        }
}*/
}
