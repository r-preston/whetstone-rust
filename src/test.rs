#![allow(unused_variables)]

extern crate whetstone;

use whetstone::{bindings, NumericType, Parser, Syntax, Value};

pub fn custom_add<T: NumericType>(args: &[T]) -> Value<T> {
    Ok(args[0] + args[1])
}
static MY_FUNCTIONS: &[(&str, bindings::FunctionPointer<f32>, usize)] =
    &[("CustomAdd", custom_add, 2)];

fn main() {
    match bindings::register_bindings(MY_FUNCTIONS) {
        Ok(_) => (),
        Err(e) => panic!("{}", e.message),
    }

    let factory = Parser::<f32>::new(Syntax::Standard).unwrap();
    let equation = factory.parse("x-1.3").unwrap();
    let equation2 = factory.parse("f(x, y) = -x+y").unwrap();
    let equation3 = factory.parse("f = 2*x^2").unwrap();
    let equation4 = factory.parse("function(x, y,z) = 2x+3").unwrap();
    println!("{}", equation.evaluate(&[("x", 0.0)]).unwrap());
}
