use core::slice::Iter;
use std::cell::Cell;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::rc::Rc;

use crate::expressions::Expression;
use crate::{return_error, Error, ErrorType, NumericType, Value, VariableValues};

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
    // holds a list of expressions evaluated left to right
    data: Vec<Box<dyn Expression<'a, T>>>,
    // holds Rcs for each variable in the equation, which are also held by Variables in `data`
    variables: HashMap<String, Rc<Cell<T>>>,
    // indicates return type of this equation
    phantom: PhantomData<T>,
}

impl<'a, T: NumericType> Equation<'a, T> {
    pub fn evaluate(&self, variables: VariableValues<T>) -> Value<T> {
        if self.data.is_empty() {
            return_error!(ErrorType::InvalidObject, "Equation is empty");
        }
        for &(label, value) in variables.iter() {
            self.set_variable(label, value)?;
        }
        self.evaluate_equation(&mut self.data.iter())
    }

    pub fn set_variable(&self, label: &str, value: T) -> Value<T> {
        match self.variables.get(label) {
            Some(value_cell) => {
                value_cell.replace(value);
                return Ok(value_cell.get());
            },
            None => {
                return_error!(ErrorType::NoSuchVariable, "Equation does not contain a variable with that label");
            }
        }
    }

    pub(crate) fn new(label: &str) -> Equation<T> {
        Equation {
            label: label.to_string(),
            data: Vec::new(),
            variables: HashMap::new(),
            phantom: PhantomData,
        }
    }

    fn evaluate_equation(&self, iter: &mut Iter<Box<dyn Expression<'a, T>>>) -> Value<'a, T> {
        let expression = match iter.next() {
            Some(expression) => expression,
            None => {
                // this shouldn't happen as it implies an expression takes a number of arguments that aren't in the data vec
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
