use super::{super::*, serialization_mode::*};

use {serde::ser::*, tracing::trace};

//
// Map
//

impl Map {
    pub fn with_serialization_mode<'a>(
        &'a self,
        serialization_mode: &'a SerializationMode,
    ) -> MapWithSerializationMode<'a> {
        MapWithSerializationMode::new(self, serialization_mode)
    }

    pub fn serialize_with_mode<S: Serializer>(
        &self,
        serializer: S,
        serialization_mode: &SerializationMode,
    ) -> Result<S::Ok, S::Error> {
        // Map with string keys

        if serialization_mode.map == MapSerializationMode::KeysAsStrings {
            let mut map = serializer.serialize_map(Some(self.value.len()))?;
            for (key, value) in &self.value {
                map.serialize_entry(&key.to_map_string_key(), value)?;
            }
            return map.end();
        }

        // Map as list?

        if serialization_mode.map.might_be_list() {
            let (mut as_map, hint) = match &serialization_mode.map {
                MapSerializationMode::AsList(hint) => (false, hint),
                MapSerializationMode::AsListIfNonStringKey(hint) => (true, hint),
                mode => panic!("unexpected map serialization mode: {:?}", mode),
            };

            if as_map {
                // Do we have a non-string key?
                for key in self.value.keys() {
                    match key {
                        Value::String(_) => {}

                        _ => {
                            trace!("map has a non-string key");
                            as_map = false;
                            break;
                        }
                    }
                }
            }

            if !as_map {
                match hint {
                    Some(hint) => {
                        trace!("map as list wrapped in single-key map with key: {}", hint);

                        // TODO: because Serde doesn't serialize iters we must collect all entries
                        let mut entries = Vec::with_capacity(self.value.len());
                        for (key, value) in self.value.iter() {
                            entries.push((
                                key.with_serialization_mode(serialization_mode),
                                value.with_serialization_mode(serialization_mode),
                            ));
                        }

                        let mut map = serializer.serialize_map(Some(1))?;
                        map.serialize_entry(&hint, &entries)?;
                        return map.end();
                    }

                    None => {
                        trace!("map as list");
                        let mut seq = serializer.serialize_seq(Some(self.value.len()))?;
                        for (key, value) in self.value.iter() {
                            let entry = (
                                key.with_serialization_mode(serialization_mode),
                                value.with_serialization_mode(serialization_mode),
                            );
                            seq.serialize_element(&entry)?;
                        }
                        return seq.end();
                    }
                }
            }
        }

        // Map as map

        let mut map = serializer.serialize_map(Some(self.value.len()))?;
        for (key, value) in &self.value {
            map.serialize_entry(
                &key.with_serialization_mode(serialization_mode),
                &value.with_serialization_mode(serialization_mode),
            )?;
        }
        map.end()
    }
}

impl Serialize for Map {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(self.value.len()))?;
        for (key, value) in &self.value {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}

//
// MapWithSerializationMode
//

pub struct MapWithSerializationMode<'a> {
    pub map: &'a Map,
    pub serialization_mode: &'a SerializationMode,
}

impl<'a> MapWithSerializationMode<'a> {
    pub fn new(map: &'a Map, serialization_mode: &'a SerializationMode) -> Self {
        Self { map, serialization_mode }
    }
}

impl<'a> Serialize for MapWithSerializationMode<'a> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.map.serialize_with_mode(serializer, self.serialization_mode)
    }
}
