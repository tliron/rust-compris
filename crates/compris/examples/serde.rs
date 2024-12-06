pub fn main() {
    let yaml = r#"
- !!bool True
- ~
- true
- 2
- 3
- key1: value1
  key2: [ 4, 5, 6 ]
"#;

    let mut reader = cpd::read::Reader::new(yaml.as_bytes(), cpd::Format::YAML);
    let content = reader.read().unwrap();

    let mut serializer = cpd::ser::Serializer::new_for_stdout().with_format(cpd::Format::JSON).with_pretty(true);
    serializer.write(&content).unwrap();
}
