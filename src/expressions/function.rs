use std::marker::PhantomData;

use crate::{NumericType, Value};

use super::Expression;

pub struct Function<T> {
    function: Box<dyn Fn(&[Value<T>]) -> Value<T>>,
    input_count: u32,
    phantom: PhantomData<T>
}

impl<T: NumericType> Expression<T> for Function<T> {
    fn evaluate(&self, values: &[Value<T>]) -> Value<T> {
        (self.function)(values)
    }

    fn num_inputs(&self) -> u32 {
        self.input_count
    }
}