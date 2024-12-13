use super::{super::normal::*, citation::*, context::*, error::*, resolve::*, result::*};

use {
    kutil_std::error::*,
    std::{collections::*, hash::*},
};

// Self

impl<ContextT, ErrorT> Resolve<Value, ContextT, ErrorT> for Value
where
    ContextT: ResolveContext,
    ErrorT: ResolveError,
{
    fn resolve_for<ErrorRecipientT>(
        &self,
        _context: Option<&ContextT>,
        _ancestor: Option<&Value>,
        _errors: &mut ErrorRecipientT,
    ) -> ResolveResult<Value, ErrorT>
    where
        ErrorRecipientT: ErrorRecipient<ErrorT>,
    {
        Ok(Some(self.clone()))
    }
}

// Primitives

impl<ContextT, ErrorT> Resolve<i64, ContextT, ErrorT> for Value
where
    ContextT: ResolveContext,
    ErrorT: ResolveError,
{
    fn resolve_for<ErrorRecipientT>(
        &self,
        context: Option<&ContextT>,
        ancestor: Option<&Value>,
        errors: &mut ErrorRecipientT,
    ) -> ResolveResult<i64, ErrorT>
    where
        ErrorRecipientT: ErrorRecipient<ErrorT>,
    {
        Ok(match self.try_into() {
            Ok(value) => Some(value),
            Err(err) => {
                errors.give(err.with_citation_for(self, context, ancestor))?;
                None
            }
        })
    }
}

impl<ContextT, ErrorT> Resolve<u64, ContextT, ErrorT> for Value
where
    ContextT: ResolveContext,
    ErrorT: ResolveError,
{
    fn resolve_for<ErrorRecipientT>(
        &self,
        context: Option<&ContextT>,
        ancestor: Option<&Value>,
        errors: &mut ErrorRecipientT,
    ) -> ResolveResult<u64, ErrorT>
    where
        ErrorRecipientT: ErrorRecipient<ErrorT>,
    {
        Ok(match self.try_into() {
            Ok(value) => Some(value),
            Err(err) => {
                errors.give(err.with_citation_for(self, context, ancestor))?;
                None
            }
        })
    }
}

impl<ContextT, ErrorT> Resolve<bool, ContextT, ErrorT> for Value
where
    ContextT: ResolveContext,
    ErrorT: ResolveError,
{
    fn resolve_for<ErrorRecipientT>(
        &self,
        context: Option<&ContextT>,
        ancestor: Option<&Value>,
        errors: &mut ErrorRecipientT,
    ) -> ResolveResult<bool, ErrorT>
    where
        ErrorRecipientT: ErrorRecipient<ErrorT>,
    {
        Ok(match self.try_into() {
            Ok(value) => Some(value),
            Err(err) => {
                errors.give(err.with_citation_for(self, context, ancestor))?;
                None
            }
        })
    }
}

impl<ContextT, ErrorT> Resolve<f64, ContextT, ErrorT> for Value
where
    ContextT: ResolveContext,
    ErrorT: ResolveError,
{
    fn resolve_for<ErrorRecipientT>(
        &self,
        context: Option<&ContextT>,
        ancestor: Option<&Value>,
        errors: &mut ErrorRecipientT,
    ) -> ResolveResult<f64, ErrorT>
    where
        ErrorRecipientT: ErrorRecipient<ErrorT>,
    {
        Ok(match self.try_into() {
            Ok(value) => Some(value),
            Err(err) => {
                errors.give(err.with_citation_for(self, context, ancestor))?;
                None
            }
        })
    }
}

impl<ContextT, ErrorT> Resolve<String, ContextT, ErrorT> for Value
where
    ContextT: ResolveContext,
    ErrorT: ResolveError,
{
    fn resolve_for<ErrorRecipientT>(
        &self,
        context: Option<&ContextT>,
        ancestor: Option<&Value>,
        errors: &mut ErrorRecipientT,
    ) -> ResolveResult<String, ErrorT>
    where
        ErrorRecipientT: ErrorRecipient<ErrorT>,
    {
        Ok(match self.try_into() {
            Ok(value) => Some(value),
            Err(err) => {
                errors.give(err.with_citation_for(self, context, ancestor))?;
                None
            }
        })
    }
}

// Collections

