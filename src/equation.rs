use std::cell::{RefCell, RefMut};
use std::collections::HashMap;
use std::rc::Rc;

use crate::expressions::Expression;
use crate::{
    error::{return_error, Error, ErrorType},
    NumericType,
};

pub type Value<T> = Result<T, Error>;

pub struct Equation<T: NumericType> {
    // holds a list of expressions evaluated left to right
    data: Vec<Box<dyn Expression<ExprType = T>>>,
    variable_names: Vec<String>,
    // holds Rcs for each variable in the equation, which are also held by Variables in `data`
    variables: HashMap<String, Rc<RefCell<T>>>,
}

impl<T: NumericType> Equation<T> {
    pub(crate) fn new(
        data: Vec<Box<dyn Expression<ExprType = T>>>,
        variables: HashMap<String, Rc<RefCell<T>>>,
    ) -> Equation<T> {
        let variable_names = variables.keys().cloned().collect();
        Equation {
            data,
            variable_names,
            variables,
        }
    }

    pub fn evaluate(&self) -> Value<T> {
        if self.data.is_empty() {
            return_error!(ErrorType::NotInitialisedError, "Equation is empty");
        }

        let mut output_stack = Vec::new();

        for expression in &self.data {
            if expression.num_inputs() > output_stack.len() {
                return_error!(
                    ErrorType::SyntaxError,
                    "Function {} requires {} inputs but the output stack contains {}",
                    expression,
                    expression.num_inputs(),
                    output_stack.len()
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

    pub fn variable(&self, label: &str) -> Result<RefMut<T>, Error> {
        match self.variables.get(label) {
            Some(var_cell) => match var_cell.try_borrow_mut() {
                Ok(var) => Ok(var),
                Err(..) => return_error!(
                    ErrorType::VariableAccessError,
                    "Variable '{}' already mutably borrowed",
                    label
                ),
            },
            None => return_error!(
                ErrorType::VariableAccessError,
                "Equation does not contain variable '{}'",
                label
            ),
        }
    }

    pub fn variables(&self) -> &[String] {
        &self.variable_names
    }
}
