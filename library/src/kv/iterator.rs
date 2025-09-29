use super::super::normal::*;

//
// KeyValuePairIterator
//

/// Iterator of key-value pairs.
pub trait KeyValuePairIterator<AnnotatedT> {
    /// Next.
    ///
    /// Important: An error returned here does *not* mean that there are no more entries, just that
    /// the current iteration caused an error. Future ones might not. To exhaust the iterator, keep
    /// calling this function until it returns [None].
    ///
    /// Also note that the [Err] here is a tuple of the actual error with the value that caused it,
    /// so you likely won't be able to use the `?` operator directly on the result.
    fn next(
        &mut self,
    ) -> Result<Option<(&Variant<AnnotatedT>, &Variant<AnnotatedT>)>, (MalformedError<AnnotatedT>, &Variant<AnnotatedT>)>;
}

//
// IntoKeyValuePairIterator
//

/// Iterator of key-value pairs.
pub trait IntoKeyValuePairIterator<AnnotatedT> {
    /// Next.
    ///
    /// Important: An error returned here does *not* mean that there are no more entries, just that
    /// the current iteration caused an error. Future ones might not. To exhaust the iterator, keep
    /// calling this function until it returns [None].
    ///
    /// Also note that the [Err] here is a tuple of the actual error with the value that caused it,
    /// so you likely won't be able to use the `?` operator directly on the result.
    fn next(
        &mut self,
    ) -> Result<Option<(Variant<AnnotatedT>, Variant<AnnotatedT>)>, (MalformedError<AnnotatedT>, Variant<AnnotatedT>)>;
}
