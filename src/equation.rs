use core::slice::Iter;
use std::cell::Cell;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::rc::Rc;

use crate::expressions::Expression;
use crate::{
    error::{return_error, Error, ErrorType},
    NumericType,
};

// input and return types from an Equation
pub type VariableValues<'a, T> = &'a [(&'a str, T)];
pub type Value<T> = Result<T, Error>;

pub struct Equation<T: NumericType> {
    label: String,
    // holds a list of expressions evaluated left to right
    data: Vec<Box<dyn Expression<ExprType = T>>>,
    // holds Rcs for each variable in the equation, which are also held by Variables in `data`
    variables: HashMap<String, Rc<Cell<T>>>,
    // indicates return type of this equation
    phantom: PhantomData<T>,
}

impl<T: NumericType> Equation<T> {
    pub(crate) fn new(label: &str) -> Equation<T> {
        Equation {
            label: label.to_string(),
            data: Vec::new(),
            variables: HashMap::new(),
            phantom: PhantomData,
        }
    }

    pub fn evaluate(&self, variables: VariableValues<T>) -> Value<T> {
        if self.data.is_empty() {
            return_error!(ErrorType::NotInitialisedError, "Equation is empty".to_string());
        }
        for &(label, value) in variables.iter() {
            self.set_variable(label, value)?;
        }
        self.evaluate_equation(&mut self.data.iter())
    }

    pub(crate) fn add_variable(&mut self, label: &str) {
        self.variables
            .insert(label.to_string(), Rc::new(Cell::new(T::from(0.0).unwrap())));
    }

    pub fn set_variable(&self, label: &str, value: T) -> Value<T> {
        match self.variables.get(label) {
            Some(value_cell) => {
                value_cell.replace(value);
                return Ok(value_cell.get());
            }
            None => {
                return_error!(
                    ErrorType::NoSuchVariableError,
                    "Equation does not contain a variable with that label".to_string()
                );
            }
        }
    }

    pub(crate) fn label(&self) -> String {
        let vars: Vec<String> = self.variables.keys().cloned().collect();
        return format!("{}({})", self.label, vars.join(","));
    }

    pub(crate) fn set_data(&mut self, data: Vec<Box<dyn Expression<ExprType = T>>>) {
        self.data = data;
    }

    fn evaluate_equation(&self, iter: &mut Iter<Box<dyn Expression<ExprType = T>>>) -> Value<T> {
        let expression = match iter.next() {
            Some(expression) => expression,
            None => {
                // this shouldn't happen as it implies an expression takes a number of arguments that aren't in the data vec
                return_error!(
                    ErrorType::InternalError,
                    "An unexpected error occured, equation data is internally inconsistent"
                        .to_string()
                );
            }
        };
        let mut input_values: Vec<T> = Vec::new();
        for _ in 0..expression.num_inputs() {
            input_values.push(self.evaluate_equation(iter)?);
        }
        expression.evaluate(input_values.as_slice())
    }
}
