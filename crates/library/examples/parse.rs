mod utils;

use {
    compris::{parse::*, *},
    kutil_cli::debug::*,
};

pub fn main() {
    let yaml = r#"- [true, True, !!bool true]
- [null, Null, ~]
- &my-anchor [ -1, element, 1.5 ]
- *my-anchor
- key1: value1
  key2: value2
- {complex_key: complex_value}: 123456"#;

    // Parse into normal value types

    let value = Parser::new(Format::YAML).parse_from_string(yaml).expect("parse");

    utils::heading("from YAML", true);
    value.print_debug();

    let xjson = r#"[
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
]"#;

    // This reader will interpret the XJSON hints and create the correct normal value types

    let value = parse::Parser::new(Format::XJSON).parse_from_string(xjson).expect("parse");

    utils::heading("from XJSON", false);
    value.print_debug();
}
