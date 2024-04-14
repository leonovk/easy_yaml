# Easy Yaml

Easy work with Yaml files

## Dependency

```bash
cargo add easy_yaml
```

## Usage

```rust
use easy_yaml::*;

let yaml = EasyYaml::new("tests/test.yaml");
let result = yaml.get("key2.key3.key4");

match result {
    Some(v) => match v {
        String(s) => assert_eq!("value2".to_string(), s),
        _ => panic!("Value: {:?}", v),
    },
    None => panic!("-_-"),
}
```
