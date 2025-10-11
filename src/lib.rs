pub mod bindings;
mod equation;
mod error;
mod expressions;
mod parser;
pub mod syntax;

use std::fmt::Display;

/*
 * A factory structure that generates Equations by parsing strings
 */
pub use parser::Parser;
/*
 * A structure representing a mathematical function of a number of variables
 */
pub use equation::{Equation, Value};

pub use error::{Error, ErrorType};

// define constraint for the type of value used by an Equation
pub trait NumericType:
    num_traits::float::Float + bindings::FunctionBindings + std::str::FromStr + Display + 'static
{
}
