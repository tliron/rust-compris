pub fn main() {
    let yaml = r#"
- !!bool True
- ~
- true
- 2
- 3.0
- key1: value1
  key2: [ 4, 5, 6 ]
"#;

    let mut reader = compris::read::Reader::new(yaml.as_bytes(), compris::Format::YAML);
    let content = reader.read().unwrap();

    println!("{}", content);
}
