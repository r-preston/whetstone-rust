use std::{cell::RefCell, rc::Rc};

use super::Expression;
use crate::{equation::Value, NumericType};

pub struct Variable<T: NumericType> {
    label: String,
    value: Rc<RefCell<T>>,
}

impl<T: NumericType> Variable<T> {
    pub fn new(label: &str, shared_value: &Rc<RefCell<T>>) -> Variable<T> {
        Variable {
            label: label.to_string(),
            value: Rc::clone(shared_value),
        }
    }
}

impl<T: NumericType> Expression for Variable<T> {
    type ExprType = T;

    fn evaluate(&self, _values: &[T]) -> Value<T> {
        Ok(self.value.borrow().clone())
    }

    fn num_inputs(&self) -> usize {
        0
    }
}

impl<T: NumericType> std::fmt::Display for Variable<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.label)
    }
}
