use whetstone::{Error, Parser};

extern crate whetstone;

fn main() -> Result<(), Error>
{
    env_logger::init();

    let factory = Parser::<f32>::new(whetstone::syntax::Syntax::Standard)?;
    let _ = factory.parse("sin x + cos x")?;
    log::info!("-----------------------------");
    let _ = factory.parse("sin(x) + cos(x)")?;

    Ok(())
}