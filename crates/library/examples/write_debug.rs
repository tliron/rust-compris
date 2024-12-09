// You need this for the trait functions!
use compris::WriteDebug;

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

    content.write_debug(&mut anstream::stdout()).unwrap();

    // Note that you *can* write to standard stdout, too, but colorization won't be controllable:
    // content.write_debug(&mut std::io::stdout()).unwrap();
}
