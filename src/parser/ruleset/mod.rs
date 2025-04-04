mod definitions;
mod rule;

use super::Syntax;
use crate::{
    error::{return_error, Error, ErrorType},
    NumericType,
};
use rule::Rule;

use std::fs;

pub struct Ruleset<T: NumericType> {
    rules: Vec<Box<Rule<T>>>,
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

impl<T: NumericType> Ruleset<T> {
    pub fn load_ruleset(path: &str) -> Result<Ruleset<T>, Error> {
        let json_string = match fs::read_to_string(path) {
            Ok(data) => data,
            Err(msg) => {
                return_error!(ErrorType::FileNotFound, msg.to_string());
            }
        };
        let json_rules: serde_json::Value = match serde_json::from_str(&json_string) {
            Ok(json) => json,
            Err(msg) => {
                return_error!(ErrorType::FileReadError, msg.to_string());
            }
        };
        return Ok(Ruleset { rules: Vec::new() });
    }
}
