use crate::error::StateMachineResult;

use std::collections::HashMap;
use proc_macro2::{TokenTree, Span, Ident, TokenStream};
use quote::quote;

pub struct States {
    pub initial: Ident,
    pub definition: TokenStream,
    pub defaults: HashMap<Ident, TokenStream>,
}

impl Default for States {
    fn default() -> Self {
        Self {
            initial: Ident::new("__invalid__", Span::call_site()),
            definition: TokenStream::new(),
            defaults: HashMap::new(),
        }
    }
}

pub fn parse_states(
    iter: &mut Iterator<Item = TokenTree>,
    span: Span,
) -> StateMachineResult<States> {
    iter.next();

    let mut defaults = HashMap::new();
    defaults.insert(
        Ident::new("Stopped", Span::call_site()),
        (quote! {State::Stopped()}).into(),
    );
    defaults.insert(
        Ident::new("Running", Span::call_site()),
        (quote! {State::Running(<String as core::default::Default>::default())}).into(),
    );
    defaults.insert(
        Ident::new("Paused", Span::call_site()),
        (quote! {State::Paused()}).into(),
    );

    Ok(States {
        initial: Ident::new("Stopped", Span::call_site()),
        definition: (quote! {
            Stopped(),
            Running(String),
            Paused(),
        }).into(),
        defaults: defaults,
    })
}
