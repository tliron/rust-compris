*Work in progress, not ready for general use*

[![crates.io](https://img.shields.io/crates/v/compris?color=%23227700)](https://crates.io/crates/compris)
[![docs.rs](https://img.shields.io/badge/docs.rs-latest?color=grey)](https://docs.rs/compris/latest/compris/)

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

Get started with the [API documentation](https://docs.rs/compris/latest/compris/) and the
[examples](crates/library/examples/).

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
utility functions for convenient access and transformation of the nested data.

The types also include file location information (row and column) for debugging textual
format sources (YAML, JSON, and XML).

All formats are enabled by default but can be turned on selectively using
[`default-features = false`](https://doc.rust-lang.org/cargo/reference/features.html#dependency-features).

Need more formats? We accept contributions and suggestions!

Traversal
---------

Included are ergonomic facilities for accessing nested values by path.

[Example](crates/library/examples/traverse.rs).

Serialization
-------------

Compris's normal value types can be serialized via [serde](https://serde.rs/) (optional
`serde` feature).

But we also allow you to attach "serialization modes" that allow some control over seralization
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

[Example](crates/library/examples/serialize.rs).

Deserialization
---------------

As with serialization, we provide a common API to deserialize from all supported representation
formats (optional `serde` feature).

However, there is a twist, as this is done in two phases. Internally, we *first* read the format
into Compris's normal value types before deserializing. This enables our full feature set, though
it can be considered inefficient if you do not need these values.

But there is additional utility here. Often you will be working with Compris's normal values
types, not the raw formats. Do you need to populate your own structs and enums from them? Instead
of doing it manually, you can "deserialize" directly from there. No representation format is
involved and no parsing is done. This feature merely uses serde's deserialization mechanism to
efficiently handle the data placement.

[Example](crates/library/examples/deserialize.rs).

CLI Tool
--------

Also included in this repository is a CLI tool for querying and converting between all
supported CPS format. It also supports reading CPS from a wide variety of URL types, via
[read-url](https://github.com/tliron/rust-read-url).

To install:

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
