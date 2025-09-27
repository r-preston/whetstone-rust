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
        if equation_string.is_empty() {
            syntax_error!("Equation string should not be empty");
        }

        let mut expressions: Vec<Box<dyn Expression<ExprType = T>>> = Vec::new();
        let mut operator_stack: Vec<(Box<Rule<T>>, Option<Box<dyn Expression<ExprType = T>>>)> =
            Vec::new();

        let mut remainder = equation_string.trim().to_string();
        let mut last_token: Option<Category> = None;
        while !remainder.is_empty() {
            let (rule, matched_str, remaining_str) =
                self.match_next_token(&remainder, &last_token)?;
            remainder = remaining_str.trim().to_string();
            last_token = Some(rule.category());
            /*
             * if the token is:
             *   - a number:
             *        put it into the output queue
             *   - a function:
             *        push it onto the operator stack
             *   - an operator o1:
             *        while (
             *          there is an operator o2 at the top of the operator stack which is not a left parenthesis,
             *          and (o2 has greater precedence than o1 or (o1 and o2 have the same precedence and o1 is left-associative))
             *        ):
             *            pop o2 from the operator stack into the output queue
             *        push o1 onto the operator stack
             *   - a ",":
             *        while the operator at the top of the operator stack is not a left parenthesis:
             *            pop the operator from the operator stack into the output queue
             *   - a left parenthesis (i.e. "("):
             *        push it onto the operator stack
             *   - a right parenthesis (i.e. ")"):
             *        while the operator at the top of the operator stack is not a left parenthesis:
             *            {assert the operator stack is not empty}
             *            // If the stack runs out without finding a left parenthesis, then there are mismatched parentheses.
             *            pop the operator from the operator stack into the output queue
             *        {assert there is a left parenthesis at the top of the operator stack}
             *        pop the left parenthesis from the operator stack and discard it
             *        if there is a function token at the top of the operator stack, then:
             *            pop the function from the operator stack into the output queue
             */
            let expression = rule.expression(&matched_str)?;
            match rule.category() {
                Category::Literals | Category::Constants | Category::Variables => {
                    expressions.push(expression.unwrap())
                }
                Category::Functions => operator_stack.push((rule, expression)),
                Category::Operators | Category::ImplicitOperators => {
                    while let Some((top_of_stack_rule, _)) = operator_stack.last() {
                        if Category::OpenBrackets == top_of_stack_rule.category()
                            || !(top_of_stack_rule.precedence() > rule.precedence()
                                || (top_of_stack_rule.precedence() == rule.precedence()
                                    && rule.left_associative()))
                        {
                            break;
                        }
                        match operator_stack.pop() {
                            Some((_, top_of_stack_expression)) => {
                                if let Some(expr) = top_of_stack_expression {
                                    expressions.push(expr)
                                }
                            }
                            _ => (),
                        }
                    }
                    operator_stack.push((rule, expression))
                }
                Category::Separators => {
                    while let Some((top_of_stack_rule, _)) = operator_stack.last() {
                        if Category::OpenBrackets == top_of_stack_rule.category() {
                            break;
                        }
                        match operator_stack.pop() {
                            Some((_, top_of_stack_expression)) => {
                                if let Some(expr) = top_of_stack_expression {
                                    expressions.push(expr)
                                }
                            }
                            _ => (),
                        }
                    }
                }
                Category::OpenBrackets => operator_stack.push((rule, expression)),
                Category::CloseBrackets => {
                    /*
                     *  while the operator at the top of the operator stack is not a left parenthesis:
                     *      {assert the operator stack is not empty}
                     *      // If the stack runs out without finding a left parenthesis, then there are mismatched parentheses.
                     *      pop the operator from the operator stack into the output queue
                     *  {assert there is a left parenthesis at the top of the operator stack}
                     *  pop the left parenthesis from the operator stack and discard it
                     *  if there is a function token at the top of the operator stack, then:
                     *      pop the function from the operator stack into the output queue}
                     */
                    while operator_stack
                        .last()
                        .is_none_or(|val| val.0.category() != Category::OpenBrackets)
                    {
                        let (_rule, expression) = match operator_stack.pop() {
                            Some(x) => x,
                            None => syntax_error!("Invalid closing bracket"),
                        };
                        if let Some(expr) = expression {
                            expressions.push(expr);
                        }
                    }
                    match operator_stack.pop() {
                        None => syntax_error!("Mismatched bracket"),
                        Some((rule, _expression)) => {
                            if rule.category() != Category::OpenBrackets {
                                syntax_error!("Mismatched bracket")
                            }
                        }
                    }
                    if operator_stack
                        .last()
                        .is_some_and(|(rule, _)| rule.category() == Category::Functions)
                    {
                        if let Some(op) = operator_stack.pop().unwrap().1 {
                            expressions.push(op);
                        }
                    }
                }
            }
            /*
             *   // After the while loop, pop the remaining items from the operator stack into the output queue.
             *   while there are tokens on the operator stack:
             *       // If the operator token on the top of the stack is a parenthesis, then there are mismatched parentheses.
             *       {assert the operator on top of the stack is not a (left) parenthesis}
             *       pop the operator from the operator stack onto the output queue
             */
        }

        Ok(Equation::new(expressions))
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
            .filter(|(_, matched, _)| !matched.is_empty())
            .count()
            > 0;
        let mut matching_rules: Vec<&(&Box<Rule<T>>, &str, &str)> = valid_rules
            .iter()
            .filter(|(_, matched, _)| !has_valid_non_implicit_rules || !matched.is_empty())
            .collect();

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
        while let Some((matching_rule, matched_text, remaining_equation)) = matching_rules.pop() {
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
