use std::{cell::Cell, rc::Rc};

use regex::Regex;

use super::Expression;
use crate::{equation::Value, NumericType};

pub fn validate_label(label: &str) -> bool {
    let invalid_chars = Regex::new(r"[\s()%^*/><+=-]").unwrap();
    !(label.is_empty() && invalid_chars.is_match(label))
}

pub struct Variable<T: NumericType> {
    label: String,
    value: Rc<Cell<T>>,
}

impl<T: NumericType> Variable<T> {
    pub fn new(label: &str, initial_value: T) -> Variable<T> {
        Variable {
            label: label.to_string(),
            value: Rc::new(Cell::new(initial_value)),
        }
    }
}

impl<T: NumericType> Expression for Variable<T> {
    type ExprType = T;

    fn evaluate(&self, _values: &[T]) -> Value<T> {
        Ok(self.value.get())
    }

    fn num_inputs(&self) -> usize {
        0
    }
}
