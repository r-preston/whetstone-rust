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
    // holds a list of expressions evaluated left to right
    data: Vec<Box<dyn Expression<ExprType = T>>>,
    // holds Rcs for each variable in the equation, which are also held by Variables in `data`
    variables: HashMap<String, Rc<Cell<T>>>,
    // indicates return type of this equation
    phantom: PhantomData<T>,
}

impl<T: NumericType> Equation<T> {
    pub(crate) fn new(
        data: Vec<Box<dyn Expression<ExprType = T>>>,
        variables: HashMap<String, Rc<Cell<T>>>,
    ) -> Equation<T> {
        Equation {
            data,
            variables,
            phantom: PhantomData,
        }
    }

    pub fn evaluate(&self, variables: VariableValues<T>) -> Value<T> {
        if self.data.is_empty() {
            return_error!(ErrorType::NotInitialisedError, "Equation is empty");
        }
        // set variables
        for &(label, value) in variables.iter() {
            self.set_variable(label, value)?;
        }

        let mut output_stack = Vec::new();

        for expression in &self.data {
            if expression.num_inputs() > output_stack.len() {
                return_error!(
                ErrorType::SyntaxError,
                "Function {} requires {} inputs but the output stack contains {}",
                expression, expression.num_inputs(), output_stack.len()

            );
            }
            let input_values = output_stack.split_off(output_stack.len() - expression.num_inputs());
            output_stack.push(expression.evaluate(input_values.as_slice())?);
        }

        if output_stack.len() != 1 {
            return_error!(
                ErrorType::SyntaxError,
                "Equation does not evaluate to a single value"
            );
        }

        return Ok(output_stack[0]);
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
                    "Equation does not contain a variable with that label"
                );
            }
        }
    }

    pub fn variables(&self) -> Vec<String> {
        self.variables.keys().cloned().collect()
    }
}
