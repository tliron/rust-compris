use super::super::dyn_annotated::*;

use {kutil_cli::debug::*, std::error::*};

//
// DynAnnotatedError
//

/// A [DynAnnotated] [Error].
pub trait DynAnnotatedError: DynAnnotated + DynDebuggable + Error {}

//
// CapturedAnnotatedError
//

/// A thread-safe [DynAnnotatedError].
pub type CapturedAnnotatedError = Box<dyn DynAnnotatedError + Send + Sync>;

//
// BoxedAnnotatedError
//

/// A non-thread-safe (less constrained) version of [CapturedAnnotatedError].
pub type BoxedAnnotatedError = Box<dyn DynAnnotatedError>;
