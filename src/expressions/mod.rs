pub mod constant;
pub mod function;
pub mod variable;

use crate::{NumericType, Value};

pub trait Expression<T: NumericType> {
    fn evaluate(&self, values: &[T]) -> Value<T>;

    fn num_inputs(&self) -> usize;
}
