/// Traverse a value by calling [compris::Value::get] recursively.
///
/// The first argument is the starting [compris::Value]. The following arguments
/// are a sequence of keys, which will be tried one at a time. Any non-map or
/// missing key will cause the macro to stop and return [None].
///
/// The keys are either [Value] or anything that implements [Into<Value>], which
/// includes all the supported primtive types.
#[macro_export]
macro_rules! traverse(
    ( $value:expr ) => ( Option::<&$crate::Value>::Some(&$value) );

    ( $value:expr, $key:expr ) => ( $value.get($key) );

    ( $value:expr, $key:expr, $( $next_key:expr ),+ $( , )? ) => (
        match $crate::traverse!( $value, $key ) {
            Some(value) => $crate::traverse!( value $( , $next_key )+ ),
            None => None,
        }
    );
);

/// Traverse a value by calling [Value::get_mut] recursively.
///
/// The first argument is the starting [compris::Value]. The following arguments
/// are a sequence of keys, which will be tried one at a time. Any non-map or
/// missing key will cause the macro to stop and return [None].
///
/// The keys are either [Value] or anything that implements [Into<Value>], which
/// includes all the supported primtive types.
#[macro_export]
macro_rules! traverse_mut(
    ( $value:expr ) => ( Option::<&mut $crate::Value>::Some($value) );

    ( $value:expr, $key:expr ) => ( $value.get_mut($key) );

    ( $value:expr, $key:expr, $( $next_key:expr ),+ $( , )? ) => (
        match $crate::traverse_mut!( $value, $key ) {
            Some(value) => $crate::traverse_mut!( value $( , $next_key )+ ),
            None => None,
        }
    );
);
