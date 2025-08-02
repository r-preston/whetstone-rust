use super::Expression;
use crate::{equation::Value, NumericType};

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

impl<T: NumericType> Expression for Constant<T> {
    type ExprType = T;

    fn evaluate(&self, _values: &[T]) -> Value<T> {
        Ok(self.value)
    }

    fn num_inputs(&self) -> usize {
        0
    }
}
