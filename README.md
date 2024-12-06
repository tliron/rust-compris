*Work in progress, not ready for general use*

Compris: Composite Primitive Schema (CPS)
=========================================

A Rust library to work with Composite Primitive Schema (CPS) data in various formats.

What is the Composite Primitive Schema?

It refers to primitive data types (numbers, booleans, strings, etc.), with the addition
of lists and maps, allowing for nested structures (hence, it is composite). It is a concept
that is a schema that is very widely used but has remarkably remained unnamed. Until now.
You're welcome. See [here](CPS.md) for full details.

Some people gloss it as "JSON", but that's misleading and ultimately unhelpful because JSON
is merely one representation format for the data, and rather limited at that (e.g.
implementations do not always preserve the distinction between integers and floats). So
instead of saying "let's just store it in JSON", say "let's just store it in CPS", and use
Compris to do the heavy lifting, allowing you (and your users) to select from any supported
representation format.

J'ai compris!

Supported Representation Formats
--------------------------------

* [YAML](https://yaml.org/)
* [JSON](https://www.json.org/), including an "XJSON" convention for extending JSON to support all CPS types
* [XML](https://www.w3.org/XML/) via a conventional schema (work in progress)
* [CBOR](https://cbor.io/)
* [MessagePack](https://msgpack.org/)

Compris can read any of these formats into normalized Rust data types. This saves you from having
to deal with different types from different parsing libraries. These types also include source
location information (column and row) for debugging.

All formats are enabled by default, but can be turned off selectively using
[`default-features = false`](https://doc.rust-lang.org/cargo/reference/features.html#dependency-features).

Need more formats? We accept contributions and suggestions!

Serialization
-------------

Our normalized types can be serialized via [serde](https://serde.rs/) (optional `serde`
feature). We support pretty printing for human readability, including colorization for
terminals.

Actually, this feature can be used with any Rust type that supports serde serialization,
not just our normalized types. It is thus useful if you just need to easily write to
various formats.

`cpsq`
------

Also included is a `cpsq` CLI tool for querying and converting any CPS format.

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