impl<ItemT, ContextT, ErrorT> Resolve<Vec<ItemT>, ContextT, ErrorT> for Value
where
    ContextT: ResolveContext,
    ErrorT: ResolveError,
    Value: Resolve<ItemT, ContextT, ErrorT>,
{
    fn resolve_for<'a, ErrorRecipientT>(
        &'a self,
        context: Option<&ContextT>,
        mut ancestor: Option<&'a Value>,
        errors: &mut ErrorRecipientT,
    ) -> ResolveResult<Vec<ItemT>, ErrorT>
    where
        ErrorRecipientT: ErrorRecipient<ErrorT>,
    {
        if ancestor.is_none() {
            ancestor = Some(self)
        }

        let mut r = Vec::new();

        match self {
            Value::List(list) => {
                // Resolve each item of the list
                for item in list {
                    if let Some(item) = item.resolve_for(context, ancestor, errors)? {
                        r.push(item);
                    }
                }
            }

            _ => errors
                .give(IncompatibleValueTypeError::new(self, "list", None).with_citation_for(self, context, ancestor))?,
        }

        Ok(Some(r))
    }
}

impl<KeyT, ValueT, ContextT, ErrorT> Resolve<HashMap<KeyT, ValueT>, ContextT, ErrorT> for Value
where
    ContextT: ResolveContext,
    ErrorT: ResolveError,
    KeyT: Hash + Eq,
    Value: Resolve<KeyT, ContextT, ErrorT>,
    Value: Resolve<ValueT, ContextT, ErrorT>,
{
    fn resolve_for<'a, ErrorRecipientT>(
        &'a self,
        context: Option<&ContextT>,
        mut ancestor: Option<&'a Value>,
        errors: &mut ErrorRecipientT,
    ) -> ResolveResult<HashMap<KeyT, ValueT>, ErrorT>
    where
        ErrorRecipientT: ErrorRecipient<ErrorT>,
    {
        if ancestor.is_none() {
            ancestor = Some(self)
        }

        let mut r = HashMap::new();

        match self {
            Value::Map(map) => {
                for (key, value) in &map.value {
                    // Resolve each key and value of every entry
                    let key = key.resolve_for(context, ancestor, errors)?;
                    let value = value.resolve_for(context, ancestor, errors)?;
                    if let Some(key) = key {
                        if let Some(value) = value {
                            r.insert(key, value);
                        }
                    }
                }
            }

            Value::List(list) => {
                for item in list {
                    match item {
                        Value::List(list) => {
                            if let Some((key, value)) = list.to_couple() {
                                // Resolve each key and value of every entry
                                let key = key.resolve_for(context, ancestor, errors)?;
                                let value = value.resolve_for(context, ancestor, errors)?;
                                if let Some(key) = key {
                                    if let Some(value) = value {
                                        if r.insert(key, value).is_some() {
                                            errors.give(
                                                IncompatibleValueTypeError::new(
                                                    item,
                                                    "list",
                                                    Some("without repeating keys"),
                                                )
                                                .with_citation_for(item, context, ancestor),
                                            )?;
                                        }
                                    }
                                }
                            } else {
                                errors.give(
                                    IncompatibleValueTypeError::new(item, "list", Some("of length 2"))
                                        .with_citation_for(item, context, ancestor),
                                )?;
                            }
                        }

                        _ => errors.give(
                            IncompatibleValueTypeError::new(item, "list", Some("of length 2"))
                                .with_citation_for(item, context, ancestor),
                        )?,
                    }
                }
            }

            _ => errors.give(
                IncompatibleValueTypeError::new(self, "map or list", Some("of lists of length 2"))
                    .with_citation_for(self, context, ancestor),
            )?,
        }

        Ok(Some(r))
    }
}

// Option<T>

impl<OptionalT, ContextT, ErrorT> Resolve<Option<OptionalT>, ContextT, ErrorT> for Value
where
    ContextT: ResolveContext,
    ErrorT: ResolveError,
    Value: Resolve<OptionalT, ContextT, ErrorT>,
{
    fn resolve_for<ErrorRecipientT>(
        &self,
        context: Option<&ContextT>,
        ancestor: Option<&Value>,
        errors: &mut ErrorRecipientT,
    ) -> ResolveResult<Option<OptionalT>, ErrorT>
    where
        ErrorRecipientT: ErrorRecipient<ErrorT>,
    {
        Ok(Some(self.resolve_for(context, ancestor, errors)?))
    }
}
