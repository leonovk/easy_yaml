use std::path::Path;
use serde_yml::Value;
mod files;
mod serde;

pub struct EasyYaml<T: AsRef<Path>> {
    path: T
}

impl<T: AsRef<Path>> EasyYaml<T> {
    pub fn new(yaml_path: T) -> EasyYaml<T> {
        EasyYaml { path: yaml_path }
    }

    pub fn get(self, key: String) -> Option<Value> {
        let reader = files::yaml_data(self.path);
        let yaml_values = serde::value(reader);

        match yaml_values.get(key) {
            Some(v) => Some(v.clone()),
            None => None
        }
    }
}
