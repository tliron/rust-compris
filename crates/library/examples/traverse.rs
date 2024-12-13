use {
    anstream::println,
    compris::{normal::*, read::*, *},
    kutil_cli::debug::*,
    owo_colors::*,
};

pub fn main() {
    let heading = Styles::default().heading;

    let yaml = r#"
hello:
  world:
    10:
      how:
      - are you:
          doing
"#;

    let value = Reader::new(Format::YAML).read_from_string(yaml).unwrap();

    // The first argument for "traverse!" is our starting point and then it's a sequence of
    // keys as bare primitive expressions or normal value types (see examples/literal.rs)

    let found_value = traverse!(value, "hello", "world", 10, "how", 0, "are you").unwrap();
    println!("{}", "found by macro:".style(heading));
    found_value.print_debug();

    // The macro works with a literal path, but there's also a traversal function that accepts
    // an array

    let mut path = normal_vec!["hello", "world", 10];
    path.push(normal!("how"));
    let found_value = value.traverse(&path).unwrap();
    println!("\n{}", "found by array:".style(heading));
    found_value.print_debug();

    // If "traverse!" hits a non-map or a missing key along the way, it stops and returns None

    let result = traverse!(value, "hello", "world", "how", "are you"); // we missed a key
    println!("\n{}\n{:?}", "did we find anything?".style(heading), result);

    // There are two ways to discover the type of what we found.

    // We can do regular matching on the value enum

    match found_value {
        // won't happen
        Value::Integer(integer) => println!("\n{}\n{}", "it matches an integer:".style(heading), integer.value),

        Value::Text(text) => println!("\n{}\n{}", "it matches text:".style(heading), text.value),

        _ => (),
    }

    // Or we can use try_from or try_into directly on primitives

    if let Ok(integer) = i64::try_from(found_value) {
        // won't happen
        println!("\n{}\n{}", "it can be converted to an integer:".style(heading), integer);
    } else if let Ok(string) = <&str>::try_from(found_value) {
        println!("\n{}\n{}", "it can be converted to a string:".style(heading), string);
    }

    // Path::find gets us a human-readable path between two values

    if let Some(path) = Path::find(&value, found_value) {
        println!("\n{}\n{}", "path to found value:".style(heading), path);
    }
}
