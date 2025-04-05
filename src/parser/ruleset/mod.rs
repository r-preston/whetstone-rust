pub mod bindings;
pub mod definitions;
mod rule;

use super::Syntax;
use crate::{
    error::{return_error, Error, ErrorType},
    NumericType,
};
use bindings::BindingMap;
use num_traits::ToPrimitive;
use rule::{Associativity, Rule, RuleCategory};

use std::{collections::HashSet, fs};

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
    pub fn load_ruleset(path: &str, function_bindings: BindingMap<T>) -> Result<Ruleset<T>, Error> {
        let json_string: String = match fs::read_to_string(path) {
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

        let mut defined_categories: HashSet<RuleCategory> = HashSet::new();
        for json_rule in json_rules {
            match json_rule {
                serde_json::Value::Object(category) => {
                    // parse category enum
                    let category_str = match category.get("category") {
                        Some(serde_json::Value::String(category_name)) => category_name,
                        _ => return_error!(
                            ErrorType::SyntaxFileError,
                            "Category objects must have string field 'category'".to_string()
                        ),
                    };
                    let category_enum = match RuleCategory::from_string(category_str) {
                        Ok(category_enum) => category_enum,
                        Err(error) => {
                            return_error!(
                                ErrorType::SyntaxFileError,
                                format!(
                                    "Value '{}' for field 'category' is not valid",
                                    category_str
                                )
                            )
                        }
                    };
                    match defined_categories.insert(category_enum.clone()) {
                        false => return_error!(
                            ErrorType::SyntaxFileError,
                            "Field 'category' of category objects must be unique".to_string()
                        ),
                        true => {}
                    }
                    // get defaults for `follows` and `precedes`
                    let context_arrays: Vec<(&str, Vec<RuleCategory>)> =
                        vec![("follows", Vec::new()), ("precedes", Vec::new())];

                    for mut context_array in context_arrays {
                        // check field is an array
                        let default_values = match category.get(context_array.0) {
                            Some(serde_json::Value::Array(json_vec)) => json_vec,
                            _ => return_error!(
                                ErrorType::SyntaxFileError,
                                format!("Category objects must have field '{}' that is an array of category names", context_array.0)
                            ),
                        };
                        // check array is array of valid category enums, and populate context_arrays with parsed enums
                        for val in default_values {
                            match val.as_str() {
                                Some(category_str) => match RuleCategory::from_string(category_str)
                                {
                                    Ok(category_enum) => context_array.1.push(category_enum),
                                    Err(_) => return_error!(
                                        ErrorType::SyntaxFileError,
                                        format!(
                                            "Value '{}' in array '{}' is not a valid category",
                                            category_str, context_array.0
                                        )
                                    ),
                                },
                                None => return_error!(
                                    ErrorType::SyntaxFileError,
                                    format!(
                                        "Values in array '{}' must be strings",
                                        context_array.0
                                    )
                                ),
                            }
                        }
                    }
                    // get precedence
                    let default_precedence = match category.get("precedence") {
                        Some(serde_json::Value::Number(precedence)) => match precedence.as_u64().unwrap_or(u64::MAX).to_u32() {
                            Some(n) => n,
                            None => return_error!(
                                ErrorType::SyntaxFileError,
                                "Field 'precedence' is not a valid 32 bit integer".to_string()
                            )
                        },
                        Some(_) => return_error!(
                            ErrorType::SyntaxFileError,
                            "Field 'precedence' is not a valid 32 bit integer".to_string()
                        ),
                        None => match category_enum {
                            RuleCategory::Operator | RuleCategory::Function => return_error!(
                                ErrorType::SyntaxFileError,
                                "Field 'precedence' of category objects is required when category is Function or Operator".to_string()
                            ),
                            _ => 0
                        }
                    };
                    // get associativity
                    let default_associativity = match category.get("associativity") {
                        Some(serde_json::Value::String(assoc_str)) => match assoc_str.as_str() {
                            "LeftToRight" => Associativity::LeftToRight,
                            "RightToLeft" => Associativity::RightToLeft,
                            _ => return_error!(
                                ErrorType::SyntaxFileError,
                                "Field 'associativity' must be either \"LeftToRight\" or \"RightToLeft\"".to_string()
                            ),
                        },
                        Some(_) => return_error!(
                            ErrorType::SyntaxFileError,
                            "Field 'associativity' must be either \"LeftToRight\" or \"RightToLeft\"".to_string()
                        ),
                        None => Associativity::LeftToRight
                    };
                }
                _ => {
                    return_error!(
                        ErrorType::FileReadError,
                        format!("Syntax file should be a list of category objects")
                    );
                }
            }
        }

        return Ok(rules);
    }
}
