// You need this for the trait functions
use compris::PrintDebugAnstream;

pub fn main() {
    // Use "normal!"" with a single bare primitive expression
    let value = compris::normal!("hello");
    anstream::print!("literal value: ");
    value.print_debug();

    // Use "normal_list!"" with a sequence of bare primitive expressions
    // (Use "()" for null)
    let value = compris::normal_list!["hello", 3 * 8, 6.2, true, ()];
    anstream::println!("\nliteral list:");
    value.print_debug();

    // Use "normal_map!"" for maps with a sequence of key-value tuples
    let value = compris::normal_map![("key", 1i32), (2u8, "value"),];
    anstream::println!("\nliteral map:");
    value.print_debug();

    // You can nest as well as mix bare primitive expressions with values
    let value = compris::normal_map![
        ("key", compris::normal_list![4, 5, compris::Integer::new(6)]),
        (
            // This is the key
            compris::normal_map![("complex_key1", "complex_value1"), ("complex_key2", "complex_value2")],
            // This is the value
            compris::normal_list![7, 8, 9]
        ),
    ];
    anstream::println!("\nliteral nested:");
    value.print_debug();
}
