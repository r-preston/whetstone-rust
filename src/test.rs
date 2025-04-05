#![allow(unused_variables)]

extern crate whetstone;

use whetstone::{Parser, Syntax};

fn main() {
    let factory = Parser::<f32>::new(Syntax::Standard, None).unwrap();
    let equation = factory.parse("x").unwrap();
    let equation2 = factory.parse("f(x) = x").unwrap();
    let equation3 = factory.parse("f = x").unwrap();
    let equation4 = factory.parse("function(x, y,z) = x").unwrap();
    println!("{}", equation.evaluate(&[("x", 0.0)]).unwrap());
}
