use super::{meta::*, normal::*, path::*, value::*};

use {
    kutil_cli::debug::*,
    kutil_std::error::*,
    owo_colors::*,
    std::{error::*, fmt, io},
};

//
// LocatableError
//

/// Wraps an [Error] adding an optional [Location].
pub struct LocatableError<E: Error> {
    /// Wrapped error.
    pub error: E,

    /// Optional location.
    pub location: Option<Location>,

    /// Optional path.
    pub path: Option<String>,
}

impl<E: Error> LocatableError<E> {
    /// Constructor.
    pub fn new(error: E, location: Option<Location>, path: Option<String>) -> Self {
        Self { error, location, path }
    }
}

impl<E: Error + WriteDebug> WriteDebug for LocatableError<E> {
    fn write_debug_representation<W: io::Write>(
        &self,
        writer: &mut W,
        indentation: usize,
        styles: &Styles,
    ) -> io::Result<()> {
        self.error.write_debug_representation(writer, indentation, styles)?;

        let has_path = self.path.is_some();
        let has_location = self.location.is_some();
        if has_path || has_location {
            let indent = " ".repeat(indentation);
            write!(writer, "\n{}", indent)?;

            if let Some(path) = &self.path {
                write!(writer, "{}", path.style(styles.meta))?;
            }

            if let Some(location) = &self.location {
                if has_path {
                    write!(writer, " ")?;
                }
                location.write_debug_representation(writer, indentation, styles)?;
            }
        }

        Ok(())
    }
}

impl<E: Error> Error for LocatableError<E> {}

impl<E: Error> fmt::Debug for LocatableError<E> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.error, formatter)?;
        if let Some(location) = &self.location {
            write!(formatter, " @{}", location)?;
        }
        Ok(())
    }
}

// Delegated

impl<E: Error> fmt::Display for LocatableError<E> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.error, formatter)
    }
}

//
// ToLocatableError
//

/// Adds an optional [Location] to an [Error].
pub trait ToLocatableError<E: Error> {
    /// Adds an optional [Location] and a path from an optional ancestor to an [Error].
    fn with_location(self, value: &Value, ancestor: Option<&Value>) -> LocatableError<E>;
}

impl<E: Error> ToLocatableError<E> for E {
    fn with_location(self, value: &Value, ancestor: Option<&Value>) -> LocatableError<E> {
        let location = match value.get_meta() {
            Some(meta) => meta.location.clone(),
            None => None,
        };

        let path = match ancestor {
            Some(ancestor) => match Path::find(ancestor, value) {
                Some(path) => {
                    if path.is_linear() {
                        Some(path.to_string())
                    } else {
                        None
                    }
                }
                None => None,
            },
            None => None,
        };

        LocatableError::new(self, location, path)
    }
}

//
// AccumulatedLocatableErrors
//

/// Accumulated locatable errors.
pub struct AccumulatedLocatableErrors<E: Error>(AccumulatedErrors<E>);

impl<E: Error> AccumulatedLocatableErrors<E> {
    /// Constructor.
    pub fn new() -> Self {
        Self(AccumulatedErrors::new())
    }

    /// Gets the accumulated errors.
    pub fn get_errors(&self) -> &Vec<E> {
        self.0.get_errors()
    }

    /// Fails with self if there are errors.
    pub fn check(self) -> Result<(), Self> {
        match self.0.check() {
            Ok(_) => Ok(()),
            Err(err) => Err(Self(err)),
        }
    }
}

impl<E: Error> Error for AccumulatedLocatableErrors<E> {}

impl<E: Error + WriteDebug> WriteDebug for AccumulatedLocatableErrors<E> {
    fn write_debug_representation<W: io::Write>(
        &self,
        writer: &mut W,
        mut indentation: usize,
        styles: &Styles,
    ) -> io::Result<()> {
        let indent = " ".repeat(indentation);
        indentation += 2;

        // TODO: sort

        let mut first = true;
        for error in self.get_errors() {
            if first {
                first = false;
            } else {
                write!(writer, "\n{}", indent)?;
            }

            error.write_debug_representation(writer, indentation, styles)?;
        }

        Ok(())
    }
}

// Delegated

impl<E: Error> ErrorReporter<E> for AccumulatedLocatableErrors<E> {
    fn report(&mut self, error: impl Into<E>) -> Result<(), E> {
        self.0.report(error)
    }
}

impl<E: Error> fmt::Display for AccumulatedLocatableErrors<E> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, formatter)
    }
}

impl<E: Error> fmt::Debug for AccumulatedLocatableErrors<E> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, formatter)
    }
}

// Conversions

impl<E: Error> From<AccumulatedErrors<E>> for AccumulatedLocatableErrors<E> {
    fn from(value: AccumulatedErrors<E>) -> Self {
        Self(value)
    }
}

impl<E: Error> From<E> for AccumulatedLocatableErrors<E> {
    fn from(value: E) -> Self {
        Self(value.into())
    }
}

impl<E: Error> From<AccumulatedLocatableErrors<E>> for AccumulatedErrors<E> {
    fn from(value: AccumulatedLocatableErrors<E>) -> Self {
        value.0
    }
}
