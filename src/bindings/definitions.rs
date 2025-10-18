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
pub fn pi<T: NumericType>(_: &[T]) -> Value<T> {
    Ok(T::from(3.141592653589793238462643383280).unwrap())
}
pub fn euler<T: NumericType>(_: &[T]) -> Value<T> {
    Ok(T::from(2.718281828459045235360287471352).unwrap())
}
pub fn negate<T: NumericType>(args: &[T]) -> Value<T> {
    Ok(-args[0])
}
pub fn sqrt<T: NumericType>(args: &[T]) -> Value<T> {
    Ok(args[0].sqrt())
}
pub fn sine<T: NumericType>(args: &[T]) -> Value<T> {
    Ok(args[0].sin())
}
pub fn cosine<T: NumericType>(args: &[T]) -> Value<T> {
    Ok(args[0].cos())
}
pub fn tangent<T: NumericType>(args: &[T]) -> Value<T> {
    Ok(args[0].tan())
}
pub fn inverse_sine<T: NumericType>(args: &[T]) -> Value<T> {
    Ok(args[0].asin())
}
pub fn inverse_cosine<T: NumericType>(args: &[T]) -> Value<T> {
    Ok(args[0].acos())
}
pub fn inverse_tangent<T: NumericType>(args: &[T]) -> Value<T> {
    Ok(args[0].atan())
}
pub fn hyperbolic_sine<T: NumericType>(args: &[T]) -> Value<T> {
    Ok(args[0].sinh())
}
pub fn hyperbolic_cosine<T: NumericType>(args: &[T]) -> Value<T> {
    Ok(args[0].cosh())
}
pub fn hyperbolic_tangent<T: NumericType>(args: &[T]) -> Value<T> {
    Ok(args[0].tanh())
}
pub fn inverse_hyperbolic_sine<T: NumericType>(args: &[T]) -> Value<T> {
    Ok(args[0].asinh())
}
pub fn inverse_hyperbolic_cosine<T: NumericType>(args: &[T]) -> Value<T> {
    Ok(args[0].acosh())
}
pub fn inverse_hyperbolic_tangent<T: NumericType>(args: &[T]) -> Value<T> {
    Ok(args[0].atanh())
}
pub fn cosecant<T: NumericType>(args: &[T]) -> Value<T> {
    Ok(args[0].sin().powf(T::from(-1.0).unwrap()))
}
pub fn secant<T: NumericType>(args: &[T]) -> Value<T> {
    Ok(args[0].cos().powf(T::from(-1.0).unwrap()))
}
pub fn cotangent<T: NumericType>(args: &[T]) -> Value<T> {
    Ok(args[0].tan().powf(T::from(-1.0).unwrap()))
}
pub fn log<T: NumericType>(args: &[T]) -> Value<T> {
    Ok(args[0].ln())
}
pub fn log10<T: NumericType>(args: &[T]) -> Value<T> {
    Ok(args[0].log10())
}
pub fn abs<T: NumericType>(args: &[T]) -> Value<T> {
    Ok(args[0].abs())
}
pub fn modulo<T: NumericType>(args: &[T]) -> Value<T> {
    Ok(args[0] % args[1])
}
pub fn ceiling<T: NumericType>(args: &[T]) -> Value<T> {
    Ok(args[0].ceil())
}
pub fn floor<T: NumericType>(args: &[T]) -> Value<T> {
    Ok(args[0].floor())
}
pub fn round<T: NumericType>(args: &[T]) -> Value<T> {
    Ok(args[0].round())
}
pub fn min<T: NumericType>(args: &[T]) -> Value<T> {
    Ok(args[0].min(args[1]))
}
pub fn max<T: NumericType>(args: &[T]) -> Value<T> {
    Ok(args[0].max(args[1]))
}
