/// Helper macro for implementing [Annotated](super::annotated::Annotated).
#[macro_export]
macro_rules! impl_annotated (
    ( $type:ident $(,)? ) => {
        $crate::impl_annotated!( $type, annotated );
    };

    ( $type:ident, $field:ident $(,)? ) => {
        impl<AnnotatedT> $crate::annotate::Annotated for $type<AnnotatedT>
        where
            AnnotatedT: $crate::annotate::Annotated,
        {
            fn has_annotations() -> bool {
                AnnotatedT::has_annotations()
            }

            fn get_annotations(&self) -> ::std::option::Option<&$crate::annotate::Annotations> {
                self.$field.get_annotations()
            }

            fn get_annotations_mut(&mut self) -> ::std::option::Option<&mut $crate::annotate::Annotations> {
                self.$field.get_annotations_mut()
            }

            fn set_annotations(&mut self, annotations: $crate::annotate::Annotations) {
                self.$field.set_annotations(annotations);
            }
        }
    }
);

/// Helper macro for implementing [DynAnnotatedError](super::errors::DynAnnotatedError).
#[macro_export]
macro_rules! impl_dyn_annotated_error (
    ( $type:ident $(,)? ) => {
        $crate::impl_dyn_annotated_error!( $type, annotated );
    };

    ( $type:ident, $field:ident $(,)? ) => {
        $crate::impl_annotated!($type, $field);

        impl<AnnotatedT> $type<AnnotatedT> {
            /// Captured.
            pub fn captured(self) -> $crate::annotate::CapturedAnnotatedError
            where
                AnnotatedT:
                    'static
                    + $crate::annotate::Annotated
                    + ::std::fmt::Debug
                    + ::std::marker::Send
                    + ::std::marker::Sync,
            {
                ::std::boxed::Box::new(self)
            }
        }

        impl<AnnotatedT> $crate::annotate::DynAnnotatedError for $type<AnnotatedT>
        where
            AnnotatedT:
                $crate::annotate::Annotated
                + ::std::fmt::Debug
                + ::std::marker::Send
                + ::std::marker::Sync,
        {
        }

        impl<AnnotatedT> Into<$crate::annotate::CapturedAnnotatedError> for $type<AnnotatedT>
        where
            AnnotatedT:
                'static
                + $crate::annotate::Annotated
                + ::std::fmt::Debug
                + ::std::marker::Send
                + ::std::marker::Sync,
        {
            fn into(self) -> $crate::annotate::CapturedAnnotatedError {
                self.captured()
            }
        }
    }
);
