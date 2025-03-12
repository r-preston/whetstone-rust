use std::{cell::Cell, rc::Rc};

use crate::{NumericType, Value};
use super::Expression;

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