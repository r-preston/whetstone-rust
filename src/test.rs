// parse functions with every binding in, check no errors
// combinations of brackets and operators, check values
// check latex as well
// bad syntax - brackets, tokens out of order, function with not enough args
// bindings - add and use new bindings
// create new syntax
// variable detection

extern crate whetstone;

macro_rules! assert_near {
    ($left:expr, $right:expr) => { 
        if ($left - $right).abs() > 0.00001 {
            panic!("{} and {} not similar", $left, $right);
        }
    };
}

#[cfg(test)]
mod tests {
    //use whetstone::{bindings, bindings::FunctionPointer, NumericType, Value};
    use whetstone::{syntax::Syntax, Parser, Error};

    /*
    pub fn custom_add<T: NumericType>(args: &[T]) -> Value<T> {
        Ok(args[0] + (T::from(1.0).unwrap() / args[1]))
    }
    static MY_FUNCTIONS: &[(&str, FunctionPointer<f32>, usize)] = &[("CustomAdd", custom_add, 2)];
    */

    #[test]
    fn test_standard_syntax() -> Result<(), Error> {
        let factory = Parser::<f32>::new(Syntax::Standard)?;

        assert_eq!(1.0, factory.parse("x+1")?.evaluate()?);

        let eq = factory.parse("x/x+x*x-x^x")?;
        assert_eq!(1, eq.variables().len());
        *eq.variable("x")? = 2.0;
        assert_eq!(1.0, eq.evaluate()?);

        let eq = factory.parse("pi^(e+-2)")?;
        assert_eq!(0, eq.variables().len());
        assert_near!(2.275588444, eq.evaluate()?);

        let eq = factory.parse("   max(3*x, 3*y) %min(x,y) ")?;
        assert_eq!(2, eq.variables().len());
        *eq.variable("x")? = 2.0;
        *eq.variable("y")? = 3.0;
        assert_eq!(9.0 % 2.0, eq.evaluate()?);

        let eq = factory.parse("sin x + cos(x) + tan(x)+asin(x)+(acos x+atan(x))+sec(x)+csc(x)+cot(x)")?;
        *eq.variable("x")? = 0.9;
        assert_eq!(8.64759, eq.evaluate()?);

        let eq = factory.parse("sinh cosh tanh( arsinh(arcosh(artanh 0.9 )) )")?;
        assert_near!(1.58882938089, eq.evaluate()?);

        Ok(())
    }
}
