mod utils;

use {
    anstream::println,
    compris::{ser::*, *},
    serde::Serialize,
};

#[derive(Serialize)]
#[allow(unused)]
struct User {
    name: String,
    enabled: bool,
}

pub fn main() {
    // See examples/literal.rs

    let value = without_annotations!(normal_list![
        normal_list![-1, "element", 1.5],
        normal_map![("key1", "value1"), (123.45, "value2")],
        normal_map![(compris::normal_map![("complex_key", "complex_value")], "value3")]
    ]);

    // Note: "pretty" for CBOR just means adding a newline at the end

    utils::heading("CBOR", true);
    Serializer::new(Format::CBOR).with_base64(true).with_pretty(true).print(&value).expect("print");

    // While CBOR and MessagePack support 100% of CPS, YAML and JSON do not,
    // and so we need to attach a "serialization mode" to the value, which may make some
    // compromises to ensure that the value is serializable; without these modes we could
    // get serialization errors for incompatible values

    // Note that the serialization mode only affects serialization behavior;
    // it does not change the actual values

    // In the default JSON serialization mode, all map keys are stringified (as JSON in JSON!)
    // in order to conform to JSON's requirement that keys be strings

    utils::heading("JSON", false);
    Serializer::new(Format::JSON).with_pretty(true).print_modal(&value, &SerializationMode::for_json()).expect("print");

    // Serialize to string

    let string = Serializer::new(Format::JSON).stringify_modal(&value, &SerializationMode::for_json()).expect("print");

    utils::heading("JSON stringify", false);
    println!("{}", string);

    // Below, Format::XJSON functions as just an alias for Format::JSON
    // The actual difference is in the serialization mode
    // In the case of XJSON, the "compromise" is that the resulting JSON may include type hints
    // It's still true JSON, but readers would need to know what to do with the hints

    utils::heading("XJSON", false);
    Serializer::new(Format::XJSON)
        .with_pretty(true)
        .print_modal(&value, &SerializationMode::for_xjson())
        .expect("print");

    // Finally, let's just prove that the Compris serializer can serialize anything, not
    // just normal types

    // However, note that we cannot support serialization modes for those (unless you specifically program it),
    // so you could get errors for incompatible data (this is a limitation of Serde, not Compris)

    let user = User { name: "Tal".into(), enabled: true };

    utils::heading("YAML", false);
    Serializer::new(Format::YAML).with_pretty(true).print(&user).expect("print");
}
