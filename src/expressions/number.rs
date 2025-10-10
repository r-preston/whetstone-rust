use super::Expression;
use crate::{equation::Value, NumericType};

pub struct Number<T: NumericType> {
    value: T,
}

impl<T: NumericType> Number<T> {
    pub fn new(value: T) -> Number<T> {
        Number::<T> { value }
    }
}

impl<T: NumericType> Expression for Number<T> {
    type ExprType = T;

    fn evaluate(&self, _values: &[T]) -> Value<T> {
        Ok(self.value)
    }

    fn num_inputs(&self) -> usize {
        0
    }
}

impl<T: NumericType> std::fmt::Display for Number<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
