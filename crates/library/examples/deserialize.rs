mod utils;

use {
    anstream::println,
    compris::{parse::*, *},
    serde::Deserialize,
};

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct User {
    name: String,
    credit: i64,
    enabled: bool,
    group: Option<String>,
    role: Role,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
enum Role {
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "moderator")]
    Moderator(String),
}

pub fn main() {
    // See examples/literal.rs

    let value = normal_map![
        ("name", "Tal"),
        ("credit", 800),
        ("enabled", true),
        ("group", ()),
        ("role", normal_map![("moderator", "forums")]),
    ];

    // We can "deserialize" from the normal value directly to our struct

    let user: User = value.deserialize().unwrap();

    utils::heading("from normal types", true);
    println!("{:#?}", user);

    // But we can also deserialize from a parser of any representation format
    // (note that internally it is first parsed into normal value types like above)

    let json = r#"{
    "name": "Linus",
    "credit": 250,
    "enabled": false,
    "group": null,
    "role": {"moderator": "lobby"}
}"#;

    let user: User = Parser::new(Format::JSON).deserialize_from_string(json).unwrap();

    utils::heading("from JSON", false);
    println!("{:#?}", user);
}
