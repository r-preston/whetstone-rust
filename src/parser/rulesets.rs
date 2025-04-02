use std::fs;

use crate::{expressions::Expression, return_error, Error, NumericType};

#[derive(PartialEq)]
pub enum Syntax {
    Standard,
    LaTeX,
    Custom(String),
}

pub enum RuleType {
    LeftBracket,
    RightBracket,
    Separator,
    Operator,
    Function,
    Constant,
    Variable,
}

pub enum Associativity {
    Left,
    Right,
}

fn builtin_rulesets() -> &'static [(Syntax, &'static str)] {
    &[
        (Syntax::Standard, "syntax/standard.json"),
        (Syntax::LaTeX, "syntax/latex.json"),
    ]
}

pub fn get_builtin_ruleset(syntax: &Syntax) -> Option<&'static str> {
    let builtins = builtin_rulesets();
    match builtins.iter().position(|x| x.0 == *syntax) {
        Some(index) => Some(builtins[index].1),
        None => None,
    }
}

pub fn load_ruleset<T: NumericType>(path: &str) -> Result<Ruleset<T>, Error> {
    let json_string = match fs::read_to_string(path) {
        Ok(data) => data,
        Err(msg) => { return_error!(crate::ErrorType::FileNotFound, msg.to_string()); }
    };
    let json_rules: serde_json::Value = match serde_json::from_str(&json_string) {
        Ok(json) => json,
        Err(msg) => { return_error!(crate::ErrorType::FileReadError, msg.to_string()); }
    };
    return Ok(Ruleset { rules: Vec::new() })
}

pub struct Ruleset<T: NumericType> {
    rules: Vec<Rule<T>>,
}

pub struct Rule<T: NumericType> {
    pub regex: String,
    pub priority: u32,
    pub associativity: Associativity,
    pub rule_type: RuleType,
    pub generator: Box<dyn Fn() -> dyn Expression<T>>,
}
