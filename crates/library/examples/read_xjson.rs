pub fn main() {
    let xjson = r#"
[
  {
    "simple_key1": {"$hint.int": "1"},
    "simple_key2": {"$hint.uint": "2"}
  },
  {
    "$hint.map": [
      [{"complex_key1": "complex_value1", "complex_key1a": "complex_value1a"}, {"$hint.int": "3"}],
      [{"complex_key2": "complex_value2"}, {"$hint.uint": "4"}]
    ]
  },
  {"$hint.bytes": "SGVsbG8sIHdvcmxk"},
  {"$$hint.int": ["an escaped hint", null, 1, 2, 3]}
]
"#;

    let mut reader = compris::read::Reader::new(xjson.as_bytes(), compris::Format::XJSON);
    let content = reader.read().unwrap();
    println!("{}", content);
}
