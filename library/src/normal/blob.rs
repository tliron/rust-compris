use {
    super::super::annotate::*,
    crate::{impl_normal, impl_normal_basic},
};

use {
    base64::{prelude::*, *},
    duplicate::*,
    kutil::{cli::debug::*, std::zerocopy::*},
    std::{borrow::*, fmt, io},
};

//
// Blob
//

impl_normal! {
    /// Normal blob variant.
    ///
    /// [Annotations], if present, are *ignored* for the purposes of comparison and hashing.
    ///
    /// Note that the value is a [Bytes] in order to support zero-copy cloning.
    Blob(Bytes)
}

impl_normal_basic!(Blob);

impl<AnnotatedT> Blob<AnnotatedT> {
    /// Constructor.
    pub fn new_from_base64<BytesT>(base64: BytesT) -> Result<Self, DecodeError>
    where
        AnnotatedT: Default,
        BytesT: AsRef<[u8]>,
    {
        let bytes = BASE64_STANDARD.decode(base64)?;
        Ok(Self::from(bytes))
    }

    /// To Base64.
    pub fn to_base64(&self) -> String {
        BASE64_STANDARD.encode(&self.inner)
    }

    /// As slice.
    pub fn as_slice(&self) -> &[u8] {
        &self.inner
    }
}

impl<AnnotatedT> Debuggable for Blob<AnnotatedT> {
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        context.separate(writer)?;
        context.theme.write_symbol(writer, format!("{} bytes", self.inner.len()))
    }
}

impl<AnnotatedT> fmt::Display for Blob<AnnotatedT> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} bytes", self.inner.len())
    }
}

// Conversion

impl<AnnotatedT> AsRef<[u8]> for Blob<AnnotatedT> {
    fn as_ref(&self) -> &[u8] {
        &self.inner
    }
}

#[duplicate_item(
  ToNormalT;
  [Bytes];
  [Vec<u8>];
  [&'static [u8]];
)]
impl<AnnotatedT> From<ToNormalT> for Blob<AnnotatedT>
where
    AnnotatedT: Default,
{
    fn from(bytes: ToNormalT) -> Self {
        Blob::new(bytes.into())
    }
}

impl<AnnotatedT> From<Cow<'_, [u8]>> for Blob<AnnotatedT>
where
    AnnotatedT: Default,
{
    fn from(bytes: Cow<'_, [u8]>) -> Self {
        match bytes {
            Cow::Borrowed(bytes) => bytes.to_vec().into(),
            Cow::Owned(bytes) => bytes.into(),
        }
    }
}

#[duplicate_item(
  FromT;
  [ByteString];
  [String];
  [&str];
)]
impl<AnnotatedT> From<FromT> for Blob<AnnotatedT>
where
    AnnotatedT: Default,
{
    fn from(string: FromT) -> Self {
        ByteString::from(string).into_bytes().into()
    }
}

impl<AnnotatedT> From<Cow<'_, str>> for Blob<AnnotatedT>
where
    AnnotatedT: Default,
{
    fn from(string: Cow<'_, str>) -> Self {
        match string {
            Cow::Borrowed(string) => string.into(),
            Cow::Owned(string) => string.into(),
        }
    }
}

impl<AnnotatedT> From<Blob<AnnotatedT>> for Vec<u8> {
    fn from(blob: Blob<AnnotatedT>) -> Self {
        blob.inner.into()
    }
}

impl<'own, AnnotatedT> From<&'own Blob<AnnotatedT>> for &'own [u8] {
    fn from(blob: &'own Blob<AnnotatedT>) -> Self {
        &blob.inner
    }
}
