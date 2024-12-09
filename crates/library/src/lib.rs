// https://stackoverflow.com/a/61417700
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![warn(missing_docs)]

/*!
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

For more information and usage examples see the
[home page](https://github.com/tliron/rust-compris).

J'ai compris!
*/

/// General-purpose serde deserialization plus support for normal value types.
#[cfg(feature = "serde")]
pub mod de;
/// Read from various formats into normal value types.
pub mod read;
/// General-purpose serde serialization plus support for normal value types.
#[cfg(feature = "serde")]
pub mod ser;

mod format;
mod hints;
mod normal;
mod styles;
mod write_debug;

#[allow(unused_imports)]
pub use {format::*, hints::*, normal::*, styles::*, write_debug::*};
