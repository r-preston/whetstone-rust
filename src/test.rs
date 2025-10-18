#![allow(unused_variables)]

extern crate whetstone;

use whetstone::syntax::Syntax;
use whetstone::{bindings, bindings::FunctionPointer, NumericType, Parser, Value};

pub fn custom_add<T: NumericType>(args: &[T]) -> Value<T> {
    Ok(args[0] + args[1])
}
static MY_FUNCTIONS: &[(&str, FunctionPointer<f32>, usize)] = &[("CustomAdd", custom_add, 2)];

fn main() {
    env_logger::init();

    match bindings::register_bindings(MY_FUNCTIONS) {
        Ok(_) => (),
        Err(e) => panic!("{}", e.message),
    }
    let bindings = bindings::get_bindings::<f32>();
    //let func = bindings["cheese"].label;

    //syntax::copy_syntax
    //syntax::

    let factory = Parser::<f32>::new(Syntax::Standard).unwrap();
    let factory2 = Parser::<f32>::new(Syntax::LaTeX).unwrap();
    let a = factory2.parse("x\\,+\\,1").unwrap();
    println!("{}", a.evaluate(&[]).unwrap());
    //let equation = factory.parse("x-1.3").unwrap();
    //let equation2 = factory.parse("x+y").unwrap();
    //let equation3 = factory.parse("2*x^2").unwrap();
    let equation4 = factory.parse("2sin x+3").unwrap();
    let _ = equation4.evaluate(&[("x", 0.0)]);
}
