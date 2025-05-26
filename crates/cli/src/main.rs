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

mod cli;
mod convert;
mod errors;
mod run;

use run::*;

use std::process::*;

/// Main.
pub fn main() -> ExitCode {
    kutil_cli::run::run(run)
}
