mod utils;

use {
    anstream::println,
    compris::{resolve::*, *},
    std::path::*,
};

#[derive(Debug, Resolve)]
#[resolve(context = CommonResolveContext, error = CommonResolveError)]
#[allow(dead_code)]
enum File {
    #[resolve]
    Path(PathBuf),

    #[resolve]
    Content(VecU8),

    // When such "unit" variants are selected the value of the key is otherwise ignored
    #[resolve]
    Empty,

    // Variants with named fields are not supported by #[resolve]
    // (but they can otherwise be included in the enum)
    Coordinates {
        x: f64,
        y: f64,
    },
}

pub fn main() {
    // See examples/literal.rs

    let value = normal_map![("content", "my content".as_bytes())];

    let file: File = value.resolve().unwrap().unwrap();

    utils::heading("resolved", true);
    println!("{:#?}", file);
}
