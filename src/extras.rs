use std::collections::HashMap;

use serde_json::Value;

pub trait Extras {
    fn to_extras(&self) -> Vec<(String, Value)>;
}

impl Extras for Value {
    fn to_extras(&self) -> Vec<(String, Value)> {
        if let Some(map) = self.as_object() {
            map.iter()
                .map(|(k, v)| (k.to_string(), v.clone()))
                .collect()
        } else {
            vec![("_".to_string(), self.clone())]
        }
    }
}

impl Extras for &[(String, Value)] {
    fn to_extras(&self) -> Vec<(String, Value)> {
        self.to_vec()
    }
}

impl Extras for HashMap<String, Value> {
    fn to_extras(&self) -> Vec<(String, Value)> {
        self.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
    }
}
