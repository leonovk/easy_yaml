#[cfg(test)]
mod tests {
    use easy_yaml::*;

    #[test]
    fn it_works() {
        let yaml = EasyYaml::new("tests/test.yaml");
        let result = yaml.get("key1");

        match result {
            Some(v) => match v {
                String(s) => assert_eq!("value1".to_string(), s),
                _ => panic!("+_+"),
            },
            None => panic!("-_-"),
        }
    }

    #[test]
    fn it_works_second() {
        let yaml = EasyYaml::new("tests/test.yaml");
        let result = yaml.get("key2.key3.key4");

        match result {
            Some(v) => match v {
                String(s) => assert_eq!("value2".to_string(), s),
                _ => panic!("Value: {:?}", v),
            },
            None => panic!("-_-"),
        }
    }
}
