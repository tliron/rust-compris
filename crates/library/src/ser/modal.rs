use super::{mode::*, serializer::Serializer as ModalSerializer};

use serde::ser::*;

//
// SerializeModal
//

/// Like [Serialize] but with support for a [SerializationMode].
pub trait SerializeModal {
    /// Serialize with [SerializationMode].
    fn serialize_modal<SerializerT>(
        &self,
        serializer: SerializerT,
        mode: &SerializationMode,
    ) -> Result<SerializerT::Ok, SerializerT::Error>
    where
        SerializerT: Serializer;
}

//
// SerializeModalRescursive
//

/// Like [Serialize] but with support for a [SerializationMode]
/// and an embedded [ModalSerializer].
pub trait SerializeModalRescursive {
    /// Serialize with [SerializationMode] and an embedded [Serializer](ModalSerializer).
    fn serialize_modal<SerializerT>(
        &self,
        serializer: SerializerT,
        mode: &SerializationMode,
        modal_serializer: &ModalSerializer,
    ) -> Result<SerializerT::Ok, SerializerT::Error>
    where
        SerializerT: Serializer;
}

//
// ModalSerializable
//

/// Provides a [Serialize] implementation for a [SerializeModal].
pub struct ModalSerializable<'own, SerializeModalT>
where
    SerializeModalT: SerializeModal,
{
    /// Wrapped serializable.
    pub serializable: &'own SerializeModalT,

    /// Serialization mode.
    pub mode: &'own SerializationMode,
}

impl<'own, SerializeModalT> ModalSerializable<'own, SerializeModalT>
where
    SerializeModalT: SerializeModal,
{
    /// Constructor.
    pub fn new(serializable: &'own SerializeModalT, mode: &'own SerializationMode) -> Self {
        Self { serializable, mode }
    }
}

// Delegates

impl<'own, SerializeModalT> Serialize for ModalSerializable<'own, SerializeModalT>
where
    SerializeModalT: SerializeModal,
{
    fn serialize<SerializerT>(&self, serializer: SerializerT) -> Result<SerializerT::Ok, SerializerT::Error>
    where
        SerializerT: Serializer,
    {
        self.serializable.serialize_modal(serializer, self.mode)
    }
}

//
// RecursiveModalSerializable
//

/// Provides a [Serialize] implementation for a [SerializeModalRescursive].
pub struct RecursiveModalSerializable<'own, SerializeModalT>
where
    SerializeModalT: SerializeModalRescursive,
{
    /// Wrapped modal serializable.
    pub serializable: &'own SerializeModalT,

    /// Serialization mode.
    pub mode: &'own SerializationMode,

    /// Modal serializer.
    pub serializer: &'own ModalSerializer,
}

impl<'own, SerializeModalT> RecursiveModalSerializable<'own, SerializeModalT>
where
    SerializeModalT: SerializeModalRescursive,
{
    /// Constructor.
    pub fn new(
        serializable: &'own SerializeModalT,
        mode: &'own SerializationMode,
        serializer: &'own ModalSerializer,
    ) -> Self {
        Self { serializable, mode, serializer }
    }
}

// Delegates

impl<'own, SerializeModalT> Serialize for RecursiveModalSerializable<'own, SerializeModalT>
where
    SerializeModalT: SerializeModalRescursive,
{
    fn serialize<SerializerT>(&self, serializer: SerializerT) -> Result<SerializerT::Ok, SerializerT::Error>
    where
        SerializerT: Serializer,
    {
        self.serializable.serialize_modal(serializer, self.mode, self.serializer)
    }
}

//
// Modal
//

/// Wraps a [SerializeModal] with a [ModalSerializable].
pub trait Modal<SerializeModalT>
where
    SerializeModalT: SerializeModal,
{
    /// Wraps a [SerializeModal] with a [ModalSerializable].
    fn modal<'own>(&'own self, mode: &'own SerializationMode) -> ModalSerializable<'own, SerializeModalT>;
}

impl<SerializeModalT> Modal<SerializeModalT> for SerializeModalT
where
    SerializeModalT: SerializeModal,
{
    fn modal<'own>(&'own self, mode: &'own SerializationMode) -> ModalSerializable<'own, Self> {
        ModalSerializable::new(self, mode)
    }
}

//
// RecursiveModal
//

/// Wraps a [SerializeModalRescursive] with a [RecursiveModalSerializable].
pub trait RecursiveModal<SerializeModalT>
where
    SerializeModalT: SerializeModalRescursive,
{
    /// Wraps a [SerializeModalRescursive] with a [RecursiveModalSerializable].
    fn modal<'own>(
        &'own self,
        mode: &'own SerializationMode,
        serializer: &'own ModalSerializer,
    ) -> RecursiveModalSerializable<'own, SerializeModalT>;
}

impl<ModalSerializeT> RecursiveModal<ModalSerializeT> for ModalSerializeT
where
    ModalSerializeT: SerializeModalRescursive,
{
    fn modal<'own>(
        &'own self,
        mode: &'own SerializationMode,
        serializer: &'own ModalSerializer,
    ) -> RecursiveModalSerializable<'own, Self> {
        RecursiveModalSerializable::new(self, mode, serializer)
    }
}
