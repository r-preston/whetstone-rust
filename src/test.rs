extern crate whetstone;

use whetstone::{EquationFactory, Syntax};

fn main() {
    let factory = EquationFactory::new(Syntax::Standard);
    let equation = factory.parse::<f32>("x").unwrap();
    println!("{}", equation.evaluate(&[("x", 0.0)]).unwrap());
}
