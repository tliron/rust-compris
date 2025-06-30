/*!
A lot like [TryFrom], except that:

1. We can accumulate errors
2. We can provide a context
*/

mod common;
mod errors;
mod iterator;
mod parser;
mod resolve;

#[allow(unused_imports)]
pub use {common::*, errors::*, iterator::*, parser::*, resolve::*};

#[cfg(feature = "derive")]
#[allow(unused_imports)]
pub use compris_macros::Resolve;
