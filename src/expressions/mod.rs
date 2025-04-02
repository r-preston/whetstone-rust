pub mod constant;
pub mod function;
pub mod variable;

use crate::{equation::Value, parser::rulesets::RuleType, NumericType};

pub struct Generator {
    expression_type: RuleType
}

pub trait Expression<T: NumericType> {
    fn evaluate(&self, values: &[T]) -> Value<T>;

    fn num_inputs(&self) -> usize;
}
