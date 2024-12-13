/// Creates a [Value](super::super::Value) from a bare primitive expression.
#[macro_export]
macro_rules! normal (
    ( $value:expr ) => ( $crate::normal::Value::from($value) );
);

/// Creates a [Value::List](super::super::Value::List) from a sequence of bare primitive expressions.
#[macro_export]
macro_rules! normal_list (
    () => ( $crate::Value::List($crate::normal::List::new()) );

    ( $( $value:expr ),+ $( , )? ) => (
        $crate::normal::Value::List($crate::normal::List::new_with([ $( $crate::normal!( $value ) ),+ ]))
    );
);

/// Creates a [Value::Map](super::super::Value::Map) from a sequence of key-value tuples.
#[macro_export]
macro_rules! normal_map (
    () => ( $crate::normal::Value::Map($crate::normal::Map::new()) );

    ( $( ( $key:expr, $value:expr ) ),+ $( , )? ) => (
        $crate::normal::Value::Map($crate::normal::Map::new_with([ $( ( $crate::normal!( $key ), $crate::normal!( $value ) ) ),+ ]))
    );
);
