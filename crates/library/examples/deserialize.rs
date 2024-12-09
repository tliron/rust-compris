// You need this for the trait functions
use compris::read::StringReader;

pub fn main() {
    // See examples/literals.rs
    let value = compris::normal_map![
        ("name", "Tal"),
        ("credit", 800),
        ("enabled", true),
        ("group", ()),
        ("role", compris::normal_map![("moderator", "forums")])
    ];

    // We can "deserialize" from the normal value directly to our struct
    let user: User = value.deserialize().unwrap();
    println!("from normal types:\n{:?}", user);

    // But we can also deserialize from a reader of any representation format
    // (note that internally it is first read into normal value types)

    let json = r#"
{
    "name": "Linus",
    "credit": 250,
    "enabled": false,
    "group": null,
    "role": {"moderator": "lobby"}
}
"#;

    let user: User = compris::read::Reader::new_for_string(json, compris::Format::JSON).deserialize().unwrap();
    println!("\nfrom JSON:\n{:?}", user);
}

#[derive(Debug, serde::Deserialize)]
#[allow(dead_code)]
struct User {
    name: String,
    credit: i64,
    enabled: bool,
    group: Option<String>,
    role: Role,
}

#[derive(Debug, serde::Deserialize)]
#[allow(dead_code)]
enum Role {
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "moderator")]
    Moderator(String),
}
