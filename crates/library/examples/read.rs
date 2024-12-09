// You need this for the trait functions
use compris::{read::StringReader, PrintDebugAnstream};

pub fn main() {
    let yaml = r#"
- [true, True, !!bool true]
- [null, Null, ~]
- [ -1, element, 1.5 ]
- key1: value1
  key2: value2
- {complex_key: complex_value}: 123456
"#;

    let value = compris::read::Reader::new_for_string(yaml, compris::Format::YAML).read().unwrap();
    value.print_debug();
}
