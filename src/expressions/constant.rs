use std::{fmt::Display, ops::Deref};

use crate::{NumericType, Value};

use super::Expression;

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

impl<T: NumericType> Expression<T> for Constant<T> {
    fn evaluate(&self, values: &[Value<T>]) -> Value<T> {
        Ok(T::from(0.0).unwrap())
    }

    fn num_inputs(&self) -> u32 {
        0
    }
}
