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
use serde::Deserialize;
use std::{collections::HashMap, fs};

pub struct Ruleset<T: NumericType> {
    rules: Vec<Box<Rule<T>>>,
}

#[derive(Deserialize)]
struct RuleJson {
    pattern: String,
    label: Option<String>,
    precedence: Option<u32>,
    associativity: Option<Associativity>,
    binding: Option<String>,
    follows: Option<Vec<Category>>,
    precedes: Option<Vec<Category>>,
}

#[derive(Deserialize)]
struct RuleCategoryJson {
    associativity: Option<Associativity>,
    precedence: Option<u32>,
    follows: Vec<Category>,
    precedes: Vec<Category>,
    rules: Vec<RuleJson>,
}

#[derive(Deserialize)]
struct RuleFileJson(
    #[serde(with = "::serde_with::rust::maps_duplicate_key_is_error")]
    HashMap<Category, RuleCategoryJson>,
);

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

        let rule_definitions = match serde_json::from_str::<RuleFileJson>(&json_string) {
            Ok(deserialized) => deserialized,
            Err(e) => return_error!(
                ErrorType::RuleParseError,
                format!("JSON error in rule definition: {:?}", e)
            ),
        };

        let mut rules: Vec<Box<Rule<T>>> = Vec::new();

        for (category, category_def) in rule_definitions.0 {
            println!("{}", category);
            for rule_def in category_def.rules {
                let pattern = match Regex::new(&format!("^({})(.*)", rule_def.pattern)) {
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
                rules.push(Box::new(match category {
                    Category::Literals => Rule::new_literal_rule(pattern, follows, precedes),
                    Category::Variables => Rule::new_variable_rule(pattern, follows, precedes),
                    Category::CloseBrackets | Category::OpenBrackets | Category::Separators => {
                        Rule::new_non_expression_rule(
                            pattern,
                            category.clone(),
                            follows,
                            precedes,
                        )
                    }
                    Category::Constants | Category::Functions | Category::Operators => {
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
                            category.clone(),
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
