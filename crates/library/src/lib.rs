// https://stackoverflow.com/a/61417700
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![warn(missing_docs)]

/*!
A Rust library to work with CPS (Composite Primitive Schema) data and parse it from and serialize
it to several binary and textual representation formats.

What is CPS? It's the implicit data schema underlying JSON and many other representation formats.
It comprises primitive data types (numbers, booleans, strings, etc.) as well as list and map
collection types. The collections allow for nested structure, hence it is "composite" (a.k.a.
"algebraic").

And yet despite being so widely used, it has been unnamed... until now. You're welcome.

CPS is sometimes glossed as "JSON", but that's misleading and ultimately unhelpful because JSON is
merely one representation format for the data, and is actually comparitively quite limited (e.g.
implementations do not often preserve the distinction between integers and floats). So instead of
saying "let's just store it as JSON", say "let's just store it as CPS", and use Compris to handle
the representation. It will allow you and your users to select from several formats at runtime.

Compris is pronounced "com-PREE". The name comes from shortening CompositePrimitiveSchema to
ComPriS.

For more information and usage examples see the
[home page](https://github.com/tliron/rust-compris).

J'ai compris!
*/

mod format;
mod macros;

/// Annotation.
pub mod annotation;

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

// /// Tags for [Debuggable](kutil_cli::debug::Debuggable).
// pub mod tag;

#[allow(unused_imports)]
pub use format::*;
