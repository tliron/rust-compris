// https://stackoverflow.com/a/61417700
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![warn(missing_docs)]

/*!
A Rust library to work with CPS (Composite Primitive Schema) data and parse it from and serialize
it to several binary and textual representation formats, such as YAML, JSON, and CBOR.

A useful side effect of this bi-direction is that Compris can be used to convert between these
formats.

What is CPS? It's the implicit, common data schema underlying these representation formats. It
comprises primitive data types (numbers, booleans, strings, etc.) as well as list and map
collection types, which enable a nested (recursive) structure. Hence it is "composite" (a.k.a.
"algebraic").

And yet despite being so widely used, CPS has been unnamed... until now. You're welcome.

CPS is sometimes glossed as "JSON", but that's misleading and ultimately unhelpful because JSON is
merely one representation format for the data, and is actually comparatively quite limited (e.g.
implementations do not often preserve the distinction between integers and floats). So instead of
saying "let's just store it as JSON", say "let's just store it as CPS", and use Compris to handle
the representation. It will allow you and your users to select from all supported formats at
runtime.

Compris is pronounced "com-PREE". The name comes from shortening CompositePrimitiveSchema to
ComPriS.

For more information and usage examples see the
[home page](https://github.com/tliron/rust-compris).

J'ai compris!
*/

mod format;
mod macros;

/// Annotate any type.
pub mod annotate;

/// General-purpose serde deserialization. Also supports normal types.
#[cfg(feature = "serde")]
pub mod de;

/// Hints for extending representation formats (such as XJSON).
pub mod hints;

/// Iterate key-value pairs.
pub mod kv;

/// Merging.
pub mod merge;

/// Normal types.
pub mod normal;

/// Parse various formats into normal types.
pub mod parse;

/// Path.
pub mod path;

/// Resolve normal types into other types.
pub mod resolve;

/// General-purpose serde serialization plus support for normal types.
#[cfg(feature = "serde")]
pub mod ser;

#[allow(unused_imports)]
pub use format::*;
