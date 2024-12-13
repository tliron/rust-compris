use {
    anstream::println,
    compris::{normal::*, parse::*, path::*, *},
    kutil_cli::debug::*,
    owo_colors::*,
};

pub fn main() {
    let heading = Theme::default().heading;

    let yaml = r#"hello:
  world:
    10: # note that this is an integer map key, not a list index!
      how:
      - are you:
          doing
"#;

    let value = Parser::new(Format::YAML).parse_from_string(yaml).unwrap();

    // The first argument for "traverse!" is our starting point and then it's a sequence of
    // map keys or list indexes as bare primitive expressions or normal value types
    // (see examples/literal.rs)

    let found_value = traverse!(value, "hello", "world", 10, "how", 0, "are you").unwrap();

    println!("{}", "found by macro:".style(heading));
    found_value.print_debug();

    // The macro above works with a literal path (no allocation), but there's also a traversal
    // function that accepts an iterator if your path is constructed dynamically

    let mut path = normal_vec!["hello", "world", 10]; // literavl vec
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
        Value::Text(text) => println!("\n{}\n{}", "it matches text:".style(heading), text.value),

        // this branch won't happen
        Value::Integer(integer) => println!("\n{}\n{}", "it matches an integer:".style(heading), integer.value),

        _ => (),
    }

    // Or we can use try_from or try_into directly on primitives

    if let Ok(string) = <&str>::try_from(found_value) {
        println!("\n{}\n{}", "it can be converted to a string:".style(heading), string);
    } else if let Ok(integer) = i64::try_from(found_value) {
        // this branch won't happen
        println!("\n{}\n{}", "it can be converted to an integer:".style(heading), integer);
    }

    // Path::find gets us all values from an ancestor to a descendent
    // and has a human-friendly fmt::Display implementation

    let found_value = traverse!(value, "hello", "world", 10, "how", 0, "are you").unwrap();
    let path = Path::find(&value, found_value).unwrap();
    println!("\n{}\n{}", "path to found value:".style(heading), path);
}
