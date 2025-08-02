use super::Expression;
use crate::{
    equation::Value,
    error::{return_error, Error, ErrorType},
    NumericType,
};

pub type FunctionPointer<T> = fn(&[T]) -> Value<T>;

pub struct Function<T> {
    pub function: FunctionPointer<T>,
    pub num_inputs: usize,
}

impl<T: NumericType> Clone for Function<T> {
    fn clone(&self) -> Self {
        Function::<T>::new(self.function, self.num_inputs)
    }
}

impl<T: NumericType> Function<T> {
    pub fn new(function: FunctionPointer<T>, num_inputs: usize) -> Function<T> {
        Function {
            function,
            num_inputs,
        }
    }
}

impl<T: NumericType> Expression for Function<T> {
    type ExprType = T;

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
