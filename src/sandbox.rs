use whetstone::{Error, Parser};

extern crate whetstone;

fn main() -> Result<(), Error> {
    env_logger::init();

    let factory = Parser::<f32>::new(whetstone::syntax::Syntax::Standard)?;
    let _ = factory.parse("sqrt{sinewave} + ln sinewave / log10 [10.0^sinewave] - ln(e)")?;

    Ok(())
}
