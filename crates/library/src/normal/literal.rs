/// Creates a [Value](super::Value) from a bare primitive expression.
#[macro_export]
macro_rules! normal (
    ( $value:expr ) => ( $crate::Value::from($value) );
);

/// Creates a [Value::List](super::Value::List) from a sequence of bare primitive expressions.
#[macro_export]
macro_rules! normal_list (
    () => ( $crate::Value::List(compris::List::new()) );

    ( $( $value:expr ),+ $( , )? ) => (
        $crate::Value::List($crate::List::new_with([ $( $crate::normal!( $value ) ),+ ]))
    );
);

/// Creates a [Value::Map](super::Value::Map) from a sequence of key-value tuples.
#[macro_export]
macro_rules! normal_map (
    () => ( $crate::Value::Map($crate::Map::new()) );

    ( $( ( $key:expr, $value:expr ) ),+ $( , )? ) => (
        $crate::Value::Map($crate::Map::new_with([ $( ( $crate::normal!( $key ), $crate::normal!( $value ) ) ),+ ]))
    );
);
