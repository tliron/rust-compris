use super::super::{
    super::{annotate::*, normal::*},
    modal::*,
    mode::*,
    serializer::Serializer as ComprisSerializer,
};

use {serde::ser::*, tracing::trace};

impl<AnnotatedT> Serialize for Map<AnnotatedT> {
    fn serialize<SerializerT>(&self, serializer: SerializerT) -> Result<SerializerT::Ok, SerializerT::Error>
    where
        SerializerT: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.inner.len()))?;
        for (key, value) in &self.inner {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}

impl<AnnotatedT> SerializeModalRescursive for Map<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn serialize_modal<SerializerT>(
        &self,
        serializer: SerializerT,
        mode: &SerializationMode,
        modal_serializer: &ComprisSerializer,
    ) -> Result<SerializerT::Ok, SerializerT::Error>
    where
        SerializerT: Serializer,
    {
        match mode.map {
            MapSerializationMode::AsMap => {
                let mut map = serializer.serialize_map(Some(self.inner.len()))?;
                for (key, value) in &self.inner {
                    map.serialize_entry(&key.modal(mode, modal_serializer), &value.modal(mode, modal_serializer))?;
                }
                map.end()
            }

            MapSerializationMode::AsSeq(_) | MapSerializationMode::AsSeqIfNonTextKey(_) => {
                let (mut as_map, hint) = match &mode.map {
                    MapSerializationMode::AsSeq(hint) => (false, hint),
                    MapSerializationMode::AsSeqIfNonTextKey(hint) => (true, hint),
                    mode => panic!("unexpected map serialization mode: {:?}", mode),
                };

                if as_map {
                    // Do we have a non-string key?
                    for key in self.inner.keys() {
                        match key {
                            Variant::Text(_) => {}

                            _ => {
                                trace!("map has a non-text key");
                                as_map = false;
                                break;
                            }
                        }
                    }
                }

                if as_map {
                    let mut map = serializer.serialize_map(Some(self.inner.len()))?;
                    for (key, value) in &self.inner {
                        map.serialize_entry(&key.modal(mode, modal_serializer), &value.modal(mode, modal_serializer))?;
                    }
                    map.end()
                } else {
                    match hint {
                        Some(hint) => {
                            trace!("map as seq wrapped in single-key map with key: {}", hint);

                            // TODO: because Serde doesn't serialize iters we must collect all entries
                            let mut entries = Vec::with_capacity(self.inner.len());
                            for (key, value) in self.inner.iter() {
                                entries.push((key.modal(mode, modal_serializer), value.modal(mode, modal_serializer)));
                            }

                            let mut map = serializer.serialize_map(Some(1))?;
                            map.serialize_entry(&hint, &entries)?;
                            map.end()
                        }

                        None => {
                            trace!("map as seq");
                            let mut seq = serializer.serialize_seq(Some(self.inner.len()))?;
                            for (key, value) in self.inner.iter() {
                                let entry = (key.modal(mode, modal_serializer), value.modal(mode, modal_serializer));
                                seq.serialize_element(&entry)?;
                            }
                            seq.end()
                        }
                    }
                }
            }

            MapSerializationMode::SerializeKeys | MapSerializationMode::SerializeKeysIfNonText => {
                let always = mode.map == MapSerializationMode::SerializeKeys;
                let stringify_serializer = modal_serializer.clone().with_pretty(false);

                let mut map = serializer.serialize_map(Some(self.inner.len()))?;
                for (key, value) in &self.inner {
                    if always || !matches!(key, Variant::Text(_)) {
                        match stringify_serializer.stringify(key) {
                            Ok(key) => map.serialize_entry(&key, &value.modal(mode, modal_serializer))?,
                            Err(error) => return Err(SerializerT::Error::custom(error)),
                        }
                    } else {
                        map.serialize_entry(&key.modal(mode, modal_serializer), &value.modal(mode, modal_serializer))?;
                    }
                }
                map.end()
            }
        }
    }
}
