use super::{
    super::annotate::*, blob::*, boolean::*, errors::*, float::*, integer::*, list::*, map::*, null::*, text::*,
    unsigned_integer::*, variant::*,
};

use {
    duplicate::*,
    kutil_std::zerocopy::*,
    ordered_float::OrderedFloat,
    std::{borrow::*, collections::*},
};

// Normal type -> Variant

#[duplicate_item(
  NormalT;
  [Null];
  [Integer];
  [UnsignedInteger];
  [Float];
  [Boolean];
  [Text];
  [Blob];
  [List];
  [Map];
)]
impl<AnnotatedT> From<NormalT<AnnotatedT>> for Variant<AnnotatedT> {
    fn from(normal: NormalT<AnnotatedT>) -> Self {
        Self::NormalT(normal)
    }
}

// Common types -> Variant

#[duplicate_item(
  ToNormalT          FromT;
  [Null]             [()];
  [Integer]          [i64];
  [Integer]          [i32];
  [Integer]          [i16];
  [Integer]          [i8];
  [Integer]          [isize];
  [UnsignedInteger]  [u64];
  [UnsignedInteger]  [u32];
  [UnsignedInteger]  [u16];
  [UnsignedInteger]  [u8];
  [UnsignedInteger]  [usize];
  [Float]            [f64];
  [Float]            [f32];
  [Float]            [OrderedFloat<f64>];
  [Boolean]          [bool];
  [Text]             [ByteString];
  [Text]             [String];
  [Text]             [&str];
  [Text]             [Cow<'_, str>];
  [Blob]             [Bytes];
  [Blob]             [Vec<u8>];
  [Blob]             [&'static [u8]];
  [Blob]             [Cow<'_, [u8]>];
  [List]             [Vec<Variant<AnnotatedT>>];
  [Map]              [BTreeMap<Variant<AnnotatedT>, Variant<AnnotatedT>>];
)]
impl<AnnotatedT> From<FromT> for Variant<AnnotatedT>
where
    AnnotatedT: Default,
{
    fn from(from_value: FromT) -> Self {
        Self::ToNormalT(ToNormalT::from(from_value))
    }
}

// Iterators -> Variant

impl<AnnotatedT> FromIterator<Variant<AnnotatedT>> for Variant<AnnotatedT>
where
    AnnotatedT: Default,
{
    fn from_iter<IntoIteratorT>(iterator: IntoIteratorT) -> Self
    where
        IntoIteratorT: IntoIterator<Item = Variant<AnnotatedT>>,
    {
        List::from_iter(iterator).into()
    }
}

impl<AnnotatedT> FromIterator<(Variant<AnnotatedT>, Variant<AnnotatedT>)> for Variant<AnnotatedT>
where
    AnnotatedT: Default,
{
    fn from_iter<IntoIteratorT>(iterator: IntoIteratorT) -> Self
    where
        IntoIteratorT: IntoIterator<Item = (Variant<AnnotatedT>, Variant<AnnotatedT>)>,
    {
        Map::from_iter(iterator).into()
    }
}

impl<const SIZE: usize, AnnotatedT> From<[(Variant<AnnotatedT>, Variant<AnnotatedT>); SIZE]> for Variant<AnnotatedT>
where
    AnnotatedT: Default,
{
    fn from(array: [(Variant<AnnotatedT>, Variant<AnnotatedT>); SIZE]) -> Self {
        Map::from(array).into()
    }
}

// Variant -> common types

#[duplicate_item(
  FromNormalT        name                  ToT;
  [Integer]          ["integer"]           [i64];
  [UnsignedInteger]  ["unsigned integer"]  [u64];
  [Float]            ["float"]             [OrderedFloat<f64>];
  [Float]            ["float"]             [f64];
  [Boolean]          ["boolean"]           [bool];
  [Text]             ["text"]              [String];
  [Text]             ["text"]              [ByteString];
  [List]             ["list"]              [Vec<Variant<AnnotatedT>>];
  [Map]              ["Map"]               [BTreeMap<Variant<AnnotatedT>, Variant<AnnotatedT>>];
)]
#[allow(unused_variables)]
impl<AnnotatedT> TryFrom<Variant<AnnotatedT>> for ToT
where
    AnnotatedT: Annotated + Clone + Default,
{
    type Error = ConversionError<AnnotatedT>;

    fn try_from(value: Variant<AnnotatedT>) -> Result<Self, Self::Error> {
        match value {
            Variant::FromNormalT(normal) => Ok(normal.inner.into()),
            _ => Err(IncompatibleVariantTypeError::new(&value, &[name]).into()),
        }
    }
}

impl<AnnotatedT> TryFrom<Variant<AnnotatedT>> for ()
where
    AnnotatedT: Annotated + Clone + Default,
{
    type Error = ConversionError<AnnotatedT>;

    fn try_from(value: Variant<AnnotatedT>) -> Result<Self, Self::Error> {
        match value {
            Variant::Null(_) => Ok(()),
            _ => Err(IncompatibleVariantTypeError::new(&value, &["null"]).into()),
        }
    }
}

