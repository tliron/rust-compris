/// Cast to a [Value](super::super::normal::Value) with
/// [Annotations](super::super::annotation::Annotations).
#[macro_export]
macro_rules! with_annotations (
    () => ( $crate::normal::Value::Nothing<$crate::annotation::WithAnnotations> );

    ( $value:expr ) => ( ($value) as $crate::normal::Value<$crate::annotation::WithAnnotations> );
);

/// Cast to a [Value](super::super::normal::Value) without
/// [Annotations](super::super::annotation::Annotations).
#[macro_export]
macro_rules! without_annotations (
    () => ( $crate::normal::Value::Nothing<$crate::annotation::WithoutAnnotations> );

    ( $value:expr ) => ( ($value) as $crate::normal::Value<$crate::annotation::WithoutAnnotations> );
);

/// Creates a [Value](super::super::normal::Value) from a bare primitive expression.
#[macro_export]
macro_rules! normal (
    () => ( $crate::normal::Value::Nothing );

    ( $value:expr ) => ( $crate::normal::Value::from($value) );
);

/// Creates a [Value::List](super::super::normal::Value::List) from a sequence of bare primitive expressions.
#[macro_export]
macro_rules! normal_list (
    () => (
        $crate::normal::Value::List(
            $crate::normal::List::new()
        )
    );

    ( $( $value:expr ),+ $( , )? ) => (
        $crate::normal::Value::List(
            $crate::normal::List::new(
                vec![ $( $crate::normal!( $value ) ),+ ]
            )
        )
    );
);

/// Creates a [Value::Map](super::super::normal::Value::Map) from a sequence of key-value tuples.
#[macro_export]
macro_rules! normal_map (
    () => (
        $crate::normal::Value::Map(
            $crate::normal::Map::new()
        )
    );

    ( $( ( $key:expr, $value:expr ) ),+ $( , )? ) => (
        $crate::normal::Value::Map(
            $crate::normal::Map::from(
                [ $( ( $crate::normal!( $key ), $crate::normal!( $value ) ) ),+ ]
            )
        )
    );
);

/// Creates a [Vec]<[Value](super::super::normal::Value)> from a sequence of bare primitive expressions.
#[macro_export]
macro_rules! normal_vec (
    ( $( $value:expr ),* $( , )? ) => (
        vec![ $( $crate::normal!( $value ) ),* ]
    );
);
