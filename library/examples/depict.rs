mod utils;

use {
    anstream::println,
    compris::{annotate::*, normal::*, *},
    kutil::cli::depict::*,
};

pub fn main() {
    // See examples/literal.rs

    let variant: Variant<Annotations> = normal_list![
        normal_list![-1, "element", 1.5],
        normal_map![("key1", "value1"), ("key2", normal_list![-3, "another element", 1.6])],
        normal_map![(normal_map![("complex_key", "complex_value")], 123456)]
    ];

    // "print_depiction" writes to anstream::stdout

    utils::heading("print_depiction", true);
    variant.print_depiction(&DEFAULT_DEPICTION_CONTEXT);

    // We can change the format

    utils::heading("print_depiction (verbose)", false);
    variant.print_depiction(&DEFAULT_DEPICTION_CONTEXT.child().with_format(DepictionFormat::Verbose));

    utils::heading("print_depiction (compact)", false);
    variant.print_depiction(&DEFAULT_DEPICTION_CONTEXT.child().with_format(DepictionFormat::Compact));

    // We can change the theme

    utils::heading("print_depiction (plain)", false);
    variant.print_depiction(&PLAIN_DEPICTION_CONTEXT);

    // Also "write_debug" to any io::Write

    utils::heading("write_debug", false);
    let mut writer = anstream::stdout();
    variant.write_depiction(&mut writer, &DEFAULT_DEPICTION_CONTEXT).expect("write_depiction");

    // You can also capture the debug output into a string

    let string = variant.to_depiction(&PLAIN_DEPICTION_CONTEXT).expect("to_depiction");
    utils::heading("to_depiction", false);
    println!("{}", string);

    // This was all just to show you that Compris normal types support the Depict trait

    // Learn more about it, including how to roll your own Depict, in kutil:
    // https://github.com/tliron/rust-kutil
}
