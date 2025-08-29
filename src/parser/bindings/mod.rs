mod definitions;

use std::collections::HashMap;
use std::sync::{LazyLock, RwLock};

use crate::error::{return_error, Error, ErrorType};
use crate::expressions::function::Function;
pub use crate::expressions::function::FunctionPointer;
use crate::NumericType;

type BindingMap<T> = HashMap<&'static str, (FunctionPointer<T>, usize)>;

pub trait FunctionBindings {
    type ExprType: NumericType;

    fn builtin_bindings() -> BindingMap<Self::ExprType>;

    fn register_bindings(
        bindings: &'static [(&str, FunctionPointer<Self::ExprType>, usize)],
    ) -> Result<(), Error>;

    fn get_binding(label: &str) -> Option<Function<Self::ExprType>>;
}

pub fn register_bindings<T: NumericType + FunctionBindings<ExprType = T>>(
    bindings: &'static [(&str, FunctionPointer<T>, usize)],
) -> Result<(), Error> {
    <T as FunctionBindings>::register_bindings(bindings)
}

macro_rules! register_supported_type {
    ( $($Type:r#ident),+ ) => { $( paste::paste! {

        static [<$Type:upper _DEFINITIONS>]: &[(&str, FunctionPointer<$Type>, usize)] = &[

            ("Add", definitions::add, 2),
            ("Subtract", definitions::subtract, 2),
            ("Multiply", definitions::multiply, 2),
            ("Divide", definitions::divide, 2),
            ("Exponent", definitions::exponent, 2),

        ];

        static [<$Type:upper _BINDINGS>]: LazyLock<RwLock<BindingMap<$Type>>> = LazyLock::new(|| {
            RwLock::new(<$Type as FunctionBindings>::builtin_bindings())
        });

        impl FunctionBindings for $Type {
            type ExprType = $Type;

            fn builtin_bindings() -> BindingMap<Self::ExprType> {
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
                        None => { map.insert(label, (*func, *arg_count)); }
                    }
                }
                map
            }

            fn register_bindings(bindings: &'static[(&str, FunctionPointer<$Type>, usize)]) -> Result<(), Error> {
                let mut binding_map = [<$Type:upper _BINDINGS>].write().unwrap();
                for (label, func, arg_count) in bindings {
                    match binding_map.contains_key(label) {
                        false => { binding_map.insert(label, (*func, *arg_count)); },
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
                Some(Function::new(binding.0, binding.1))
            }
        }

        impl crate::NumericType for $Type {}
    } )+ };
}

register_supported_type!(f32);
register_supported_type!(f64);
