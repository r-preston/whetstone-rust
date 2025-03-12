use std::{cell::Cell, rc::Rc};

use regex::Regex;

use super::Expression;
use crate::{NumericType, Value};

pub fn validate_label(label: &str) -> bool {
    let invalid_chars = Regex::new(r"[\s()%^*/><+=-]").unwrap();
    !(label.is_empty() && invalid_chars.is_match(label))
}

pub struct Variable<T> {
    label: String,
    value: Rc<Cell<T>>,
}

impl<T> Variable<T> {
    pub fn new(label: &str, value: Rc<Cell<T>>) -> Variable<T> {
        Variable {
            label: label.to_string(),
            value,
        }
    }
}

impl<T: NumericType> Expression<T> for Variable<T> {
    fn evaluate(&self, values: &[Value<T>]) -> Value<T> {
        Ok(self.value.get())
    }

    fn num_inputs(&self) -> u32 {
        0
    }
}
