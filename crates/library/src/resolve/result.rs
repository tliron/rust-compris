use super::resolve_error::*;

//
// ResolveResult
//

/// Resolve result.
pub type ResolveResult<T, E> = Result<Option<T>, E>;

//
// CommonResolveResult
//

/// Resolve result with [CommonResolveError].
pub type CommonResolveResult<T> = ResolveResult<T, CommonResolveError>;
