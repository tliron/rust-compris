use super::annotations::*;

use kutil::std::{collections::*, immutable::*};

//
// StructAnnotations
//

/// Struct [Annotations].
pub type StructAnnotations = FastHashMap<ByteString, Annotations>;

//
// AnnotatedStruct
//

/// Has [Annotations] for the struct and its fields.
pub trait AnnotatedStruct {
    /// A field's [Annotations].
    ///
    /// An empty name is used to refer to annotations for the struct itself.
    fn field_annotations(&self, name: &str) -> Option<&Annotations>;

    /// The struct's [Annotations].
    fn struct_annotations(&self) -> Option<&Annotations> {
        self.field_annotations("")
    }
}
