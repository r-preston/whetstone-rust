#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

extern crate whetstone;

use whetstone::{Equation, EquationFactory, Ruleset};

fn main() {
    let factory = EquationFactory::new(Ruleset::Standard);
    let equation = factory.equation("x");
    println!("{}", equation.evaluate(0.0));
}
