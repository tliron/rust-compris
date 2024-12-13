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
            $crate::normal::List::new_with(
                [ $( $crate::normal!( $value ) ),+ ]
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
            $crate::normal::Map::new_with(
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
