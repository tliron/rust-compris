mod utils;

use {
    anstream::println,
    compris::{resolve::*, *},
    std::path::*,
};

// See first: examples/resolve_basic.rs

// We can #[derive(Resolve)] for enums, too
#[derive(Debug, Resolve)]
#[resolve(context = CommonResolveContext, error = CommonResolveError)]
#[allow(dead_code)]
enum Data {
    // We must use #[derive(resolve)] for variants we want to support
    // If the key is not provided, it will default to the variant name
    #[resolve(key = "path")]
    Path(PathBuf),

    #[resolve(key = "content")]
    Content(String),

    #[resolve(key = "empty")]
    Empty,

    // Variants with named fields are not supported by #[resolve]
    // (but they can otherwise be included in the enum)
    Coordinates {
        x: f64,
        y: f64,
    },
}

pub fn main() {
    let value = normal_map![("content", "my content")];
    let data: Data = value.resolve().expect("resolve").expect("some");

    utils::heading("resolved content", true);
    println!("{:#?}", data);

    // For unit variants the value of the key is ignored
    // (we can set it be Variant::Null)

    let value = normal_map![("empty", normal::Null::new())];
    let data: Data = value.resolve().expect("resolve").expect("some");

    utils::heading("resolved empty", false);
    println!("{:#?}", data);
}
