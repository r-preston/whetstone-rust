use std::marker::PhantomData;

use super::Expression;
use crate::{equation::Value, return_error, Error, ErrorType, NumericType};

type InnerFunction<T> = fn(&[T]) -> Value<T>;

pub struct Function<T> {
    function: InnerFunction<T>,
    num_inputs: usize,
    phantom: PhantomData<T>,
}

impl<T: NumericType> Function<T> {
    pub fn new(function: InnerFunction<T>, num_inputs: usize) -> Function<T> {
        Function {
            function,
            num_inputs,
            phantom: PhantomData::<T>,
        }
    }
}

impl<T: NumericType> Expression<T> for Function<T> {
    fn evaluate(&self, values: &[T]) -> Value<T> {
        if values.len() != self.num_inputs {
            return_error!(
                ErrorType::InternalError,
                "Tried to call a function with a different number of arguments than expected"
                    .to_string()
            );
        }
        (self.function)(values)
    }

    fn num_inputs(&self) -> usize {
        self.num_inputs
    }
}
