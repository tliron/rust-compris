use super::{super::normal::*, errors::*};

//
// ResolveError
//

/// Resolve errors must be able to at least convert from several essential errors.
///
/// Ideally, they would also implement [Citable](super::super::cite::Citable) and
/// [Debuggable](kutil_cli::debug::Debuggable).
///
/// See source code for [CommonResolveError](super::common::CommonResolveError) as an example
/// of how to implement your own.
pub trait ResolveError:
    From<IncompatibleValueTypeError>
    + From<ConversionError>
    + From<MalformedError>
    + From<MissingRequiredKeyError>
    + From<InvalidKeyError>
{
}
