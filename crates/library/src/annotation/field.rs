use super::annotations::*;

use {bytestring::*, kutil_std::collections::*};

//
// FieldAnnotations
//

/// Field [Annotations].
pub type FieldAnnotations = FastHashMap<ByteString, Annotations>;

//
// AnnotatedFields
//

/// Has [FieldAnnotations].
pub trait AnnotatedFields {
    /// Get field [Annotations].
    fn get_field_annotations(&self, name: &str) -> Option<&Annotations>;
}
