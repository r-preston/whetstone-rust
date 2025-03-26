use crate::{expressions::function::Function, parser::definitions, NumericType};

use super::{Rule, RuleCategory, Ruleset};

pub struct StandardRuleset {}

impl<'a> Ruleset<'a> for StandardRuleset {
    fn get_rules<T: NumericType>() -> &'a [Rule<T>] {
        &[Rule {
            regex: String::from(""),
            priority: 1,
            category: RuleCategory::Bracket,
            function: Function::new(definitions::add, 2),
        }]
    }
}
