use crate::{
    equation::Value,
    error::{return_error, Error, ErrorType},
    expressions::{constant::Constant, function::Function, variable::Variable, Expression},
    NumericType,
};

use std::{fmt, marker::PhantomData};

/// The type of expression a Rule represents
#[derive(PartialEq, Clone, Eq, Hash)]
pub enum Category {
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

impl Category {
    fn string_representations() -> &'static [(Category, &'static str)] {
        &[
            (Self::Operator, "Operator"),
            (Self::Function, "Function"),
            (Self::Literal, "Literal"),
            (Self::Constant, "Constant"),
            (Self::Variable, "Variable"),
            (Self::OpenBracket, "OpenBracket"),
            (Self::CloseBracket, "CloseBracket"),
            (Self::Separator, "Separator"),
        ]
    }

    pub fn to_string(category: Category) -> Result<&'static str, Error> {
        match Self::string_representations()
            .iter()
            .find(|x| x.0 == category)
        {
            Some(val) => Ok(val.1),
            None => return_error!(
                ErrorType::InternalError,
                "No string respresentation available for enum value".to_string()
            ),
        }
    }

    pub fn from_string(string_val: &str) -> Result<Category, Error> {
        match Self::string_representations()
            .iter()
            .find(|x| x.1 == string_val)
        {
            Some(val) => Ok(val.0.clone()),
            None => return_error!(
                ErrorType::InternalError,
                format!("No match for string '{}'", string_val)
            ),
        }
    }
}

impl fmt::Display for Category {
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
#[derive(Copy, Clone)]
pub enum Associativity {
    LeftToRight,
    RightToLeft,
}

pub struct Rule<T: NumericType> {
    pattern: String,
    precedence: u32,
    category: Category,
    binding: Option<(Function<T>, Associativity)>,
    phantom: PhantomData<T>,
    follows: Vec<Category>,
    precedes: Vec<Category>,
}

impl<T: NumericType + std::str::FromStr + 'static> Rule<T> {
    pub fn new_non_expression_rule(
        pattern: String,
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

    pub fn new_function_rule(
        pattern: String,
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

    pub fn new_literal_rule(
        pattern: String,
        follows: Vec<Category>,
        precedes: Vec<Category>,
    ) -> Rule<T> {
        Rule {
            pattern,
            precedence: 0,
            category: Category::Literal,
            binding: None,
            follows,
            precedes,
            phantom: PhantomData::<T>,
        }
    }

    pub fn new_variable_rule(
        pattern: String,
        follows: Vec<Category>,
        precedes: Vec<Category>,
    ) -> Rule<T> {
        Rule {
            pattern,
            precedence: 0,
            category: Category::Variable,
            binding: None,
            follows,
            precedes,
            phantom: PhantomData::<T>,
        }
    }

    fn expression(&self, token: &str) -> Result<Box<dyn Expression<ExprType = T>>, Error> {
        match self.category {
            // Rules that produce an Expression of type Function
            Category::Operator | Category::Function | Category::Constant => match self.binding {
                Some(ref bind) => Ok(Box::new(bind.0.clone())),
                None => {
                    return_error!(ErrorType::InternalError, format!("Syntax rule '{}' is of functional type but has no function binding set {}", token, self.category));
                }
            },
            // Rules that produce an Expression of type Constant
            Category::Literal => match token.parse::<T>() {
                Ok(value) => Ok(Box::new(Constant::new(value))),
                Err(_) => {
                    return_error!(
                        ErrorType::SyntaxError,
                        format!("Could not parse literal '{}' as a number", token)
                    );
                }
            },
            // Rules that produce an Expression of type Variable
            Category::Variable => Ok(Box::new(Variable::new(
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
