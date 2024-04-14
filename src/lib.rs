use serde_yml::Value;
pub use serde_yml::Value::*;
use std::path::Path;
mod files;
mod serde;

pub struct EasyYaml<T: AsRef<Path>> {
    path: T,
}

impl<T: AsRef<Path>> EasyYaml<T> {
    pub fn new(yaml_path: T) -> EasyYaml<T> {
        EasyYaml { path: yaml_path }
    }

    pub fn get(self, key: &str) -> Option<Value> {
        let reader = files::yaml_data(self.path);
        let yaml_values = serde::value(reader);
        let binding = key.to_string();
        let keys: Vec<&str> = binding.split('.').collect();
        get_value(keys, yaml_values)
    }
}

fn get_value(keys: Vec<&str>, yaml_values: Value) -> Option<Value> {
    if keys.len() == 1 {
        return yaml_values.get(keys[0].to_string()).clone().cloned();
    } else {
        let mut i: usize = 1;
        let max_i = keys.len() - 1;
        let mut yv = yaml_values.get(keys[0].to_string()).unwrap();

        while i <= max_i {
            yv = yv.get(keys[i].to_string()).unwrap();
            i += 1;
        }

        Some(yv.clone())
    }
}
