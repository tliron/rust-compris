/*!
A lot like [TryFrom], except that:

1. We can accumulate errors
2. We can provide a context
*/

mod cite;
mod common;
mod context;
mod error;
mod errors;
mod iterator;
mod parser;
mod resolve;
mod result;

#[allow(unused_imports)]
pub use {cite::*, common::*, context::*, error::*, errors::*, iterator::*, parser::*, resolve::*, result::*};

#[cfg(feature = "derive")]
#[allow(unused_imports)]
pub use compris_macros::Resolve;
