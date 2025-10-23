use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::equation::Equation;
use crate::syntax::ruleset::{rule::Rule, Ruleset};
use crate::syntax::{get_builtin_ruleset, Category, RuleCollectionDefinition, Syntax};
use crate::{
    error::{return_error, Error, ErrorType},
    expressions::{number::Number, variable::Variable, Expression},
    NumericType,
};

macro_rules! syntax_error {
    ($($t:tt)*) => {
        return_error!(ErrorType::SyntaxError, $($t)*)
    };
}

pub struct Parser<T: NumericType> {
    syntax_rules: Ruleset<T>,
}

impl<T: NumericType<ExprType = T>> Parser<T> {
    pub fn new(syntax: Syntax) -> Result<Parser<T>, Error> {
        let json = match get_builtin_ruleset(&syntax) {
            Some(json) => json,
            None => {
                return_error!(
                    ErrorType::InternalError,
                    "Syntax has no registered definitions"
                );
            }
        };
        Self::from_json(json)
    }

    pub fn from_json(json: &str) -> Result<Parser<T>, Error> {
        let rule_definitions = match serde_json::from_str::<RuleCollectionDefinition>(json) {
            Ok(deserialized) => deserialized,
            Err(e) => return_error!(
                ErrorType::RuleParseError,
                "JSON error in rule definition: {:?}",
                e
            ),
        };
        Self::from_definitions(rule_definitions)
    }

    pub fn from_definitions(definitions: RuleCollectionDefinition) -> Result<Parser<T>, Error> {
        // load and validate rules from definitions
        Ok(Parser::<T> {
            syntax_rules: Ruleset::create(definitions)?,
        })
    }

    pub fn parse(&self, equation_string: &str) -> Result<Equation<T>, Error> {
        if equation_string.is_empty() {
            syntax_error!("Equation string should not be empty");
        }

        let mut variables: HashMap<String, Rc<RefCell<T>>> = HashMap::new();
        let mut expressions: Vec<Box<dyn Expression<ExprType = T>>> = Vec::new();
        let mut operator_stack: Vec<(Box<Rule<T>>, Option<Box<dyn Expression<ExprType = T>>>)> =
            Vec::new();

        let mut remainder = equation_string.trim().to_string();
        let mut position = equation_string.rfind(&remainder).unwrap_or(0);
        let mut last_token: Option<Category> = None;
        let mut bracket_context = Vec::new();
        while !remainder.is_empty() {
            let (rule, matched_str, remaining_str) =
                self.match_next_token(&remainder, &last_token, position)?;
            remainder = remaining_str.trim().to_string();
            if remainder.is_empty() && !rule.allowed_at_end() {
                syntax_error!(
                    "{} may not appear at the end of an expression",
                    rule.category()
                )
            }
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
            let expression = self.create_expression(&rule, &matched_str, &mut variables)?;

            match rule.category() {
                Category::Fluff => {}
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
                Category::OpenBrackets => {
                    bracket_context.push(rule.bracket_context());
                    operator_stack.push((rule, expression))
                }
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
                    if rule.bracket_context()
                        != bracket_context
                            .pop()
                            .unwrap_or_else(|| rule.bracket_context() + 1)
                    {
                        syntax_error!(
                            "{} at position {} does not match last opening bracket",
                            matched_str,
                            position
                        );
                    }

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
                        None => syntax_error!("Closing bracket '{}' at position {} used without opening bracket first", matched_str, position),
                        Some((rule, _expression)) => {
                            if rule.category() != Category::OpenBrackets {
                                syntax_error!("Closing bracket '{}' at position {} used without opening bracket first", matched_str, position);
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
            position = equation_string.rfind(&remainder).unwrap_or(position);
        } /*
           *   // After the while loop, pop the remaining items from the operator stack into the output queue.
           *   while there are tokens on the operator stack:
           *       // If the operator token on the top of the stack is a parenthesis, then there are mismatched parentheses.
           *       {assert the operator on top of the stack is not a (left) parenthesis}
           *       pop the operator from the operator stack onto the output queue
           */
        while !operator_stack.is_empty() {
            let (rule, expression) = operator_stack.pop().unwrap();
            if rule.category() == Category::OpenBrackets {
                syntax_error!("Unclosed opening bracket")
            }
            if expression.is_some() {
                expressions.push(expression.unwrap());
            }
        }

        let equation = Equation::new(expressions, variables);

        // run through equation to check for any syntax errors that were not caught by the rules
        match equation.evaluate() {
            Ok(_) => Ok(equation),
            Err(e) => match e.error_type {
                ErrorType::SyntaxError => Err(e),
                _ => Ok(equation),
            },
        }
    }

    fn match_next_token(
        &self,
        equation_string: &str,
        last_token: &Option<Category>,
        position: usize,
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
                } else if rule.category() != Category::ImplicitOperators {
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
                        "No registered rules match start of expression '{}' at position {}",
                        equation_string,
                        position
                    )
                }
                // one rule matches but context was not valid
                1 => {
                    let rule = invalid_rules[0];
                    syntax_error!(
                        "{} {} rule may not appear after {} at position {}",
                        format!("'{}'", rule.1),
                        rule.0.category(),
                        last_token_str,
                        position
                    )
                }
                // multiple rules matched but none had valid context
                _ => {
                    syntax_error!(
                        "Multiple rules match start of '{}' at position {} but none may appear after {}",
                        equation_string,
                        position,
                        last_token_str
                    )
                }
            }
        }

