use {anstream::println, compris::*, kutil_cli::debug::*, owo_colors::*};

pub fn main() {
    let heading = Theme::default().heading;

    // See examples/literal.rs

    let value = normal_list![
        normal_list![-1, "element", 1.5],
        normal_map![("key1", "value1"), ("key2", "value2")],
        normal_map![(normal_map![("complex_key", "complex_value")], 123456)]
    ];

    // "print_debug" writes to anstream::stdout

    println!("{}", "print_debug:".style(heading));
    value.print_debug();

    // You can write to io::stdout, too, via "print_debug_plain"

    println!("\n{}", "print_debug_plain:".style(heading));
    value.print_debug_plain();

    // Also "write_debug" to any io::Write

    println!("\n{}", "write_debug:".style(heading));
    let mut writer = anstream::stdout();
    value.write_debug(&mut writer).unwrap();

    // You can also capture the debug output into a string

    let string = value.to_debug_string(&Theme::plain()).unwrap();
    println!("\n{}\n{}", "to_debug_string".style(heading), string);

    // This was all just to show you that Compris normal types support the Debuggable trait

    // Learn more about it, including how to roll your own Debuggables, in kutil-cli:
    // https://github.com/tliron/rust-kutil
}
