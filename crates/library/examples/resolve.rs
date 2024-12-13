use {
    anstream::println,
    compris::{citation::*, read::*, resolve::*, *},
    kutil_cli::debug::*,
    owo_colors::*,
};

pub fn main() {
    // See examples/literal.rs

    let value = normal_map![("name", "Tal"), ("credit", 800), ("enabled", true), ("group", "moderators")];

    // Resolve the value into our struct

    let user: User = value.resolve_fail_fast().unwrap().unwrap();
    println!("{}\n{:#?}", "resolved:".yellow(), user);

    // Now let's intentionally cause errors by not specifying required fields, using wrong value
    // types and wrong keys

    // We'll read from JSON in order to demonstrate citation informaton for the error messages
    // (there's no citation for the literals above)

    // We also show that can resolve into Vecs (you can resolve into HashMaps, too)

    let json = r#"
[{
    "name": "Tal",
    "credit": "wrong type",
    "group": null
}, {
    "name": "Shiri",
    "credit": 123,
    "enabled": true,
    "mystery key!": 123
}]
"#;

    // "Fail-fast" means that we fail on the first encountered error

    let value = Reader::new(Format::JSON).with_try_integers(true).read_from_string(json).unwrap();
    let result: ResolveResult<Vec<User>, _> = value.resolve_fail_fast();

    println!("\n{}", "fail-fast error:".yellow());
    result.err().unwrap().as_debuggable_with_citation().print_debug();

    // Alternatively, we can call "resolve" to accumulate all the errors without failing
    // Note we might still get a partially-resolved result even when there are accumulated errors

    let mut errors = Errors::new();
    let user: Vec<User> = value.resolve(&mut errors).unwrap().unwrap();

    println!("\n{}\n{:#?}", "partially resolved:".yellow(), user);

    if !errors.is_empty() {
        println!("\n{}", "accumulated errors:".yellow());
        errors.errors.as_debuggable_with_citation().print_debug();
    }
}

// We are here setting the error to CommonResolveError
// If we don't explicitly set an error, derive(Resolve) will generate generic code for a ResolveError
// (which is useful if you want your struct to be resolvable in various parsing environments)

// Also note that by default unknown keys will cause an error, which we wanted to demonstrate here
// To allow unknown keys, add this to the struct: #[resolve(allow_unknown_keys)]

// Final note: though derive(Resolve) is easy and powerful, you can always implement Resolve manually
// for custom behavior

#[derive(Default, Debug, Resolve)]
#[resolve(error = CommonResolveError)]
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
