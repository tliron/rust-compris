// You need this for the trait functions
use compris::{read::StringReader, PrintDebugAnstream};

pub fn main() {
    let yaml = r#"
hello:
  world:
    10:
      how:
        are:
          you
"#;

    let value = compris::read::Reader::new_for_string(yaml, compris::Format::YAML).read().unwrap();

    // The first argument is our starting point and then it's a sequence of keys as bare primitive
    // expressions or normal value types (see examples/literals.rs)
    let found_value = compris::traverse!(value, "hello", "world", 10, "how", "are").unwrap();
    anstream::println!("found:");
    found_value.print_debug();

    // If the traverse macro hits a non-map or a missing key along the way, it stops and returns None
    let result = compris::traverse!(value, "hello", "world", "how", "are"); // we missed 10
    anstream::println!("\ndid we find anything? {:?}", result);

    // There are two ways to discover the type of what we found.
    // We can do regular matching on the value enum:
    match found_value {
        compris::Value::Integer(integer) => println!("\nit matches an integer: {}", integer.value),
        compris::Value::String(string) => println!("\nit matches a string: {}", string.value),
        _ => (),
    }

    // Or we can use try_from or try_into directly on primitives:
    if let Ok(integer) = i64::try_from(found_value) {
        anstream::println!("\nit can be converted to an integer: {}", integer);
    }
    if let Ok(string) = <&str>::try_from(found_value) {
        anstream::println!("\nit can be converted to a string: {}", string);
    }
}
