use super::{
    super::annotation::*, blob::*, boolean::*, errors::*, float::*, integer::*, list::*, map::*, null::*, text::*,
    unsigned_integer::*, value::*,
};

use {
    bytes::*,
    bytestring::*,
    duplicate::*,
    ordered_float::OrderedFloat,
    std::{borrow::*, collections::*},
};

// Normal type -> Value

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
impl<AnnotatedT> From<NormalT<AnnotatedT>> for Value<AnnotatedT> {
    fn from(normal: NormalT<AnnotatedT>) -> Self {
        Self::NormalT(normal)
    }
}

// Common types -> Value

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
  [List]             [Vec<Value<AnnotatedT>>];
  [Map]              [BTreeMap<Value<AnnotatedT>, Value<AnnotatedT>>];
)]
impl<AnnotatedT> From<FromT> for Value<AnnotatedT>
where
    AnnotatedT: Default,
{
    fn from(from_value: FromT) -> Self {
        Self::ToNormalT(ToNormalT::from(from_value))
    }
}

// Iterators -> Value

impl<AnnotatedT> FromIterator<Value<AnnotatedT>> for Value<AnnotatedT>
where
    AnnotatedT: Default,
{
    fn from_iter<IntoIteratorT>(iterator: IntoIteratorT) -> Self
    where
        IntoIteratorT: IntoIterator<Item = Value<AnnotatedT>>,
    {
        List::from_iter(iterator).into()
    }
}

impl<AnnotatedT> FromIterator<(Value<AnnotatedT>, Value<AnnotatedT>)> for Value<AnnotatedT>
where
    AnnotatedT: Default,
{
    fn from_iter<IntoIteratorT>(iterator: IntoIteratorT) -> Self
    where
        IntoIteratorT: IntoIterator<Item = (Value<AnnotatedT>, Value<AnnotatedT>)>,
    {
        Map::from_iter(iterator).into()
    }
}

// Value -> common types

#[duplicate_item(
  FromNormalT        name                  ToT;
  [Integer]          ["integer"]           [i64];
  [UnsignedInteger]  ["unsigned integer"]  [u64];
  [Float]            ["float"]             [OrderedFloat<f64>];
  [Float]            ["float"]             [f64];
  [Boolean]          ["boolean"]           [bool];
  [Text]             ["text"]              [String];
  [Text]             ["text"]              [ByteString];
  [List]             ["list"]              [Vec<Value<AnnotatedT>>];
  [Map]              ["Map"]               [BTreeMap<Value<AnnotatedT>, Value<AnnotatedT>>];
)]
#[allow(unused_variables)]
impl<AnnotatedT> TryFrom<Value<AnnotatedT>> for ToT
where
    AnnotatedT: Annotated + Clone + Default,
{
    type Error = ConversionError<AnnotatedT>;

    fn try_from(value: Value<AnnotatedT>) -> Result<Self, Self::Error> {
        match value {
            Value::FromNormalT(normal) => Ok(normal.inner.into()),
            _ => Err(IncompatibleValueTypeError::new(&value, &[name]).into()),
        }
    }
}

impl<AnnotatedT> TryFrom<Value<AnnotatedT>> for ()
where
    AnnotatedT: Annotated + Clone + Default,
{
    type Error = ConversionError<AnnotatedT>;

    fn try_from(value: Value<AnnotatedT>) -> Result<Self, Self::Error> {
        match value {
            Value::Null(_) => Ok(()),
            _ => Err(IncompatibleValueTypeError::new(&value, &["null"]).into()),
        }
    }
}

impl<AnnotatedT> TryFrom<Value<AnnotatedT>> for Bytes
where
    AnnotatedT: Annotated + Clone + Default,
{
    type Error = ConversionError<AnnotatedT>;

    fn try_from(value: Value<AnnotatedT>) -> Result<Self, Self::Error> {
        match value {
            Value::Blob(blob) => Ok(blob.inner),
            Value::Text(text) => Ok(text.inner.into_bytes()),
            _ => Err(IncompatibleValueTypeError::new(&value, &["blob", "text"]).into()),
        }
    }
}

