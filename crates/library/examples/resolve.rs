use {
    anstream::println,
    compris::{read::*, resolve::*, *},
    kutil_cli::debug::*,
    owo_colors::*,
};

pub fn main() {
    // See examples/literal.rs

    let value = normal_map![("name", "Tal"), ("credit", 800), ("enabled", true), ("group", "moderators")];

    // Resolve the value into our struct

    let user = (value.resolve_fail_fast() as CommonResolveResult<User>).unwrap().unwrap();
    println!("{}\n{:#?}", "resolved:".yellow(), user);

    // Note that above we cast the result to CommonResolveResult<T>
    // This sets the ResolveError::Other to CustomError
    // But you can provide your own error type instead via ResolveResult<T, OE>

    // Now let's intentionally cause errors by not specifying required fields, using wrong value
    // types  and wrong keys

    // We'll read from JSON in order to demonstrate location informaton for the error messages
    // (there's no location for the literals above)

    // We also show that can resolve into Vecs (you can resolve into HashMaps, too)

    let json = r#"
[{
    "name": "Tal",
    "credit": "wrong type",
    "group": null
}, {
    "name": "Tal",
    "mystery key!": 123
}]
"#;

    let value = Reader::new(Format::JSON).with_try_integers(true).read_from_string(json).unwrap();

    // "resolve_fail_fast" fails on first reported error

    let result: CommonResolveResult<Vec<User>> = value.resolve_fail_fast();
    println!("\n{}", "fail-fast error:".yellow());
    result.err().unwrap().print_debug();

    // Alternatively, we can call "resolve" to accumulate all the errors without failing
    // Note we might still get a partially-resolved result even when there are errors

    let mut errors = AccumulatedLocatableErrors::new();
    let user = (value.resolve(&mut errors) as CommonResolveResult<Vec<User>>).unwrap().unwrap();

    println!("\n{}\n{:#?}", "partially resolved:".yellow(), user);

    if !errors.get_errors().is_empty() {
        println!("\n{}", "accumulated errors:".yellow());
        errors.print_debug();
    }
}

// Note that by default unknown keys will cause an error
// To allow unknown keys, add this to the struct:
// #[resolve(allow_unknown_keys)]

#[derive(Default, Debug, Resolve)]
#[allow(dead_code)]
struct User {
    // "required" means we will get an error if the map key is not present
    #[resolve(required)]
    name: String,

    // Not "required", so will get the default value if the map key is not present
    #[resolve]
    credit: i64,

    // We can explicitly set the map key; otherwise it will be the field name
    #[resolve(required, key = "enabled")]
    is_enabled: bool,

    // An Option will allow us to resolve Value::Null (to None)
    #[resolve]
    group: Option<String>,

    // Fields without #[resolve] will get the default value
    ignored: bool,
}
