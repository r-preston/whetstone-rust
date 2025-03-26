#![allow(unused_variables)]

extern crate whetstone;

use whetstone::{EquationFactory, Syntax};

fn main() {
    let factory = EquationFactory::new(Syntax::Standard);
    let equation = factory.parse::<f32>("x").unwrap();
    let equation2 = factory.parse::<f32>("f(x) = x").unwrap();
    let equation3 = factory.parse::<f32>("f = x").unwrap();
    let equation4 = factory.parse::<f32>("function(x, y,z) = x").unwrap();
    println!("{}", equation.evaluate(&[("x", 0.0)]).unwrap());
}
