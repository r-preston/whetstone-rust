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
    use whetstone::{syntax::Syntax, Error, Parser};

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

        let eq = factory
            .parse("sin x + cos(x) + tan(x)+asin(x)+(acos x+atan(x))+sec(x)+csc(x)+cot(x)")?;
        *eq.variable("x")? = 0.9;
        assert_near!(8.64759, eq.evaluate()?);

        let eq = factory.parse("sinh cosh tanh( arsinh(arcosh(artanh 0.9 )) )")?;
        assert_near!(1.58882938089, eq.evaluate()?);

        let eq = factory.parse("sqrt{sinewave} + ln sinewave / log [10.0^sinewave] - ln(e)")?;
        assert_eq!(1, eq.variables().len());
        *eq.variable("sinewave")? = 2.0;
        assert_near!(0.7607871527, eq.evaluate()?);

        let eq = factory.parse("mod(4, abs(-3)) + round(x) + floor(x) + ceil(x-0.5)")?;
        assert_eq!(1, eq.variables().len());
        *eq.variable("x")? = 1.7;
        assert_near!(6.0, eq.evaluate()?);

        let eq = factory.parse("2x+pi y")?;
        assert_eq!(2, eq.variables().len());
        *eq.variable("x")? = 2.0;
        *eq.variable("y")? = 1.0;
        assert_near!(7.14159265, eq.evaluate()?);

        Ok(())
    }

    #[test]
    fn test_latex_syntax() -> Result<(), Error> {
        let factory = Parser::<f64>::new(Syntax::LaTeX)?;

        assert_eq!(1.0, factory.parse("x+1")?.evaluate()?);

        let eq = factory.parse("x/x+x*x-x^x")?;
        assert_eq!(1, eq.variables().len());
        *eq.variable("x")? = 2.0;
        assert_eq!(1.0, eq.evaluate()?);

        let eq = factory.parse("x\\times x \\ast x \\cdot x \\cdotp x \\centerdot x")?;
        assert_eq!(1, eq.variables().len());
        *eq.variable("x")? = 2.0;
        assert_eq!(2.0f64.powf(6.0), eq.evaluate()?);

        let eq = factory.parse("1 \\div \\left( 1 \\over \\phi \\right)")?;
        assert_eq!(1, eq.variables().len());
        *eq.variable("\\phi")? = 10.0;
        assert_eq!(10.0, eq.evaluate()?);

        let eq = factory.parse("\\Bigg(\\bigg(\\Big(\\alpha \\mod \\beta\\Big) \\bmod \\gamma\\bigg) \\pmod 2.0\\Bigg)")?;
        assert_eq!(3, eq.variables().len());
        *eq.variable("\\alpha")? = 24.0;
        *eq.variable("\\beta")? = 13.0;
        *eq.variable("\\gamma")? = 6.0;
        assert_eq!(1.0, eq.evaluate()?);

        let eq = factory.parse("\\pi^(e+-2)")?;
        assert_eq!(0, eq.variables().len());
        assert_near!(2.275588444, eq.evaluate()?);

        let eq = factory.parse("   \\max\\{3x, 3y\\} + \\min{x,y} ")?;
        assert_eq!(2, eq.variables().len());
        *eq.variable("x")? = 2.0;
        *eq.variable("y")? = 3.0;
        assert_eq!(9.0 % 2.0, eq.evaluate()?);

        let eq = factory
            .parse("\\sin x + \\cos[x] + \\tan\\lang x\\rang+\\arcsin\\big( x\\big)+\\left(\\arccos\\big[x\\big]+\\arctan\\Big[x\\Big]\\right)+\\sec\\bigg[x\\bigg]+\\csc\\Bigg[x\\Bigg]+\\cot\\left[ x\\right]")?;
        *eq.variable("x")? = 0.9;
        assert_near!(8.64759, eq.evaluate()?);

        let eq = factory.parse("\\sinh \\cosh \\tanh\\left{ \\operatorname{arsinh}(\\operatorname*{arcosh}(\\operatorname{artanh} 0.9 )) \\right}")?;
        assert_near!(1.58882938089, eq.evaluate()?);

        let eq = factory.parse(
            "\\sqrt{ \\sinewave }\\, + \\log\\, \\sinewave / \\log_10 [10.0^\\sinewave] - \\ln(e)",
        )?;
        assert_eq!(1, eq.variables().len());
        *eq.variable("sinewave")? = 2.0;
        assert_near!(0.7607871527, eq.evaluate()?);

        let eq = factory.parse(
            "\\operatorname{round}(x) + \\operatorname{floor}(x) + \\operatorname{ceil}(x-0.5)",
        )?;
        assert_eq!(1, eq.variables().len());
        *eq.variable("x")? = 1.7;
        assert_near!(5.0, eq.evaluate()?);

        Ok(())
    }
}
