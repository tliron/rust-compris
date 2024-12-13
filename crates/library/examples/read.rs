use {
    anstream::println,
    compris::{read::*, *},
    kutil_cli::debug::*,
    owo_colors::*,
};

pub fn main() {
    let heading = Styles::default().heading;

    let yaml = r#"
- [true, True, !!bool true]
- [null, Null, ~]
- [ -1, element, 1.5 ]
- key1: value1
  key2: value2
- {complex_key: complex_value}: 123456
"#;

    let value = Reader::new(Format::YAML).read_from_string(yaml).unwrap();
    println!("{}", "from YAML:".style(heading));
    value.print_debug();

    let xjson = r#"
[
  {
    "simple_key1": {"$hint.int": "1"},
    "simple_key2": {"$hint.uint": "2"}
  },
  {
    "$hint.map": [
      [{"complex_key1a": "complex_value1a", "complex_key1b": "complex_value1b"}, {"$hint.int": "3"}],
      [{"complex_key2": "complex_value2"}, {"$hint.uint": "4"}]
    ]
  },
  {"$hint.bytes": "SGVsbG8sIHdvcmxk"},
  {"$$hint.int": ["an escaped hint", null, 1, 2, 3]}
]
"#;

    // This reader will interpret the XJSON hints and create the correct normal value types

    let value = read::Reader::new(Format::XJSON).read_from_string(xjson).unwrap();
    println!("\n{}", "from XJSON:".style(heading));
    value.print_debug();
}
