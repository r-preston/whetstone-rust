#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

mod equation;

pub use equation::Equation;

pub enum Ruleset {
    Standard,
}

pub struct EquationFactory {
    ruleset: Ruleset,
}

impl EquationFactory {
    pub fn new(ruleset: Ruleset) -> EquationFactory {
        EquationFactory { ruleset }
    }

    pub fn equation(&self, equation_string: &str) -> Equation {
        Equation {}
    }
}
