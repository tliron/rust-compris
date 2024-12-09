pub fn main() {
    // See examples/literals.rs
    let value = compris::normal_list![
        compris::normal_list![-1, "element", 1.5],
        compris::normal_map![("key1", "value1"), ("key2", "value2")],
        compris::normal_map![(compris::normal_map![("complex_key", "complex_value")], 123456)]
    ];

    println!("CBOR:");
    compris::ser::Serializer::new_for_stdout()
        .with_format(compris::Format::CBOR)
        .with_base64(true)
        .with_pretty(true)
        .write(&value)
        .unwrap();

    // While CBOR and MessagePack support 100% of CPS, YAML and JSON (and XJSON) do not,
    // and so we need to attach a serialization mode to the value, which may make some
    // compromises to ensure that the value is serializable; otherwise we would get
    // serialization errors for incompatible data

    // The serialization mode only affects serialization behavior; it does not change the
    // actual data

    println!("\nJSON:");
    compris::ser::Serializer::new_for_stdout()
        .with_format(compris::Format::JSON)
        .with_pretty(true)
        .write(&value.with_serialization_mode(&compris::ser::SerializationMode::for_json()))
        .unwrap();

    // Below, Format::XJSON is just an alias for Format::JSON
    // The actual difference is in the serialization mode
    // In the case of XJSON, the "compromise" is that the resulting JSON may include type hints

    println!("\nXJSON:");
    compris::ser::Serializer::new_for_stdout()
        .with_format(compris::Format::XJSON)
        .with_pretty(true)
        .write(&value.with_serialization_mode(&compris::ser::SerializationMode::for_xjson()))
        .unwrap();
}
