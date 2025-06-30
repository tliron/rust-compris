/// Helper macro for implementing [Annotated](super::annotated::Annotated).
#[macro_export]
macro_rules! impl_annotated (
    ( $type:ident ) => {
        $crate::impl_annotated!( $type, annotated );
    };

    ( $type:ident, $field:ident ) => {
        impl<AnnotatedT> $crate::annotation::Annotated for $type<AnnotatedT>
        where
            AnnotatedT: $crate::annotation::Annotated,
        {
            fn is_annotated() -> bool {
                AnnotatedT::is_annotated()
            }

            fn get_annotations(&self) -> ::std::option::Option<&$crate::annotation::Annotations> {
                self.$field.get_annotations()
            }

            fn get_annotations_mut(&mut self) -> ::std::option::Option<&mut $crate::annotation::Annotations> {
                self.$field.get_annotations_mut()
            }

            fn set_annotations(&mut self, annotations: $crate::annotation::Annotations) {
                self.$field.set_annotations(annotations);
            }
        }
    }
);

/// Helper macro for implementing [DynAnnotated](super::dyn_annotated::DynAnnotated).
#[macro_export]
macro_rules! impl_dyn_annotated (
    ( $type:ident ) => {
        impl<AnnotatedT> $crate::annotation::DynAnnotated for $type<AnnotatedT>
        where
            AnnotatedT: $crate::annotation::Annotated,
        {
            fn get_annotations(&self) -> ::std::option::Option<&$crate::annotation::Annotations> {
                $crate::annotation::Annotated::get_annotations(self)
            }

            fn get_annotations_mut(&mut self) -> ::std::option::Option<&mut $crate::annotation::Annotations> {
                $crate::annotation::Annotated::get_annotations_mut(self)
            }

            fn set_annotations(&mut self, annotations: $crate::annotation::Annotations) {
                $crate::annotation::Annotated::set_annotations(self, annotations);
            }
        }
    }
);

/// Helper macro for implementing [DynAnnotatedError](super::errors::DynAnnotatedError).
#[macro_export]
macro_rules! impl_dyn_annotated_error (
    ( $type:ident ) => {
        $crate::impl_dyn_annotated_error!( $type, annotated );
    };

    ( $type:ident, $field:ident ) => {
        $crate::impl_annotated!($type, $field);
        $crate::impl_dyn_annotated!($type);

        impl<AnnotatedT> $type<AnnotatedT> {
            /// Captured.
            pub fn captured(self) -> $crate::annotation::CapturedAnnotatedError
            where
                AnnotatedT:
                    'static
                    + $crate::annotation::Annotated
                    + ::std::fmt::Debug
                    + ::std::marker::Send
                    + ::std::marker::Sync,
            {
                ::std::boxed::Box::new(self)
            }
        }

        impl<AnnotatedT> $crate::annotation::DynAnnotatedError for $type<AnnotatedT>
        where
            AnnotatedT:
                $crate::annotation::Annotated
                + ::std::fmt::Debug
                + ::std::marker::Send
                + ::std::marker::Sync,
        {
        }

        impl<AnnotatedT> ::kutil_cli::debug::DynDebuggable for $type<AnnotatedT> {
            fn write_debug_for(
                &self,
                mut writer: ::std::boxed::Box<&mut dyn ::std::io::Write>,
                context: &::kutil_cli::debug::DebugContext,
            ) -> ::std::io::Result<()>
            {
                ::kutil_cli::debug::Debuggable::write_debug_for(self, writer.as_mut(), context)
            }
        }

        impl<AnnotatedT> Into<$crate::annotation::CapturedAnnotatedError> for $type<AnnotatedT>
        where
            AnnotatedT:
                'static
                + $crate::annotation::Annotated
                + ::std::fmt::Debug
                + ::std::marker::Send
                + ::std::marker::Sync,
        {
            fn into(self) -> $crate::annotation::CapturedAnnotatedError {
                self.captured()
            }
        }
    }
);
