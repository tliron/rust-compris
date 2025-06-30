use super::value::*;

//
// Traversal
//

/// Traversal path.
pub type Traversal<AnnotatedT> = Vec<Value<AnnotatedT>>;

/// To [Traversal].
///
/// See [Value::traverse](super::value::Value::traverse).
pub fn to_traversal<AnnotatedT>(ref_traversal: RefTraversal<AnnotatedT>) -> Traversal<AnnotatedT>
where
    AnnotatedT: Clone,
{
    ref_traversal.iter().map(|value| (*value).clone()).collect()
}

//
// RefTraversal
//

/// [Traversal] using references.
///
/// Can be converted into a [Traversal] using [to_traversal].
pub type RefTraversal<'own, AnnotatedT> = Vec<&'own Value<AnnotatedT>>;

/// Convert to a [RefTraversal].
///
/// If it's already a [List](super::list::List) will just make sure it's not empty. Other value
/// types will be wrapped in a [List](super::list::List).
pub fn to_ref_traversal<AnnotatedT>(value: &Value<AnnotatedT>) -> Option<RefTraversal<'_, AnnotatedT>> {
    match value {
        Value::List(list) => {
            if !list.inner.is_empty() {
                return Some(list.inner.iter().collect());
            }
        }

        _ => {
            return Some(vec![value]);
        }
    }

    None
}
