use super::{super::normal::*, context::*, resolve::*, resolve_error::*, result::*};

use {
    kutil_std::error::*,
    std::{collections::*, hash::*},
};

// Self

impl<E: ResolveError> Resolve<Value, E> for Value {
    fn resolve_for<ER: ErrorRecipient<E>>(
        &self,
        _ancestor: Option<&Value>,
        _context: Option<&ResolveContext>,
        _errors: &mut ER,
    ) -> ResolveResult<Value, E> {
        Ok(Some(self.clone()))
    }
}

// Primitives

impl<E: ResolveError> Resolve<i64, E> for Value {
    fn resolve_for<ER: ErrorRecipient<E>>(
        &self,
        ancestor: Option<&Value>,
        context: Option<&ResolveContext>,
        errors: &mut ER,
    ) -> ResolveResult<i64, E> {
        Ok(match self.try_into() {
            Ok(value) => Some(value),
            Err(err) => {
                errors.give(err.with_citation_for(self, ancestor, context))?;
                None
            }
        })
    }
}

impl<E: ResolveError> Resolve<u64, E> for Value {
    fn resolve_for<ER: ErrorRecipient<E>>(
        &self,
        ancestor: Option<&Value>,
        context: Option<&ResolveContext>,
        errors: &mut ER,
    ) -> ResolveResult<u64, E> {
        Ok(match self.try_into() {
            Ok(value) => Some(value),
            Err(err) => {
                errors.give(err.with_citation_for(self, ancestor, context))?;
                None
            }
        })
    }
}

impl<E: ResolveError> Resolve<bool, E> for Value {
    fn resolve_for<ER: ErrorRecipient<E>>(
        &self,
        ancestor: Option<&Value>,
        context: Option<&ResolveContext>,
        errors: &mut ER,
    ) -> ResolveResult<bool, E> {
        Ok(match self.try_into() {
            Ok(value) => Some(value),
            Err(err) => {
                errors.give(err.with_citation_for(self, ancestor, context))?;
                None
            }
        })
    }
}

impl<E: ResolveError> Resolve<f64, E> for Value {
    fn resolve_for<ER: ErrorRecipient<E>>(
        &self,
        ancestor: Option<&Value>,
        context: Option<&ResolveContext>,
        errors: &mut ER,
    ) -> ResolveResult<f64, E> {
        Ok(match self.try_into() {
            Ok(value) => Some(value),
            Err(err) => {
                errors.give(err.with_citation_for(self, ancestor, context))?;
                None
            }
        })
    }
}

impl<E: ResolveError> Resolve<String, E> for Value {
    fn resolve_for<ER: ErrorRecipient<E>>(
        &self,
        ancestor: Option<&Value>,
        context: Option<&ResolveContext>,
        errors: &mut ER,
    ) -> ResolveResult<String, E> {
        Ok(match self.try_into() {
            Ok(value) => Some(value),
            Err(err) => {
                errors.give(err.with_citation_for(self, ancestor, context))?;
                None
            }
        })
    }
}

// Collections

impl<T, E: ResolveError> Resolve<Vec<T>, E> for Value
where
    Value: Resolve<T, E>,
{
    fn resolve_for<'a, ER: ErrorRecipient<E>>(
        &'a self,
        mut ancestor: Option<&'a Value>,
        context: Option<&ResolveContext>,
        errors: &mut ER,
    ) -> ResolveResult<Vec<T>, E> {
        if ancestor.is_none() {
            ancestor = Some(self)
        }

        let mut r = Vec::new();

        match self {
            Value::List(list) => {
                // Resolve each element of the list
                for element in &list.value {
                    if let Some(element) = element.resolve_for(ancestor, context, errors)? {
                        r.push(element);
                    }
                }
            }

            _ => {
                errors.give(IncompatibleValueTypeError::new(self, "list").with_citation_for(self, ancestor, context))?
            }
        }

        Ok(Some(r))
    }
}

impl<K, V, E: ResolveError> Resolve<HashMap<K, V>, E> for Value
where
    K: Hash + Eq,
    Value: Resolve<K, E>,
    Value: Resolve<V, E>,
{
    fn resolve_for<'a, ER: ErrorRecipient<E>>(
        &'a self,
        mut ancestor: Option<&'a Value>,
        context: Option<&ResolveContext>,
        errors: &mut ER,
    ) -> ResolveResult<HashMap<K, V>, E> {
        if ancestor.is_none() {
            ancestor = Some(self)
        }

        let mut r = HashMap::new();

        match self {
            Value::Map(map) => {
                // Resolve each key and value of every entry
                for (key, value) in &map.value {
                    let key = key.resolve_for(ancestor, context, errors)?;
                    let value = value.resolve_for(ancestor, context, errors)?;
                    if let Some(key) = key {
                        if let Some(value) = value {
                            r.insert(key, value);
                        }
                    }
                }
            }

            _ => {
                errors.give(IncompatibleValueTypeError::new(self, "map").with_citation_for(self, ancestor, context))?
            }
        }

        Ok(Some(r))
    }
}

// Option<T>

impl<T, E: ResolveError> Resolve<Option<T>, E> for Value
where
    Value: Resolve<T, E>,
{
    fn resolve_for<ER: ErrorRecipient<E>>(
        &self,
        ancestor: Option<&Value>,
        context: Option<&ResolveContext>,
        errors: &mut ER,
    ) -> ResolveResult<Option<T>, E> {
        // Null will resolve to None
        match self {
            Value::Null(_) => Ok(None),

            _ => Ok(Some(self.resolve_for(ancestor, context, errors)?)),
        }
    }
}
