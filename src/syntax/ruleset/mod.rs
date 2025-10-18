pub(crate) mod rule;

use super::{Associativity, Category};
use crate::{
    bindings::FunctionBindings,
    error::{return_error, Error, ErrorType},
    expressions::function::Function,
    syntax::RuleCollectionDefinition,
    NumericType,
};
use regex::Regex;
use rule::Rule;

pub(crate) struct Ruleset<T: NumericType>(Vec<Box<Rule<T>>>);

impl<T: NumericType<ExprType = T> + FunctionBindings> Ruleset<T> {
    pub fn create(rule_definitions: RuleCollectionDefinition) -> Result<Ruleset<T>, Error> {
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
                let json_pattern = rule_def.pattern.unwrap_or(String::new());
                let pattern = match Regex::new(&format!(
                    r"^(?i)({})(.*)",
                    match rule_def.pattern_is_regex {
                        None | Some(false) => regex::escape(&json_pattern),
                        Some(true) => json_pattern,
                    }
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
                    Category::Separators | Category::Fluff => {
                        Rule::new_non_expression_rule(
                            pattern,
                            category.clone(),
                            follows,
                        )
                    }
                    Category::CloseBrackets | Category::OpenBrackets => {
                        match rule_def.context {
                            Some(i) => Rule::new_bracket_rule(pattern, category.clone(), follows, i),
                            None => return_error!(ErrorType::RuleParseError, "Parenthesis rules require integer field 'context'"),
                        }
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
                                None => match category {
                                    Category::Operators | Category::ImplicitOperators => {return_error!(ErrorType::RuleParseError, "Field 'precedence' is required for Operator rules")}
                                    _ => 0,
                                }
                            }
                        };
                        Rule::new_function_rule(
                            pattern,
                            precedence,
                            category.clone(),
                            associativity,
                            binding,
                            follows,
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
