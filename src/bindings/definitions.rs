use crate::{equation::Value, NumericType};

pub fn add<T: NumericType>(args: &[T]) -> Value<T> {
    Ok(args[0] + args[1])
}

pub fn subtract<T: NumericType>(args: &[T]) -> Value<T> {
    Ok(args[0] - args[1])
}

pub fn multiply<T: NumericType>(args: &[T]) -> Value<T> {
    Ok(args[0] * args[1])
}

pub fn divide<T: NumericType>(args: &[T]) -> Value<T> {
    Ok(args[0] / args[1])
}

pub fn exponent<T: NumericType>(args: &[T]) -> Value<T> {
    Ok(args[0].powf(args[1]))
}
