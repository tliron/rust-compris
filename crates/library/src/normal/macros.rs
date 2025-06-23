/// Helper macro for implementing normal types.
#[macro_export]
macro_rules! impl_normal (
    ( $(#[$meta:meta])* $type:ident ( $value:ty ) ) => {
        $(#[$meta])*
        #[derive(Debug)]
        pub struct $type<AnnotationsT> {
            /// Actual value.
            pub value: $value,

            /// Annotations.
            pub annotations: AnnotationsT,
        }

        impl<AnnotationsT> $type<AnnotationsT> {
            /// Constructor.
            pub fn new(value: $value) -> Self
            where
                AnnotationsT: Default,
            {
                Self { value, annotations: AnnotationsT::default() }
            }
        }

        impl<AnnotationsT> Annotated for $type<AnnotationsT>
        where
            AnnotationsT: Annotated,
        {
            fn is_annotated() -> bool {
                AnnotationsT::is_annotated()
            }

            fn get_annotations(&self) -> Option<&Annotations> {
                self.annotations.get_annotations()
            }

            fn get_annotations_mut(&mut self) -> Option<&mut Annotations> {
                self.annotations.get_annotations_mut()
            }

            fn set_annotations(&mut self, annotations: Annotations) {
                self.annotations.set_annotations(annotations);
            }
        }

        impl<AnnotationsT> Clone for $type<AnnotationsT>
        where
            AnnotationsT: Clone,
        {
            fn clone(&self) -> Self {
                Self { value: self.value.clone(), annotations: self.annotations.clone() }
            }
        }

        impl<AnnotationsT> Default for $type<AnnotationsT>
        where
            AnnotationsT: Default,
        {
            fn default() -> Self {
                Self { value: <$value>::default(), annotations: AnnotationsT::default() }
            }
        }

        impl<AnnotationsT> std::cmp::PartialEq for $type<AnnotationsT> {
            fn eq(&self, other: &Self) -> bool {
                self.value.eq(&other.value)
            }
        }

        impl<AnnotationsT> std::cmp::Eq for $type<AnnotationsT> {}

        impl<AnnotationsT> std::cmp::PartialOrd for $type<AnnotationsT> {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                self.value.partial_cmp(&other.value)
            }
        }

        impl<AnnotationsT> std::cmp::Ord for $type<AnnotationsT> {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.value.cmp(&other.value)
            }
        }

        impl<AnnotationsT> std::hash::Hash for $type<AnnotationsT> {
            fn hash<HasherT>(&self, state: &mut HasherT)
            where
                HasherT: std::hash::Hasher,
            {
                self.value.hash(state);
            }
        }
    }
);

/// Helper macro for implementing normal types.
#[macro_export]
macro_rules! impl_normal_basic (
    ( $type:ident ) => {
        impl<AnnotationsT> $type<AnnotationsT> {
            /// Removes all [Annotations].
            pub fn without_annotations(self) -> $type<WithoutAnnotations> {
                $type::new(self.value)
            }

            /// Into different annotations.
            pub fn into_annotated<NewAnnotationsT>(self) -> $type<NewAnnotationsT>
            where
                AnnotationsT: Annotated,
                NewAnnotationsT: Annotated + Default,
            {
                let new_self = $type::new(self.value);
                if AnnotationsT::is_annotated()
                    && NewAnnotationsT::is_annotated()
                    && let Some(annotations) = self.annotations.get_annotations()
                {
                    new_self.with_annotations(annotations.clone())
                } else {
                    new_self
                }
            }
        }
    }
);
