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
use rule::{Associativity, Category, Rule};

use std::{collections::HashSet, fs};

pub struct Ruleset<T: NumericType> {
    rules: Vec<Box<Rule<T>>>,
}

struct RuleCategory {
    category_enum: Category,
    default_associativity: Associativity,
    default_precedence: u32,
    default_follows: Option<Vec<Category>>,
    default_precedes: Option<Vec<Category>>,
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
        let json_string: String = match fs::read_to_string(path) {
            Ok(data) => data,
            Err(msg) => {
                return_error!(ErrorType::FileNotFound, msg.to_string());
            }
        };
        let json_categories = match serde_json::from_str(&json_string) {
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

        let mut rules = Ruleset { rules: Vec::new() };

        let mut defined_categories: HashSet<Category> = HashSet::new();
        for json_category in json_categories {
            match json_category {
                serde_json::Value::Object(category) => {
                    let category_enum =
                        Self::parse_category_field(&category, &mut defined_categories)?;
                    // contains default values for each rule
                    let rule_category = RuleCategory {
                        category_enum: category_enum.clone(),
                        default_associativity: Self::parse_associativity_field(
                            &category,
                            Associativity::LeftToRight,
                        )?,
                        default_precedence: match category_enum {
                            Category::Operator | Category::Function => {
                                match Self::parse_precedence_field(&category)? {
                                    Some(n) => n,
                                    _ => {
                                        return_error!(
                                            ErrorType::SyntaxFileError,
                                            "Field 'precedence' of category objects is required when category is Function or Operator".to_string()
                                        )
                                    }
                                }
                            }
                            _ => 0,
                        },
                        default_follows: Self::parse_category_array_field(&category, "follows")?,
                        default_precedes: Self::parse_category_array_field(&category, "precedes")?,
                    };

                    let rules_json = match category.get("rules") {
                        Some(serde_json::Value::Array(rules)) => rules,
                        _ => return_error!(
                            ErrorType::SyntaxFileError,
                            "Category objects must have non-empty array field 'rules'".to_string()
                        ),
                    };

                    for rule in rules_json {
                        rules.rules.push(Box::new(Self::parse_rule(
                            rule,
                            &rule_category,
                            &function_bindings,
                        )?));
                    }
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

    fn parse_rule(
        rule_json: &serde_json::Value,
        category_defaults: &RuleCategory,
        function_bindings: &BindingMap<T>,
    ) -> Result<Rule<T>, Error> {
        let rule_object = match rule_json {
            serde_json::Value::Object(obj) => obj,
            _ => {
                return_error!(
                    ErrorType::FileReadError,
                    format!("Rules list should be a list of rule objects")
                );
            }
        };

        let pattern = match rule_object.get("pattern") {
            Some(serde_json::Value::String(ptn)) => ptn,
            _ => return_error!(
                ErrorType::SyntaxFileError,
                "Rule objects must have non-empty string field 'pattern'".to_string()
            ),
        }
        .to_string();
        if pattern.is_empty() {
            return_error!(
                ErrorType::SyntaxFileError,
                "Rule objects must have non-empty string field 'pattern'".to_string()
            );
        }

        let follows = Self::parse_category_array_field(&rule_object, "follows")?.unwrap_or(
            category_defaults
                .default_follows
                .clone()
                .unwrap_or_default(),
        );
        let precedes = Self::parse_category_array_field(&rule_object, "precedes")?.unwrap_or(
            category_defaults
                .default_precedes
                .clone()
                .unwrap_or_default(),
        );
        if follows.is_empty() {
            return_error!(
                ErrorType::SyntaxFileError,
                "Rule object has no defined rules for 'follows'".to_string()
            );
        }
        if precedes.is_empty() {
            return_error!(
                ErrorType::SyntaxFileError,
                "Rule object has no defined rules for 'precedes'".to_string()
            );
        }

        Ok(match category_defaults.category_enum {
            Category::Literal => Rule::new_literal_rule(pattern, follows, precedes),
            Category::Variable => Rule::new_variable_rule(pattern, follows, precedes),
            Category::CloseBracket | Category::OpenBracket | Category::Separator => {
                Rule::new_non_expression_rule(
                    pattern,
                    category_defaults.category_enum.clone(),
                    follows,
                    precedes,
                )
            }
            Category::Constant | Category::Function | Category::Operator => {
                let label = match rule_object.get("label") {
                    Some(serde_json::Value::String(s)) => s.as_str(),
                    _ => return_error!(
                        ErrorType::SyntaxFileError,
                        "Function, Operator and Constant rules require string field 'Label'"
                            .to_string()
                    ),
                };
                let binding = match function_bindings.get(label) {
                    Some(f) => f,
                    _ => return_error!(
                        ErrorType::SyntaxFileError,
                        format!("No binding found for label '{}'", label)
                    ),
                };
                Rule::new_function_rule(
                    pattern,
                    Self::parse_precedence_field(&rule_object)?
                        .unwrap_or(category_defaults.default_precedence),
                    category_defaults.category_enum.clone(),
                    Self::parse_associativity_field(
                        &rule_object,
                        category_defaults.default_associativity.clone(),
                    )?,
                    binding.function,
                    follows,
                    precedes,
                    binding.num_inputs,
                )
            }
        })
    }

    // parses category enum from json
    fn parse_category_field(
        json_object: &serde_json::Map<String, serde_json::Value>,
        used_categories: &mut HashSet<Category>,
    ) -> Result<Category, Error> {
        let category_str = match json_object.get("category") {
            Some(serde_json::Value::String(category_name)) => category_name,
            _ => return_error!(
                ErrorType::SyntaxFileError,
                "Category objects must have string field 'category'".to_string()
            ),
        };
        let category_enum = match Category::from_string(category_str) {
            Ok(category_enum) => category_enum,
            Err(_) => {
                return_error!(
                    ErrorType::SyntaxFileError,
                    format!("Value '{}' for field 'category' is not valid", category_str)
                )
            }
        };
        match used_categories.insert(category_enum.clone()) {
            false => return_error!(
                ErrorType::SyntaxFileError,
                "Field 'category' of category objects must be unique".to_string()
            ),
            true => Ok(category_enum),
        }
    }

    fn parse_category_array_field(
        json_object: &serde_json::Map<String, serde_json::Value>,
        field_label: &str,
    ) -> Result<Option<Vec<Category>>, Error> {
        // get defaults for `follows` and `precedes`
        let mut context_array = Vec::new();

        // check field is an array
        let default_values = match json_object.get(field_label) {
            Some(serde_json::Value::Array(json_vec)) => json_vec,
            Some(_) => return_error!(
                ErrorType::SyntaxFileError,
                format!("Field '{}' must be an array of category names", field_label)
            ),
            None => {
                return Ok(None);
            }
        };
        // check array is array of valid category enums, and populate context_arrays with parsed enums
        for val in default_values {
            match val.as_str() {
                Some(category_str) => match Category::from_string(category_str) {
                    Ok(category_enum) => context_array.push(category_enum),
                    Err(_) => return_error!(
                        ErrorType::SyntaxFileError,
                        format!(
                            "Value '{}' in array '{}' is not a valid category",
                            category_str, field_label
                        )
                    ),
                },
                None => return_error!(
                    ErrorType::SyntaxFileError,
                    format!("Values in array '{}' must be strings", field_label)
                ),
            }
        }

        Ok(Some(context_array))
    }

    // parse precedence from json object
    fn parse_precedence_field(
        json_object: &serde_json::Map<String, serde_json::Value>,
    ) -> Result<Option<u32>, Error> {
        match json_object.get("precedence") {
            Some(serde_json::Value::Number(precedence)) => {
                match precedence.as_u64().unwrap_or(u64::MAX).to_u32() {
                    Some(n) => Ok(Some(n)),
                    None => return_error!(
                        ErrorType::SyntaxFileError,
                        "Field 'precedence' is not a valid 32 bit unsigned integer".to_string()
                    ),
                }
            }
            Some(_) => return_error!(
                ErrorType::SyntaxFileError,
                "Field 'precedence' is not a valid 32 bit unsigned integer".to_string()
            ),
            None => Ok(None),
        }
    }

    fn parse_associativity_field(
        json_object: &serde_json::Map<String, serde_json::Value>,
        default_value: Associativity,
    ) -> Result<Associativity, Error> {
        match json_object.get("associativity") {
            Some(serde_json::Value::String(assoc_str)) => match assoc_str.as_str() {
                "LeftToRight" => Ok(Associativity::LeftToRight),
                "RightToLeft" => Ok(Associativity::RightToLeft),
                _ => return_error!(
                    ErrorType::SyntaxFileError,
                    "Field 'associativity' must be either \"LeftToRight\" or \"RightToLeft\""
                        .to_string()
                ),
            },
            Some(_) => return_error!(
                ErrorType::SyntaxFileError,
                "Field 'associativity' must be either \"LeftToRight\" or \"RightToLeft\""
                    .to_string()
            ),
            None => Ok(default_value),
        }
    }
}
