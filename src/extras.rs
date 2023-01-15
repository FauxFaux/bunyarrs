use std::collections::HashMap;
use std::iter;

use serde_json::Value;

type Pairs = Box<dyn Iterator<Item = (String, Value)>>;

pub trait Extras
where
    Self: Sized,
{
    fn into_extras(self) -> Pairs;
}

impl Extras for Value {
    fn into_extras(self) -> Pairs {
        match self {
            Value::Object(map) => Box::new(map.into_iter()),
            _ => Box::new(iter::once(("_".to_string(), self.clone()))),
        }
    }
}

impl Extras for &[(String, Value)] {
    fn into_extras(self) -> Pairs {
        Box::new(self.to_vec().into_iter())
    }
}

impl Extras for HashMap<String, Value> {
    fn into_extras(self) -> Pairs {
        Box::new(self.into_iter())
    }
}

impl Extras for () {
    fn into_extras(self) -> Pairs {
        Box::new(iter::empty())
    }
}

#[macro_export]
macro_rules! vars {
    ($($var:ident),+ $(,)?) => {
        serde_json::json!({
            $(stringify!($var): serde_json::json!($var)),+
        })
    };
}

#[macro_export]
macro_rules! vars_dbg {
    ($($var:ident),+ $(,)?) => {
        serde_json::json!({
            $(stringify!($var): format!("{:?}", $var)),+
        })
    };
}
