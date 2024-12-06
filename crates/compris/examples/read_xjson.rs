pub fn main() {
    let xjson = r#"
[
  {
    "simple_key1": {"$ard.integer": "1"},
    "simple_key2": {"$ard.uinteger": "2"}
  },
  {
    "$ard.map": [
      [{"complex_key1": "complex_value1", "complex_key1a": "complex_value1a"}, {"$ard.integer": "3"}],
      [{"complex_key2": "complex_value2"}, {"$ard.uinteger": "4"}]
    ]
  }
]
"#;

    let mut reader = cpd::read::Reader::new(xjson.as_bytes(), cpd::Format::XJSON);
    let content = reader.read().unwrap();
    println!("{}", content);
}
