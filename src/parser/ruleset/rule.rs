use crate::{
    equation::Value,
    error::{return_error, Error, ErrorType},
    expressions::{function::Function, number::Number, variable::Variable, Expression},
    NumericType,
};
use regex::{Captures, Regex};
use serde::Deserialize;

use std::{fmt, marker::PhantomData};

/// The type of expression a Rule represents
#[derive(PartialEq, Copy, Clone, Eq, Hash, Deserialize)]
pub(crate) enum Category {
    /// an operation on two values, e.g. +, *, ^
    Operators,
    /// an operation on two values, e.g. +, *, ^ that matches no characters and is implied by context
    ImplicitOperators,
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
            Self::ImplicitOperators => write!(f, "Implicit Operators"),
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
#[derive(Copy, Clone, Deserialize, PartialEq)]
pub(crate) enum Associativity {
    LeftToRight,
    RightToLeft,
}

#[derive(Clone)]
pub(crate) struct Rule<T: NumericType> {
    pattern: Regex,
    precedence: u32,
    category: Category,
    binding: Option<(Function<T>, Associativity)>,
    phantom: PhantomData<T>,
    follows: Vec<Category>,
}

impl<T: NumericType + std::str::FromStr + 'static> Rule<T> {
    pub fn new_non_expression_rule(
        pattern: Regex,
        category: Category,
        follows: Vec<Category>,
    ) -> Rule<T> {
        Rule {
            pattern,
            precedence: 0,
            category,
            binding: None,
            follows,
            phantom: PhantomData::<T>,
        }
    }

    pub fn new_function_rule(
        pattern: Regex,
        precedence: u32,
        category: Category,
        associativity: Associativity,
        binding: fn(&[T]) -> Value<T>,
        follows: Vec<Category>,
        num_arguments: usize,
    ) -> Rule<T> {
        Rule {
            pattern,
            precedence,
            category,
            binding: (Some((Function::new(binding, num_arguments), associativity))),
            follows,
            phantom: PhantomData::<T>,
        }
    }

    pub fn new_literal_rule(pattern: Regex, follows: Vec<Category>) -> Rule<T> {
        Rule {
            pattern,
            precedence: 0,
            category: Category::Literals,
            binding: None,
            follows,
            phantom: PhantomData::<T>,
        }
    }

    pub fn new_variable_rule(pattern: Regex, follows: Vec<Category>) -> Rule<T> {
        Rule {
            pattern,
            precedence: 0,
            category: Category::Variables,
            binding: None,
            follows,
            phantom: PhantomData::<T>,
        }
    }

    pub fn allowed_at_start(&self) -> bool {
        match self.category {
            Category::Constants
            | Category::Functions
            | Category::Literals
            | Category::OpenBrackets
            | Category::Variables => true,
            Category::CloseBrackets
            | Category::Operators
            | Category::ImplicitOperators
            | Category::Separators => false,
        }
    }

    pub fn allowed_at_end(&self) -> bool {
        match self.category {
            Category::CloseBrackets
            | Category::Constants
            | Category::Literals
            | Category::Variables => true,
            Category::Functions
            | Category::OpenBrackets
            | Category::Operators
            | Category::ImplicitOperators
            | Category::Separators => false,
        }
    }

    pub fn can_follow(&self, token: Option<Category>) -> bool {
        match token {
            Some(category) => self.follows.contains(&category),
            None => self.allowed_at_start(),
        }
    }

    pub fn category(&self) -> Category {
        self.category
    }

    pub fn priority(&self) -> u32 {
        match self.category {
            Category::OpenBrackets | Category::CloseBrackets | Category::Separators => 5,
            Category::Operators => 4,
            Category::Functions | Category::Constants => 3,
            Category::Literals => 2,
            Category::Variables => 1,
            Category::ImplicitOperators => 0,
        }
    }

    pub fn precedence(&self) -> u32 {
        self.precedence
    }

    pub fn left_associative(&self) -> bool {
        match &self.binding {
            Some((_, associativity)) => *associativity == Associativity::LeftToRight,
            None => false,
        }
    }

    pub fn matches(&self, eq_str: &str) -> bool {
        match eq_str.is_empty() {
            true => self.allowed_at_end(),
            false => self.pattern.find(eq_str).is_some(),
        }
    }

    pub fn get_match<'a>(&self, eq_str: &'a str) -> Option<(&'a str, &'a str)> {
        let res: Captures<'a> = self.pattern.captures(eq_str)?;
        Some((res.get(1)?.into(), res.get(2)?.into()))
    }

    pub fn expression(
        &self,
        token: &str,
    ) -> Result<Option<Box<dyn Expression<ExprType = T>>>, Error> {
        match self.category {
            // Rules that produce an Expression of type Function
            Category::Operators | Category::Functions => {
                match self.binding {
                    Some(ref bind) => Ok(Some(Box::new(bind.0.clone()))),
                    None => {
                        return_error!(ErrorType::InternalError, "Syntax rule '{}' is of functional type but has no function binding set {}", token, self.category);
                    }
                }
            }
            Category::Constants => {
                match self.binding {
                    Some(ref bind) => Ok(Some(Box::new(Number::new((bind.0.function)(&[])?)))),
                    None => {
                        return_error!(ErrorType::InternalError, "Syntax rule '{}' is of functional type but has no function binding set {}", token, self.category);
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
            Category::Variables => Ok(Some(Box::new(Variable::new(
                token,
                <Variable<T> as Expression>::ExprType::from(0.0).unwrap(),
            )))),
            // Rules that do not correspond to an Expression
            _ => {
                Ok(None)
                //return_error!(ErrorType::InternalError, "Attempted to get expression for syntax rule '{}' with expressionless category {}", token, self.category);
            }
        }
    }
}
