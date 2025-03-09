

pub struct Variable<T> {
    label: String,
    value: T,
}

impl<T> Variable<T> {
    pub fn new(label: &str, value: T) -> Variable<T> {
        Variable{label: label.to_string(), value}
    }
}