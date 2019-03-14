use crate::error::{StateMachineResult, StateMachineError};

use std::collections::HashMap;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Variant, Ident, punctuated::Punctuated, token::Comma};

pub struct States {
    pub initial: Ident,
    pub definition: Punctuated<Variant, Comma>,
    pub defaults: HashMap<Ident, TokenStream>,
}

impl Default for States {
    fn default() -> Self {
        Self {
            initial: Ident::new("__invalid__", Span::call_site()),
            definition: Punctuated::new(),
            defaults: HashMap::new(),
        }
    }
}

pub fn get_states() -> States {
    let mut definition = Punctuated::new();
    let mut defaults = HashMap::new();

    definition.push(syn::parse((quote! { Idle { remaining: f32, } }).into()).unwrap());
    definition.push(syn::parse((quote! { Filling(f32) }).into()).unwrap());
    definition.push(syn::parse((quote! { Empty }).into()).unwrap());

    defaults.insert(
        Ident::new("Idle", Span::call_site()),
        quote! { Idle { remaining: <f32 as core::default::Default>::default() } },
    );
    defaults.insert(
        Ident::new("Filling", Span::call_site()),
        quote! { Filling(<f32 as core::default::Default>::default()) },
    );
    defaults.insert(
        Ident::new("Empty", Span::call_site()),
        quote! { Empty },
    );

    States {
        initial: Ident::new("Idle", Span::call_site()),
        definition,
        defaults,
    }
}
