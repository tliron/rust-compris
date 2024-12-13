use {
    anstream::println,
    compris::{cite::*, parse::*, resolve::*, *},
    kutil_cli::debug::*,
    owo_colors::*,
};

// Note that #[derive(Resolve)] requires an implementation of the Default trait
// (so we can just #[derive(Default)], too)
#[derive(Default, Debug, Resolve)]
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

    // We can explicitly set the map key; otherwise it will be the field name
    #[resolve(required, key = "enabled")]
    is_enabled: bool,

    // We must explicitly allow null if we want to support it
    // Note that we can also use #resolve(ignore_null), which will leave the field at the default value
    // Otherwise null will cause an error, because it can only be resolved to itself
    #[resolve(null = Some("no group".into()))]
    group: Option<String>,

    // Fields without #[resolve] will be ignored by resolution and just have the default value
    ignored: bool,
}

pub fn main() {
    let heading = Theme::default().heading;

    // See examples/literal.rs

    let value = normal_map![("name", "Tal"), ("credit", 800), ("enabled", true), ("group", "moderators")];

    // Simplest! Resolve the value into our struct

    let user: User = value.resolve().unwrap().unwrap();

    println!("{}\n{:#?}", "resolved:".style(heading), user);

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

    println!("\n{}", "fail-fast error:".style(heading));
    result.err().unwrap().to_cited().print_debug();

    // The "resolve" functions used above use "fail-fast" mode, meaning that we failed on the first
    // first encountered resolution error

    // Alternatively, we can call "resolve_into" to accumulate all the errors without failing
    // Note we might still get a partially-resolved result even when there are accumulated errors
    // (this is not guaranteed)

    let mut errors = Errors::new();
    let users: Vec<User> = value.resolve_into(&mut errors).unwrap().unwrap();

    println!("\n{}\n{:#?}", "partially resolved:".style(heading), users);

    if !errors.is_empty() {
        println!("\n{}", "accumulated errors:".style(heading));
        errors.to_cited().print_debug();
    }
}
