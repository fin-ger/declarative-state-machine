#![recursion_limit="512"]
#![feature(proc_macro_diagnostic)]

extern crate proc_macro;

mod machine;
mod events;
mod handlers;
mod states;
mod transitions;
mod semantic;
mod generate;

#[proc_macro]
pub fn state_machine(machine: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let result = syn::parse(machine.into())
        .and_then(semantic::parse_semantic);

    match result {
        Err(err) => {
            err.to_compile_error().into()
        },
        Ok(machine) => {
            generate::generate(machine).into()
        }
    }
}
