#![allow(unused_variables)]

extern crate whetstone;

use whetstone::{Parser, Syntax};

fn main() {
    let factory = Parser::<f32>::new(Syntax::Standard, None).unwrap();
    let equation = factory.parse("x-1.3").unwrap();
    let equation2 = factory.parse("f(x, y) = -x+y").unwrap();
    let equation3 = factory.parse("f = 2*x^2").unwrap();
    let equation4 = factory.parse("function(x, y,z) = 2x+3").unwrap();
    println!("{}", equation.evaluate(&[("x", 0.0)]).unwrap());
}
