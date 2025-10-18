use crate::syntax::{Associativity, Category};
use crate::{expressions::function::Function, NumericType};
use regex::{Captures, Regex};

#[derive(Clone)]
pub(crate) struct Rule<T: NumericType> {
    pattern: Regex,
    precedence: u32,
    category: Category,
    binding: Option<(Function<T>, Associativity)>,
    follows: Vec<Category>,
    context: i32,
}

impl<T: NumericType> Rule<T> {
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
            context: 0,
        }
    }

    pub fn new_bracket_rule(
        pattern: Regex,
        category: Category,
        follows: Vec<Category>,
        pair_context: i32,
    ) -> Rule<T> {
        Rule {
            pattern,
            precedence: 0,
            category,
            binding: None,
            follows,
            context: pair_context,
        }
    }

    pub fn new_function_rule(
        pattern: Regex,
        precedence: u32,
        category: Category,
        associativity: Associativity,
        binding: Function<T>,
        follows: Vec<Category>,
    ) -> Rule<T> {
        Rule {
            pattern,
            precedence,
            category,
            binding: (Some((binding, associativity))),
            follows,
            context: 0,
        }
    }

    pub fn new_literal_rule(pattern: Regex, follows: Vec<Category>) -> Rule<T> {
        Rule {
            pattern,
            precedence: 0,
            category: Category::Literals,
            binding: None,
            follows,
            context: 0,
        }
    }

    pub fn new_variable_rule(pattern: Regex, follows: Vec<Category>) -> Rule<T> {
        Rule {
            pattern,
            precedence: 0,
            category: Category::Variables,
            binding: None,
            follows,
            context: 0,
        }
    }

    pub fn allowed_at_start(&self) -> bool {
        match self.category {
            Category::Constants
            | Category::Functions
            | Category::Literals
            | Category::OpenBrackets
            | Category::Fluff
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
            | Category::Fluff
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
            Some(Category::Fluff) => true,
            Some(category) => self.follows.contains(&category),
            None => self.allowed_at_start(),
        }
    }

    pub fn category(&self) -> Category {
        self.category
    }

    pub fn binding(&self) -> &Option<(Function<T>, Associativity)> {
        &self.binding
    }

    pub fn bracket_context(&self) -> i32 {
        self.context
    }

    pub fn priority(&self) -> u32 {
        match self.category {
            Category::OpenBrackets | Category::CloseBrackets | Category::Separators => 5,
            Category::Operators => 4,
            Category::Functions | Category::Constants => 3,
            Category::Literals => 2,
            Category::Variables => 1,
            Category::ImplicitOperators => 0,
            Category::Fluff => 0,
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
}
