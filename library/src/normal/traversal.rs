use super::variant::*;

//
// Traversal
//

/// Traversal path.
pub type Traversal<AnnotatedT> = Vec<Variant<AnnotatedT>>;

/// To [Traversal].
///
/// See [Variant::traverse](super::variant::Variant::traverse).
pub fn to_traversal<AnnotatedT>(ref_traversal: RefTraversal<AnnotatedT>) -> Traversal<AnnotatedT>
where
    AnnotatedT: Clone,
{
    ref_traversal.iter().map(|variant| (*variant).clone()).collect()
}

//
// RefTraversal
//

/// [Traversal] using references.
///
/// Can be converted into a [Traversal] using [to_traversal].
pub type RefTraversal<'own, AnnotatedT> = Vec<&'own Variant<AnnotatedT>>;

/// Convert to a [RefTraversal].
///
/// If it's already a [List](super::list::List) will just make sure it's not empty. Other variant
/// types will be wrapped in a [List](super::list::List).
pub fn to_ref_traversal<AnnotatedT>(variant: &Variant<AnnotatedT>) -> Option<RefTraversal<'_, AnnotatedT>> {
    match variant {
        Variant::List(list) => {
            if !list.inner.is_empty() {
                return Some(list.inner.iter().collect());
            }
        }

        _ => {
            return Some(vec![variant]);
        }
    }

    None
}
