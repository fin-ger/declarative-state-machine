use crate::machine::keywords;
use syn::{Ident, punctuated::Punctuated, token::Comma, Block, FnArg};
use syn::parse::{Parse, ParseStream, Result};

#[derive(Debug)]
pub struct Handler {
    pub name: Ident,
    pub params: Punctuated<FnArg, Comma>,
    pub body: Block,
}

impl Parse for Handler {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<keywords::handler>()?;
        let name = input.parse()?;
        let content;
        syn::parenthesized!(content in input);
        let params = content.parse_terminated(FnArg::parse)?;
        let body = input.parse()?;

        Ok(Self {
            name,
            params,
            body,
        })
    }
}
