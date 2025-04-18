#[derive(Debug)]
pub enum ErrorType {
    /// Provided syntax file does not exist
    FileNotFound,
    /// Syntax file exists but could not be read or has invalid contents
    FileReadError,
    /// Equation string is improperly formatted and could not be parsed
    SyntaxFileError,
    /// Equation string is improperly formatted and could not be parsed
    ParseError,
    /// Attempt to evaluate an uninitialised function
    NotInitialised,
    /// Tried to set value of a variable that doesn't exist
    NoSuchVariable,
    /// Internal logic error
    InternalError,
}
#[derive(Debug)]
pub struct Error {
    pub error_type: ErrorType,
    pub message: String,
}
macro_rules! return_error {
    ($error_type:expr, $message:expr) => {
        return Err(Error {
            error_type: $error_type,
            message: $message,
        })
    };
}
pub(crate) use return_error;
