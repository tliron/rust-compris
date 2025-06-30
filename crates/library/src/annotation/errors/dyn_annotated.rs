use super::super::dyn_annotated::*;

use {kutil_cli::debug::*, std::error::*};

//
// DynAnnotatedError
//

/// [DynAnnotated] [Error].
pub trait DynAnnotatedError: DynAnnotated + DynDebuggable + Error + Send + Sync {}

//
// CapturedAnnotatedError
//

/// Captured [DynAnnotatedError].
pub type CapturedAnnotatedError = Box<dyn DynAnnotatedError>;
