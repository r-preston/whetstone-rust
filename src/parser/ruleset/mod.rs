pub mod bindings;
pub mod definitions;
mod rule;

use super::Syntax;
use crate::{
    error::{return_error, Error, ErrorType},
    NumericType,
};
use bindings::BindingMap;
use regex::Regex;
use rule::{Associativity, Category, Rule};
use serde::{Deserialize, Serialize};
use std::fs;

pub struct Ruleset<T: NumericType> {
    rules: Vec<Box<Rule<T>>>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct RuleJson {
    pattern: String,
    label: Option<String>,
    precedence: Option<u32>,
    associativity: Option<Associativity>,
    binding: Option<String>,
    follows: Option<Vec<Category>>,
    precedes: Option<Vec<Category>>,
}

#[derive(Serialize, Deserialize)]
struct RuleCategoryJson {
    category: Category,
    associativity: Associativity,
    precedence: u32,
    follows: Vec<Category>,
    precedes: Vec<Category>,
    rules: Vec<RuleJson>,
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

impl<T: NumericType + 'static> Ruleset<T> {
    pub fn load_ruleset(path: &str, function_bindings: BindingMap<T>) -> Result<Ruleset<T>, Error> {
        if !fs::exists(path).unwrap_or(false) {
            return_error!(
                ErrorType::FileNotFoundError,
                format!("Could not find '{path}'")
            );
        }

        let json_string: String = match fs::read_to_string(path) {
            Ok(data) => data,
            Err(msg) => {
                return_error!(ErrorType::FileReadError, msg.to_string());
            }
        };

        let rule_definitions = match serde_json::from_str::<Vec<RuleCategoryJson>>(&json_string) {
            Ok(deserialized) => deserialized,
            Err(e) => return_error!(
                ErrorType::RuleParseError,
                format!("JSON error in rule definition: {:?}", e)
            ),
        };

        let mut rules = Vec::new();

        for category_def in rule_definitions {
            for rule_def in category_def.rules {
                let pattern = match Regex::new(&rule_def.pattern) {
                    Ok(re) => re,
                    Err(e) => {
                        return_error!(
                            ErrorType::RuleParseError,
                            format!("Rule pattern is not a valid regex: {}", e)
                        )
                    }
                };
                let follows = rule_def
                    .follows
                    .unwrap_or_else(|| category_def.follows.clone());
                let precedes = rule_def
                    .precedes
                    .unwrap_or_else(|| category_def.precedes.clone());
                rules.push(Box::new(match category_def.category {
                    Category::Literal => Rule::new_literal_rule(pattern, follows, precedes),
                    Category::Variable => Rule::new_variable_rule(pattern, follows, precedes),
                    Category::CloseBracket | Category::OpenBracket | Category::Separator => {
                        Rule::new_non_expression_rule(
                            pattern,
                            category_def.category.clone(),
                            follows,
                            precedes,
                        )
                    }
                    Category::Constant | Category::Function | Category::Operator => {
                        let label = match rule_def.label {
                            Some(s) => s, _ => return_error!(ErrorType::RuleParseError,format!("Function, Operator and Constant rules require string field 'label'")),
                        };

                        let binding = match function_bindings.get(label.as_str()) {
                            Some(f) => f,
                            _ => return_error!(
                                ErrorType::RuleParseError,
                                format!("No binding found for label '{}'", label)
                            ),
                        };

                        Rule::new_function_rule(
                            pattern,
                            rule_def.precedence.unwrap_or(category_def.precedence),
                            category_def.category.clone(),
                            rule_def.associativity.unwrap_or(category_def.associativity),
                            binding.function,
                            follows,
                            precedes,
                            binding.num_inputs,
                        )
                    }
                }))
            }
        }

        return Ok(Ruleset { rules });
    }
}
