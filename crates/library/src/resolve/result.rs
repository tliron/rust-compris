use super::common::*;

//
// ResolveResult
//

/// Resolve result.
pub type ResolveResult<ResolvedT, ErrorT> = Result<Option<ResolvedT>, ErrorT>;

//
// CommonResolveResult
//

/// Resolve result with [CommonResolveError].
pub type CommonResolveResult<ResolvedT> = ResolveResult<ResolvedT, CommonResolveError>;
