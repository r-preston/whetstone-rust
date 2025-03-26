mod standard;

use standard::StandardRuleset;

use crate::{expressions::function::Function, NumericType, Syntax};

pub fn get_rulesets() -> &'static [(Syntax, impl Ruleset)] {
    &[(Syntax::Standard, StandardRuleset {})]
}

pub trait Ruleset {
    fn get_rules<T: NumericType>() -> &[Rule<T>];
}

pub enum RuleCategory {
    Bracket,
    Operator,
    Function,
    Constant,
    Variable,
}

pub struct Rule<T: NumericType> {
    pub regex: String,
    pub priority: u32,
    pub category: RuleCategory,
    pub function: Function<T>,
}
