mod utils;

use {anstream::println, compris::*, kutil_cli::debug::*};

pub fn main() {
    // See examples/literal.rs

    let value = normal_list![
        normal_list![-1, "element", 1.5],
        normal_map![("key1", "value1"), ("key2", normal_list![-3, "another element", 1.6])],
        normal_map![(normal_map![("complex_key", "complex_value")], 123456)]
    ];

    // "print_debug" writes to anstream::stdout

    utils::heading("print_debug", true);
    value.print_debug();

    // All functions have _with_format variants (the default is "reduced")

    utils::heading("print_debug (verbose)", false);
    value.print_debug_with_format(DebugFormat::Verbose);

    utils::heading("print_debug (compact)", false);
    value.print_debug_with_format(DebugFormat::Compact);

    // You can write to io::stdout, too, via "print_debug_plain"

    utils::heading("print_debug_plain", false);
    value.print_debug_plain();

    // Also "write_debug" to any io::Write

    utils::heading("write_debug", false);
    let mut writer = anstream::stdout();
    value.write_debug(&mut writer).unwrap();

    // You can also capture the debug output into a string

    let string = value.to_debug_string(&Theme::plain()).unwrap();
    utils::heading("to_debug_string", false);
    println!("{}", string);

    // This was all just to show you that Compris normal types support the Debuggable trait

    // Learn more about it, including how to roll your own Debuggables, in kutil-cli:
    // https://github.com/tliron/rust-kutil
}
