use {
    anstream::println,
    compris::{read::*, *},
    owo_colors::*,
    serde::Deserialize,
};

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
    println!("{}\n{:#?}", "from normal types:".yellow(), user);

    // But we can also deserialize from a reader of any representation format
    // (note that internally it is first read into normal value types like above)

    let json = r#"
{
    "name": "Linus",
    "credit": 250,
    "enabled": false,
    "group": null,
    "role": {"moderator": "lobby"}
}
"#;

    let user: User = Reader::new(Format::JSON).deserialize_from_string(json).unwrap();
    println!("\n{}\n{:#?}", "from JSON:".yellow(), user);
}

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
