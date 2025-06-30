/// Helper macro for implementing normal types.
#[macro_export]
macro_rules! impl_normal (
    ( $(#[$meta:meta])* $type:ident ( $inner:ty ) ) => {
        $(#[$meta])*
        #[derive(::std::fmt::Debug)]
        pub struct $type<AnnotatedT> {
            /// Inner.
            pub inner: $inner,

            /// Annotated.
            pub annotated: AnnotatedT,
        }

        impl<AnnotatedT> $type<AnnotatedT> {
            /// Constructor.
            pub fn new(inner: $inner) -> Self
            where
                AnnotatedT: ::std::default::Default,
            {
                Self { inner, annotated: AnnotatedT::default() }
            }
        }

        $crate::impl_annotated!($type);

        impl<AnnotatedT> ::std::clone::Clone for $type<AnnotatedT>
        where
            AnnotatedT: ::std::clone::Clone,
        {
            fn clone(&self) -> Self {
                Self { inner: self.inner.clone(), annotated: self.annotated.clone() }
            }
        }

        impl<AnnotatedT> ::std::default::Default for $type<AnnotatedT>
        where
            AnnotatedT: ::std::default::Default,
        {
            fn default() -> Self {
                Self { inner: <$inner>::default(), annotated: AnnotatedT::default() }
            }
        }

        impl<AnnotatedT> ::std::cmp::PartialEq for $type<AnnotatedT> {
            fn eq(&self, other: &Self) -> bool {
                self.inner.eq(&other.inner)
            }
        }

        impl<AnnotatedT> ::std::cmp::Eq for $type<AnnotatedT> {}

        impl<AnnotatedT> ::std::cmp::PartialOrd for $type<AnnotatedT> {
            fn partial_cmp(&self, other: &Self) -> ::std::option::Option<::std::cmp::Ordering> {
                self.inner.partial_cmp(&other.inner)
            }
        }

        impl<AnnotatedT> ::std::cmp::Ord for $type<AnnotatedT> {
            fn cmp(&self, other: &Self) -> ::std::cmp::Ordering {
                self.inner.cmp(&other.inner)
            }
        }

        impl<AnnotatedT> ::std::hash::Hash for $type<AnnotatedT> {
            fn hash<HasherT>(&self, state: &mut HasherT)
            where
                HasherT: ::std::hash::Hasher,
            {
                self.inner.hash(state);
            }
        }
    }
);

/// Helper macro for implementing normal types.
#[macro_export]
macro_rules! impl_normal_basic (
    ( $type:ident ) => {
        impl<AnnotatedT> $type<AnnotatedT> {
            /// Remove all [Annotations].
            pub fn without_annotations(self) -> $type<WithoutAnnotations> {
                $type::new(self.inner)
            }

            /// Into different [Annotated] implementation.
            pub fn into_annotated<NewAnnotationsT>(self) -> $type<NewAnnotationsT>
            where
                AnnotatedT: Annotated,
                NewAnnotationsT: Annotated + Default,
            {
                let new_self = $type::new(self.inner);
                if AnnotatedT::is_annotated()
                    && NewAnnotationsT::is_annotated()
                    && let Some(annotations) = self.annotated.get_annotations()
                {
                    new_self.with_annotations(annotations.clone())
                } else {
                    new_self
                }
            }
        }
    }
);
