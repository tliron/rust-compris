use super::resolve::*;

use kutil_std::error::*;

//
// ResolveResult
//

/// Resolve result.
pub type ResolveResult<ResolvedT, AnnotatedT> = Result<Option<ResolvedT>, ResolveError<AnnotatedT>>;

//
// ResolveErrors
//

/// Resolve [Errors].
pub type ResolveErrors<AnnotatedT> = Errors<ResolveError<AnnotatedT>>;
