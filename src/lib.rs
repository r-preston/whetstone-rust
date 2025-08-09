#![allow(dead_code)]

mod equation;
mod error;
mod expressions;
mod parser;

/*
 * A factory structure that generates Equations by parsing strings
 */
pub use expressions::function::FunctionPointer;
pub use parser::bindings;
pub use parser::{Parser, Syntax};
/*
 * A structure representing a mathematical function of a number of variables
 */
pub use equation::{Equation, Value};

pub use error::{Error, ErrorType};

// define constraint for the type of value used by an Equation
pub trait NumericType:
    num_traits::float::Float + bindings::FunctionBindings + std::str::FromStr
{
}
