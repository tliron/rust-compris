use super::super::*;

use std::string::String as StdString;

//
// SerializationMode
//

/// Serialization mode.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct SerializationMode {
    /// Integer serialization mode.
    pub integer: IntegerSerializationMode,

    /// Unsigned integer serialization mode.
    pub unsigned_integer: UnsignedIntegerSerializationMode,

    /// Float serialization mode.
    pub float: FloatSerializationMode,

    /// Bytes serialization mode.
    pub bytes: BytesSerializationMode,

    /// Map serialization mode.
    pub map: MapSerializationMode,
}

impl SerializationMode {
    /// Default serialization mode for a format, if available.
    pub fn for_format(format: &Format) -> Option<Self> {
        match format {
            Format::YAML => Some(Self::for_yaml()),
            Format::JSON => Some(Self::for_json()),
            Format::XJSON => Some(Self::for_xjson()),
            _ => None,
        }
    }

    /// Default serialization mode for YAML.
    ///
    /// * [BytesSerializationMode::AsBase64]
    pub fn for_yaml() -> Self {
        Self { bytes: BytesSerializationMode::AsBase64(None), ..Default::default() }
    }

    /// Default serialization mode for JSON.
    ///
    /// * [BytesSerializationMode::AsBase64]
    /// * [MapSerializationMode::KeysAsStrings]
    pub fn for_json() -> Self {
        Self {
            bytes: BytesSerializationMode::AsBase64(None),
            map: MapSerializationMode::KeysAsStrings,
            ..Default::default()
        }
    }

    /// Default serialization mode for XJSON.
    ///
    /// * [IntegerSerializationMode::AsString] with [XJSON_HINT_INTEGER]
    /// * [UnsignedIntegerSerializationMode::AsString] with [XJSON_HINT_UNSIGNED_INTEGER]
    /// * [BytesSerializationMode::AsBase64] with [XJSON_HINT_BYTES]
    /// * [MapSerializationMode::AsListIfNonStringKey] with [XJSON_HINT_MAP]
    pub fn for_xjson() -> Self {
        let hints = Hints::xjson();
        Self {
            integer: IntegerSerializationMode::AsString(Some(hints.integer)),
            unsigned_integer: UnsignedIntegerSerializationMode::AsString(Some(hints.unsigned_integer)),
            bytes: BytesSerializationMode::AsBase64(Some(hints.bytes)),
            map: MapSerializationMode::AsListIfNonStringKey(Some(hints.map)),
            ..Default::default()
        }
    }
}

//
// IntegerSerializationMode
//

/// Integer serialization mode.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum IntegerSerializationMode {
    /// Integers serialized as integers (the default).
    #[default]
    AsInteger,

    /// Integers serialized as unsigned integers if they are non-negative.
    /// Otherwise serialized as integers.
    AsUnsignedIntegerIfNonNegative,

    /// Integers serialized as floats.
    AsFloat,

    /// Integers serialized as strings.
    ///
    /// If a hint is provided, then the integer string will be wrapped in a single-key map
    /// with the hint as the key. This map ignores the [MapSerializationMode].
    /// Can be deserialized by [read::to_hinted_value].
    AsString(Option<StdString>),
}

impl IntegerSerializationMode {
    /// Whether integers could potentially be serialized as floats.
    pub fn might_be_float(&self) -> bool {
        match self {
            IntegerSerializationMode::AsFloat => true,
            _ => false,
        }
    }
}

//
// UnsignedIntegerSerializationMode
//

/// Unsigned integer serialization mode.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum UnsignedIntegerSerializationMode {
    /// Unsigned integers serialized as unsigned integers (the default).
    #[default]
    AsUnsignedInteger,

    /// Unsigned integers serialized as integers.
    AsInteger,

    /// Unsigned integers serialized as floats.
    AsFloat,

    /// Unsigned integers serialized as strings.
    ///
    /// If a hint is provided, then the unsigned integer string will be wrapped in a single-key map
    /// with the hint as the key. This map ignores the [MapSerializationMode].
    /// Can be deserialized by [read::to_hinted_value].
    AsString(Option<StdString>),
}

impl UnsignedIntegerSerializationMode {
    /// Whether unsigned integers could potentially be serialized as integers.
    pub fn might_be_integer(&self) -> bool {
        match self {
            UnsignedIntegerSerializationMode::AsInteger => true,
            _ => false,
        }
    }
}

//
// FloatSerializationMode
//

/// Float serialization mode.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum FloatSerializationMode {
    /// Floats serialized as floats (the default).
    #[default]
    AsFloat,

    /// Floats serialized as integers (after [f64::trunc]).
    AsInteger,

    /// Floats serialized as integers if they have no fraction.
    /// Otherwise serialized as floats.
    AsIntegerIfFractionless,

    /// Floats serialized as strings.
    ///
    /// If a hint is provided, then the float string will be wrapped in a single-key map
    /// with the hint as the key. This map ignores the [MapSerializationMode].
    /// Can be deserialized by [read::to_hinted_value].
    AsString(Option<StdString>),
}

impl FloatSerializationMode {
    /// Whether floats could potentially be serialized as integers.
    pub fn might_be_integer(&self) -> bool {
        match self {
            FloatSerializationMode::AsInteger | FloatSerializationMode::AsIntegerIfFractionless => true,
            _ => false,
        }
    }
}

//
// BytesSerializationMode
//

/// Bytes serialization mode.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum BytesSerializationMode {
    /// Bytes serialized as bytes (the default).
    #[default]
    AsBytes,

    /// Bytes serialized as Base64 strings.
    ///
    /// If a hint is provided, then the Base64 string will be wrapped in a single-key map
    /// with the hint as the key. This map ignores the [MapSerializationMode].
    /// Can be deserialized by [read::to_hinted_value].
    AsBase64(Option<StdString>),
}

//
// MapSerializationMode
//

/// Map serialization mode.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum MapSerializationMode {
    /// Maps serialized as maps (the default).
    #[default]
    AsMap,

    /// Coerces all map keys to strings.
    KeysAsStrings,

    /// Maps serialized as lists of key-value pairs.
    ///
    /// If a hint is provided, then the list will be wrapped in a single-key map
    /// with the hint as the key. This map ignores the [MapSerializationMode].
    AsList(Option<StdString>),

    /// Maps serialized as lists of key-value pairs if one of the map keys is not a string.
    /// Otherwise serialized as maps.
    ///
    /// If a hint is provided, then the list will be wrapped in a single-key map
    /// with the hint as the key. This map ignores the [MapSerializationMode].
    /// Can be deserialized by [read::to_hinted_value].
    AsListIfNonStringKey(Option<StdString>),
}

impl MapSerializationMode {
    /// Whether maps could potentially be serialized as lists.
    pub fn might_be_list(&self) -> bool {
        match self {
            MapSerializationMode::AsList(_) | MapSerializationMode::AsListIfNonStringKey(_) => true,
            _ => false,
        }
    }
}
