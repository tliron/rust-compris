use {
    super::super::annotation::*,
    crate::{impl_normal, impl_normal_basic},
};

use {
    base64::{prelude::*, *},
    bytes::*,
    bytestring::*,
    duplicate::*,
    kutil_cli::debug::*,
    std::{borrow::*, fmt, io},
};

//
// Blob
//

impl_normal! {
    /// Normal blob value.
    ///
    /// Annotations, if present, are *ignored* for the purposes of comparison and hashing.
    ///
    /// Note that the value is a [Bytes] in order to support zero-copy cloning.
    Blob(Bytes)
}

impl_normal_basic!(Blob);

impl<AnnotationsT> Blob<AnnotationsT> {
    /// Constructor.
    pub fn new_from_base64<BytesT>(base64: BytesT) -> Result<Self, DecodeError>
    where
        AnnotationsT: Default,
        BytesT: AsRef<[u8]>,
    {
        let bytes = BASE64_STANDARD.decode(base64)?;
        Ok(Self::from(bytes))
    }

    /// To Base64.
    pub fn to_base64(&self) -> String {
        BASE64_STANDARD.encode(&self.value)
    }
}

impl<AnnotationsT> Debuggable for Blob<AnnotationsT> {
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        context.separate(writer)?;
        context.theme.write_symbol(writer, format!("{} bytes", self.value.len()))
    }
}

impl<AnnotationsT> fmt::Display for Blob<AnnotationsT> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} bytes", self.value.len())
    }
}

// Conversion

#[duplicate_item(
  ToNormalT;
  [Bytes];
  [Vec<u8>];
  [&'static [u8]];
)]
impl<AnnotationsT> From<ToNormalT> for Blob<AnnotationsT>
where
    AnnotationsT: Default,
{
    fn from(bytes: ToNormalT) -> Self {
        Blob::new(bytes.into())
    }
}

impl<AnnotationsT> From<Cow<'_, [u8]>> for Blob<AnnotationsT>
where
    AnnotationsT: Default,
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
impl<AnnotationsT> From<FromT> for Blob<AnnotationsT>
where
    AnnotationsT: Default,
{
    fn from(string: FromT) -> Self {
        ByteString::from(string).into_bytes().into()
    }
}

impl<AnnotationsT> From<Cow<'_, str>> for Blob<AnnotationsT>
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

impl<AnnotationsT> From<Blob<AnnotationsT>> for Bytes {
    fn from(blob: Blob<AnnotationsT>) -> Self {
        blob.value
    }
}

impl<AnnotationsT> From<Blob<AnnotationsT>> for Vec<u8> {
    fn from(blob: Blob<AnnotationsT>) -> Self {
        blob.value.into()
    }
}

impl<'own, AnnotationsT> From<&'own Blob<AnnotationsT>> for &'own [u8] {
    fn from(blob: &'own Blob<AnnotationsT>) -> Self {
        &blob.value
    }
}
