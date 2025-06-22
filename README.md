*Pre-release: ready for general use but the APIs are still in flux*

[![crates.io](https://img.shields.io/crates/v/compris?color=%23227700)](https://crates.io/crates/compris)
[![docs.rs](https://img.shields.io/badge/docs.rs-latest?color=grey)](https://docs.rs/compris/latest/compris/)

Compris
=======

A Rust library to work with CPS (Composite Primitive Schema) data and parse it from and serialize it to several binary and textual representation formats.

What is CPS? It's the implicit data schema underlying JSON and many other representation formats. It comprises primitive data types (numbers, booleans, strings, etc.) as well as list and map collection types. The collections allow for nested structure, hence it is "composite" (a.k.a. "algebraic").

And yet despite being so widely used, it has been unnamed... until now. You're welcome.

CPS is sometimes glossed as "JSON", but that's misleading and ultimately unhelpful because JSON is merely one representation format for the data, and is actually comparatively quite limited (e.g. implementations do not often preserve the distinction between integers and floats). So instead of saying "let's just store it as JSON", say "let's just store it as CPS", and use Compris to handle the representation. It will allow you and your users to select from several formats at runtime.

See [here](https://github.com/tliron/rust-compris/blob/main/CPS.md) for a full descripton of CPS.

Compris is pronounced "com-PREE". The name comes from shortening CompositePrimitiveSchema to ComPriS.

Get started with the [API documentation](https://docs.rs/compris/latest/compris/) and the [examples](https://github.com/tliron/rust-compris/tree/main/crates/library/examples).

J'ai compris!

Supported Representation Formats
--------------------------------

* [YAML](https://yaml.org/)
* [JSON](https://www.json.org/), including an "XJSON" convention for JSON to support all
  CPS types
* [XML](https://www.w3.org/XML/) via a conventional schema (*work in progress*)
* [CBOR](https://cbor.io/)
* [MessagePack](https://msgpack.org/)

All formats are enabled by default but can be turned on selectively using
[`default-features = false`](https://doc.rust-lang.org/cargo/reference/features.html#dependency-features).

Need more formats? We accept contributions and suggestions!

Compris can parse any of these formats into its "normal" value types, which provide many utility functions for convenient access and transformation of the nested data.

To an extent, the basic normal "Value" type serves as an equivalent to the "any-type" variables that are at the core of dynamically typed languages, such as Python and JavaScript. Except that in Compris it's entirely static: a simple enum with very little generics, lots of useful blanket traits, a sprinkling of macros, and absolutely no `dyn`.

The normal types also include file location information (row and column) as metadata, for referring back to the textual format sources (YAML, JSON, and XML).

The implementation relies on the [bytes](https://github.com/tokio-rs/bytes) and [bytestring](https://crates.io/crates/bytestring) libraries to ensure low-cost cloning.

[Example](https://github.com/tliron/rust-compris/blob/main/crates/library/examples/parse.rs).

Traverse
--------

Included are ergonomic facilities for accessing nested values by path and for presenting paths in a human-readable format.

[Example](https://github.com/tliron/rust-compris/blob/main/crates/library/examples/traverse.rs).

Resolve
-------

Convert the normal value types to your own types.

The API is simple but extensible, making use of a `#[derive(Resolve)]` procedural macro (with the `derive` feature) that generates resolution code for you while also allowing you to implement your own semantics with your own context and error types. Errors can provide citation information allowing for detailed and useful syntax error reports. For example, an IDE could parse the citation and highlight all the errors where they occur in the source files.

Does Compris's resolve feature sound a bit like Serde deserialization? At its simplest, it can provide the same results (and we do support Serde, too; see below). However, Compris's resolve is more flexible in that it allows for accumulating errors (instead of failing on the first error, like Serde), as well as configurable handling of nulls and undeclared fields. It even lets you to provide your own custom context type.

Compris's resolve is designed as a foundation for sophisticated CPS-based syntax parsers. You can even create your own procedural macros to generate specialized implementations that go beyond `#[derive(Resolve)]`. Our [source code](crates/macros) might help you get a grip on this challenging corner of Rust programming.

[Basic example](https://github.com/tliron/rust-compris/blob/main/crates/library/examples/resolve_basic.rs), [enum example](https://github.com/tliron/rust-compris/blob/main/crates/library/examples/resolve_enum.rs), [advanced example](https://github.com/tliron/rust-compris/blob/main/crates/library/examples/resolve_advanced.rs).

Serialization
-------------

Compris's normal value types can be serialized via [Serde](https://serde.rs/) (optional `serde` feature).

We also allow you to attach "serialization modes" that allow some control over seralization behavior. For example, `FloatSerializationMode::AsI64IfFractionless` will try to convert floats to integers when possible. This would happen *only* for serialization, on-the-fly, and does not modify your in-memory data.

Serialization modes are great for optimizing or fixing your data for limited (or broken) consumers, but they can also work around the limitations of YAML and JSON. In particular, we introduce an "XJSON" serialization mode, which allows JSON to support all of CPS. Compris can also parse and deserialize XJSON. Read more about XJSON [here](CPS.md).

We provide serializers for all supported representation formats behind a common API so that they can be selected at runtime. For the textual formats, we support pretty printing for human readability, including colorization for terminals. For the binary formats, we support optional Base64 encoding.

This general-purpose serialization API can be used with any Rust type that supports Serde's `Serialize` trait, not just our normal types. It is thus useful if your program needs to serialize to a range of different formats and you would rather use a single crate with a single API.

[Example](https://github.com/tliron/rust-compris/blob/main/crates/library/examples/serialize.rs).

Deserialization
---------------

As with serialization, we provide a common API to deserialize from all supported representation formats (optional `serde` feature).

However, there is a twist, as this is internally done in two phases. We *first* parse the format into Compris's normal value types and only then deserialize those to your types. This enables our full feature set, though that interim step can be considered inefficient.

But there is additional utility in the interim step. Often you will be working with Compris's normal values types, not the raw formats. Do you need to populate your own structs and enums from them? Instead of doing it manually, you can "deserialize" directly. No representation format is involved and no parsing is done. This feature merely uses Serde's deserialization mechanism to efficiently handle the data placement. Generally, the `resolve` feature mentioned above does the same and is more flexible, but if you're using types that already support Serde, then this will "just work".

[Example](https://github.com/tliron/rust-compris/blob/main/crates/library/examples/deserialize.rs).

CLI Tool
--------

Also included in this repository is a CLI tool for querying and converting between all supported CPS formats. It also supports reading CPS from a wide variety of URL types, via [read-url](https://github.com/tliron/rust-read-url).

To install:

```sh
cargo install compris-cli
```

Example of converting YAML from stdin to XJSON:

```sh
cat my_text.yaml | compris --input-format=yaml --format=xjson
```

References
----------

Libraries implementing the CPS concept for other languages:

* [Go](https://github.com/tliron/go-ard)
* [Python](https://github.com/tliron/python-ard)

License
-------

Like much of the Rust ecosystem, licensed under your choice of either of

* [Apache License, Version 2.0](https://github.com/tliron/rust-compris/blob/main/LICENSE-APACHE)
* [MIT license](https://github.com/tliron/rust-compris/blob/main/LICENSE-MIT)

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
