mod utils;

use {
    anstream::println,
    compris::{resolve::*, *},
    std::path::*,
};

// See first: examples/resolve_basic.rs

// We can #[derive(Resolve)] for enums, too
#[derive(Debug, Resolve)]
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
    let variant = without_annotations!(normal_map![("content", "my content")]);
    let data: Data = variant.resolve().expect("resolve");

    utils::heading("resolved content", true);
    println!("{:#?}", data);

    // For unit variants the value of the key is ignored

    let variant = without_annotations!(normal_map![("empty", ())]);
    let data: Data = variant.resolve().expect("resolve");

    utils::heading("resolved empty", false);
    println!("{:#?}", data);
}
