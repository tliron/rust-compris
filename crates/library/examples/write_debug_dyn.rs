// You need this for the trait functions!
use compris::WriteDebugDyn;

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

    // If your writer is a dyn of std::io::Write instead of a concrete implementation of it
    // then we still got you covered!
    let writer: &mut dyn std::io::Write = &mut anstream::stdout();
    content.write_debug_dyn(writer).unwrap();
}
