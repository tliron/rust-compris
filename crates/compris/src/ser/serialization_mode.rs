use super::super::hints::*;

use std::sync::*;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct SerializationMode {
    pub integer: IntegerSerializationMode,
    pub unsigned_integer: UnsignedIntegerSerializationMode,
    pub float: FloatSerializationMode,
    pub bytes: BytesSerializationMode,
    pub map: MapSerializationMode,
}

impl SerializationMode {
    pub fn for_yaml() -> &'static Self {
        static ONCE: OnceLock<SerializationMode> = OnceLock::new();
        ONCE.get_or_init(|| Self { bytes: BytesSerializationMode::AsBase64(None), ..Default::default() })
    }

    pub fn for_json() -> &'static Self {
        static ONCE: OnceLock<SerializationMode> = OnceLock::new();
        ONCE.get_or_init(|| Self {
            bytes: BytesSerializationMode::AsBase64(None),
            map: MapSerializationMode::AsListIfNonStringKey(None),
            ..Default::default()
        })
    }

    pub fn for_xjson() -> &'static Self {
        static ONCE: OnceLock<SerializationMode> = OnceLock::new();
        ONCE.get_or_init(|| Self {
            integer: IntegerSerializationMode::AsString(Some(ARD_HINT_INTEGER.into())),
            unsigned_integer: UnsignedIntegerSerializationMode::AsString(Some(ARD_HINT_UNSIGNED_INTEGER.into())),
            bytes: BytesSerializationMode::AsBase64(Some(ARD_HINT_BYTES.into())),
            map: MapSerializationMode::AsListIfNonStringKey(Some(ARD_HINT_MAP.into())),
            ..Default::default()
        })
    }
}

//
// IntegerSerializationMode
//

/// Bytes serialization mode.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum IntegerSerializationMode {
    // Integers serialized as integers (the default).
    #[default]
    AsInteger,

    // Integers serialized as unsigned integers if they are non-negative.
    // Otherwise serialized as integers.
    AsUnsignedIntegerIfNonNegative,

    // Integers serialized as floats.
    AsFloat,

    // Integers serialized as strings.
    //
    // If a hint is provided, then the integer string will be wrapped in a single-key map
    // with the hint as the key. This map ignores the [MapSerializationMode].
    // Can be deserialized by [to_hinted_value].
    AsString(Option<String>),
}

impl IntegerSerializationMode {
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

/// Bytes serialization mode.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum UnsignedIntegerSerializationMode {
    // Unsigned integers serialized as unsigned integers (the default).
    #[default]
    AsUnsignedInteger,

    // Unsigned integers serialized as integers.
    AsInteger,

    // Unsigned integers serialized as floats.
    AsFloat,

    // Unsigned integers serialized as strings.
    //
    // If a hint is provided, then the unsigned integer string will be wrapped in a single-key map
    // with the hint as the key. This map ignores the [MapSerializationMode].
    // Can be deserialized by [to_hinted_value].
    AsString(Option<String>),
}

impl UnsignedIntegerSerializationMode {
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

/// Bytes serialization mode.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum FloatSerializationMode {
    // Floats serialized as floats (the default).
    #[default]
    AsFloat,

    // Floats serialized as integers (after [f64::trunc]).
    AsInteger,

    // Floats serialized as integers if they have no fraction.
    // Otherwise serialized as floats.
    AsIntegerIfFractionless,

    // Floats serialized as strings.
    //
    // If a hint is provided, then the float string will be wrapped in a single-key map
    // with the hint as the key. This map ignores the [MapSerializationMode].
    // Can be deserialized by [to_hinted_value].
    AsString(Option<String>),
}

impl FloatSerializationMode {
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
    // Bytes serialized as bytes (the default).
    #[default]
    AsBytes,

    // Bytes serialized as Base64 strings.
    //
    // If a hint is provided, then the Base64 string will be wrapped in a single-key map
    // with the hint as the key. This map ignores the [MapSerializationMode].
    // Can be deserialized by [to_hinted_value].
    AsBase64(Option<String>),
}

//
// MapSerializationMode
//

/// Map serialization mode.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum MapSerializationMode {
    // Maps serialized as maps (the default).
    #[default]
    AsMap,

    // Coerces all map keys to strings.
    KeysAsStrings,

    // Maps serialized as lists of key-value pairs.
    //
    // If a hint is provided, then the list will be wrapped in a single-key map
    // with the hint as the key. This map ignores the [MapSerializationMode].
    AsList(Option<String>),

    // Maps serialized as lists of key-value pairs if one of the map keys is not a string.
    // Otherwise serialized as maps.
    //
    // If a hint is provided, then the list will be wrapped in a single-key map
    // with the hint as the key. This map ignores the [MapSerializationMode].
    // Can be deserialized by [to_hinted_value].
    AsListIfNonStringKey(Option<String>),
}

impl MapSerializationMode {
    pub fn might_be_list(&self) -> bool {
        match self {
            MapSerializationMode::AsList(_) | MapSerializationMode::AsListIfNonStringKey(_) => true,
            _ => false,
        }
    }
}
