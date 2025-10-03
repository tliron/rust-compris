use {
    super::super::annotate::*,
    crate::{impl_normal, impl_normal_basic},
};

use {
    kutil::{cli::depict::*, std::immutable::*},
    std::{borrow::*, fmt, io},
};

//
// Text
//

impl_normal! {
    /// Normal text variant.
    ///
    /// [Annotations], if present, are *ignored* for the purposes of comparison and hashing.
    ///
    /// Note that the value is a [ByteString] in order to support zero-copy cloning.
    ///
    /// We didn't call this struct "String" in order to avoid ambiguity with the built-in [String].
    Text(ByteString)
}

impl_normal_basic!(Text);

impl<AnnotatedT> Text<AnnotatedT> {
    /// As string.
    pub fn as_str(&self) -> &str {
        self.inner.as_ref()
    }
}

impl<AnnotatedT> Depict for Text<AnnotatedT> {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        context.separate(writer)?;
        write!(writer, "{}", context.theme.string(format!("{:?}", self.inner)))
    }
}

impl<AnnotatedT> fmt::Display for Text<AnnotatedT> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.inner, formatter)
    }
}

// Conversions

impl<AnnotatedT> AsRef<str> for Text<AnnotatedT> {
    fn as_ref(&self) -> &str {
        &self.inner
    }
}

impl<AnnotatedT> From<String> for Text<AnnotatedT>
where
    AnnotatedT: Default,
{
    fn from(string: String) -> Self {
        Self::from(ByteString::from(string))
    }
}

impl<AnnotatedT> From<&'static str> for Text<AnnotatedT>
where
    AnnotatedT: Default,
{
    fn from(string: &'static str) -> Self {
        Self::from(ByteString::from_static(string))
    }
}

impl<AnnotatedT> From<Cow<'_, str>> for Text<AnnotatedT>
where
    AnnotatedT: Default,
{
    fn from(string: Cow<'_, str>) -> Self {
        match string {
            Cow::Borrowed(string) => ByteString::from(string).into(),
            Cow::Owned(string) => string.into(),
        }
    }
}

impl<AnnotatedT> From<Text<AnnotatedT>> for String {
    fn from(text: Text<AnnotatedT>) -> Self {
        text.inner.into()
    }
}

impl<'own, AnnotatedT> From<&'own Text<AnnotatedT>> for &'own str {
    fn from(text: &'own Text<AnnotatedT>) -> Self {
        &text.inner
    }
}
