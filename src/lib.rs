#![allow(dead_code)]
#![allow(unused_variables)]

mod equation;
mod error;
mod expressions;
mod parser;

/*
 * A factory structure that generates Equations by parsing strings
 */
pub use parser::{Parser, Syntax};
/*
 * A structure representing a mathematical function of a number of variables
 */
pub use equation::Equation;

// define constraint for the type of value used by an Equation
pub trait NumericType: num_traits::float::Float {}
impl<T: num_traits::float::Float> NumericType for T {}
