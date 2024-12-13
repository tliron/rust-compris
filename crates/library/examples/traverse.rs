mod utils;

use {
    anstream::println,
    compris::{normal::*, parse::*, path::*, *},
    kutil_cli::debug::*,
};

pub fn main() {
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

    utils::heading("found by macro", true);
    found_value.print_debug();

    // The macro above works with a literal path (no allocation), but there's also a traversal
    // function that accepts an iterator if your path is constructed dynamically

    let mut path = normal_vec!["hello", "world", 10];
    path.push(normal!("how"));
    let found_value = value.traverse(&path).unwrap();

    utils::heading("found by array", false);
    found_value.print_debug();

    // If "traverse!" hits a non-map or a missing key along the way, it stops and returns None

    let result = traverse!(value, "hello", "world", "how", "are you"); // we missed a key

    utils::heading("did we find anything? (nope!)", false);
    println!("{:?}", result);

    // There are two ways to discover the type of what we found.

    // We can do regular matching on the value enum

    match found_value {
        Value::Text(text) => {
            utils::heading("it matches text", false);
            println!("{}", text.value);
        }

        // this branch won't happen; just an example
        Value::Integer(_) => {}

        _ => (),
    }

    // Or we can use try_from or try_into directly on primitives

    if let Ok(string) = <&str>::try_from(found_value) {
        utils::heading("it can be converted to a string", false);
        println!("{}", string);
    } else if let Ok(_) = i64::try_from(found_value) {
        // this branch won't happen; just an example
    }

    // Route::find gets us all values from an ancestor to a descendent
    // (Use Path::find instead for an owned version that doesn't keep the value references)

    let found_value = traverse!(value, "hello", "world", 10, "how", 0, "are you").unwrap();
    let route = Route::find(&value, found_value).unwrap();

    utils::heading("route to found value", false);
    route.print_debug();
}
