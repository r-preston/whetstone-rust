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

impl<T: NumericType> Expression<T> for Constant<T> {
    fn evaluate(&self, values: &[Value<T>]) -> Value<T> {
        Ok(self.value)
    }

    fn num_inputs(&self) -> u32 {
        0
    }
}
