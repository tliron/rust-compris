*Work in progress, not ready for general use*

Compris
=======

A Rust library to access, change, read, and write CPS (Composite Primitive Schema) data in
several textual and binary representation formats.

What's CPS? It's that schema that comprises primitive data types (numbers, booleans, strings,
etc.) as well as list and map collection types. The collections allow for nested structure,
hence it is a composite schema. This concept is very widely used but has remarkably been
unnamed... until now. You're welcome.

CPS is sometimes glossed it as "JSON", but that's misleading and ultimately unhelpful because
JSON is merely one representation format for the data, and is actually comparitively quite
limited (e.g. implementations do not often preserve the distinction between integers and
floats). So instead of saying "let's just store it as JSON", say "let's just store it as CPS",
and use Compris to handle the representation. It will allow you and your users to select from
several formats at runtime.

Compris is pronounced "com-PREE". It comes from CompositePrimitiveSchema, or ComPriS for short.

See [here](CPS.md) for a full descripton of CPS.

And check out the [examples](crates/library/examples/).

J'ai compris!

Supported Representation Formats
--------------------------------

* [YAML](https://yaml.org/)
* [JSON](https://www.json.org/), including an "XJSON" convention for JSON to support all
  CPS types
* [XML](https://www.w3.org/XML/) via a conventional schema (work in progress)
* [CBOR](https://cbor.io/)
* [MessagePack](https://msgpack.org/)

Compris can read any of these formats into its "normal" value types, which provide many
functions for convenient access and transformation of the nested data.

The types also include file location information (row and column) for debugging textual
format sources (YAML, JSON, and XML).

All formats are enabled by default but can be turned on selectively using
[`default-features = false`](https://doc.rust-lang.org/cargo/reference/features.html#dependency-features).

Need more formats? We accept contributions and suggestions!

Serialization
-------------

Compris's normal value types can be serialized via [serde](https://serde.rs/) (optional
`serde` feature).

The feature also supports "serialization modes" that allow some control over seralization
behavior. For example, `FloatSerializationMode::AsIntegerIfFractionless` will try to convert
floats to integers when possible. This would happen *only* during serialization, on-the-fly,
and does not modify your in-memory data.

We additionally provide serializers for all supported representation formats behind a common
API so that they can be selected at runtime. For the textual formats, we support pretty printing
for human readability, including colorization for terminals. For the binary formats, we support
Base64 encoding.

Actually, this common serialization API can be used with any Rust type that supports serde
serialization, not just our normal types. It is thus useful if your program needs to serialize
to a range of different formats and you would rather use a single crate with a single API.

CLI Tool
--------

Also included in this repsository is a CLI tool for querying and converting any CPS
format. To install:

```sh
cargo install compris-cli
```

Example of converting YAML in stdin to XJSON:

```sh
cat my_text.yaml | compris --input-format=yaml --format=xjson
```

References
----------

Libraries implementing this concept for other languages:

* [Go](https://github.com/tliron/go-ard)
* [Python](https://github.com/tliron/python-ard)

License
-------

Like much of the Rust ecosystem, licensed under your choice of either of

* [Apache License, Version 2.0](LICENSE-APACHE)
* [MIT license](LICENSE-MIT)

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
