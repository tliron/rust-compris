use {
    anstream::{print, println},
    compris::{citation::*, read::*, resolve::*, *},
    kutil_cli::debug::*,
    owo_colors::*,
    std::collections::*,
};

// See first: examples/resolve_basic.rs

#[derive(Default, Debug, Resolve)]
#[resolve(context = CommonResolveContext, error = CommonResolveError)]
#[allow(dead_code)]
struct User {
    #[resolve(required)]
    name: String,

    #[resolve]
    credit: i64,

    #[resolve(required, key = "enabled")]
    is_enabled: bool,

    #[resolve(null = Some("no group".into()))]
    group: Option<String>,

    // By default unknown keys cause errors
    // But we can resolve and collate them instead
    // (To ignore them instead, use #[resolve(ignore_other_keys)] on the struct)
    // The other_keys field must support ".insert(_, _)"
    // Both key and value will be resolved
    // You can use other attributes on this field, too, like "null", "ignore_null", etc.
    #[resolve(other_keys, null = -100)]
    other: HashMap<String, i64>,

    // Another neat trick: you can collate all field citations
    // The citations field must support ".insert(String, Citation)"
    #[resolve(citations)]
    citations: HashMap<String, Citation>,
}

pub fn main() {
    let heading = Styles::default().heading;

    let json = r#"
[{
    "name": "Tal",
    "credit": "wrong type",
    "group": null
}, {
    "name": "Shiri",
    "credit": 123,
    "enabled": true,
    "mystery key 1!": 456,
    "mystery key 2!": null
}]
"#;

    let value = Reader::new(Format::JSON).with_try_integers(true).read_from_string(json).unwrap();

    let mut errors = Errors::new();
    let users: Vec<User> = value.resolve_into(&mut errors).unwrap().unwrap();

    println!("\n{}\n{:#?}", "partially resolved:".style(heading), users);

    if !errors.is_empty() {
        println!("\n{}", "accumulated errors:".style(heading));
        errors.to_cited().print_debug();
    }

    println!("\n{}", "citations:".style(heading));
    for (index, user) in users.iter().enumerate() {
        println!("User[{}]: ", index);
        for (field_name, citation) in &user.citations {
            print!("  {}: ", field_name);
            citation.print_debug();
        }
    }
}
