// You need these for the trait functions
use compris::{PrintDebugAnstream, ToDebugString, WriteDebug, WriteDebugDyn};

pub fn main() {
    // See examples/literals.rs
    let value = compris::normal_list![
        compris::normal_list![-1, "element", 1.5],
        compris::normal_map![("key1", "value1"), ("key2", "value2")],
        compris::normal_map![(compris::normal_map![("complex_key", "complex_value")], 123456)]
    ];

    println!("print_debug:");
    value.print_debug();

    // Note that you *can* write to standard stdout, too, but colorization won't be controllable:
    // use compris::PrintDebug

    // Write to any io::Write
    let mut writer = anstream::stdout();
    println!("\nwrite_debug:");
    value.write_debug(&mut writer).unwrap();

    // If your writer is a dyn of std::io::Write instead of a concrete implementation of it
    // then we still got you covered with this variant
    let writer = &mut writer as &mut dyn std::io::Write;
    println!("\nwrite_debug_dyn:");
    value.write_debug_dyn(writer).unwrap();

    // Finally, you can also write to a string
    let string = value.to_debug_string().unwrap();
    println!("\nto_debug_string:\n{}", string);
}
