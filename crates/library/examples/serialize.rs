use {
    anstream::println,
    compris::{ser::*, *},
    kutil_cli::debug::*,
    owo_colors::*,
    serde::Serialize,
};

#[derive(Serialize)]
#[allow(unused)]
struct User {
    name: String,
    enabled: bool,
}

pub fn main() {
    let heading = Theme::default().heading;

    // See examples/literal.rs

    let value = normal_list![
        normal_list![-1, "element", 1.5],
        normal_map![("key1", "value1"), (123.45, "value2")],
        normal_map![(compris::normal_map![("complex_key", "complex_value")], "value3")]
    ];

    println!("{}", "CBOR:".style(heading));

    Serializer::new(Format::CBOR).with_base64(true).with_pretty(true).write_to_stdout(&value).unwrap();

    // While CBOR and MessagePack support 100% of CPS, YAML and JSON do not,
    // and so we need to attach a "serialization mode" to the value, which may make some
    // compromises to ensure that the value is serializable; without these modes we could
    // get serialization errors for incompatible values

    // Note that the serialization mode only affects serialization behavior;
    // it does not change the actual values

    // In the default JSON serialization mode, all map keys are stringified (as JSON!) in order
    // to conform to JSON's requirement that keys be strings

    println!("\n{}", "JSON:".style(heading));

    Serializer::new(Format::JSON)
        .with_pretty(true)
        .write_to_stdout_modal(&value, &SerializationMode::for_json())
        .unwrap();

    // Serialize to string

    let string = Serializer::new(Format::JSON).stringify_modal(&value, &SerializationMode::for_json()).unwrap();

    println!("\n{}\n{}", "JSON stringify:".style(heading), string);

    // Below, Format::XJSON functions as just an alias for Format::JSON
    // The actual difference is in the serialization mode
    // In the case of XJSON, the "compromise" is that the resulting JSON may include type hints
    // It's still true JSON, but readers would need to know what to do with the hints

    println!("\n{}", "XJSON:".style(heading));

    Serializer::new(Format::XJSON)
        .with_pretty(true)
        .write_to_stdout_modal(&value, &SerializationMode::for_xjson())
        .unwrap();

    // Finally, let's just prove that the Compris serializer can serialize anything, not
    // just normal types

    // However, note that we cannot support serialization modes for those, so you could get errors
    // for incompatible data (this is a limitation of Serde)

    let user = User { name: "Tal".into(), enabled: true };

    println!("\n{}", "YAML:".style(heading));

    Serializer::new(Format::YAML).with_pretty(true).write_to_stdout(&user).unwrap();
}
