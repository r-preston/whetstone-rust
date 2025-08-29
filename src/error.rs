#[derive(Debug)]
pub enum ErrorType {
    /// Provided syntax file does not exist
    FileNotFoundError,
    /// Provided syntax file could not be read
    FileReadError,
    /// Syntax file is improperly formatted and could not be parsed, with line number
    RuleParseError,
    /// Equation string is improperly formatted and could not be parsed
    SyntaxError,
    /// Attempt to evaluate an uninitialised function
    NotInitialisedError,
    /// Tried to set value of a variable that doesn't exist
    NoSuchVariableError,
    /// Internal logic error
    InternalError,
    /// An error occurred registering function bindings
    BindingError,
}
#[derive(Debug)]
pub struct Error {
    pub error_type: ErrorType,
    pub message: String,
}
macro_rules! return_error {
    ($error_type:expr, $($message:tt)*) => {
        return Err(Error {
            error_type: $error_type,
            message: format!($($message)*),
        })
    };
}
pub(crate) use return_error;
