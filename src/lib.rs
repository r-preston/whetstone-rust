#![allow(dead_code)]
#![allow(unused_variables)]

mod equation;
mod expressions;
mod parser;

pub use equation::Equation;
pub use parser::rulesets::Syntax;
use parser::rulesets::{get_builtin_ruleset, load_ruleset, Ruleset};

use expressions::variable::validate_label;
use regex::Regex;

// define constraint for the type of value used by an Equation
pub trait NumericType: num_traits::float::Float {}
impl<T: num_traits::float::Float> NumericType for T {}

#[derive(Debug)]
pub enum ErrorType {
    FileNotFound,
    FileReadError,
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

pub struct EquationFactory<T: NumericType> {
    syntax: Syntax,
    syntax_rules: Ruleset<T>
}

impl<T: NumericType> EquationFactory<T> {
    pub fn new(syntax: Syntax) -> Result<EquationFactory<T>, Error> {
        let rule_file: &str = match syntax {
            Syntax::Custom(ref file) => file,
            ref builtin => match get_builtin_ruleset(&builtin) {
                Some(ruleset) => ruleset,
                None => {
                    return_error!(
                        ErrorType::InternalError,
                        "Syntax does not have any built-in rules registered".to_string()
                    );
                }
            },
        };
        match load_ruleset(&rule_file) {
            Ok(syntax_rules) => Ok(EquationFactory::<T> {
                syntax,
                syntax_rules
            }),
            Err(message) => Err(message),
        }
    }

    pub fn parse(&self, equation_string: &str) -> Result<Equation<T>, Error> {
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

        // get label, if specified
        let mut equation = Equation::new(match captures.get(1) {
            Some(label) => label.as_str(),
            None => "f",
        });
        // get explicit variables, if present
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
                equation.add_variable(trimmed_var);
            }
        }
        // actual equation
        let expression_string = match captures.get(3) {
            Some(group) => group.as_str(),
            None => {
                return_error!(
                    ErrorType::InvalidInput,
                    "Could not parse equation".to_string()
                );
            }
        };

        Ok(equation)
    }
}
