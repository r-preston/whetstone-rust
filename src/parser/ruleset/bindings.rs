use std::collections::HashMap;

use crate::expressions::function::FunctionPointer;

pub use crate::expressions::function::Function;

pub type BindingMap<T> = HashMap<&'static str, Function<T>>;

pub trait BuiltinBindings {
    type ExprType;
    fn get_bindings() -> BindingMap<Self::ExprType> {
        HashMap::new()
    }
}

impl BuiltinBindings for f32 {
    type ExprType = f32;
    fn get_bindings() -> BindingMap<Self::ExprType> {
        let mut map: BindingMap<Self::ExprType> = HashMap::new();
        for def in F32_DEFINITIONS {
            map.insert(def.0, Function::new(def.1, def.2));
        }
        map
    }
}

impl BuiltinBindings for f64 {
    type ExprType = f64;
    fn get_bindings() -> BindingMap<Self::ExprType> {
        let mut map: BindingMap<Self::ExprType> = HashMap::new();
        for def in F64_DEFINITIONS {
            map.insert(def.0, Function::new(def.1, def.2));
        }
        map
    }
}

static F32_DEFINITIONS: &[(&str, FunctionPointer<f32>, usize)] = &[];
static F64_DEFINITIONS: &[(&str, FunctionPointer<f64>, usize)] = &[];
