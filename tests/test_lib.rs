#[cfg(test)]
mod tests {
    use easy_yaml::EasyYaml;
    use serde_yml::Value::String;

    #[test]
    fn it_works() {
        let yaml = EasyYaml::new("tests/test.yaml");
        let result = yaml.get("key1".to_string());

        match result {
            Some(v) => match v {
                String(s) => assert_eq!("value1".to_string(), s),
                _ => panic!("+_+")
            },
            None => panic!("-_-")
        }
    }
}
