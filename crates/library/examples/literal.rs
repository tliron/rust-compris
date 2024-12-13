mod utils;

use {
    compris::{normal::*, *},
    kutil_cli::debug::*,
};

pub fn main() {
    // Use "normal!" with a single bare primitive expression

    let value = normal!("hello");

    // See examples/debug.rs

    utils::heading("literal value", true);
    value.print_debug();

    // Use "normal_list!" with a sequence of bare primitive expressions
    // (Use "()" for a literal null)

    let value = normal_list!["hello", 3 * 8, 6.2, true, ()];

    utils::heading("literal list", false);
    value.print_debug();

    // Use "normal_map!" for maps via a sequence of key-value pairs

    let value = normal_map![("key", 5i32), (6u8, "value")];

    utils::heading("literal map", false);
    value.print_debug();

    // You can nest as well as mix bare primitive expressions with normal types

    let value = normal_map![
        ("key", normal_list![4, 5, Integer::new(6)]),
        (
            // This is the key (it's complex)
            normal_map![("complex_key1", "complex_value1"), ("complex_key2", "complex_value2")],
            // This is the value
            normal_list![7, 8, 9]
        ),
    ];

    utils::heading("literal nested", false);
    value.print_debug();
}
