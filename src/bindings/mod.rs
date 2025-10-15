mod definitions;

use std::collections::HashMap;
use std::sync::{LazyLock, RwLock};

use crate::error::{return_error, Error, ErrorType};
use crate::expressions::function::Function;
pub use crate::expressions::function::FunctionPointer;
use crate::NumericType;

type BindingMap<T> = HashMap<&'static str, Function<T>>;

pub trait FunctionBindings {
    type ExprType: NumericType;

    fn get_bindings() -> BindingMap<Self::ExprType>;

    fn register_bindings(
        bindings: &'static [(&str, FunctionPointer<Self::ExprType>, usize)],
    ) -> Result<(), Error>;

    fn get_binding(label: &str) -> Option<Function<Self::ExprType>>;
}

pub fn register_bindings<T: NumericType + FunctionBindings<ExprType = T>>(
    bindings: &'static [(&'static str, FunctionPointer<T>, usize)],
) -> Result<(), Error> {
    <T as FunctionBindings>::register_bindings(bindings)
}

pub fn get_bindings<T: NumericType + FunctionBindings<ExprType = T>>() -> BindingMap<T> {
    <T as FunctionBindings>::get_bindings()
}

macro_rules! register_supported_type {
    ( $($Type:r#ident),+ ) => { $( paste::paste! {

        static [<$Type:upper _DEFINITIONS>]: &[(&'static str, FunctionPointer<$Type>, usize)] = &[

            ("Pi", definitions::pi, 0),
            ("Euler", definitions::euler, 0),
            ("Add", definitions::add, 2),
            ("Subtract", definitions::subtract, 2),
            ("Multiply", definitions::multiply, 2),
            ("Divide", definitions::divide, 2),
            ("Exponent", definitions::exponent, 2),
            ("Negate", definitions::negate, 1),
            ("SquareRoot", definitions::sqrt, 1),
            ("Sine", definitions::sine, 1),
            ("Cosine", definitions::cosine, 1),
            ("Tangent", definitions::tangent, 1),
            ("Arcsine", definitions::inverse_sine, 1),
            ("Arccosine", definitions::inverse_cosine, 1),
            ("Arctangent", definitions::inverse_tangent, 1),
            ("HypSine", definitions::hyperbolic_sine, 1),
            ("HypCosine", definitions::hyperbolic_cosine, 1),
            ("HypTangent", definitions::hyperbolic_tangent, 1),
            ("InvHypSine", definitions::inverse_hyperbolic_sine, 1),
            ("InvHypCosine", definitions::inverse_hyperbolic_cosine, 1),
            ("InvHypTangent", definitions::inverse_hyperbolic_tangent, 1),
            ("Cosecant", definitions::cosecant, 1),
            ("Secant", definitions::secant, 1),
            ("Cotangent", definitions::cotangent, 1),
            ("LogBaseE", definitions::log, 1),
            ("LogBase10", definitions::log10, 1),
            ("Modulus", definitions::abs, 1),

        ];

        static [<$Type:upper _BINDINGS>]: LazyLock<RwLock<BindingMap<$Type>>> = LazyLock::new(|| {
            RwLock::new(<$Type as FunctionBindings>::get_bindings())
        });

        impl FunctionBindings for $Type {
            type ExprType = $Type;

            fn get_bindings() -> BindingMap<Self::ExprType> {
                let mut map: BindingMap<Self::ExprType> = HashMap::with_capacity(
                    [<$Type:upper _DEFINITIONS>].len()
                );
                for (label, func, arg_count) in [<$Type:upper _DEFINITIONS>] {
                    match map.get(label) {
                        Some(_) => {
                            panic!(
                                "Built-in binding already registered for label '{}' and type {}",
                                label, std::any::type_name::<$Type>()
                            );
                        },
                        None => { map.insert(label, Function::new(label, func, *arg_count)); }
                    }
                }
                map
            }

            fn register_bindings(bindings: &'static[(&str, FunctionPointer<$Type>, usize)]) -> Result<(), Error> {
                let mut binding_map = [<$Type:upper _BINDINGS>].write().unwrap();
                for (label, func, arg_count) in bindings {
                    match binding_map.contains_key(label) {
                        false => { binding_map.insert(label, Function::new(label, func, *arg_count)); },
                        true => {
                            return_error!(
                                ErrorType::BindingError,
                                "Binding already registered for label '{}' and type {}",
                                label, std::any::type_name::<$Type>()
                            )
                        }
                    }
                }
                Ok(())
            }

            fn get_binding(label: &str) -> Option<Function<Self::ExprType>> {
                let bindings = [<$Type:upper _BINDINGS>].read().unwrap();
                let binding = bindings.get(label)?;
                Some(binding.clone())
            }
        }

        impl crate::NumericType for $Type {}
    } )+ };
}

register_supported_type!(f32);
register_supported_type!(f64);
