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
impl<AnnotationsT> From<NormalT<AnnotationsT>> for Value<AnnotationsT> {
    fn from(normal: NormalT<AnnotationsT>) -> Self {
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
  [List]             [Vec<Value<AnnotationsT>>];
  [Map]              [BTreeMap<Value<AnnotationsT>, Value<AnnotationsT>>];
)]
impl<AnnotationsT> From<FromT> for Value<AnnotationsT>
where
    AnnotationsT: Default,
{
    fn from(from_value: FromT) -> Self {
        Self::ToNormalT(ToNormalT::from(from_value))
    }
}

// Iterators -> Value

impl<AnnotationsT> FromIterator<Value<AnnotationsT>> for Value<AnnotationsT>
where
    AnnotationsT: Default,
{
    fn from_iter<IntoIteratorT>(iterator: IntoIteratorT) -> Self
    where
        IntoIteratorT: IntoIterator<Item = Value<AnnotationsT>>,
    {
        List::from_iter(iterator).into()
    }
}

impl<AnnotationsT> FromIterator<(Value<AnnotationsT>, Value<AnnotationsT>)> for Value<AnnotationsT>
where
    AnnotationsT: Default,
{
    fn from_iter<IntoIteratorT>(iterator: IntoIteratorT) -> Self
    where
        IntoIteratorT: IntoIterator<Item = (Value<AnnotationsT>, Value<AnnotationsT>)>,
    {
        Map::from_iter(iterator).into()
    }
}

// Value -> native types (possible cloning)

#[duplicate_item(
  FromNormalT        name                  ToT;
  [Integer]          ["integer"]           [i64];
  [UnsignedInteger]  ["unsigned integer"]  [u64];
  [Float]            ["float"]             [OrderedFloat<f64>];
  [Float]            ["float"]             [f64];
  [Boolean]          ["boolean"]           [bool];
  [Text]             ["text"]              [String];
  [Text]             ["text"]              [ByteString];
  [List]             ["list"]              [Vec<Value<AnnotationsT>>];
  [Map]              ["Map"]               [BTreeMap<Value<AnnotationsT>, Value<AnnotationsT>>];
)]
#[allow(unused_variables)]
impl<AnnotationsT> TryFrom<Value<AnnotationsT>> for ToT
where
    AnnotationsT: Annotated + Clone + Default,
{
    type Error = ConversionError<AnnotationsT>;

    fn try_from(value: Value<AnnotationsT>) -> Result<Self, Self::Error> {
        match value {
            Value::FromNormalT(normal) => Ok(normal.value.into()),
            _ => Err(IncompatibleValueTypeError::new(&value, &[name]).into()),
        }
    }
}

impl<AnnotationsT> TryFrom<Value<AnnotationsT>> for ()
where
    AnnotationsT: Annotated + Clone + Default,
{
    type Error = ConversionError<AnnotationsT>;

    fn try_from(value: Value<AnnotationsT>) -> Result<Self, Self::Error> {
        match value {
            Value::Null(_) => Ok(()),
            _ => Err(IncompatibleValueTypeError::new(&value, &["null"]).into()),
        }
    }
}

impl<AnnotationsT> TryFrom<Value<AnnotationsT>> for Bytes
where
    AnnotationsT: Annotated + Clone + Default,
{
    type Error = ConversionError<AnnotationsT>;

    fn try_from(value: Value<AnnotationsT>) -> Result<Self, Self::Error> {
        match value {
            Value::Blob(blob) => Ok(blob.value),
            Value::Text(text) => Ok(text.value.into_bytes()),
            _ => Err(IncompatibleValueTypeError::new(&value, &["blob", "text"]).into()),
        }
    }
}

// &Value -> native types (possible cloning)

#[duplicate_item(
  FromNormalT  name         ToT                                                   normal_value;
  [Null]       ["null"]     [()]                                                  [()];
  [Float]      ["float"]    [OrderedFloat<f64>]                                   [normal.value.into()];
  [Boolean]    ["boolean"]  [bool]                                                [normal.value];
  [Text]       ["text"]     [String]                                              [normal.value.clone().into()];
  [Text]       ["text"]     [ByteString]                                          [normal.value.clone()];
  [List]       ["list"]     [Vec<Value<AnnotationsT>>]                            [normal.value.clone()];
  [Map]        ["Map"]      [BTreeMap<Value<AnnotationsT>, Value<AnnotationsT>>]  [normal.value.clone()];
)]
#[allow(unused_variables)]
impl<AnnotationsT> TryFrom<&Value<AnnotationsT>> for ToT
where
    AnnotationsT: Annotated + Clone + Default,
{
    type Error = ConversionError<AnnotationsT>;

    fn try_from(value: &Value<AnnotationsT>) -> Result<Self, Self::Error> {
        match value {
            Value::FromNormalT(normal) => Ok(normal_value),
            _ => Err(IncompatibleValueTypeError::new(value, &[name]).into()),
        }
    }
}

impl<AnnotationsT> TryFrom<&Value<AnnotationsT>> for Bytes
where
    AnnotationsT: Annotated + Clone + Default,
{
    type Error = ConversionError<AnnotationsT>;

    fn try_from(value: &Value<AnnotationsT>) -> Result<Self, Self::Error> {
        match value {
            Value::Blob(blob) => Ok(blob.value.clone()),
            Value::Text(text) => Ok(text.value.clone().into_bytes()),
            _ => Err(IncompatibleValueTypeError::new(&value, &["blob", "text"]).into()),
        }
    }
}

// &Value -> native references

#[duplicate_item(
    FromNormalT  name       ToT;
    [Text]       ["text"]   [str];
    [Blob]       ["blob"]   [[u8]];
    [List]       ["list"]   [Vec<Value<AnnotationsT>>];
    [Map]        ["map"]    [BTreeMap<Value<AnnotationsT>, Value<AnnotationsT>>];
  )]
impl<'own, AnnotationsT> TryFrom<&'own Value<AnnotationsT>> for &'own ToT
where
    AnnotationsT: Annotated + Clone + Default,
{
    type Error = ConversionError<AnnotationsT>;

    fn try_from(value: &'own Value<AnnotationsT>) -> Result<Self, Self::Error> {
        match value {
            Value::FromNormalT(normal) => Ok(normal.into()),
            _ => Err(IncompatibleValueTypeError::new(value, &[name]).into()),
        }
    }
}

// &Value -> numbers

#[duplicate_item(
    ToT      name;
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
impl<AnnotationsT> TryFrom<&Value<AnnotationsT>> for ToT
where
    AnnotationsT: Annotated + Clone + Default,
{
    type Error = ConversionError<AnnotationsT>;

    fn try_from(value: &Value<AnnotationsT>) -> Result<Self, Self::Error> {
        match value {
            Value::Integer(integer) => num_traits::cast::<_, ToT>(integer.value)
                .ok_or_else(|| CastingError::new(&integer.to_string(), name).into()),

            Value::UnsignedInteger(unsigned_integer) => num_traits::cast::<_, ToT>(unsigned_integer.value)
                .ok_or_else(|| CastingError::new(&unsigned_integer.to_string(), name).into()),

            Value::Float(float) => num_traits::cast::<f64, ToT>(float.value.into())
                .ok_or_else(|| CastingError::new(&float.to_string(), name).into()),

            _ => Err(IncompatibleValueTypeError::new(value, &["integer", "unsigned integer", "float"]).into()),
        }
    }
}
