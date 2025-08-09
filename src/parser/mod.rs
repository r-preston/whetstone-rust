pub mod bindings;
pub mod ruleset;

use crate::equation::Equation;
use crate::expressions::Expression;
use crate::{
    error::{return_error, Error, ErrorType},
    NumericType,
};
use ruleset::Ruleset;

// TODO: remove
use crate::expressions::variable::validate_label;

use regex::Regex;

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
                        "Syntax does not have any built-in rules registered".to_string()
                    );
                }
            },
        };
        // generate map of function bindings
        /*
        let mut bindings = T::get_bindings();
        match custom_bindings {
            Some(existing) => bindings.extend(existing),
            None => (),
        };
        */
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
            return_error!(
                ErrorType::SyntaxError,
                "Equation string should not be empty".to_string()
            );
        }
        let labeled_func = Regex::new(r"^(?:([^\(\)]+?)(?:\((.*)\))?\s*=\s*)?(.+)$").unwrap();
        let captures = match labeled_func.captures(equation_string) {
            Some(captures) => captures,
            None => {
                return_error!(
                    ErrorType::SyntaxError,
                    "Could not parse equation".to_string()
                );
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
                    return_error!(
                        ErrorType::SyntaxError,
                        format!("Explicit variable '{}' is not valid", trimmed_var)
                    );
                }
                equation.add_variable(trimmed_var);
            }
        }
        // actual equation
        let expression_string = match captures.get(3) {
            Some(group) => group.as_str(),
            None => {
                return_error!(
                    ErrorType::SyntaxError,
                    "Could not parse equation".to_string()
                );
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
            return_error!(ErrorType::SyntaxError, "Equation is empty".to_string());
        }

        let expressions = Vec::new();

        Ok(expressions)
    }
}
