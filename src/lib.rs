#![allow(dead_code)]
#![allow(unused_variables)]

mod equation;
mod expressions;

pub use equation::Equation;
pub use expressions::variable::Variable;
pub use expressions::value::Value;
//use function::Function;

// define constraint for the type of value used by an Equation
pub trait NumericType: num_traits::float::Float {}
impl<T: num_traits::float::Float> NumericType for T {}


#[derive(Debug)]
pub enum ErrorType {
    SyntaxError,
    InvalidObject,
}
#[derive(Debug)]
pub struct Error<'a> {
    pub error_type: ErrorType,
    pub message: &'a str,
}
macro_rules! return_error {
    ($error_type:expr, $message:literal) => {
        return Err(Error{
            error_type: $error_type,
            message: $message
        });
    };
}
pub(crate) use return_error;

pub enum Syntax {
    Standard,
}

pub struct EquationFactory {
    syntax: Syntax,
}

impl EquationFactory {
    pub fn new(syntax: Syntax) -> EquationFactory {
        EquationFactory { syntax }
    }

    pub fn parse<T: NumericType>(&self, equation_string: &str) -> Result<Equation<T>, Error> {
        Ok(Equation::new("f"))
    }
}
