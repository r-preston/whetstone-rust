pub mod rule;

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

pub(crate) struct Ruleset<T: NumericType>(Vec<Box<Rule<T>>>);

#[derive(Deserialize)]
struct RuleJson {
    pattern: Option<String>,
    precedence: Option<u32>,
    associativity: Option<Associativity>,
    binding: Option<String>,
    may_follow: Option<Vec<Category>>,
}

#[derive(Deserialize)]
struct RuleCategoryJson {
    default_associativity: Option<Associativity>,
    default_precedence: Option<u32>,
    may_follow: Vec<Category>,
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
            return_error!(ErrorType::FileNotFoundError, "Could not find '{path}'");
        }

        let json_string: String = match fs::read_to_string(path) {
            Ok(data) => data,
            Err(msg) => {
                return_error!(ErrorType::FileReadError, "{}", msg);
            }
        };

        let rule_definitions = match serde_json::from_str::<RuleFileJson>(&json_string) {
            Ok(deserialized) => deserialized,
            Err(e) => return_error!(
                ErrorType::RuleParseError,
                "JSON error in rule definition: {:?}",
                e
            ),
        };

        let mut rules: Vec<Box<Rule<T>>> = Vec::new();

        for (category, category_def) in rule_definitions.0 {
            for rule_def in category_def.rules {
                if rule_def.pattern.is_none() {
                    if category != Category::ImplicitOperators {
                        return_error!(ErrorType::RuleParseError, "Rules require field 'pattern'")
                    }
                } else if category == Category::ImplicitOperators {
                    return_error!(
                        ErrorType::RuleParseError,
                        "ImplicitOperators no not support field 'pattern'"
                    )
                }
                let pattern = match Regex::new(&format!(
                    "^({})(.*)",
                    rule_def.pattern.unwrap_or(String::new())
                )) {
                    Ok(re) => re,
                    Err(e) => {
                        return_error!(
                            ErrorType::RuleParseError,
                            "Rule pattern is not a valid regex: {}",
                            e
                        )
                    }
                };
                let follows = rule_def
                    .may_follow
                    .unwrap_or_else(|| category_def.may_follow.clone());
                rules.push(Box::new(match category {
                    Category::Literals => Rule::new_literal_rule(pattern, follows),
                    Category::Variables => Rule::new_variable_rule(pattern, follows),
                    Category::CloseBrackets | Category::OpenBrackets | Category::Separators => {
                        Rule::new_non_expression_rule(
                            pattern,
                            category.clone(),
                            follows,
                        )
                    }
                    Category::Constants | Category::Functions | Category::Operators | Category::ImplicitOperators => {
                        let binding = match rule_def.binding {
                            Some(s) => s, _ => return_error!(ErrorType::RuleParseError, "Function, Operator and Constant rules require string field 'binding'"),
                        };

                        let binding_opt: Option<Function<T>> = <T as FunctionBindings>::get_binding(&binding);
                        let binding: Function<T> = match binding_opt {
                            Some(f) => f,
                            _ => return_error!(
                                ErrorType::RuleParseError,
                                "No binding found with label '{}' and type {}", binding, std::any::type_name::<T>()
                            ),
                        };
                        let associativity = rule_def.associativity.unwrap_or(category_def.default_associativity.unwrap_or(Associativity::LeftToRight));
                        let precedence = match category_def.default_precedence {
                            Some(n) => n,
                            None => match rule_def.precedence {
                                Some(n) => n,
                                None => {return_error!(ErrorType::RuleParseError, "Field 'precedence' is required for Operator and Function rules")}
                            }
                        };
                        Rule::new_function_rule(
                            pattern,
                            precedence,
                            category.clone(),
                            associativity,
                            binding.function,
                            follows,
                            binding.num_inputs,
                            &binding.label
                        )
                    }
                }));
            }
        }

        return Ok(Ruleset(rules));
    }
}

impl<T: NumericType> core::ops::Deref for Ruleset<T> {
    type Target = Vec<Box<Rule<T>>>;

    fn deref(self: &'_ Self) -> &'_ Self::Target {
        &self.0
    }
}
