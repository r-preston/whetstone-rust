use std::marker::PhantomData;

use super::Expression;
use crate::{NumericType, Value};

type InnerFunction<T> = fn(&[Value<T>]) -> Value<T>;

pub struct Function<T> {
    function: InnerFunction<T>,
    num_inputs: u32,
    phantom: PhantomData<T>,
}

impl<T: NumericType> Function<T> {
    pub fn new(function: InnerFunction<T>, num_inputs: u32) -> Function<T> {
        Function {
            function,
            num_inputs,
            phantom: PhantomData::<T>,
        }
    }
}

impl<T: NumericType> Expression<T> for Function<T> {
    fn evaluate(&self, values: &[Value<T>]) -> Value<T> {
        (self.function)(values)
    }

    fn num_inputs(&self) -> u32 {
        self.num_inputs
    }
}
