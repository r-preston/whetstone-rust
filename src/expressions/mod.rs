pub(crate) mod function;
pub(crate) mod number;
pub(crate) mod variable;

use crate::equation::Value;

pub trait Expression: std::fmt::Display {
    type ExprType;

    fn evaluate(&self, values: &[Self::ExprType]) -> Value<Self::ExprType>;

    fn num_inputs(&self) -> usize;
}
