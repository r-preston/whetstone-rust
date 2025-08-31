pub mod bindings;
pub mod ruleset;

use crate::equation::Equation;
use crate::expressions::Expression;
use crate::parser::ruleset::rule::{Category, Rule};
use crate::{
    error::{return_error, Error, ErrorType},
    NumericType,
};
use ruleset::Ruleset;

// TODO: remove
use crate::expressions::variable::validate_label;

use regex::Regex;

macro_rules! syntax_error {
    ($($t:tt)*) => {
        return_error!(ErrorType::SyntaxError, $($t)*)
    };
}

#[derive(PartialEq)]
pub enum Syntax {
    Standard,
    LaTeX,
    Custom(String),
}

pub struct Parser<T: NumericType> {
    syntax: Syntax,
    syntax_rules: Ruleset<T>,
}

impl<T: NumericType<ExprType = T> + 'static> Parser<T> {
    pub fn new(syntax: Syntax) -> Result<Parser<T>, Error> {
        // get json file containing rule definitions
        let rule_file: &str = match syntax {
            // if user provides custom rules file
            Syntax::Custom(ref file) => file,
            // user chooses a built-in ruleset
            ref builtin => match ruleset::get_builtin_ruleset(&builtin) {
                Some(ruleset) => ruleset,
                None => {
                    return_error!(
                        ErrorType::InternalError,
                        "Syntax does not have rules registered"
                    );
                }
            },
        };
        // load and validate rules from file
        match Ruleset::load_ruleset(&rule_file) {
            Ok(syntax_rules) => Ok(Parser::<T> {
                syntax,
                syntax_rules,
            }),
            Err(message) => Err(message),
        }
    }

    pub fn parse(&self, equation_string: &str) -> Result<Equation<T>, Error> {
        /*
         * equations can have the following forms:
         * <expression>
         * <label> = <expression>
         * <label>(<variables>) = <expression>
         *
         * If no label is provided, the function is labelled 'f'.
         * If a comma separated list of variables is provided between brackets after the label, the parser will only accept explicitly named variables in the expression
         */
        if equation_string.is_empty() {
            syntax_error!("Equation string should not be empty");
        }
        let labeled_func = Regex::new(r"^(?:([^\(\)]+?)(?:\((.*)\))?\s*=\s*)?(.+)$").unwrap();
        let captures = match labeled_func.captures(equation_string) {
            Some(captures) => captures,
            None => {
                syntax_error!("Could not parse equation");
            }
        };

        // get label, if specified
        let mut equation = Equation::new(match captures.get(1) {
            Some(label) => label.as_str(),
            None => "f",
        });
        // get explicit variables, if present
        if captures.get(2).is_some() {
            // split list of variables
            let variables = captures.get(2).unwrap().as_str().split(",");
            for variable in variables.into_iter() {
                let trimmed_var = variable.trim();
                if !validate_label(trimmed_var) {
                    syntax_error!("Explicit variable '{}' is not valid", trimmed_var);
                }
                equation.add_variable(trimmed_var);
            }
        }
        // actual equation
        let expression_string = match captures.get(3) {
            Some(group) => group.as_str(),
            None => {
                syntax_error!("Could not parse equation");
            }
        };

        equation.set_data(self.parse_equation(expression_string)?);

        Ok(equation)
    }

    fn parse_equation(
        &self,
        equation_string: &str,
    ) -> Result<Vec<Box<dyn Expression<ExprType = T>>>, Error> {
        if equation_string.is_empty() {
            syntax_error!("Equation is empty");
        }

        let expressions = Vec::new();
        //let operand_stack = Vec::new();

        let mut remainder = equation_string.trim().to_string();
        let mut last_token: Option<Category> = None;
        while !remainder.is_empty() {
            let (rule, _, remaining_str) = self.match_next_token(&remainder, &last_token)?;
            remainder = remaining_str.trim().to_string();
            last_token = Some(rule.category());
        }

        Ok(expressions)
    }

    fn match_next_token(
        &self,
        equation_string: &str,
        last_token: &Option<Category>,
    ) -> Result<(Box<Rule<T>>, String, String), Error> {
        // find all rules that match the next token of the equation

        let mut invalid_rules = Vec::new();
        let mut valid_rules = Vec::new();
        // get all rules that match the given equation substring
        for rule in self.syntax_rules.as_slice() {
            if let Some((matched, other)) = rule.get_match(equation_string) {
                let context_valid = rule.can_follow(*last_token);
                if context_valid {
                    valid_rules.push((rule, matched, other.trim()));
                } else {
                    invalid_rules.push((rule, matched, other.trim()));
                }
            }
        }

        // Get number of rules that match given eq string and are valid in the context of last_token.
        // Additionally, filter out any implicit rules (rules that match zero characters) if any non-implicit rules are valid
        let has_valid_non_implicit_rules = valid_rules
            .iter()
            .filter(|(_, matched, _)| !matched.is_empty()).count() > 0;
        let mut matching_rules: Vec<&(&Box<Rule<T>>, &str, &str)> = valid_rules.iter().filter(|(_, matched, _)| !has_valid_non_implicit_rules || !matched.is_empty()).collect();

        if matching_rules.len() == 1 {
            // exactly one valid matching rule - can return straight away
            let rule = matching_rules[0];
            return Ok((rule.0.clone(), rule.1.to_string(), rule.2.to_string()));
        } else if matching_rules.is_empty() {
            // no valid rules - generate helpful error message
            let last_token_str = if last_token.is_none() {
                "start of equation"
            } else {
                &last_token.unwrap().to_string()
            };
            match invalid_rules.len() {
                // string doesn't match any rule regex
                0 => {
                    syntax_error!(
                        "No registered rules match start of expression '{}'",
                        equation_string
                    )
                }
                // one rule matches but context was not valid
                1 => {
                    let rule = invalid_rules[0];
                    syntax_error!(
                        "{} {} rule may not appear after {}",
                        if rule.1.is_empty() {
                            format!("Implicit")
                        } else {
                            format!("'{}'", rule.1)
                        },
                        rule.0.category(),
                        last_token_str
                    )
                }
                // multiple rules matched but none had valid context
                _ => {
                    syntax_error!(
                        "Multiple rules match start of '{}' but none may appear after {}",
                        equation_string,
                        last_token_str
                    )
                }
            }
        }

        // Multiple rules match the string and are valid after the last token:
        // sort rules descending by rule priority and then by number of characters matched
        matching_rules.sort_by(|a, b| match b.0.priority().cmp(&a.0.priority()) {
            std::cmp::Ordering::Equal => b.1.len().cmp(&a.1.len()),
            unequal => unequal,
        });

        // Of the shortlist ordered by priority, pick the first matched rule for which the next token is valid
        while let Some((matching_rule, matched_text, remaining_equation)) = matching_rules.pop()
        {
            for rule in self.syntax_rules.as_slice() {
                // check if next token matches this rule
                if !rule.matches(remaining_equation) {
                    continue;
                }
                // check if next token context is valid
                if !rule.can_follow(Some(matching_rule.category())) {
                    continue;
                }

                return Ok((
                    (*matching_rule).clone(),
                    matched_text.to_string(),
                    remaining_equation.to_string(),
                ));
            }
        }

        syntax_error!("'{}' does not match any registered rule", equation_string);
    }
}
