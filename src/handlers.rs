use syn::{Ident, punctuated::Punctuated, token::Comma, Block, FnArg};
use proc_macro2::Span;
use quote::quote;

#[derive(Debug)]
pub struct Handler {
    pub name: Ident,
    pub params: Punctuated<FnArg, Comma>,
    pub body: Block,
}

pub fn get_handlers() -> Vec<Handler> {
    let mut handlers = Vec::new();

    {
        let mut params = Punctuated::new();
        params.push(syn::parse((quote! { _from: Option<&State> }).into()).unwrap());
        params.push(syn::parse((quote! { to: &mut State }).into()).unwrap());
        params.push(syn::parse((quote! { volume: f32 }).into()).unwrap());
        let body = syn::parse((quote! {{
            if let State::Filling(ref mut filling_volume) = to {
                *filling_volume = volume;
            }
        }}).into()).unwrap();

        handlers.push(Handler {
            params,
            body,
            name: Ident::new("fill_bottle", Span::call_site()),
        });
    }

    {
        let mut params = Punctuated::new();
        params.push(syn::parse((quote! { _from: Option<&State> }).into()).unwrap());
        params.push(syn::parse((quote! { to: &mut State }).into()).unwrap());
        let body = syn::parse((quote! {{
            if let State::Idle { ref mut remaining } = to {
                *remaining = 42.0;
            }
        }}).into()).unwrap();

        handlers.push(Handler {
            params,
            body,
            name: Ident::new("fuel_tank", Span::call_site()),
        });
    }

    {
        let mut params = Punctuated::new();
        params.push(syn::parse((quote! { from: Option<&State> }).into()).unwrap());
        params.push(syn::parse((quote! { to: &mut State }).into()).unwrap());
        let body = syn::parse((quote! {{
            if let Some(State::Filling(ref volume)) = from {
                if let State::Idle { ref mut remaining } = to {
                    *remaining -= volume;
                }
            }
        }}).into()).unwrap();

        handlers.push(Handler {
            params,
            body,
            name: Ident::new("bottle_full", Span::call_site()),
        });
    }

    handlers
}
