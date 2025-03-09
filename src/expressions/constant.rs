use std::{fmt::Display, ops::Deref};

use crate::NumericType;

pub struct Constant<T: NumericType> {
    value: T,
}

impl<T: NumericType> Constant<T> {
    pub fn new(value: T) -> Constant<T> {
        Constant::<T> { value }
    }

    pub fn value(&self) -> &T {
        &self.value
    }
}

impl<T: NumericType> Deref for Constant<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.value()
    }
}

impl<T: Display + NumericType> Display for Constant<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value())
    }
}
