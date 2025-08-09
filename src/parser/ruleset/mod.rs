mod rule;

use super::Syntax;
use crate::{
    error::{return_error, Error, ErrorType},
    expressions::function::Function,
    parser::bindings::FunctionBindings,
    NumericType,
};
use regex::Regex;
use rule::{Associativity, Category, Rule};
use serde::{Deserialize, Serialize};
use std::fs;

pub struct Ruleset<T: NumericType> {
    rules: Vec<Box<Rule<T>>>,
}

#[derive(Serialize, Deserialize)]
struct RuleJson {
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
    associativity: Option<Associativity>,
    precedence: Option<u32>,
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

impl<T: NumericType<ExprType = T> + FunctionBindings + 'static> Ruleset<T> {
    pub fn load_ruleset(path: &str) -> Result<Ruleset<T>, Error> {
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
        for category in rule_definitions.iter() {
            let category_definitions = rule_definitions
                .iter()
                .filter(|x| x.category == category.category)
                .count();
            if category_definitions > 1 {
                return_error!(
                    ErrorType::RuleParseError,
                    format!("Multiple definitions for category '{}'", category.category)
                );
            }
        }

        let mut rules: Vec<Box<Rule<T>>> = Vec::new();

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

                        let binding_opt: Option<Function<T>> = <T as FunctionBindings>::get_binding(&label);
                        let binding: Function<T> = match binding_opt {
                            Some(f) => f,
                            _ => return_error!(
                                ErrorType::RuleParseError,
                                format!("No binding found for label '{}' and type {}", label, "f32")
                            ),
                        };
                        let associativity = rule_def.associativity.unwrap_or(category_def.associativity.unwrap_or(Associativity::LeftToRight));
                        let precedence = match category_def.precedence {
                            Some(n) => n,
                            None => match rule_def.precedence {
                                Some(n) => n,
                                None => {return_error!(ErrorType::RuleParseError, format!("Field 'precedence' is required for Operator and Function rules"))}
                            }
                        };
                        Rule::new_function_rule(
                            pattern,
                            precedence,
                            category_def.category.clone(),
                            associativity,
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
