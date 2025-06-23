use {
    super::super::annotation::*,
    crate::{impl_normal, impl_normal_basic},
};

use {
    bytestring::*,
    duplicate::*,
    kutil_cli::debug::*,
    std::{borrow::*, fmt, io},
};

//
// Text
//

impl_normal! {
    /// Normal text value.
    ///
    /// Annotations, if present, are *ignored* for the purposes of comparison and hashing.
    ///
    /// Note that the value is a [ByteString] in order to support zero-copy cloning.
    ///
    /// We didn't call this struct "String" in order to avoid ambiguity with the built-in [String].
    Text(ByteString)
}

impl_normal_basic!(Text);

impl<AnnotationsT> Text<AnnotationsT> {
    /// As string.
    pub fn as_str(&self) -> &str {
        self.value.as_ref()
    }
}

impl<AnnotationsT> Debuggable for Text<AnnotationsT> {
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        context.separate(writer)?;
        write!(writer, "{}", context.theme.string(format!("{:?}", self.value)))
    }
}

impl<AnnotationsT> fmt::Display for Text<AnnotationsT> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.value, formatter)
    }
}

// Conversions

#[duplicate_item(
  ToNormalT;
  [ByteString];
  [String];
  [&str];
)]
impl<AnnotationsT> From<ToNormalT> for Text<AnnotationsT>
where
    AnnotationsT: Default,
{
    fn from(string: ToNormalT) -> Self {
        Self::new(string.into())
    }
}

impl<AnnotationsT> From<Cow<'_, str>> for Text<AnnotationsT>
where
    AnnotationsT: Default,
{
    fn from(string: Cow<'_, str>) -> Self {
        match string {
            Cow::Borrowed(string) => string.into(),
            Cow::Owned(string) => string.into(),
        }
    }
}

impl<AnnotationsT> From<Text<AnnotationsT>> for String {
    fn from(text: Text<AnnotationsT>) -> Self {
        text.into()
    }
}

impl<'own, AnnotationsT> From<&'own Text<AnnotationsT>> for &'own str {
    fn from(text: &'own Text<AnnotationsT>) -> Self {
        &text.value
    }
}