impl<AnnotatedT> TryFrom<Variant<AnnotatedT>> for Bytes
where
    AnnotatedT: Annotated + Clone + Default,
{
    type Error = ConversionError<AnnotatedT>;

    fn try_from(value: Variant<AnnotatedT>) -> Result<Self, Self::Error> {
        match value {
            Variant::Blob(blob) => Ok(blob.inner),
            Variant::Text(text) => Ok(text.inner.into_bytes()),
            _ => Err(IncompatibleVariantTypeError::new(&value, &["blob", "text"]).into()),
        }
    }
}

// &Variant -> inner types (via cloning or copying)

#[duplicate_item(
  FromNormalT  name         ToT                                                   normal_value;
  [Null]       ["null"]     [()]                                                  [()];
  [Float]      ["float"]    [OrderedFloat<f64>]                                   [normal.inner.into()];
  [Boolean]    ["boolean"]  [bool]                                                [normal.inner];
  [Text]       ["text"]     [String]                                              [normal.inner.clone().into()];
  [Text]       ["text"]     [ByteString]                                          [normal.inner.clone()];
  [List]       ["list"]     [Vec<Variant<AnnotatedT>>]                            [normal.inner.clone()];
  [Map]        ["Map"]      [BTreeMap<Variant<AnnotatedT>, Variant<AnnotatedT>>]  [normal.inner.clone()];
)]
#[allow(unused_variables)]
impl<AnnotatedT> TryFrom<&Variant<AnnotatedT>> for ToT
where
    AnnotatedT: Annotated + Clone + Default,
{
    type Error = ConversionError<AnnotatedT>;

    fn try_from(value: &Variant<AnnotatedT>) -> Result<Self, Self::Error> {
        match value {
            Variant::FromNormalT(normal) => Ok(normal_value),
            _ => Err(IncompatibleVariantTypeError::new(value, &[name]).into()),
        }
    }
}

impl<AnnotatedT> TryFrom<&Variant<AnnotatedT>> for Bytes
where
    AnnotatedT: Annotated + Clone + Default,
{
    type Error = ConversionError<AnnotatedT>;

    fn try_from(value: &Variant<AnnotatedT>) -> Result<Self, Self::Error> {
        match value {
            Variant::Blob(blob) => Ok(blob.inner.clone()),
            Variant::Text(text) => Ok(text.inner.clone().into_bytes()),
            _ => Err(IncompatibleVariantTypeError::new(&value, &["blob", "text"]).into()),
        }
    }
}

// &Variant -> inner references

#[duplicate_item(
    FromNormalT  name       ToT;
    [Text]       ["text"]   [str];
    [Blob]       ["blob"]   [[u8]];
    [List]       ["list"]   [Vec<Variant<AnnotatedT>>];
    [Map]        ["map"]    [BTreeMap<Variant<AnnotatedT>, Variant<AnnotatedT>>];
  )]
impl<'own, AnnotatedT> TryFrom<&'own Variant<AnnotatedT>> for &'own ToT
where
    AnnotatedT: Annotated + Clone + Default,
{
    type Error = ConversionError<AnnotatedT>;

    fn try_from(value: &'own Variant<AnnotatedT>) -> Result<Self, Self::Error> {
        match value {
            Variant::FromNormalT(normal) => Ok(normal.into()),
            _ => Err(IncompatibleVariantTypeError::new(value, &[name]).into()),
        }
    }
}

// &Variant -> numbers

#[duplicate_item(
    NumberT  name;
    [i128]   ["128-bit integer"];
    [i64]    ["64-bit integer"];
    [i32]    ["32-bit integer"];
    [i16]    ["16-bit integer"];
    [i8]     ["8-bit integer"];
    [isize]  ["system integer"];
    [u128]   ["128-bit unsigned integer"];
    [u64]    ["64-bit unsigned integer"];
    [u32]    ["32-bit unsigned integer"];
    [u16]    ["16-bit unsigned integer"];
    [u8]     ["8-bit unsigned integer"];
    [usize]  ["system unsigned integer"];
    [f64]    ["64-bit float"];
    [f32]    ["32-bit float"];
  )]
impl<AnnotatedT> TryFrom<&Variant<AnnotatedT>> for NumberT
where
    AnnotatedT: Annotated + Clone + Default,
{
    type Error = ConversionError<AnnotatedT>;

    fn try_from(value: &Variant<AnnotatedT>) -> Result<Self, Self::Error> {
        match value {
            Variant::Integer(integer) => num_traits::cast::<_, NumberT>(integer.inner)
                .ok_or_else(|| CastingError::new(value.clone(), name.into()).into()),

            Variant::UnsignedInteger(unsigned_integer) => num_traits::cast::<_, NumberT>(unsigned_integer.inner)
                .ok_or_else(|| CastingError::new(value.clone(), name.into()).into()),

            Variant::Float(float) => num_traits::cast::<f64, NumberT>(float.inner.into())
                .ok_or_else(|| CastingError::new(value.clone(), name.into()).into()),

            _ => Err(IncompatibleVariantTypeError::new(value, &["integer", "unsigned integer", "float"]).into()),
        }
    }
}
