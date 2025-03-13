use crate::{NumericType, Value};

pub fn add<T: NumericType>(args: &mut [T]) -> Value<T> {
    Ok(args[0] + args[1])
}
