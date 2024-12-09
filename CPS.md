The Composite Primitive Schema (CPS)
====================================

We'll define CPS to contain these primitive data types:

* **null** (a singleton)
* **signed integer** (up to 64 bits)
* **unsigned integer** (up to 64 bits)
* **float** (up to 64 bits)
* **boolean** (single bit)
* **string** (any encoding)
* **byte array**

As well as two collection types:

* **list** (elements of any type, including collections)
* **map** (list of key-value pairs in an undefined order; keys and values of any type, including
  collections)

The power of CPS is its universality. The above primitives and basic collection types can be
easily consumed by practically any programming language, stored in practically any database, and
can also be transmitted in a wide variety of broadly-supported formats.


Representation Formats and Limitations
--------------------------------------

Not all formats can do it all, so be sure to pick the right ones for your use case, and be
aware of necessary workarounds for others.

### CBOR and MessagePack

Both [CBOR](https://cbor.io/) and [MessagePack](https://msgpack.org/) support all of CPS
(and more). They are not human-readable, but have the advantage of using less RAM, bandwidth,
and compute power to parse and serialize than the textual formats.

### YAML

YAML 1.2, when including the common [JSON schema](https://yaml.org/spec/1.2/spec.html#id2803231)),
supports *most* of CPS.

It does lack a distinction between signed and unsigned integers. If you need full 64-bit
unsigned integers, which cannot be guaranteed casting to signed integers, then it might be
best to encode them as string representations, e.g. in decimal or hex.

It also does not support byte arrays, though note that YAML 1.1 did draft a
[`!!binary`](https://yaml.org/type/binary.html) type. Compris can be configured to support
it, but other implementations may not. A common workaround is to encode byte arrays as Base64
strings, which is Compris's default serialization mode for YAML.

### JSON

Though it's the most popular format, it's also the most limited.

First, it does not distinguish between any number type. Parsers handle this challenge in
a variety of ways, from just assuming they are all floats (bad!) to always encoding them as
decimal strings. Compris gives you some control over how numbers are parsed.

Second, JSON map keys must be strings. By default Compris will stringify all JSON keys into
a JSON representation, which can then be parse. JSON in JSON! However, Compris also has
a serialization mode for serializing maps as lists of key-value lists, and can even do so
only when a map has a non-string key.

Finally, there is not support for byte arrays. By default Compris will serialize them as Base64
strings.

### XJSON

JSON can be retrofitted to support all of CPS by introducing simple conventions. Though there
have been a few attempts to do so in the past, we find them all lacking. So here we introduce
"XJSON" (eXtended JSON).

The idea is to wrap values in a single-key map where the key is a "hint" for readers on how
to interpret the value. This results in 100% JSON. The "hint" can be handled by a low-level
parser that knows how to handle XJSON, or in high-level application code after the parser has
parsed these single-key maps.

We'll introduce the four hints via examples. Integers and unsigned integers are decimal
strings. Here's an array of two of them:

```json
[
    {"$hint.int": "-123456"},
    {"$hint.uint": "123456"}
]
```

Byte arrays are Base64-encoded strings. Here it's used as a value in a map:

```json
{"binary content": {"$hint.bytes": "SGVsbG8sIHdvcmxk"}}
```

Maps are arrays where each element is a map entry, an array of the key and value (always
length 2). Here we combine it with other hints:

```json
"$hint.map": [
    ["simple key", "simple value"],
    [{"complex key1": "complex value1", "complex key2": "complex value2"}, {"$hint.int": "3"}],
]
```

For the edge case in which you actually have single-key maps with these hints as keys, double
the `$` sign to escape:

```json
{"$$hint.int": ["anything", null, 1, 2, 3]}
```

The above should be understood as the following, raw:

```json
{"$hint.int": ["anything", null, 1, 2, 3]}
```

### XML

We just need a CPS schema for XML. TODO!


CPS and Programming Languages
-----------------------------

We must highlight our controversial decision to allow CPS map keys to be *collections*, which
indeed can can be arbitrarily nested. However, the built-in or standard hashmaps in many
programming languages do not always allow for this, at least not trivially.

### Rust

Compris's normal value types all support
[`std::hash::Hash`](https://doc.rust-lang.org/beta/std/hash/trait.Hash.html) as well as other
trait requirements for map keys, so they can be used in practically any generics-based map
implementation.

Note that we chose [`OrderMap`](https://github.com/indexmap-rs/ordermap) for our normal map
implementation. The rationale:

1. [`std::collections::HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html)
   does not support the `Hash` trait, so it's ruled out.
2. [`std::collections::BTreeMap`](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html)
   *does* support `Hash`, but has the unwanted side effect of keeping the keys sorted.
3. OrderMap's less invasive side effect is that it retains insertion order, which might actually
   be useful for certain debugging purposes (note that CPS intentionally leaves map order
   undefined).

### Go

Unfortunately, the most popular Go YAML parser does not easily support arbitrarily complex keys
(see this [issue](https://github.com/go-yaml/yaml/issues/502)). We made an independent library,
[yamlkeys](https://github.com/tliron/yamlkeys), to make this easier.

### Python

Likewise, the popular Python [ruamel.yaml](https://yaml.readthedocs.io) parser does not easily
support arbitrarily complex keys. We solved this by extending ruamel.yaml in our
[Python ARD library](https://github.com/tliron/python-ard).

### JavaScript

See the discussion of JSON, above. JSON stands for "JavaScript Object Notation", so those
limitations come from JavaScript.

It might be nice to have JavaScript library to work with our XJSON conventions. TODO!
