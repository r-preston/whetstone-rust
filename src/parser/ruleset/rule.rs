use crate::{
    equation::Value,
    error::{return_error, Error, ErrorType},
    expressions::{constant::Constant, function::Function, variable::Variable, Expression},
    NumericType,
};

use std::fmt;

/// The type of expression a Rule represents
pub enum RuleCategory {
    /// an operation on two values, e.g. +, *, ^
    Operator,
    /// a function of 1 or more arguments, e.g. sin, ln
    Function,
    /// a number such as 2, -0.5 etc
    Literal,
    /// mathematical constant such as pi or e
    Constant,
    /// placeholder for a value that can be changed for each evaluation
    Variable,
    /// opening parenthesis
    OpenBracket,
    /// closing parenthesis
    CloseBracket,
    /// tokens that are required by the syntax but have no direct affect, for example the separator between function arguments
    Separator,
}

impl fmt::Display for RuleCategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Operator => write!(f, "Operator"),
            Self::Function => write!(f, "Function"),
            Self::Literal => write!(f, "Literal"),
            Self::Constant => write!(f, "Constant"),
            Self::Variable => write!(f, "Variable"),
            Self::OpenBracket => write!(f, "OpenBracket"),
            Self::CloseBracket => write!(f, "CloseBracket"),
            Self::Separator => write!(f, "Separator"),
        }
    }
}

/// The order in which operations with equal precedence should be resolved
pub enum Associativity {
    LeftToRight,
    RightToLeft,
}

pub struct Rule<T: NumericType> {
    pattern: String,
    precedence: u32,
    category: RuleCategory,
    binding: Option<(Function<T>, Associativity)>,
}

impl<T: NumericType + std::str::FromStr + 'static> Rule<T> {
    fn new_non_expression_rule(pattern: String, category: RuleCategory) -> Rule<T> {
        Rule {
            pattern,
            precedence: 0,
            category,
            binding: None,
        }
    }

    fn new_function_rule(
        pattern: String,
        precedence: u32,
        category: RuleCategory,
        associativity: Associativity,
        binding: fn(&[T]) -> Value<T>,
        num_arguments: usize,
    ) -> Rule<T> {
        Rule {
            pattern,
            precedence,
            category,
            binding: (Some((Function::new(binding, num_arguments), associativity))),
        }
    }

    fn new_literal_rule(pattern: String) -> Rule<T> {
        Rule {
            pattern,
            precedence: 0,
            category: RuleCategory::Literal,
            binding: None,
        }
    }

    fn new_variable_rule(pattern: String) -> Rule<T> {
        Rule {
            pattern,
            precedence: 0,
            category: RuleCategory::Variable,
            binding: None,
        }
    }

    fn expression(&self, token: &str) -> Result<Box<dyn Expression<ExprType = T>>, Error> {
        match self.category {
            // Rules that produce an Expression of type Function
            RuleCategory::Operator | RuleCategory::Function | RuleCategory::Constant => {
                match self.binding {
                    Some(ref bind) => Ok(Box::new(bind.0.clone())),
                    None => {
                        return_error!(ErrorType::InternalError, format!("Syntax rule '{}' is of functional type but has no function binding set {}", token, self.category));
                    }
                }
            }
            // Rules that produce an Expression of type Constant
            RuleCategory::Literal => match token.parse::<T>() {
                Ok(value) => Ok(Box::new(Constant::new(value))),
                Err(error) => {
                    return_error!(
                        ErrorType::ParseError,
                        format!("Could not parse literal '{}' as a number", token)
                    );
                }
            },
            // Rules that produce an Expression of type Variable
            RuleCategory::Variable => Ok(Box::new(Variable::new(
                token,
                <Variable<T> as Expression>::ExprType::from(0.0).unwrap(),
            ))),
            // Rules that do not correspond to an Expression
            _ => {
                return_error!(ErrorType::InternalError, format!("Attempted to get expression for syntax rule '{}' with expressionless category {}", token, self.category));
            }
        }
    }
}
