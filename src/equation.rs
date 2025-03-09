use core::slice::Iter;
use std::marker::PhantomData;

use crate::expressions::Expression;
use crate::{return_error, Error, ErrorType, NumericType, Value, Variable};

// equation has three components: constant, variable, function.
// all return Value
// constant: holds a constant value
// variable: holds a value set on invocation
// function: returns a value based on zero or more arguments

// approaches:
// 1) list of things of type Component that have an evaluate() method that returns a Value
// 2) separate lists for each type of thing

pub struct Equation<'a, T: NumericType> {
    label: String,
    data: Vec<Box<dyn Expression<'a, T>>>,
    phantom: PhantomData<T>,
}

impl<'a, T: NumericType> Equation<'a, T> {
    pub fn evaluate(&self, variables: &[Variable<T>]) -> Value<T> {
        if self.data.is_empty() {
            return_error!(ErrorType::InvalidObject, "Equation is empty");
        }
        // todo: set variable values
        self.evaluate_equation(&mut self.data.iter())
    }

    pub(crate) fn new(label: &str) -> Equation<T> {
        Equation {
            label: label.to_string(),
            data: Vec::new(),
            phantom: PhantomData,
        }
    }

    fn evaluate_equation(&self, iter: &mut Iter<Box<dyn Expression<'a, T>>>) -> Value<'a, T> {
        let expression = match iter.next() {
            Some(expression) => expression,
            None => {
                return_error!(
                    ErrorType::InvalidObject,
                    "An unexpected error occured, equation data is internally inconsistent"
                );
            }
        };
        let mut input_values: Vec<Value<T>> = Vec::new();
        for i in 0..expression.num_inputs() {
            input_values.push(self.evaluate_equation(iter))
        }
        expression.evaluate(input_values.as_slice())
    }
}
