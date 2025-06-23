mod utils;

use {
    anstream::{print, println},
    compris::{annotation::*, normal::*, parse::*, resolve::*, *},
    kutil_cli::debug::*,
    std::{collections::*, fmt},
};

// See first: examples/resolve_basic.rs

#[derive(Debuggable, Default, Resolve)]
// By default #[derive(Resolve)] will add a generic parameter for annotations
// But if we want to define and use it in our type then we must specify it via #[resolve(annotations_parameter=...)]
// We're also adding our own generic parameter, ExtraT, just to show that it is possible
#[resolve(annotations_parameter=AnnotationsT)]
#[allow(dead_code)]
struct User<AnnotationsT, ExtraT>
where
    AnnotationsT: fmt::Debug,
    ExtraT: Default + fmt::Debug,
{
    #[resolve(required)]
    #[debuggable(style(string))]
    name: String,

    #[resolve]
    #[debuggable(style(number))]
    credit: i32,

    #[resolve(required, key = "enabled")]
    #[debuggable(style(symbol))]
    is_enabled: bool,

    #[resolve(null = Some("no group".into()))]
    #[debuggable(option, style(string))]
    group: Option<String>,

    // Here we use the generic annotations parameter we defined and specified
    // (Value resolves into itself)
    #[resolve]
    #[debuggable(option, as(debuggable))]
    metadata: Option<Value<AnnotationsT>>,

    // By default unknown keys cause errors
    // But we can resolve and collate them instead
    // (To ignore them instead, use #[resolve(ignore_other_keys)] on the struct)
    // The field marked with "other_keys" must support ".insert(_, _)"
    // Both key and value will be resolved upon insertion
    // You can use other "resolve" attributes on this field, too, like "null", "ignore_null", etc.
    #[resolve(other_keys, null = -100)]
    #[debuggable(iter(kv), key_style(string), style(number))]
    other: HashMap<String, i64>,

    // Our generic field
    #[debuggable(skip)]
    extra: ExtraT,

    // And for our final trick: you can collate all field annotations to a field
    // This will also generate implementations of `Annotated` and `AnnotatedFields` for this struct
    #[resolve(annotations)]
    #[debuggable(skip)]
    annotations: FieldAnnotations,
}

pub fn main() {
    let json = r#"[{
    "name": "Tal",
    "credit": "wrong type",
    "group": null,
    "metadata": {"anything": "we want"}
}, {
    "name": "Shiri",
    "credit": 123,
    "enabled": true,
    "mystery key 1!": 456,
    "mystery key 2!": null
}]"#;

    let value = with_annotations!(
        Parser::new(Format::JSON)
            .with_source("json".into())
            .with_try_integers(true)
            .parse_from_string(json)
            .expect("parse")
    );

    let mut errors = ResolveErrors::new();
    let users: Vec<User<_, isize>> =
        value.resolve_with_errors(&mut errors).expect("errors should be accumulated").expect("some");

    utils::heading("partially resolved", true);
    for user in &users {
        user.print_debug();
    }

    if !errors.is_empty() {
        println!();
        errors.annotated_debuggables(Some("accumulated errors".into())).print_debug();
    }

    utils::heading("annotations", false);
    for (index, user) in users.iter().enumerate() {
        println!("User[{}]: ", index);

        // We'll convert to BTreeMap so it will be sorted
        let annotations: BTreeMap<_, _> = user.annotations.iter().collect();

        for (field_name, annotation) in annotations {
            print!("  ");
            if !field_name.is_empty() {
                print!("{}: ", field_name);
            }
            annotation.print_debug();
        }
    }
}
