use std::collections::HashMap;

use super::definitions as defs;
use crate::expressions::function::Function;
use crate::expressions::function::FunctionPointer;

pub type BindingMap<T> = HashMap<&'static str, Function<T>>;

pub trait BuiltinBindings {
    type ExprType;
    fn get_bindings() -> BindingMap<Self::ExprType>;
}

macro_rules! register_supported_type {
    ( $($Type:r#ident),+ ) => { $( paste::paste! {

        static [<$Type:upper _DEFINITIONS>]: &[(&str, FunctionPointer<$Type>, usize)] = &[
            
            ("Add", defs::add, 2),
            ("Subtract", defs::subtract, 2),
            ("Multiply", defs::multiply, 2),
            ("Divide", defs::divide, 2),
            ("Exponent", defs::exponent, 2),

        ];

        impl BuiltinBindings for $Type {
            type ExprType = $Type;
            fn get_bindings() -> BindingMap<Self::ExprType> {
                let mut map: BindingMap<Self::ExprType> = HashMap::with_capacity(
                    [<$Type:upper _DEFINITIONS>].len()
                );
                for def in [<$Type:upper _DEFINITIONS>] {
                    map.insert(def.0, Function::new(def.1, def.2));
                }
                map
            }
        }

        impl crate::NumericType for $Type {}

    } )+ };
}

register_supported_type!(f32);
register_supported_type!(f64);
