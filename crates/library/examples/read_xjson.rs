// You need this for the trait functions
use compris::{read::StringReader, PrintDebugAnstream};

pub fn main() {
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

    // The reader will interpret the hints and create the correct normal value types
    let value = compris::read::Reader::new_for_string(xjson, compris::Format::XJSON).read().unwrap();
    value.print_debug();
}
