pub(crate) mod ruleset;

use serde::Deserialize;
use std::{collections::HashMap, fmt};

#[derive(PartialEq, Clone)]
pub enum Syntax {
    Standard,
    LaTeX,
}

/// The type of expression a Rule represents
#[derive(PartialEq, Copy, Clone, Eq, Hash, Deserialize)]
pub enum Category {
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
    /// tokens with no syntactic meaning that can be ignored
    Fluff,
}

/// The order in which operations with equal precedence should be resolved
#[derive(Copy, Clone, Deserialize, PartialEq)]
pub enum Associativity {
    LeftToRight,
    RightToLeft,
}

#[derive(Deserialize)]
pub struct RuleDefinition {
    pub pattern: Option<String>,
    pub pattern_is_regex: Option<bool>,
    pub precedence: Option<u32>,
    pub associativity: Option<Associativity>,
    pub binding: Option<String>,
    pub may_follow: Option<Vec<Category>>,
    pub context: Option<i32>,
}

#[derive(Deserialize)]
pub struct RuleCategoryDefinition {
    pub default_associativity: Option<Associativity>,
    pub default_precedence: Option<u32>,
    pub may_follow: Vec<Category>,
    pub rules: Vec<RuleDefinition>,
}

#[derive(Deserialize)]
pub struct RuleCollectionDefinition(
    #[serde(with = "::serde_with::rust::maps_duplicate_key_is_error")]
    pub  HashMap<Category, RuleCategoryDefinition>,
);

fn builtin_rulesets() -> &'static [(Syntax, &'static str)] {
    &[
        (Syntax::Standard, include_str!("json/standard.json")),
        (Syntax::LaTeX, include_str!("json/latex.json")),
    ]
}

pub(crate) fn get_builtin_ruleset(syntax: &Syntax) -> Option<&'static str> {
    let builtins = builtin_rulesets();
    match builtins.iter().position(|x| x.0 == *syntax) {
        Some(index) => Some(builtins[index].1),
        None => None,
    }
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
            Self::OpenBrackets => write!(f, "Opening Brackets"),
            Self::CloseBrackets => write!(f, "Closing Brackets"),
            Self::Separators => write!(f, "Separators"),
            Self::Fluff => write!(f, "Fluff"),
        }
    }
}
