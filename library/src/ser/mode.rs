use super::super::{hints::*, *};

//
// SerializationMode
//

/// Serialization mode.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SerializationMode {
    /// Serialization mode for integers.
    pub integer: IntegerSerializationMode,

    /// Serialization mode for unsigned integers.
    pub unsigned_integer: UnsignedIntegerSerializationMode,

    /// Serialization mode for floats.
    pub float: FloatSerializationMode,

    /// Serialization mode for blobs.
    pub blob: BlobSerializationMode,

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
    /// * [BytesSerializationMode::StringifyBase64]
    pub fn for_yaml() -> Self {
        Self { blob: BlobSerializationMode::StringifyBase64(None), ..Default::default() }
    }

    /// Default serialization mode for JSON.
    ///
    /// * [BytesSerializationMode::StringifyBase64]
    /// * [MapSerializationMode::SerializeKeysIfNonText]
    pub fn for_json() -> Self {
        Self {
            blob: BlobSerializationMode::StringifyBase64(None),
            map: MapSerializationMode::SerializeKeysIfNonText,
            ..Default::default()
        }
    }

    /// Default serialization mode for XJSON.
    ///
    /// * [IntegerSerializationMode::Stringify] with a hint
    /// * [UnsignedIntegerSerializationMode::Stringify] with a hint
    /// * [BytesSerializationMode::StringifyBase64] with a hint
    /// * [MapSerializationMode::AsSeqIfNonTextKey] with a hint
    pub fn for_xjson() -> Self {
        let hints = Hints::xjson();
        Self {
            integer: IntegerSerializationMode::Stringify(Some(hints.integer)),
            unsigned_integer: UnsignedIntegerSerializationMode::Stringify(Some(hints.unsigned_integer)),
            blob: BlobSerializationMode::StringifyBase64(Some(hints.bytes)),
            map: MapSerializationMode::AsSeqIfNonTextKey(Some(hints.map)),
            ..Default::default()
        }
    }
}

//
// IntegerSerializationMode
//

/// Integer serialization mode.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum IntegerSerializationMode {
    /// Integers are serialized as i64 (the default).
    #[default]
    AsI64,

    /// Integers are serialized as u64 if they are non-negative. Otherwise serialized as i64.
    AsU64IfNonNegative,

    /// Integers are serialized as f64.
    ///
    /// If information would be lost will cause a serialization error.
    AsF64,

    /// Stringify integers in decimal.
    ///
    /// If a hint is provided, then the string will be wrapped in a single-key map with the hint as
    /// the key. This map ignores the [MapSerializationMode].
    ///
    /// Can be deserialized by
    /// [Variant::to_hinted_variant](super::super::normal::Variant::to_hinted_variant).
    Stringify(Option<String>),
}

impl IntegerSerializationMode {
    /// Whether integers could potentially be serialized as floats.
    pub fn might_be_float(&self) -> bool {
        matches!(self, IntegerSerializationMode::AsF64)
    }
}

//
// UnsignedIntegerSerializationMode
//

/// Unsigned integer serialization mode.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum UnsignedIntegerSerializationMode {
    /// Unsigned integers are serialized as u64 (the default).
    #[default]
    AsU64,

    /// Unsigned integers are serialized as i64.
    ///
    /// If information would be lost will cause a serialization error.
    AsI64,

    /// Unsigned integers are serialized as floats.
    ///
    /// If information would be lost will cause a serialization error.
    AsF64,

    /// Stringify unsigned integers in decimal.
    ///
    /// If a hint is provided, then the string will be wrapped in a single-key map with the hint as
    /// the key. This map ignores the [MapSerializationMode].
    ///
    /// Can be deserialized by
    /// [Variant::to_hinted_variant](super::super::normal::Variant::to_hinted_variant).
    Stringify(Option<String>),
}

impl UnsignedIntegerSerializationMode {
    /// Whether unsigned integers could potentially be serialized as integers.
    pub fn might_be_integer(&self) -> bool {
        matches!(self, UnsignedIntegerSerializationMode::AsI64)
    }
}

//
// FloatSerializationMode
//

/// Float serialization mode.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum FloatSerializationMode {
    /// Floats are serialized as f64 (the default).
    #[default]
    AsF64,

    /// Floats are serialized as i64 (after [f64::trunc]).
    ///
    /// If information would be lost will cause a serialization error.
    AsI64,

    /// Floats are serialized as integers if they have no fraction and will not lose information by
    /// conversion. Otherwise serialized as floats.
    AsI64IfWhole,

    /// Stringify floats in decimal.
    ///
    /// If a hint is provided, then the string will be wrapped in a single-key map with the hint as
    /// the key. This map ignores the [MapSerializationMode].
    ///
    /// Can be deserialized by
    /// [Variant::to_hinted_variant](super::super::normal::Variant::to_hinted_variant).
    Stringify(Option<String>),
}

impl FloatSerializationMode {
    /// Whether floats could potentially be serialized as integers.
    pub fn might_be_integer(&self) -> bool {
        matches!(self, FloatSerializationMode::AsI64 | FloatSerializationMode::AsI64IfWhole)
    }
}

//
// BlobSerializationMode
//

/// Blob serialization mode.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum BlobSerializationMode {
    /// Blobs are serialized as bytes (the default).
    #[default]
    AsBytes,

    /// Stringify blobs as Base64.
    ///
    /// If a hint is provided, then the string will be wrapped in a single-key map with the hint as
    /// the key. This map ignores the [MapSerializationMode].
    ///
    /// Can be deserialized by
    /// [Variant::to_hinted_variant](super::super::normal::Variant::to_hinted_variant).
    StringifyBase64(Option<String>),
}

//
// MapSerializationMode
//

/// Map serialization mode.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum MapSerializationMode {
    /// Maps are serialized as maps (the default).
    #[default]
    AsMap,

    /// Maps are serialized as sequences of key-value pairs.
    ///
    /// If a hint is provided, then the sequence will be wrapped in a single-key map with the hint
    /// as the key. This map ignores the [MapSerializationMode].
    ///
    /// Can be deserialized by
    /// [Variant::to_hinted_variant](super::super::normal::Variant::to_hinted_variant).
    AsSeq(Option<String>),

    /// Maps are serialized as sequences of key-value pairs *only if* there is a key that is *not*
    /// [Text](super::super::normal::Text). Otherwise serialized as maps.
    ///
    /// If a hint is provided, then the sequence will be wrapped in a single-key map with the hint
    /// as the key. This map ignores the [MapSerializationMode].
    ///
    /// Can be deserialized by
    /// [Variant::to_hinted_variant](super::super::normal::Variant::to_hinted_variant).
    AsSeqIfNonTextKey(Option<String>),

    /// [Serializer::stringify](super::serializer::Serializer::stringify) all map keys before
    /// serialization.
    ///
    /// Deserialization would thus require parsing these serialized keys as embedded documents.
    ///
    /// Note that the format depends on the serializer, e.g. serialized keys would be different
    /// for YAML and JSON.
    SerializeKeys,

    /// [Serializer::stringify](super::serializer::Serializer::stringify) map keys that are not
    /// [Text](super::super::normal::Text) before serialization. [Text](super::super::normal::Text)
    /// keys will be serialized normally.
    ///
    /// Deserialization would thus require parsing the serialized keys as embedded documents.
    ///
    /// Note that the format depends on the serializer, e.g. serialized keys would be different
    /// for YAML and JSON.
    SerializeKeysIfNonText,
}
