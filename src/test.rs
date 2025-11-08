// bad syntax - brackets, tokens out of order, function with not enough args
// bindings - add and use new bindings
// create new syntax

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
    use whetstone::syntax::{self, Category, RuleDefinition};
    use whetstone::{bindings, bindings::FunctionPointer, NumericType, Value};
    use whetstone::{syntax::Syntax, Parser};

    #[test]
    fn test_standard_syntax() {
        let factory = Parser::<f32>::new(Syntax::Standard).unwrap();

        assert_eq!(1.0, factory.parse("x+1").unwrap().evaluate().unwrap());

        let eq = factory.parse("x/x+x*x-x^x").unwrap();
        assert_eq!(1, eq.variables().len());
        *eq.variable("x").unwrap() = 2.0;
        assert_eq!(1.0, eq.evaluate().unwrap());

        let eq = factory.parse("pi^(e+-2)").unwrap();
        assert_eq!(0, eq.variables().len());
        assert_near!(2.275588444, eq.evaluate().unwrap());

        let eq = factory.parse("   max(3*x, 3*y) %min(x,y) ").unwrap();
        assert_eq!(2, eq.variables().len());
        *eq.variable("x").unwrap() = 2.0;
        *eq.variable("y").unwrap() = 3.0;
        assert_eq!(9.0 % 2.0, eq.evaluate().unwrap());

        let eq = factory
            .parse("sin x + cos(x) + tan(x)+asin(x)+(acos x+atan(x))+sec(x)+csc(x)+cot(x)")
            .unwrap();
        *eq.variable("x").unwrap() = 0.9;
        assert_near!(8.64759, eq.evaluate().unwrap());

        let eq = factory
            .parse("sinh cosh tanh( arsinh(arcosh(artanh 0.9 )) )")
            .unwrap();
        assert_near!(1.58882938089, eq.evaluate().unwrap());

        let eq = factory
            .parse("sqrt{sinewave} + ln sinewave / log [10.0^sinewave] - ln(e)")
            .unwrap();
        assert_eq!(1, eq.variables().len());
        *eq.variable("sinewave").unwrap() = 2.0;
        assert_near!(0.7607871527, eq.evaluate().unwrap());

        let eq = factory
            .parse("mod(4, abs(-3)) + round(x) + floor(x) + ceil(x-0.5)")
            .unwrap();
        assert_eq!(1, eq.variables().len());
        *eq.variable("x").unwrap() = 1.7;
        assert_near!(6.0, eq.evaluate().unwrap());

        let eq = factory.parse("2x+pi y").unwrap();
        assert_eq!(2, eq.variables().len());
        *eq.variable("x").unwrap() = 2.0;
        *eq.variable("y").unwrap() = 1.0;
        assert_near!(7.14159265, eq.evaluate().unwrap());
    }

    #[test]
    fn test_latex_syntax() {
        let factory = Parser::<f64>::new(Syntax::LaTeX).unwrap();

        assert_eq!(1.0, factory.parse("x+1").unwrap().evaluate().unwrap());

        let eq = factory.parse("x/x+x*x-x^x").unwrap();
        assert_eq!(1, eq.variables().len());
        *eq.variable("x").unwrap() = 2.0;
        assert_eq!(1.0, eq.evaluate().unwrap());

        let eq = factory
            .parse("x\\times x \\ast x \\cdot x \\cdotp x \\centerdot x")
            .unwrap();
        assert_eq!(1, eq.variables().len());
        *eq.variable("x").unwrap() = 2.0;
        assert_eq!(2.0f64.powf(6.0), eq.evaluate().unwrap());

        let eq = factory
            .parse("1 \\div \\left( 1 \\over \\phi \\right)")
            .unwrap();
        assert_eq!(1, eq.variables().len());
        *eq.variable("\\phi").unwrap() = 10.0;
        assert_eq!(10.0, eq.evaluate().unwrap());

        let eq = factory.parse("\\Bigg(\\bigg(\\Big(\\alpha \\mod \\beta\\Big) \\bmod \\gamma\\bigg) \\pmod 2.0\\Bigg)").unwrap();
        assert_eq!(3, eq.variables().len());
        *eq.variable("\\alpha").unwrap() = 24.0;
        *eq.variable("\\beta").unwrap() = 13.0;
        *eq.variable("\\gamma").unwrap() = 6.0;
        assert_eq!(1.0, eq.evaluate().unwrap());

        let eq = factory.parse("\\pi^(e+-2)").unwrap();
        assert_eq!(0, eq.variables().len());
        assert_near!(2.275588444, eq.evaluate().unwrap());

        let eq = factory.parse("   \\max\\{3x, 3y\\} + \\min{x,y} ").unwrap();
        assert_eq!(2, eq.variables().len());
        *eq.variable("x").unwrap() = 2.0;
        *eq.variable("y").unwrap() = 3.0;
        assert_eq!(11.0, eq.evaluate().unwrap());

        let eq = factory
            .parse("\\sin x + \\cos[x] + \\tan\\lang x\\rang+\\arcsin\\big( x\\big)+\\left(\\arccos\\big[x\\big]+\\arctan\\Big[x\\Big]\\right)+\\sec\\bigg[x\\bigg]+\\csc\\Bigg[x\\Bigg]+\\cot\\left[ x\\right]").unwrap();
        *eq.variable("x").unwrap() = 0.9;
        assert_near!(8.64759, eq.evaluate().unwrap());

        let eq = factory.parse("\\sinh \\cosh \\tanh\\left{ \\operatorname{arsinh}(\\operatorname*{arcosh}(\\operatorname{artanh} 0.9 )) \\right}").unwrap();
        assert_near!(1.58882938089, eq.evaluate().unwrap());

        let eq = factory.parse(
            "\\sqrt{ \\sinewave }\\, + \\log\\, \\sinewave / \\log_10 [10.0^\\sinewave] - \\ln(e)",
        ).unwrap();
        assert_eq!(1, eq.variables().len());
        *eq.variable("\\sinewave").unwrap() = 2.0;
        assert_near!(0.7607871527, eq.evaluate().unwrap());

        let eq = factory
            .parse(
                "\\operatorname{round}(x) + \\operatorname{floor}(x) + \\operatorname{ceil}(x-0.5)",
            )
            .unwrap();
        assert_eq!(1, eq.variables().len());
        *eq.variable("x").unwrap() = 1.7;
        assert_near!(5.0, eq.evaluate().unwrap());
    }

    #[test]
    fn test_order_of_operations() {
        let factory = Parser::<f32>::new(Syntax::Standard).unwrap();

        assert_eq!(8.0, factory.parse("2*(2+2)").unwrap().evaluate().unwrap());
        assert_eq!(
            21.0,
            factory.parse("(1+2)*(3+4)").unwrap().evaluate().unwrap()
        );
        assert_eq!(6.0, factory.parse("16-5*2").unwrap().evaluate().unwrap());
        assert_eq!(1.0, factory.parse("10-3^2").unwrap().evaluate().unwrap());
        assert_eq!(23.0, factory.parse("40/2+3").unwrap().evaluate().unwrap());
        assert_eq!(21.0, factory.parse("(8-1)*3").unwrap().evaluate().unwrap());
        assert_eq!(24.0, factory.parse("16+4*2").unwrap().evaluate().unwrap());
        assert_eq!(16.0, factory.parse("10+3*2").unwrap().evaluate().unwrap());
        assert_eq!(7.0, factory.parse("8/2+12/4").unwrap().evaluate().unwrap());
        assert_eq!(5.0, factory.parse("3*10/5-1").unwrap().evaluate().unwrap());
        assert_eq!(8.0, factory.parse("6+6/3").unwrap().evaluate().unwrap());
        assert_eq!(20.0, factory.parse("8+3(5-1)").unwrap().evaluate().unwrap());
        assert_eq!(28.0, factory.parse("9*2+20/2").unwrap().evaluate().unwrap());
        assert_eq!(
            52.0,
            factory.parse("6*(7+3)-8").unwrap().evaluate().unwrap()
        );
        assert_eq!(
            42.0,
            factory.parse("(4+3)*(7-1)").unwrap().evaluate().unwrap()
        );
        assert_eq!(6.0, factory.parse("(7+9-4)/2").unwrap().evaluate().unwrap());
        assert_eq!(
            41.0,
            factory.parse("5+4*(2+7)").unwrap().evaluate().unwrap()
        );
        assert_eq!(49.0, factory.parse("(2+5)^2").unwrap().evaluate().unwrap());
        assert_eq!(0.0, factory.parse("10-2*5").unwrap().evaluate().unwrap());
        assert_eq!(
            1.0,
            factory
                .parse("sqrt(10+6)-(1+2)")
                .unwrap()
                .evaluate()
                .unwrap()
        );
        assert_eq!(
            3.0,
            factory.parse("sqrt 4 + 1").unwrap().evaluate().unwrap()
        );
    }

    #[test]
    fn test_bracket_matching() {
        let factory = Parser::<f32>::new(Syntax::Standard).unwrap();

        assert_eq!(2.0, factory.parse("(2)").unwrap().evaluate().unwrap());
        assert_eq!(6.0, factory.parse("(2)[3]").unwrap().evaluate().unwrap());

        factory
            .parse("(2")
            .expect_err("Mistmatched bracket not detected");
        factory
            .parse("2}")
            .expect_err("Mistmatched bracket not detected");
        factory
            .parse(")+2")
            .expect_err("Mistmatched bracket not detected");
        factory
            .parse("(2[)]")
            .expect_err("Invalid use of brackets not detected");
        factory
            .parse("{2(}2)")
            .expect_err("Invalid use of brackets not detected");
    }

    #[test]
    fn test_bad_syntax() {
        let factory = Parser::<f32>::new(Syntax::Standard).unwrap();

        factory
            .parse("")
            .expect_err("Should return error for empty equation");
        factory
            .parse("+")
            .expect_err("Should return error for invalid operator usage");
        factory
            .parse("2+-")
            .expect_err("Should return error for invalid operator usage");
        factory
            .parse("sin")
            .expect_err("Should return error for argumentless function");
        factory
            .parse("2 2 2")
            .expect_err("Should return error for meaningless drivel");
    }

    #[test]
    fn test_modify_syntax() {
        let mut modified_standard: syntax::RuleCollectionDefinition =
            syntax::get_definitions(Syntax::Standard).unwrap();
        let var_rule = &mut modified_standard
            .get_mut(&Category::Variables)
            .unwrap()
            .rules[0];
        var_rule.pattern = Some("x".to_string());
        var_rule.pattern_is_regex = Some(false);

        let factory = Parser::<f32>::from_definitions(modified_standard).unwrap();

        let eq = factory.parse("x+1").unwrap();
        assert_eq!(1.0, eq.evaluate().unwrap());

        factory
            .parse("y+1")
            .expect_err("y should not be considered a valid variable in our modified syntax");
    }

    #[test]
    fn test_new_syntax() {
        const JSON: &str = "{
                \"Operators\": {
                    \"may_follow\": [\"Literals\", \"Variables\"],
                    \"rules\": [
                        {
                            \"pattern\": \"+\",
                            \"binding\": \"Add\",
                            \"precedence\": 1
                        }
                    ]
                },
                \"Literals\": {
                    \"may_follow\": [\"Operators\"],
                    \"rules\": [
                        {
                            \"pattern\": \"[0-9]+\",
                            \"pattern_is_regex\": true
                        }
                    ]
                },
                \"Variables\": {
                    \"may_follow\": [\"Operators\"],
                    \"rules\": [
                        {
                            \"pattern\": \"x\"
                        }
                    ]
                }
            }";
        println!("{}", JSON);
        let factory = Parser::<f32>::from_json(JSON).unwrap();

        let eq = factory.parse("2 + x").unwrap();
        *eq.variable("x").unwrap() = 3.0;
        assert_eq!(5.0, eq.evaluate().unwrap());

        factory.parse("y").expect_err("y is not a valid variable");
        factory.parse("2*3").expect_err("* operator not defined");
        factory
            .parse("2.0")
            .expect_err("only integer literals defined");
    }

    pub fn custom_func<T: NumericType>(args: &[T]) -> Value<T> {
        Ok(args[0] + (T::from(1.0).unwrap() / args[0]))
    }
    static MY_FUNCTIONS: &[(&str, FunctionPointer<f32>, usize)] = &[("CustomFunc", custom_func, 1)];

    #[test]
    fn test_add_bindings() {
        // register bindings defined above with whetstone
        let _ = bindings::register_bindings(MY_FUNCTIONS).unwrap();

        // modify ruleset to add a function using this binding
        let mut modified_standard: syntax::RuleCollectionDefinition =
            syntax::get_definitions(Syntax::Standard).unwrap();
        let func_rules = &mut modified_standard.get_mut(&Category::Functions).unwrap();
        func_rules.rules.push(RuleDefinition {
            pattern: Some("self_plus_half_self".to_string()),
            pattern_is_regex: Some(false),
            precedence: Some(1),
            associativity: Some(syntax::Associativity::LeftToRight),
            binding: Some("CustomFunc".to_string()),
            may_follow: None,
            context: None,
        });

        // create parser using modified ruleset
        let factory = Parser::<f32>::from_definitions(modified_standard).unwrap();

        // parse equation using new binding
        let eq = factory.parse("self_plus_half_self(2)").unwrap();
        assert_eq!(2.5, eq.evaluate().unwrap());
    }
}
