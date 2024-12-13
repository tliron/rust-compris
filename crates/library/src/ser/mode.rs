use super::super::{hints::*, *};

//
// SerializationMode
//

/// Serialization mode.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct SerializationMode {
    /// Serialization mode for integers.
    pub integer: IntegerSerializationMode,

    /// Serialization mode for unsigned integer.
    pub unsigned_integer: UnsignedIntegerSerializationMode,

    /// Serialization mode for floats.
    pub float: FloatSerializationMode,

    /// Serialization mode for bytes.
    pub bytes: BytesSerializationMode,

    /// Serialization mode for maps.
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
    /// * [MapSerializationMode::KeysAsTexts]
    pub fn for_json() -> Self {
        Self {
            bytes: BytesSerializationMode::AsBase64(None),
            map: MapSerializationMode::KeysAsText,
            ..Default::default()
        }
    }

    /// Default serialization mode for XJSON.
    ///
    /// * [IntegerSerializationMode::AsText] with a hint
    /// * [UnsignedIntegerSerializationMode::AsText] with a hint
    /// * [BytesSerializationMode::AsBase64] with a hint
    /// * [MapSerializationMode::AsListIfNonTextKey] with a hint
    pub fn for_xjson() -> Self {
        let hints = Hints::xjson();
        Self {
            integer: IntegerSerializationMode::AsText(Some(hints.integer)),
            unsigned_integer: UnsignedIntegerSerializationMode::AsText(Some(hints.unsigned_integer)),
            bytes: BytesSerializationMode::AsBase64(Some(hints.bytes)),
            map: MapSerializationMode::AsListIfNonTextKey(Some(hints.map)),
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

    /// Integers serialized as text.
    ///
    /// If a hint is provided, then the integer text will be wrapped in a single-key map
    /// with the hint as the key. This map ignores the [MapSerializationMode].
    /// Can be deserialized by [read::to_hinted_value].
    AsText(Option<String>),
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

    /// Unsigned integers serialized as text.
    ///
    /// If a hint is provided, then the unsigned integer text will be wrapped in a single-key map
    /// with the hint as the key. This map ignores the [MapSerializationMode].
    /// Can be deserialized by [read::to_hinted_value].
    AsText(Option<String>),
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

    /// Floats serialized as text.
    ///
    /// If a hint is provided, then the float text will be wrapped in a single-key map
    /// with the hint as the key. This map ignores the [MapSerializationMode].
    /// Can be deserialized by [read::to_hinted_value].
    AsText(Option<String>),
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

    /// Bytes serialized as Base64 text.
    ///
    /// If a hint is provided, then the Base64 text will be wrapped in a single-key map
    /// with the hint as the key. This map ignores the [MapSerializationMode].
    /// Can be deserialized by [read::to_hinted_value].
    AsBase64(Option<String>),
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

    /// Coerces all map keys to text.
    KeysAsText,

    /// Maps serialized as lists of key-value pairs.
    ///
    /// If a hint is provided, then the list will be wrapped in a single-key map
    /// with the hint as the key. This map ignores the [MapSerializationMode].
    AsList(Option<String>),

    /// Maps serialized as lists of key-value pairs if one of the map keys is not text.
    /// Otherwise serialized as maps.
    ///
    /// If a hint is provided, then the list will be wrapped in a single-key map
    /// with the hint as the key. This map ignores the [MapSerializationMode].
    /// Can be deserialized by [read::to_hinted_value].
    AsListIfNonTextKey(Option<String>),
}

impl MapSerializationMode {
    /// Whether maps could potentially be serialized as lists.
    pub fn might_be_list(&self) -> bool {
        match self {
            MapSerializationMode::AsList(_) | MapSerializationMode::AsListIfNonTextKey(_) => true,
            _ => false,
        }
    }
}
