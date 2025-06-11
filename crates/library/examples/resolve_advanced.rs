mod utils;

use {
    anstream::{print, println},
    bytestring::*,
    compris::{cite::*, normal::*, parse::*, resolve::*, *},
    kutil_cli::debug::*,
    kutil_std::error::*,
    std::collections::*,
};

// See first: examples/resolve_basic.rs

#[derive(Debug, Default, Resolve)]
#[allow(dead_code)]
struct User {
    #[resolve(required)]
    name: String,

    #[resolve]
    credit: i32,

    #[resolve(required, key = "enabled")]
    is_enabled: bool,

    #[resolve(null = Some("no group".into()))]
    group: Option<String>,

    // By default unknown keys cause errors
    // But we can resolve and collate them instead
    // (To ignore them instead, use #[resolve(ignore_other_keys)] on the struct)
    // The field marked with "other_keys" must support ".insert(_, _)"
    // Both key and value will be resolved upon insertion
    // You can use other "resolve" attributes on this field, too, like "null", "ignore_null", etc.
    #[resolve(other_keys, null = -100)]
    other: HashMap<String, i64>,

    // Another neat trick: you can collate all field citations
    // The citations field must support ".insert(String, Citation)"
    // The citation for the struct itself will be under the empty string key
    #[resolve(citations)]
    citations: HashMap<ByteString, Citation>,
}

pub fn main() {
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

    let value = Parser::new(Format::JSON).with_try_integers(true).parse_from_string(json).expect("parse");

    // Unlike resolve_basic.rs, here need generic parameters because our derived implementation is generic
    // (It's quite a verbose syntax here, but in real-world uses the types would probably be inferred)

    let mut errors = Errors::new();
    let users =
        <Value as Resolve<Vec<User>, CommonResolveContext, CommonResolveError>>::resolve_into(&value, &mut errors)
            .expect("resolve")
            .expect("some");

    utils::heading("partially resolved", true);
    println!("{:#?}", users);

    if !errors.is_empty() {
        utils::heading("accumulated errors", false);
        errors.to_cited().print_debug();
    }

    utils::heading("citations", false);
    for (index, user) in users.iter().enumerate() {
        println!("User[{}]: ", index);
        for (field_name, citation) in &user.citations {
            print!("  ");
            if !field_name.is_empty() {
                print!("{}: ", field_name);
            }
            citation.print_debug();
        }
    }
}
