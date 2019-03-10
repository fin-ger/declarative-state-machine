#![recursion_limit="512"]
#![feature(proc_macro_diagnostic)]

extern crate proc_macro;

mod error;
mod events;
mod states;
mod transitions;
mod syntax;
mod semantic;
mod generate;

use proc_macro::{Diagnostic, Level};
use error::StateMachineError;

#[proc_macro]
pub fn state_machine(machine: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let result = syntax::parse_syntax(machine.into())
        .and_then(semantic::parse_semantic);

    match result {
        Err(StateMachineError::NoFurtherTokens) => {
            Diagnostic::new(Level::Error, "internal parser error, please report to upstream!")
                .emit();

            proc_macro::TokenStream::new()
        },
        Err(StateMachineError::CompilationFailure(diagnostics)) => {
            diagnostics.emit();

            proc_macro::TokenStream::new()
        },
        Ok(machine) => {
            generate::generate(machine).into()
        }
    }
}
