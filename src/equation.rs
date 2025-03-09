use std::marker::PhantomData;

use crate::{Error, ErrorType, NumericType, return_error};
use crate::expressions::{value::Value, variable::Variable};

trait Expression {

}
// equation has three components: constant, variable, function.
// all return Value
// constant: holds a constant value
// variable: holds a value set on invocation
// function: returns a value based on zero or more arguments

// approaches:
// 1) list of things of type Component that have an evaluate() method that returns a Value
// 2) separate lists for each type of thing

pub struct Equation<T: NumericType> {
    label: String,
    data: Vec<Box<dyn Expression>>,
    phantom: PhantomData<T>
}

impl<T: NumericType> Equation::<T> {
    pub fn evaluate(&self, variables: &[Variable<T>]) -> Result<Value<T>, Error> {
        if self.data.is_empty() {
            return_error!(ErrorType::InvalidObject, "Equation is empty");
        }
        Ok(Value::new(T::from(0.0).unwrap()))
    }

    pub(crate) fn new(label: &str) -> Equation<T> {
        Equation {
            label: label.to_string(),
            data: Vec::new(),
            phantom: PhantomData
        }
    }

    fn evaluate_equation(&self, iter: impl IntoIterator) {

    }
}