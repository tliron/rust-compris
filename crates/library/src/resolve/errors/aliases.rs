use super::resolve::*;

use kutil_std::error::*;

//
// ResolveResult
//

/// Resolve result.
pub type ResolveResult<ResolvedT, AnnotationsT> = Result<Option<ResolvedT>, ResolveError<AnnotationsT>>;

//
// ResolveErrors
//

/// Resolve [Errors].
pub type ResolveErrors<AnnotationsT> = Errors<ResolveError<AnnotationsT>>;
