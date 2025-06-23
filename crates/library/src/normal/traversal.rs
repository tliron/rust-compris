use super::value::*;

//
// Traversal
//

/// Traversal path.
pub type Traversal<AnnotationsT> = Vec<Value<AnnotationsT>>;

/// To [Traversal].
///
/// See [Value::traverse](super::value::Value::traverse).
pub fn to_traversal<AnnotationsT>(ref_traversal: RefTraversal<AnnotationsT>) -> Traversal<AnnotationsT>
where
    AnnotationsT: Clone,
{
    ref_traversal.iter().map(|value| (*value).clone()).collect()
}

//
// RefTraversal
//

/// [Traversal] using references.
///
/// Can be converted into a [Traversal] using [to_traversal].
pub type RefTraversal<'own, AnnotationsT> = Vec<&'own Value<AnnotationsT>>;

/// Convert to a [RefTraversal].
///
/// If it's already a [List](super::list::List) will just make sure it's not empty. Other value
/// types will be wrapped in a [List](super::list::List).
pub fn to_ref_traversal<AnnotationsT>(value: &Value<AnnotationsT>) -> Option<RefTraversal<'_, AnnotationsT>> {
    match value {
        Value::List(list) => {
            if !list.value.is_empty() {
                return Some(list.value.iter().collect());
            }
        }

        _ => {
            return Some(vec![value]);
        }
    }

    None
}
