/// Cast to a [Variant](super::super::normal::Variant) with
/// [Annotations](super::super::annotate::Annotations).
#[macro_export]
macro_rules! with_annotations (
    () => ( $crate::normal::Variant::Nothing<$crate::annotate::WithAnnotations> );

    ( $value:expr $(,)? ) => ( ($value) as $crate::normal::Variant<$crate::annotate::WithAnnotations> );
);

/// Cast to a [Variant](super::super::normal::Variant) without
/// [Annotations](super::super::annotate::Annotations).
#[macro_export]
macro_rules! without_annotations (
    () => ( $crate::normal::Variant::Nothing<$crate::annotate::WithoutAnnotations> );

    ( $value:expr $(,)? ) => ( ($value) as $crate::normal::Variant<$crate::annotate::WithoutAnnotations> );
);

/// Creates a [Variant](super::super::normal::Variant) from a bare primitive expression.
#[macro_export]
macro_rules! normal (
    () => ( $crate::normal::Variant::Nothing );

    ( $value:expr $(,)? ) => ( $crate::normal::Variant::from($value) );
);

/// Creates a [Variant::List](super::super::normal::Variant::List) from a sequence of bare primitive expressions.
#[macro_export]
macro_rules! normal_list (
    () => (
        $crate::normal::Variant::List(
            $crate::normal::List::default()
        )
    );

    ( $( $value:expr ),+ $(,)? ) => (
        $crate::normal::Variant::List(
            $crate::normal::List::new(
                vec![ $( $crate::normal!( $value ) ),+ ]
            )
        )
    );
);

/// Creates a [Variant::Map](super::super::normal::Variant::Map) from a sequence of key-value tuples.
#[macro_export]
macro_rules! normal_map (
    () => (
        $crate::normal::Variant::Map(
            $crate::normal::Map::default()
        )
    );

    ( $( ( $key:expr, $value:expr ) ),+ $(,)? ) => (
        $crate::normal::Variant::Map(
            $crate::normal::Map::from(
                [ $( ( $crate::normal!( $key ), $crate::normal!( $value ) ) ),+ ]
            )
        )
    );
);

/// Creates a [Vec]<[Variant](super::super::normal::Variant)> from a sequence of bare primitive expressions.
#[macro_export]
macro_rules! normal_vec (
    ( $( $value:expr ),* $(,)? ) => (
        vec![ $( $crate::normal!( $value ) ),* ]
    );
);
