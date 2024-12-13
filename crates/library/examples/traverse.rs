use {
    anstream::println,
    compris::{read::*, *},
    kutil_cli::debug::*,
    owo_colors::*,
};

pub fn main() {
    let yaml = r#"
hello:
  world:
    10:
      how:
      - are you:
          doing
"#;

    let value = Reader::new(Format::YAML).read_from_string(yaml).unwrap();

    // The first argument is our starting point and then it's a sequence of keys as bare primitive
    // expressions or normal value types (see examples/literal.rs)

    let found_value = traverse!(value, "hello", "world", 10, "how", 0, "are you").unwrap();
    println!("{}", "found:".yellow());
    found_value.print_debug();

    // If the traverse macro hits a non-map or a missing key along the way, it stops and returns None

    let result = traverse!(value, "hello", "world", "how", "are you"); // we missed a key
    println!("\n{}\n{:?}", "did we find anything?".yellow(), result);

    // There are two ways to discover the type of what we found.

    // We can do regular matching on the value enum

    match found_value {
        // won't happen
        Value::Integer(integer) => println!("\n{}\n{}", "it matches an integer:".yellow(), integer.value),

        Value::Text(text) => println!("\n{}\n{}", "it matches text:".yellow(), text.value),

        _ => (),
    }

    // Or we can use try_from or try_into directly on primitives

    if let Ok(integer) = i64::try_from(found_value) {
        // won't happen
        println!("\n{}\n{}", "it can be converted to an integer:".yellow(), integer);
    }
    if let Ok(string) = <&str>::try_from(found_value) {
        println!("\n{}\n{}", "it can be converted to a string:".yellow(), string);
    }

    // Path::find gets us a human-readable path between two values

    if let Some(path) = Path::find(&value, found_value) {
        println!("\n{}\n{}", "path to found value:".yellow(), path);
    }
}
