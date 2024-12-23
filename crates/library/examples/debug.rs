use {anstream::println, compris::*, kutil_cli::debug::*, owo_colors::*};

pub fn main() {
    let heading = Styles::default().heading;

    // See examples/literal.rs

    let value = normal_list![
        normal_list![-1, "element", 1.5],
        normal_map![("key1", "value1"), ("key2", "value2")],
        normal_map![(normal_map![("complex_key", "complex_value")], 123456)]
    ];

    // This writes to anstream::stdout()

    println!("{}", "print_debug:".style(heading));
    value.print_debug();

    // Note that you *can* write to io::stdout(), too, but it's without colorization

    println!("\n{}", "print_debug_plain:".style(heading));
    value.print_debug_plain();

    // Write to any io::Write

    let mut writer = anstream::stdout();
    println!("\n{}", "write_debug:".style(heading));
    value.write_debug(&mut writer).unwrap();

    // You can also capture the (plain) debug output into a string

    let string = value.to_debug_string().unwrap();
    println!("\n{}\n{}", "to_debug_string".style(heading), string);
}
