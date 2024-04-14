use serde_yml::Value;
use std::fs::File;
use std::io::BufReader;

pub fn value(reader: BufReader<File>) -> Value {
  serde_yml::from_reader(reader).expect("Failed to parse YAML")
}
