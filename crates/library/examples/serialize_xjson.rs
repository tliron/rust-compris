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

    let mut reader = compris::read::Reader::new(yaml.as_bytes(), compris::Format::YAML);
    let content = reader.read().unwrap();

    // We attach a serialization mode (it does not change the actual data)
    let serialization_mode = compris::ser::SerializationMode::for_xjson();
    let content = content.with_serialization_mode(&serialization_mode);

    // For actual serialization, Format::XJSON is actually an alias for Format::JSON
    let mut serializer =
        compris::ser::Serializer::new_for_stdout().with_format(compris::Format::XJSON).with_pretty(true);
    serializer.write(&content).unwrap();
}
