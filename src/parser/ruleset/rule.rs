use crate::{
    equation::Value,
    error::{return_error, Error, ErrorType},
    expressions::{constant::Constant, function::Function, variable::Variable, Expression},
    NumericType,
};
use regex::Regex;
use serde::Deserialize;

use std::{fmt, marker::PhantomData};

/// The type of expression a Rule represents
#[derive(PartialEq, Clone, Eq, Hash, Deserialize)]
pub enum Category {
    /// an operation on two values, e.g. +, *, ^
    Operators,
    /// a function of 1 or more arguments, e.g. sin, ln
    Functions,
    /// a number such as 2, -0.5 etc
    Literals,
    /// mathematical constant such as pi or e
    Constants,
    /// placeholder for a value that can be changed for each evaluation
    Variables,
    /// opening parenthesis
    OpenBrackets,
    /// closing parenthesis
    CloseBrackets,
    /// tokens that are required by the syntax but have no direct affect, for example the separator between function arguments
    Separators,
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Operators => write!(f, "Operators"),
            Self::Functions => write!(f, "Functions"),
            Self::Literals => write!(f, "Literals"),
            Self::Constants => write!(f, "Constants"),
            Self::Variables => write!(f, "Variables"),
            Self::OpenBrackets => write!(f, "OpenBrackets"),
            Self::CloseBrackets => write!(f, "CloseBrackets"),
            Self::Separators => write!(f, "Separators"),
        }
    }
}

/// The order in which operations with equal precedence should be resolved
#[derive(Copy, Clone, Deserialize)]
pub(crate) enum Associativity {
    LeftToRight,
    RightToLeft,
}

pub struct Rule<T: NumericType> {
    pattern: Regex,
    precedence: u32,
    category: Category,
    binding: Option<(Function<T>, Associativity)>,
    phantom: PhantomData<T>,
    follows: Vec<Category>,
    precedes: Vec<Category>,
}

impl<T: NumericType + std::str::FromStr + 'static> Rule<T> {
    pub(crate) fn new_non_expression_rule(
        pattern: Regex,
        category: Category,
        follows: Vec<Category>,
        precedes: Vec<Category>,
    ) -> Rule<T> {
        Rule {
            pattern,
            precedence: 0,
            category,
            binding: None,
            follows,
            precedes,
            phantom: PhantomData::<T>,
        }
    }

    pub(crate) fn new_function_rule(
        pattern: Regex,
        precedence: u32,
        category: Category,
        associativity: Associativity,
        binding: fn(&[T]) -> Value<T>,
        follows: Vec<Category>,
        precedes: Vec<Category>,
        num_arguments: usize,
    ) -> Rule<T> {
        Rule {
            pattern,
            precedence,
            category,
            binding: (Some((Function::new(binding, num_arguments), associativity))),
            follows,
            precedes,
            phantom: PhantomData::<T>,
        }
    }

    pub(crate) fn new_literal_rule(
        pattern: Regex,
        follows: Vec<Category>,
        precedes: Vec<Category>,
    ) -> Rule<T> {
        Rule {
            pattern,
            precedence: 0,
            category: Category::Literals,
            binding: None,
            follows,
            precedes,
            phantom: PhantomData::<T>,
        }
    }

    pub(crate) fn new_variable_rule(
        pattern: Regex,
        follows: Vec<Category>,
        precedes: Vec<Category>,
    ) -> Rule<T> {
        Rule {
            pattern,
            precedence: 0,
            category: Category::Variables,
            binding: None,
            follows,
            precedes,
            phantom: PhantomData::<T>,
        }
    }

    fn allowed_at_start(&self) -> bool {
        match self.category {
            Category::Constants
            | Category::Functions
            | Category::Literals
            | Category::OpenBrackets
            | Category::Variables => true,
            Category::CloseBrackets | Category::Operators | Category::Separators => false,
        }
    }

    fn allowed_at_end(&self) -> bool {
        match self.category {
            Category::CloseBrackets
            | Category::Constants
            | Category::Literals
            | Category::Variables => true,
            Category::Functions
            | Category::OpenBrackets
            | Category::Operators
            | Category::Separators => false,
        }
    }

    fn expression(&self, token: &str) -> Result<Box<dyn Expression<ExprType = T>>, Error> {
        match self.category {
            // Rules that produce an Expression of type Function
            Category::Operators | Category::Functions | Category::Constants => match self.binding {
                Some(ref bind) => Ok(Box::new(bind.0.clone())),
                None => {
                    return_error!(ErrorType::InternalError, format!("Syntax rule '{}' is of functional type but has no function binding set {}", token, self.category));
                }
            },
            // Rules that produce an Expression of type Constant
            Category::Literals => match token.parse::<T>() {
                Ok(value) => Ok(Box::new(Constant::new(value))),
                Err(_) => {
                    return_error!(
                        ErrorType::SyntaxError,
                        format!("Could not parse literal '{}' as a number", token)
                    );
                }
            },
            // Rules that produce an Expression of type Variable
            Category::Variables => Ok(Box::new(Variable::new(
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
