use super::{super::*, errors::*};

use {
    kutil_std::error::*,
    std::{collections::*, error::*, hash::*},
};

/// Resolve result.
pub type ResolveResult<T, OE> = Result<Option<T>, ResolveError<OE>>;

/// Resolve result where [ResolveError::Other] is [CustomError].
pub type CommonResolveResult<T> = Result<Option<T>, ResolveError<CustomError>>;

//
// Resolve
//

/// Resolve one type into another.
pub trait Resolve<T, OE: Error = CustomError> {
    /// Resolve one type into another.
    ///
    /// In the case of an error, implementations should report errors using [ErrorReporter::report] on the
    /// provided error_reporter.
    ///
    /// They can report custom errors using [ResolveError::Other]. Its type is determined by the `OE` generic
    /// paramater.
    ///
    /// Only if [report](ErrorReporter::report) itself fails should they return an error here, which allows for a
    /// fail-fast mode. They should not otherwise return an error.
    ///
    /// The optional ancestor argument can be used with [Path::find] for detailed errors.
    fn resolve_for<ER: ErrorReporter<ResolveError<OE>>>(
        &self,
        error_reporter: &mut ER,
        ancestor: Option<&Value>,
    ) -> ResolveResult<T, OE>;

    /// Resolve one type into another.
    fn resolve<ER: ErrorReporter<ResolveError<OE>>>(&self, error_reporter: &mut ER) -> ResolveResult<T, OE> {
        self.resolve_for(error_reporter, None)
    }

    /// Resolve one type into another while failing on first reported error.
    ///
    /// Uses [FailFastErrorReporter].
    fn resolve_fail_fast(&self) -> ResolveResult<T, OE> {
        self.resolve(&mut FailFastErrorReporter)
    }
}

impl<OE: Error> Resolve<Value, OE> for Value {
    fn resolve_for<ER: ErrorReporter<ResolveError<OE>>>(
        &self,
        _error_reporter: &mut ER,
        _ancestor: Option<&Value>,
    ) -> ResolveResult<Value, OE> {
        // Values resolve into themselves by cloning
        Ok(Some(self.clone()))
    }
}

impl<T, OE: Error> Resolve<Option<T>, OE> for Value
where
    Value: Resolve<T, OE>,
{
    fn resolve_for<ER: ErrorReporter<ResolveError<OE>>>(
        &self,
        error_reporter: &mut ER,
        ancestor: Option<&Value>,
    ) -> ResolveResult<Option<T>, OE> {
        // Nothing or Null will resolve to None
        match self {
            Value::Nothing | Value::Null(_) => Ok(None),
            _ => Ok(Some(self.resolve_for(error_reporter, ancestor)?)),
        }
    }
}

impl<OE: Error> Resolve<i64, OE> for Value {
    fn resolve_for<ER: ErrorReporter<ResolveError<OE>>>(
        &self,
        error_reporter: &mut ER,
        ancestor: Option<&Value>,
    ) -> ResolveResult<i64, OE> {
        Ok(match self.try_into() {
            Ok(value) => Some(value),
            Err(err) => {
                error_reporter.report(err.with_location(self, ancestor))?;
                None
            }
        })
    }
}

impl<OE: Error> Resolve<u64, OE> for Value {
    fn resolve_for<ER: ErrorReporter<ResolveError<OE>>>(
        &self,
        error_reporter: &mut ER,
        ancestor: Option<&Value>,
    ) -> ResolveResult<u64, OE> {
        Ok(match self.try_into() {
            Ok(value) => Some(value),
            Err(err) => {
                error_reporter.report(err.with_location(self, ancestor))?;
                None
            }
        })
    }
}

impl<OE: Error> Resolve<bool, OE> for Value {
    fn resolve_for<ER: ErrorReporter<ResolveError<OE>>>(
        &self,
        error_reporter: &mut ER,
        ancestor: Option<&Value>,
    ) -> ResolveResult<bool, OE> {
        Ok(match self.try_into() {
            Ok(value) => Some(value),
            Err(err) => {
                error_reporter.report(err.with_location(self, ancestor))?;
                None
            }
        })
    }
}

impl<OE: Error> Resolve<f64, OE> for Value {
    fn resolve_for<ER: ErrorReporter<ResolveError<OE>>>(
        &self,
        error_reporter: &mut ER,
        ancestor: Option<&Value>,
    ) -> ResolveResult<f64, OE> {
        Ok(match self.try_into() {
            Ok(value) => Some(value),
            Err(err) => {
                error_reporter.report(err.with_location(self, ancestor))?;
                None
            }
        })
    }
}

impl<OE: Error> Resolve<String, OE> for Value {
    fn resolve_for<ER: ErrorReporter<ResolveError<OE>>>(
        &self,
        error_reporter: &mut ER,
        ancestor: Option<&Value>,
    ) -> ResolveResult<String, OE> {
        Ok(match self.try_into() {
            Ok(value) => Some(value),
            Err(err) => {
                error_reporter.report(err.with_location(self, ancestor))?;
                None
            }
        })
    }
}

impl<T, OE: Error> Resolve<Vec<T>, OE> for Value
where
    Value: Resolve<T, OE>,
{
    fn resolve_for<'a, ER: ErrorReporter<ResolveError<OE>>>(
        &'a self,
        error_reporter: &mut ER,
        mut ancestor: Option<&'a Value>,
    ) -> ResolveResult<Vec<T>, OE> {
        if ancestor.is_none() {
            ancestor = Some(self);
        }

        let mut r = Vec::new();

        match self {
            Value::List(list) => {
                // Resolve each element of the list
                for element in &list.value {
                    if let Some(element) = element.resolve_for(error_reporter, ancestor)? {
                        r.push(element);
                    }
                }
            }

            _ => {
                error_reporter.report(IncompatibleValueTypeError::new(self, "list").with_location(self, ancestor))?;
            }
        }

        Ok(Some(r))
    }
}

impl<K, V, OE: Error> Resolve<HashMap<K, V>, OE> for Value
where
    K: Hash + Eq,
    Value: Resolve<K, OE>,
    Value: Resolve<V, OE>,
{
    fn resolve_for<'a, ER: ErrorReporter<ResolveError<OE>>>(
        &'a self,
        error_reporter: &mut ER,
        mut ancestor: Option<&'a Value>,
    ) -> ResolveResult<HashMap<K, V>, OE> {
        if ancestor.is_none() {
            ancestor = Some(self);
        }

        let mut r = HashMap::new();

        match self {
            Value::Map(map) => {
                // Resolve each key and value of every entry
                for (key, value) in &map.value {
                    let key = key.resolve_for(error_reporter, ancestor)?;
                    let value = value.resolve_for(error_reporter, ancestor)?;
                    if let Some(key) = key {
                        if let Some(value) = value {
                            r.insert(key, value);
                        }
                    }
                }
            }

            _ => {
                error_reporter.report(IncompatibleValueTypeError::new(self, "map").with_location(self, ancestor))?;
            }
        }

        Ok(Some(r))
    }
}
