use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Value {
    Null,
    Number(f64),
    String(String),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
}
