mod utils;

use {
    anstream::println,
    compris::{annotate::*, parse::*, resolve::*, *},
    kutil_cli::debug::*,
};

// Note that #[derive(Resolve)] requires an implementation of the Default trait,
// which we here get from #[derive(Default)]
#[derive(Debug, Default, Resolve)]
// To avoid compiler warnings
#[allow(dead_code)]
struct User {
    // "required" means we will get an error if the key is not present in the map
    // "single" means that we can resolve on *just* this field as a shorter notation
    // (only one field can be marked as "single")
    #[resolve(required, single)]
    name: String,

    // Not "required", so will have the default value if the key is not present in the map
    #[resolve]
    credit: i32,

    // We can explicitly set the map key; otherwise it is the field name
    #[resolve(required, key = "enabled")]
    is_enabled: bool,

    // Normally Variant::Null will cause an error when resolved
    // (Null can only be resolved into Null; why would you want a struct field that is Null?)
    // But we can explicitly add support for Null
    // There are two options:
    // Either `resolve(null = ...)` with the value assigned to null (we do this here)
    // Or `resolve(ignore_null)`, which will leave the field at the default value
    #[resolve(null = Some("no group".into()))]
    group: Option<String>,

    // Fields without #[resolve] will be ignored and just have the default value
    ignored: bool,
}

pub fn main() {
    // See examples/literal.rs

    let variant =
        without_annotations!(normal_map![("name", "Tal"), ("credit", 800), ("enabled", true), ("group", "moderators")]);

    // Simplest! Resolve the variant into our struct
    // ("fail fast" means that we will fail on the first error and return it)

    let user: User = variant.resolve().expect("resolve");

    utils::heading("resolved", true);
    println!("{:#?}", user);

    // Actually, this is even simpler:
    // We can resolve for just the field tagged as "single" (in this case it's the `name` field)
    // (a.k.a. "short notation")

    let variant = without_annotations!(normal!("Tal"));

    let user: User = variant.resolve().expect("resolve");

    utils::heading("resolved (single)", false);
    println!("{:#?}", user);

    // Now let's intentionally cause errors by not specifying required fields, using wrong value
    // types, and specifying unsupported keys

    // We'll also parse JSON first in order to demonstrate annotations for the error message
    // (there's are no annotations for the literal values above)

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

    let variant =
        with_annotations!(Parser::new(Format::JSON).with_try_integers(true).parse_from_string(json).expect("parse"));

    // Note that we can resolve directly into Vecs (and HashMaps, too)

    let result: Result<Vec<User>, _> = variant.resolve();

    utils::heading("fail-fast error", false);
    result.err().expect("error").annotated_debuggable().print_debug();

    // Instead of failing fast, we can call "resolve_with_errors" to accumulate all the errors *without* failing
    // Note that we might still get a partially-resolved result even when there are accumulated errors,
    // but that behavior depends on the resolver implementations

    let mut errors = ResolveErrors::default();
    let users: Vec<User> =
        variant.resolve_with_errors(&mut errors).expect("errors should be accumulated").expect("some");

    utils::heading("partially resolved", false);
    println!("{:#?}", users);

    if !errors.is_empty() {
        println!();
        errors.annotated_debuggables(Some("accumulated errors".into())).print_debug();
    }

    // Continue to examples/resolve_advanced.rs to learn more
}
