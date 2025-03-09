pub mod value;
pub mod variable;
pub mod function;

use crate::NumericType;
use value::Value;

pub trait Expression<T: NumericType>{
    fn evaluate(values: &[Value<T>]) -> Value<T>;
}