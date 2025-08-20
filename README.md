*Pre-release: ready for general use but the APIs are still in flux*

[![crates.io](https://img.shields.io/crates/v/compris?color=%23227700)](https://crates.io/crates/compris)
[![docs.rs](https://img.shields.io/badge/docs.rs-latest?color=grey)](https://docs.rs/compris/latest/compris/)

Compris
=======

A Rust library to work with CPS (Composite Primitive Schema) data and parse it from and serialize it to several binary and textual representation formats, such as YAML, JSON, and CBOR.

A useful side effect of this bi-direction is that Compris can be used to convert between these formats.

What is CPS? It's the implicit, common data schema underlying these representation formats. It comprises primitive data types (numbers, booleans, strings, etc.) as well as list and map collection types, which enable a nested (recursive) structure. Hence it is "composite" (a.k.a. "algebraic").

And yet despite being so widely used, CPS has been unnamed... until now. You're welcome.

CPS is sometimes glossed as "JSON", but that's misleading and ultimately unhelpful because JSON is merely one representation format for the data, and is actually comparatively quite limited (e.g. implementations do not often preserve the distinction between integers and floats). So instead of saying "let's just store it as JSON", say "let's just store it as CPS", and use Compris to handle the representation. It will allow you and your users to select from all supported formats at runtime.

See [here](https://github.com/tliron/compris/blob/main/CPS.md) for a full description of CPS.

Compris is pronounced "com-PREE". The name comes from shortening CompositePrimitiveSchema to ComPriS.

Get started with the [API documentation](https://docs.rs/compris/latest/compris/) and the [examples](https://github.com/tliron/compris/tree/main/crates/library/examples).

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

Parsing and Normal Types
------------------------

Compris can parse any of these formats into its "normal" `Variant`, which provides many utility functions for convenient access and transformation of nested data.

The normal variant type serves as an equivalent to the "any-type" variables that are at the core of dynamically typed languages, such as Python and JavaScript. Except that in Compris it's entirely static: a simple enum with lots of useful blanket traits, a sprinkling of helper macros, and absolutely no `dyn`. Furthermore note that blobs and strings rely on the [bytes](https://github.com/tokio-rs/bytes) and [bytestring](https://crates.io/crates/bytestring) libraries to ensure low-cost cloning in high-throughput scenarios, such as server request handling.

Each normal variant can also include "annotations", including the source filename and span in the file (row and column), which can be used for citing textual sources (YAML, JSON, and XML). This allows Compris to provide very detailed error messages for higher-level grammars, IDEs, etc. The annotations feature is enabled via a generic parameter to avoid paying for it when not needed.

[Example](https://github.com/tliron/compris/blob/main/crates/library/examples/parse.rs).

Path Traversal
--------------

Included are ergonomic facilities for accessing nested variants by path and for presenting paths in a human-readable format.

[Example](https://github.com/tliron/compris/blob/main/crates/library/examples/traverse.rs).

Resolving
---------

Compris can convert (we call this "resolve") the normal types to your own custom types.

The API is simple but extensible, making use of a `#[derive(Resolve)]` procedural macro (with the `derive` feature) that generates the resolving code for your structs and enums while also allowing you to implement your own semantics and validations. This feature is designed to serve as a foundation for sophisticated CPS-based syntax parsers.

[Basic example](https://github.com/tliron/compris/blob/main/crates/library/examples/resolve_basic.rs), [enum example](https://github.com/tliron/compris/blob/main/crates/library/examples/resolve_enum.rs), [advanced example](https://github.com/tliron/compris/blob/main/crates/library/examples/resolve_advanced.rs).

> Does the resolve feature sound a bit like Serde deserialization? At its simplest, they both provide the same result (and Compris does support Serde, too; see below). However, resolve is far more straightforward and efficient in that it is designed for CPS. Per type, you only need to implement one function in one trait. Furthermore, it allows for accumulating annotated errors (instead of always failing on the first error, like Serde), as well as configurable handling of nulls and undeclared fields.

Serde Serialization
-------------------

Compris provides a common serializer API for [Serde](https://serde.rs/) (with the `serde` feature), which allows the format to be selected at runtime. For the textual formats, Compris also supports pretty printing for human readability, including colorization for terminals. For the binary formats, Compris supports optional Base64 encoding.

This API additionally supports ["serialization modes"](https://docs.rs/compris/latest/compris/ser/struct.SerializationMode.html) that allow some control over serialization behavior. For example, `FloatSerializationMode::AsI64IfWhole` will try to convert floats to integers if they are whole numbers. This would happen *only* for serialization, on-the-fly, and does not modify your in-memory data.

Serialization modes are useful for optimizing or fixing your data for limited (or broken) consumers, but they can also work around the limitations of YAML and JSON. In particular, Compris introduces an "XJSON" serialization mode, which allows JSON to support all of CPS via standard "hints". Compris can also parse and deserialize XJSON. Read more about XJSON [here](https://github.com/tliron/compris/blob/main/CPS.md#xjson).

This general-purpose serialization API can be used with any Rust type that supports Serde's `Serialize` trait, not only Compris normal types. It is thus useful if your program needs to serialize to a range of different formats and you would rather use a single crate with a single API.

[Example](https://github.com/tliron/compris/blob/main/crates/library/examples/serialize.rs).

Serde Deserialization
---------------------

As with serialization, Compris provides a common API to deserialize from all supported representation formats (optional `serde` feature).

However, there is a twist, as this is internally done in two phases. Compris *first* parses the format into normal types and only then deserializes them to your `Deserialize` types. This enables the full feature set of Compris.

For example, if you want to feed a `Deserialize` type with data, but don't want to have to through a representation format and a parser, then you can model the data using Compris normal types. The "resolve" feature (see above) can do this, too, and should generally be more efficient than going through Serde, but if you're using types that already support Serde `Deserialize` then this will "just work".

[Example](https://github.com/tliron/compris/blob/main/crates/library/examples/deserialize.rs).

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

* [Apache License, Version 2.0](https://github.com/tliron/compris/blob/main/LICENSE-APACHE)
* [MIT license](https://github.com/tliron/compris/blob/main/LICENSE-MIT)

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
