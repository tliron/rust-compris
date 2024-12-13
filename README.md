*Pre-release: ready for general use but the APIs are still in flux*

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

See [here](CPS.md) for a full descripton of CPS.

Compris is pronounced "com-PREE". The name comes from shortening CompositePrimitiveSchema to
ComPriS.

Get started with the [API documentation](https://docs.rs/compris/latest/compris/) and the
[examples](crates/library/examples/).

J'ai compris!

Supported Representation Formats
--------------------------------

* [YAML](https://yaml.org/)
* [JSON](https://www.json.org/), including an "XJSON" convention for JSON to support all
  CPS types
* [XML](https://www.w3.org/XML/) via a conventional schema (*work in progress*)
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

Included are ergonomic facilities for accessing nested values by path and for presenting
paths in a human-readable format.

[Example](crates/library/examples/traverse.rs).

Resolution
----------

The optional `resolve` feature allows you to "resolve" the normal value types into your own
types.

The API is simple but extensible, making use of a `#[derive(Resolve)]` procedural macro that
generates resolution code for you while also allowing you to implement your own resolution
semantics with your own context and error types. Errors can provide citation information
allowing for detailed and useful syntax error reports. For example, an IDE could parse the
citation and highlight all the errors where they occur in the source files.

Compris resolution is designed to be a foundation for sophisticated CPS-based syntax parsers. You
can even create your own procedural macros to generate specialized resolution implementations that
go beyond `#[derive(Resolve)]`. (Our source code might help you get a grip on this challenging
corner of Rust programming.)

Does Compris resolution sound a bit like Serde deserialization? At its simplest, it can provide
the same results (and we do support Serde, too; see below). However, Compris's resolution is more
flexible in that it allows for accumulating errors (instead of failing on the first error, like
Serde), as well as configurable handling of nulls, undeclared fields, and more.

[Basic example](crates/library/examples/resolve_basic.rs),
[advanced example](crates/library/examples/resolve_advanced.rs).

Serialization
-------------

Compris's normal value types can be serialized via [Serde](https://serde.rs/) (optional
`serde` feature).

But we also allow you to attach "serialization modes" that allow some control over seralization
behavior. For example, `FloatSerializationMode::AsIntegerIfFractionless` will try to convert
floats to integers when possible. This would happen *only* during serialization, on-the-fly,
and does not modify your in-memory data.

We additionally provide serializers for all supported representation formats behind a common
API so that they can be selected at runtime. For the textual formats, we support pretty printing
for human readability, including colorization for terminals. For the binary formats, we support
optional Base64 encoding.

This general-purpose serialization API can be used with any Rust type that supports Serde
serialization, not just our normal types. It is thus useful if your program needs to serialize to
a range of different formats and you would rather use a single crate with a single API.

[Example](crates/library/examples/serialize.rs).

Deserialization
---------------

As with serialization, we provide a common API to deserialize from all supported representation
formats (optional `serde` feature).

However, there is a twist, as this is interally done in two phases. We *first* read the format
into Compris's normal value types and only then deserialize those to your types. This enables our
full feature set, though that interim step can be considered inefficient.

But there is additional utility here. Often you will be working with Compris's normal values
types, not the raw formats. Do you need to populate your own structs and enums from them? Instead
of doing it manually, you can "deserialize" directly. No representation format is involved and no
parsing is done. This feature merely uses Serde's deserialization mechanism to efficiently handle
the data placement. Generally, the `resolve` feature mentioned above does the same and is more
flexible, but if you're using types that already support Serde, then this will just work.

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
