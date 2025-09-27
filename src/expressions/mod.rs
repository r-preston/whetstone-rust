pub mod function;
pub mod number;
pub mod variable;

use crate::equation::Value;

pub trait Expression {
    type ExprType;

    fn evaluate(&self, values: &[Self::ExprType]) -> Value<Self::ExprType>;

    fn num_inputs(&self) -> usize;
}