        // Multiple rules match the string and are valid after the last token:
        // sort rules descending by rule priority and then by number of characters matched
        matching_rules.sort_by(|a, b| match a.0.priority().cmp(&b.0.priority()) {
            std::cmp::Ordering::Equal => a.1.len().cmp(&b.1.len()),
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

        syntax_error!(
            "Expression '{}' at position {} does not match any registered rule",
            equation_string,
            position
        );
    }

    fn create_expression(
        &self,
        rule: &Rule<T>,
        token: &str,
        variables: &mut HashMap<String, Rc<RefCell<T>>>,
    ) -> Result<Option<Box<dyn Expression<ExprType = T>>>, Error> {
        match rule.category() {
            // Rules that produce an Expression of type Function
            Category::ImplicitOperators | Category::Operators | Category::Functions => {
                match rule.binding() {
                    Some(ref bind) => Ok(Some(Box::new(bind.0.clone()))),
                    None => {
                        return_error!(ErrorType::InternalError, "Syntax rule '{}' is of functional type but has no function binding set {}", token, rule.category());
                    }
                }
            }
            Category::Constants => {
                match rule.binding() {
                    Some(ref bind) => Ok(Some(Box::new(Number::new((bind.0.function)(&[])?)))),
                    None => {
                        return_error!(ErrorType::InternalError, "Syntax rule '{}' is of functional type but has no function binding set {}", token, rule.category());
                    }
                }
            }
            // Rules that produce an Expression of type Number
            Category::Literals => match token.parse::<T>() {
                Ok(value) => Ok(Some(Box::new(Number::new(value)))),
                Err(_) => {
                    return_error!(
                        ErrorType::SyntaxError,
                        "Could not parse literal '{}' as a number",
                        token
                    );
                }
            },
            // Rules that produce an Expression of type Variable
            Category::Variables => {
                if !variables.contains_key(token) {
                    variables.insert(
                        token.to_string(),
                        Rc::new(RefCell::new(
                            <Variable<T> as Expression>::ExprType::from(0.0).unwrap(),
                        )),
                    );
                }

                Ok(Some(Box::new(Variable::new(
                    token,
                    variables.get(token).unwrap(),
                ))))
            }
            // Rules that do not correspond to an Expression
            Category::CloseBrackets
            | Category::OpenBrackets
            | Category::Separators
            | Category::Fluff => Ok(None),
        }
    }
}
