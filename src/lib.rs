#![allow(dead_code)]

mod equation;
mod error;
mod expressions;
mod parser;

use parser::ruleset::bindings::BuiltinBindings;
/*
 * A factory structure that generates Equations by parsing strings
 */
pub use parser::{Parser, Syntax};
/*
 * A structure representing a mathematical function of a number of variables
 */
pub use equation::{Equation, Value};

pub use crate::parser::ruleset::bindings;
pub use error::{Error, ErrorType};

// define constraint for the type of value used by an Equation
pub trait NumericType: num_traits::float::Float + BuiltinBindings + std::str::FromStr {}
impl NumericType for f32 {}
impl NumericType for f64 {}
