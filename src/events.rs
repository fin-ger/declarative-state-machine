use crate::error::StateMachineResult;
use proc_macro2::Span;
use syn::{punctuated::Punctuated, Ident, FnArg, token::Comma};
use quote::quote;

#[derive(Debug)]
pub struct Event {
    pub name: Ident,
    pub params: Punctuated<FnArg, Comma>,
}

//pub fn parse_event(iter: &mut Iterator<Item = TokenTree>, span: Span) -> StateMachineResult<Event> {
//}

pub fn get_events() -> Vec<Event> {
    let mut events = Vec::new();
    let mut params = Punctuated::new();
    params.push_value(syn::parse((quote! { volume: f32 }).into()).unwrap());
    events.push(Event {
        name: Ident::new("fill", Span::call_site()),
        params,
    });

    events.push(Event {
        name: Ident::new("full", Span::call_site()),
        params: Punctuated::new(),
    });

    events.push(Event {
        name: Ident::new("fuel", Span::call_site()),
        params: Punctuated::new(),
    });

    events.push(Event {
        name: Ident::new("dump", Span::call_site()),
        params: Punctuated::new(),
    });

    events
}
