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
        let json_rules = match serde_json::from_str(&json_string) {
            Ok(json) => match json {
                serde_json::Value::Array(json_array) => json_array,
                _ => {
                    return_error!(
                        ErrorType::FileReadError,
                        format!("Syntax file should be a list of rule objects")
                    );
                }
            },
            Err(msg) => {
                return_error!(ErrorType::FileReadError, msg.to_string());
            }
        };

        let rules = Ruleset { rules: Vec::new() };

        for json_rule in json_rules {
            match json_rule {
                serde_json::Value::Object(rule) => {}
                _ => {
                    return_error!(
                        ErrorType::FileReadError,
                        format!("Syntax file should be a list of rule objects")
                    );
                }
            }
        }

        return Ok(rules);
    }
}
