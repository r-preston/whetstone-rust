pub mod constant;
pub mod function;
pub mod variable;

use crate::{NumericType, Value};

pub trait Expression<'a, T: NumericType> {
    fn evaluate(&self, values: &[Value<T>]) -> Value<'a, T>;

    fn num_inputs(&self) -> u32;
}
