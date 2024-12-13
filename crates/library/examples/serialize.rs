use {
    anstream::println,
    compris::{ser::*, *},
    kutil_cli::debug::*,
    owo_colors::*,
};

pub fn main() {
    let heading = Styles::default().heading;

    // See examples/literal.rs

    let value = normal_list![
        normal_list![-1, "element", 1.5],
        normal_map![("key1", "value1"), ("key2", "value2")],
        normal_map![(compris::normal_map![("complex_key", "complex_value")], 123456)]
    ];

    println!("{}", "CBOR:".style(heading));
    Serializer::new(Format::CBOR).with_base64(true).with_pretty(true).write_to_stdout(&value).unwrap();

    // While CBOR and MessagePack support 100% of CPS, YAML and JSON do not,
    // and so we need to attach a "serialization mode" to the value, which may make some
    // compromises to ensure that the value is serializable; without these modes we could
    // get serialization errors for incompatible data

    // Note thjat the serialization mode only affects serialization behavior;
    // it does not change the actual data

    // In the default JSON serialization mode, all map keys are stringified in order to conform
    // with JSON's requirement

    println!("\n{}", "JSON:".style(heading));
    Serializer::new(Format::JSON)
        .with_pretty(true)
        .write_to_stdout(&value.with_serialization_mode(&SerializationMode::for_json()))
        .unwrap();

    // Below, Format::XJSON functions as just an alias for Format::JSON
    // The actual difference is in the serialization mode
    // In the case of XJSON, the "compromise" is that the resulting JSON may include type hints
    // It's still true JSON, but readers would need to know what to do with the hints

    println!("\n{}", "XJSON:".style(heading));
    Serializer::new(Format::XJSON)
        .with_pretty(true)
        .write_to_stdout(&value.with_serialization_mode(&SerializationMode::for_xjson()))
        .unwrap();
}
