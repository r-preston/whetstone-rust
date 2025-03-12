use std::{cell::Cell, rc::Rc};

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
