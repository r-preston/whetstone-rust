#![allow(dead_code)]
#![allow(unused_variables)]

mod equation;
mod expressions;

pub use equation::Equation;

use expressions::variable::validate_label;
use regex::Regex;

// define constraint for the type of value used by an Equation
pub trait NumericType: num_traits::float::Float {}
impl<T: num_traits::float::Float> NumericType for T {}

#[derive(Debug)]
pub enum ErrorType {
    SyntaxError,
    InvalidInput,
    NotInitialised,
    NoSuchVariable,
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
        });
    };
}
pub(crate) use return_error;

// input and return types from an Equation
pub type VariableValues<'a, T> = &'a [(&'a str, T)];
type Value<T> = Result<T, Error>;

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
        /*
         * equations can have the following forms:
         * <expression>
         * <label> = <expression>
         * <label>(<variables>) = <expression>
         *
         * If no label is provided, the function is labelled 'f'.
         * If a comma separated list of variables is provided between brackets after the label, the parser will only accept explicitly named variables in the expression
         */
        if equation_string.is_empty() {
            return_error!(
                ErrorType::InvalidInput,
                "Equation string should not be empty".to_string()
            );
        }
        let labeled_func = Regex::new(r"^(?:([^\(\)]+?)(?:\((.*)\))?\s*=\s*)?(.+)$").unwrap();
        let captures = match labeled_func.captures(equation_string) {
            Some(captures) => captures,
            None => {
                return_error!(
                    ErrorType::InvalidInput,
                    "Could not parse equation".to_string()
                );
            }
        };

        let mut equation = Equation::new(match captures.get(1) {
            Some(label) => label.as_str(),
            None => "f",
        });
        if captures.get(2).is_some() {
            // split list of variables
            let variables = captures.get(2).unwrap().as_str().split(",");
            for variable in variables.into_iter() {
                let trimmed_var = variable.trim();
                if !validate_label(trimmed_var) {
                    return_error!(
                        ErrorType::InvalidInput,
                        format!("Explicit variable '{}' is not valid", trimmed_var)
                    );
                }
            }
        }

        Ok(equation)
    }
}
