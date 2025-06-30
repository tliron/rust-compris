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

    // Parse into normal types

    let variant =
        with_annotations!(Parser::new(Format::YAML).with_source("yaml".into()).parse_from_string(yaml).expect("parse"));

    utils::heading("from YAML", true);
    variant.annotated_debuggable().print_debug_with_format(DebugFormat::Verbose);

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

    // This reader will interpret the XJSON hints and create the correct normal types

    let variant = with_annotations!(
        parse::Parser::new(Format::XJSON).with_source("xjson".into()).parse_from_string(xjson).expect("parse")
    );

    utils::heading("from XJSON", false);
    variant.annotated_debuggable().print_debug_with_format(DebugFormat::Verbose);
}
