use crate::machine::keywords;
use syn::{punctuated::Punctuated, Token, Ident, FnArg, token::Comma};
use syn::parse::{Parse, ParseStream, Result};

#[derive(Debug)]
pub struct Event {
    pub name: Ident,
    pub params: Punctuated<FnArg, Comma>,
}

impl Parse for Event {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<keywords::event>()?;
        let name = input.parse()?;
        let content;
        syn::parenthesized!(content in input);
        let params = content.parse_terminated(FnArg::parse)?;
        input.parse::<Token![;]>()?;

        Ok(Self {
            name,
            params,
        })
    }
}
