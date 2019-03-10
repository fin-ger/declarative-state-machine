use proc_macro::Diagnostic;

pub enum StateMachineError {
    NoFurtherTokens,
    CompilationFailure(Diagnostic),
}

impl From<Diagnostic> for StateMachineError {
    fn from(diag: Diagnostic) -> StateMachineError {
        StateMachineError::CompilationFailure(diag)
    }
}

pub type StateMachineResult<T> = Result<T, StateMachineError>;
