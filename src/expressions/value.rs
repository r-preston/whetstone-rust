use std::{fmt::Display, ops::Deref};

use crate::NumericType;

pub struct Value<T: NumericType> {
    value: T,
}

impl<T: NumericType> Value<T> {
    pub fn new(value: T) -> Value<T> {
        Value::<T> { value }
    }

    pub fn value(&self) -> &T {
        &self.value
    }
}

impl<T: NumericType> Deref for Value<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.value()
    }
}

impl<T: Display + NumericType> Display for Value<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value())
    }
}