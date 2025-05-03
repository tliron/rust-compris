use super::{
    boolean::*, bytes::*, errors::*, float::*, integer::*, list::*, map::*, null::*, text::*, unsigned_integer::*,
    value::*,
};

use {duplicate::*, ordered_float::OrderedFloat, std::collections::*};

// Normal type -> Value

#[duplicate_item(
  _Normal;
  [Null];
  [Integer];
  [UnsignedInteger];
  [Float];
  [Boolean];
  [Text];
  [Bytes];
  [List];
  [Map];
)]
impl From<_Normal> for Value {
    fn from(normal: _Normal) -> Self {
        Self::_Normal(normal)
    }
}

// Common types -> Value

#[duplicate_item(
  _ToNormal          _From;
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
  [Text]             [String];
  [Bytes]            [Vec<u8>];
  [Bytes]            [&[u8]];
  [Text]             [&str];
  [List]             [Vec<Value>];
  [Map]              [BTreeMap<Value, Value>];
)]
impl From<_From> for Value {
    fn from(from_value: _From) -> Self {
        Self::_ToNormal(from_value.into())
    }
}

// Value -> native types

#[duplicate_item(
  _FromNormal        _Name                 _To                       _As;
  [Null]             ["null"]              [()]                      [()];
  [Float]            ["float"]             [OrderedFloat<f64>]       [normal.value.into()];
  [Boolean]          ["boolean"]           [bool]                    [normal.value];
  [Text]             ["text"]              [String]                  [normal.value.clone()];
  [Bytes]            ["bytes"]             [Vec<u8>]                 [normal.value.clone()];
  [List]             ["list"]              [Vec<Value>]              [normal.value.clone()];
  [Map]              ["Map"]               [BTreeMap<Value, Value>]  [normal.value.clone()];
)]
#[allow(unused_variables)]
impl TryFrom<&Value> for _To {
    type Error = ConversionError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::_FromNormal(normal) => Ok(_As),
            _ => Err(IncompatibleValueTypeError::new(value, &[_Name]).into()),
        }
    }
}

// Value -> native references

#[duplicate_item(
    _FromNormal  _Name      _To;
    [Text]       ["text"]   [str];
    [Bytes]      ["bytes"]  [[u8]];
    [List]       ["list"]   [Vec<Value>];
    [Map]        ["map"]    [BTreeMap<Value, Value>];
  )]
impl<'own> TryFrom<&'own Value> for &'own _To {
    type Error = ConversionError;

    fn try_from(value: &'own Value) -> Result<Self, Self::Error> {
        match value {
            Value::_FromNormal(normal) => Ok(normal.into()),
            _ => Err(IncompatibleValueTypeError::new(value, &[_Name]).into()),
        }
    }
}

// Value -> numbers

#[duplicate_item(
    _To      _Name;
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
impl TryFrom<&Value> for _To {
    type Error = ConversionError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Integer(integer) => num_traits::cast::<_, _To>(integer.value)
                .ok_or_else(|| CastingError::new(&integer.to_string(), _Name).into()),

            Value::UnsignedInteger(unsigned_integer) => num_traits::cast::<_, _To>(unsigned_integer.value)
                .ok_or_else(|| CastingError::new(&unsigned_integer.to_string(), _Name).into()),

            Value::Float(float) => num_traits::cast::<f64, _To>(float.value.into())
                .ok_or_else(|| CastingError::new(&float.to_string(), _Name).into()),

            _ => Err(IncompatibleValueTypeError::new(value, &["integer", "unsigned integer", "float"]).into()),
        }
    }
}
