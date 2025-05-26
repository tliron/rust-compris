use super::value::*;

//
// ValuePath
//

/// Value path.
pub type ValuePath = Vec<Value>;

/// To [ValuePath].
///
/// See [Value::traverse](super::value::Value::traverse).
pub fn to_value_path(ref_value_path: RefValuePath) -> ValuePath {
    ref_value_path.iter().map(|value| (*value).clone()).collect()
}

//
// RefValuePath
//

/// [ValuePath] path using references.
///
/// Can be converted into a [ValuePath] using [to_value_path].
pub type RefValuePath<'own> = Vec<&'own Value>;

/// Convert to a [RefValuePath].
///
/// If it's already a [List](super::list::List) will just make sure it's not empty. Other value
/// types will be wrapped in a [List](super::list::List).
pub fn to_ref_value_path(value: &Value) -> Option<RefValuePath<'_>> {
    match value {
        Value::List(list) => {
            if !list.value.is_empty() {
                return Some(list.value.iter().collect());
            }
        }

        _ => {
            return Some(vec![value]);
        }
    }

    None
}
