mod utils;

use {
    anstream::println,
    compris::{cite::*, parse::*, resolve::*, *},
    kutil_cli::debug::*,
    kutil_std::error::*,
};

// Note that #[derive(Resolve)] requires an implementation of the Default trait,
// which we here get from #[derive(Default)]
#[derive(Debug, Default, Resolve)]
// We are here setting the context and error to CommonResolveContext and CommonResolveError
// If we don't explicitly set either or both, #[derive(Resolve)] will generate generic code
// (which is useful if you want your struct to be resolvable in various parsing environments)
#[resolve(context = CommonResolveContext, error = CommonResolveError)]
// To avoid compiler warnings
#[allow(dead_code)]
struct User {
    // "required" means we will get an error if the key is not present in the map
    #[resolve(required)]
    name: String,

    // Not "required", so will have the default value if the key is not present in the map
    #[resolve]
    credit: i32,

    // We can explicitly set the map key; otherwise it is the field name
    #[resolve(required, key = "enabled")]
    is_enabled: bool,

    // We must explicitly allow null if we want to support it
    // Note that we can also use #resolve(ignore_null), which will leave the field at the default value
    // By default null will cause an error, because it can only be resolved into a rather useless
    // Value::Null field
    #[resolve(null = Some("no group".into()))]
    group: Option<String>,

    // Fields without #[resolve] will be ignored by resolve and just have the default value
    ignored: bool,
}

pub fn main() {
    // See examples/literal.rs

    let value = normal_map![("name", "Tal"), ("credit", 800), ("enabled", true), ("group", "moderators")];

    // Simplest! Resolve the value into our struct

    let user: User = value.resolve().unwrap().unwrap();

    utils::heading("resolved", true);
    println!("{:#?}", user);

    // Now let's intentionally cause errors by not specifying required fields, using wrong value
    // types, and specifying unsupported keys

    // We'll also parse JSON in order to demonstrate citation informaton for the error message
    // (there's no citation for the literals above)

    let json = r#"[{
    "name": "Tal",
    "credit": "wrong type",
    "group": null
}, {
    "name": "Shiri",
    "credit": 123,
    "enabled": true,
    "mystery key 1!": 456,
    "mystery key 2!": null
}]"#;

    let value = Parser::new(Format::JSON).with_try_integers(true).parse_from_string(json).unwrap();

    // Note that we can resolve directly into Vecs (and HashMaps, too)

    let result: ResolveResult<Vec<User>, _> = value.resolve();

    utils::heading("fail-fast error", false);
    result.err().unwrap().to_cited().print_debug();

    // The "resolve" functions used above use "fail-fast" mode, meaning that we fail on the first
    // first encountered error

    // Alternatively, we can call "resolve_into" to accumulate all the errors without failing
    // Note that we might still get a partially-resolved result even when there are accumulated errors,
    // but that depends on the implementation

    let mut errors = Errors::new();
    let users: Vec<User> = value.resolve_into(&mut errors).unwrap().unwrap();

    utils::heading("partially resolved", false);
    println!("{:#?}", users);

    if !errors.is_empty() {
        utils::heading("accumulated errors", false);
        errors.to_cited().print_debug();
    }

    // Continue to examples/resolve_advanced.rs to learn more
}
