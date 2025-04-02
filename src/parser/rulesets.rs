use crate::Error;

#[derive(PartialEq)]
pub enum Syntax {
    Standard,
    LaTeX,
    Custom(String),
}

pub enum RuleCategory {
    LeftBracket,
    RightBracket,
    Separator,
    Operator,
    Function,
    Constant,
    Variable,
}

fn builtin_rulesets() -> &'static [(Syntax, &'static str)] {
    &[
        (Syntax::Standard, "syntax/standard.json"),
        (Syntax::LaTeX, "syntax/latex.json"),
    ]
}

pub fn get_builtin_ruleset(syntax: &Syntax) -> Option<&'static str> {
    let builtins = builtin_rulesets();
    match builtins.iter().position(|x| x.0 == *syntax) {
        Some(index) => Some(builtins[index].1),
        None => None,
    }
}

pub fn load_ruleset(ruleset: &str) -> Result<Ruleset, Error> {
    Ok(Ruleset { rules: Vec::new() })
}

pub struct Ruleset {
    rules: Vec<Rule>,
}

pub struct Rule {
    pub regex: String,
    pub priority: u32,
    pub category: RuleCategory,
    //pub function: Function<T>,
}
