extern crate whetstone;

use whetstone::{EquationFactory, Syntax, Variable};

fn main() {
    let factory = EquationFactory::new(Syntax::Standard);
    let equation = factory.parse::<f32>("x").unwrap();
    println!("{}", equation.evaluate(&[Variable::new("x", 0.0)]).unwrap());
}
