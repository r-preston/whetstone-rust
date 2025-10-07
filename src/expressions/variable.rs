use std::{cell::Cell, rc::Rc};

use super::Expression;
use crate::{equation::Value, NumericType};

pub struct Variable<T: NumericType> {
    label: String,
    value: Rc<Cell<T>>,
}

impl<T: NumericType> Variable<T> {
    pub fn new(label: &str, shared_value: &Rc<Cell<T>>) -> Variable<T> {
        Variable {
            label: label.to_string(),
            value: Rc::clone(shared_value),
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
