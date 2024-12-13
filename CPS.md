The Composite Primitive Schema (CPS)
====================================

We'll define CPS to contain these primitive data types:

* **null** (a singleton)
* **signed integer** (up to 64 bits)
* **unsigned integer** (up to 64 bits)
* **float** ([IEEE 754](https://en.wikipedia.org/wiki/IEEE_754), up to 64 bits)
* **boolean** (single bit)
* **string** (any encoding)
* **byte array**

As well as two recursive collection types:

* **list** (ordered sequence of zero or more elements of any type, including collections)
* **map** (list of key-value pairs in an undefined order; keys and values of any type, including
  collections; keys are unique for the map)

The power of such a purposely reductive definition is its ubiquity. The above primitives and
collection types are present in practically any programming language, can be stored in practically
any database, and can also be transmitted in a wide variety of broadly-supported representation
formats.

Importantly, you do not have to support 100% of CPS in your use of it. Rather, treat it as the outer
limit of possibility. Going beyond CPS can lead to compatibility challenges, but by avoiding
some of its features you can increase compatibility.

We'll detail some of these challenges and workarounds below. But first let's highlight two potential
pain points:

### Pain Point #1: That Stubborn Null

The inclusion of **null** in this definition may be controversial
(its [inventor regrets it](https://www.infoq.com/presentations/Null-References-The-Billion-Dollar-Mistake-Tony-Hoare/))
but as it stands it's unapologetically present in all CPS representation formats. And it's popular.

But please be careful with it. As the only singleton value in CPS, it is often used and abused for
a variety of confusing and conflicting semantic purposes. Examples include signifying "nothing" for
optional values, where it's not obvious whether or what default value would be assigned instead,
nor whether or how assigning **null** is different from not assigning a value at all. Another common
use is that it's a catch-all for the expected type's zero value (0 for numbers, empty string for
strings, empty list for lists, false for booleans, etc.). Zero values are, emphatically, *not*
"nothing". Finally, and worst, we've seen it used as a magical value that results in very specific
behavior that cannot be reasonably inferred in context. It seems easy to reach for it as a solution
because it's available.

The bottom line is that it's very likely that **null** may not be doing what the user thinks it should
be doing—which is how bugs happen. If you insist on allowing **null** in your use of CPS, make sure
to carefully document the rationale and effect of its use per instance.

### Pain Point #2: Composite Map Keys

Another controversial decision is our allowance for **map** keys to be *any* value, including
recursively composite values. Here we deliberately chose flexibility over ubiquity. Unfortunately,
composite keys can present a challenge for some representation formats and even some programming
languages. Indeed, in many cases built-in hashmaps only allow keys to be strings. (Looking at you,
JavaScript.)

There are workarounds, but they are non-trivial. Again, you do not have to support 100% of CPS and
can simply decide to not make use of composite keys if they cause more trouble than they are worth.
And think twice: maybe you don't even need them.


CPS and Representation Formats
------------------------------

Not all formats can do it all, so be sure to pick the right ones for your use case, and be
aware of necessary workarounds for others.

### CBOR and MessagePack

Both [CBOR](https://cbor.io/) and [MessagePack](https://msgpack.org/) support all of CPS
(and more). They are not human-readable, but have the advantage of using less RAM, bandwidth,
and compute power to parse and serialize than the textual formats.

They are both highly recommended for machine-to-machine conversations. If you're using a
textual format for that, stop. It's easy enough to convert them to text when necessary
(Compris can do it for you).

Which to choose? Due to some unfortunate drama, CBOR is a (non-enforced)
[IETF standard](https://datatracker.ietf.org/doc/html/rfc8949) while MessagePack is not.
CBOR also supports arbitrary-length streams, a feature with dubious applicability.
MessagePack is a bit more complex, but that complexity allows for potentially smaller
payloads. Both work great for CPS.

### YAML

It's best to dismiss versions of YAML before 1.2, because they had ambiguities and too many
optional features.

YAML 1.2, when including the common [JSON schema](https://yaml.org/spec/1.2/spec.html#id2803231)),
supports *most* of CPS.

It does lack a distinction between signed and unsigned integers. If you need full 64-bit
unsigned integers, which cannot be guaranteed casting to signed integers, then it might be
best to encode them as string representations, e.g. in decimal or (smaller) hex.

It also does not support byte arrays, though note that YAML 1.1 did draft a
[`!!binary`](https://yaml.org/type/binary.html) type. Compris can be configured to support
it, but other implementations may not, so do be careful with it. A common workaround is to
encode byte arrays as Base64 strings, which is Compris's default serialization mode for
**bytes** in YAML.

By the way, YAML with the JSON schema is a superset of JSON. That means any YAML parser is
also a JSON parser! If you need to parse both and are trying to conserve code, you can
disable JSON parsing and simply treat JSON as YAML, though note that a dedicated JSON parser
may be more efficient.

### JSON

Though it's the most popular format, it's also the most limited.

Firstly, it does not distinguish between any number type. Parsers handle this challenge in
a variety of ways, from just assuming they are all floats (bad!) to always encoding them as
decimal strings (wasteful). Compris gives you some control over how numbers are parsed.

Secondly, JSON map keys must be strings. By default Compris will stringify all JSON keys into
a JSON representation, which can then be parsed. JSON in JSON! However, Compris also has
a serialization mode for serializing maps as lists of key-value lists, and can even do so
conditionally, only when a map has a non-string key.

Finally, JSON does not support byte arrays. By default Compris will serialize them as Base64
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

Byte arrays are Base64-encoded strings. Here we use bytes as a value in a map:

```json
{"binary content": {"$hint.bytes": "SGVsbG8sIHdvcmxk"}}
```

Maps are arrays in which each element is a map entry, itself an array of the key and value
(always length 2). Here we combine it with other hints:

```json
"$hint.map": [
    ["simple key", "simple value"],
    [{"complex key1": "complex value1", "complex key2": "complex value2"}, {"$hint.int": "3"}],
]
```

Note that hinted maps *can* contain duplicate keys (later ones will override previous ones).
We allow this behavior because JSON maps do, too, in most implementations.

For the edge case in which you actually have a single-key map with one of these hints as a key,
double the `$` sign to escape:

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

### Rust

Compris's normal value types all support
[`Hash`](https://doc.rust-lang.org/beta/std/hash/trait.Hash.html) as well as other
trait requirements for map keys, so they can be used in practically any generics-based map
implementation, including sorted trees.

Note that we chose [`BTreeMap`](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html)
for our normal map implementation in order to allow maps to be used in complex keys. By contrast,
[`HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html) does not
support `Hash` and does have deterministic order.

### Go

Unfortunately, the most popular Go YAML parser does not easily support arbitrarily complex keys
(see this [issue](https://github.com/go-yaml/yaml/issues/502)). We made an independent library,
[yamlkeys](https://github.com/tliron/yamlkeys), to make this easier.

### Python

Likewise, Python's popular [ruamel.yaml](https://yaml.readthedocs.io) parser does not easily
support arbitrarily complex keys. We solved this by extending ruamel.yaml in our
[Python ARD library](https://github.com/tliron/python-ard).

### JavaScript

See the discussion of JSON, above. JSON stands for "JavaScript Object Notation", so those
limitations come straight from JavaScript.

It might be nice to have JavaScript library to work with our XJSON conventions. TODO!