// &Value -> inner types (via cloning or copying)

#[duplicate_item(
  FromNormalT  name         ToT                                                   normal_value;
  [Null]       ["null"]     [()]                                                  [()];
  [Float]      ["float"]    [OrderedFloat<f64>]                                   [normal.inner.into()];
  [Boolean]    ["boolean"]  [bool]                                                [normal.inner];
  [Text]       ["text"]     [String]                                              [normal.inner.clone().into()];
  [Text]       ["text"]     [ByteString]                                          [normal.inner.clone()];
  [List]       ["list"]     [Vec<Value<AnnotatedT>>]                            [normal.inner.clone()];
  [Map]        ["Map"]      [BTreeMap<Value<AnnotatedT>, Value<AnnotatedT>>]  [normal.inner.clone()];
)]
#[allow(unused_variables)]
impl<AnnotatedT> TryFrom<&Value<AnnotatedT>> for ToT
where
    AnnotatedT: Annotated + Clone + Default,
{
    type Error = ConversionError<AnnotatedT>;

    fn try_from(value: &Value<AnnotatedT>) -> Result<Self, Self::Error> {
        match value {
            Value::FromNormalT(normal) => Ok(normal_value),
            _ => Err(IncompatibleValueTypeError::new(value, &[name]).into()),
        }
    }
}

impl<AnnotatedT> TryFrom<&Value<AnnotatedT>> for Bytes
where
    AnnotatedT: Annotated + Clone + Default,
{
    type Error = ConversionError<AnnotatedT>;

    fn try_from(value: &Value<AnnotatedT>) -> Result<Self, Self::Error> {
        match value {
            Value::Blob(blob) => Ok(blob.inner.clone()),
            Value::Text(text) => Ok(text.inner.clone().into_bytes()),
            _ => Err(IncompatibleValueTypeError::new(&value, &["blob", "text"]).into()),
        }
    }
}

// &Value -> inner references

#[duplicate_item(
    FromNormalT  name       ToT;
    [Text]       ["text"]   [str];
    [Blob]       ["blob"]   [[u8]];
    [List]       ["list"]   [Vec<Value<AnnotatedT>>];
    [Map]        ["map"]    [BTreeMap<Value<AnnotatedT>, Value<AnnotatedT>>];
  )]
impl<'own, AnnotatedT> TryFrom<&'own Value<AnnotatedT>> for &'own ToT
where
    AnnotatedT: Annotated + Clone + Default,
{
    type Error = ConversionError<AnnotatedT>;

    fn try_from(value: &'own Value<AnnotatedT>) -> Result<Self, Self::Error> {
        match value {
            Value::FromNormalT(normal) => Ok(normal.into()),
            _ => Err(IncompatibleValueTypeError::new(value, &[name]).into()),
        }
    }
}

// &Value -> numbers

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
impl<AnnotatedT> TryFrom<&Value<AnnotatedT>> for NumberT
where
    AnnotatedT: Annotated + Clone + Default,
{
    type Error = ConversionError<AnnotatedT>;

    fn try_from(value: &Value<AnnotatedT>) -> Result<Self, Self::Error> {
        match value {
            Value::Integer(integer) => num_traits::cast::<_, NumberT>(integer.inner)
                .ok_or_else(|| CastingError::new(value, name.into()).into()),

            Value::UnsignedInteger(unsigned_integer) => num_traits::cast::<_, NumberT>(unsigned_integer.inner)
                .ok_or_else(|| CastingError::new(value, name.into()).into()),

            Value::Float(float) => num_traits::cast::<f64, NumberT>(float.inner.into())
                .ok_or_else(|| CastingError::new(value, name.into()).into()),

            _ => Err(IncompatibleValueTypeError::new(value, &["integer", "unsigned integer", "float"]).into()),
        }
    }
}
