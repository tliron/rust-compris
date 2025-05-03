use super::{
    super::{super::normal::*, cite::*, context::*, error::*, resolve::*, result::*},
    blob::*,
};

use {
    kutil_std::error::*,
    std::{fs::*, io, path::*},
};

//
// LoadableBlob
//

/// A [Blob] that be specified either as explicit content or as a path from which to read the content.
///
/// Resolves from a single-key map where the key is either "content" or "path".
#[derive(Clone, Debug)]
pub enum LoadableBlob {
    /// Content.
    Content(Blob),

    /// Path.
    Path(PathBuf),
}

impl LoadableBlob {
    /// Gets the content.
    ///
    /// If it's a path, will attempt to read from the path.
    pub fn get(self) -> io::Result<Vec<u8>> {
        self.try_into()
    }

    /// If it's explicit content, returns self.
    ///
    /// If it's a path, will attempt to read from the path and return a new [LoadableBlob]
    /// with explicit content.
    pub fn as_content(self) -> io::Result<Self> {
        match self {
            Self::Content(_) => Ok(self),
            Self::Path(path) => {
                let content = read(path)?;
                Ok(Self::Content(content.into()))
            }
        }
    }
}

impl Default for LoadableBlob {
    fn default() -> Self {
        Self::Content(Blob::default())
    }
}

impl From<Blob> for LoadableBlob {
    fn from(blob: Blob) -> Self {
        Self::Content(blob)
    }
}

impl TryInto<Vec<u8>> for LoadableBlob {
    type Error = io::Error;

    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        match self {
            Self::Content(content) => Ok(content.into()),
            Self::Path(path) => read(path),
        }
    }
}

impl TryInto<Blob> for LoadableBlob {
    type Error = io::Error;

    fn try_into(self) -> Result<Blob, Self::Error> {
        let blob: Vec<u8> = self.try_into()?;
        Ok(blob.into())
    }
}

impl<ContextT, ErrorT> Resolve<LoadableBlob, ContextT, ErrorT> for Value
where
    ContextT: ResolveContext,
    ErrorT: ResolveError,
{
    fn resolve_for<'own, ErrorRecipientT>(
        &'own self,
        context: Option<&ContextT>,
        mut ancestor: Option<&'own Value>,
        errors: &mut ErrorRecipientT,
    ) -> ResolveResult<LoadableBlob, ErrorT>
    where
        ErrorRecipientT: ErrorRecipient<ErrorT>,
    {
        if ancestor.is_none() {
            ancestor = Some(self)
        }

        Ok(match self.to_key_value_pair() {
            Some((key, value)) => match key {
                Self::Text(text) => match text.value.as_str() {
                    "content" => Resolve::resolve_for(value, context, ancestor, errors)?.map(LoadableBlob::Content),

                    "path" => Resolve::resolve_for(value, context, ancestor, errors)?.map(LoadableBlob::Path),

                    key => {
                        errors.give(
                            MalformedError::new("Blob", &format!("key is not \"content\" or \"path\": {}", key))
                                .with_citation_for(self, context, ancestor),
                        )?;
                        None
                    }
                },

                _ => {
                    errors.give(
                        IncompatibleValueTypeError::new(self, &["text"]).with_citation_for(self, context, ancestor),
                    )?;
                    None
                }
            },

            None => {
                errors.give(
                    MalformedError::new("map", "is not a single-key map").with_citation_for(self, context, ancestor),
                )?;
                None
            }
        })
    }
}
