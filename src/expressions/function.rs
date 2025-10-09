use super::Expression;
use crate::{
    equation::Value,
    error::{return_error, Error, ErrorType},
    NumericType,
};

pub type FunctionPointer<T> = fn(&[T]) -> Value<T>;

pub struct Function<T> {
    pub label: String,
    pub function: FunctionPointer<T>,
    pub num_inputs: usize,
}

impl<T: NumericType> Clone for Function<T> {
    fn clone(&self) -> Self {
        Function::<T>::new(self.function, self.num_inputs, self.label.clone())
    }
}

impl<T: NumericType> Function<T> {
    pub fn new(function: FunctionPointer<T>, num_inputs: usize, label: String) -> Function<T> {
        Function {
            label,
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
            );
        }
        (self.function)(values)
    }

    fn num_inputs(&self) -> usize {
        self.num_inputs
    }
}

impl<T> std::fmt::Display for Function<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Function[{}({})]", self.label, self.num_inputs)
    }
}
