use super::{super::*, errors::*};

use {base64::prelude::*, tracing::trace};

/// Attempts to convert a map to a hinted value.
pub fn to_hinted_value(map: &Map) -> Result<Option<Value>, ReadError> {
    if map.value.len() != 1 {
        return Ok(None);
    }

    let (key, value) = map.value.iter().next().unwrap();

    if let Value::String(hint) = key {
        match hint.value.as_str() {
            ARD_HINT_INTEGER => match value {
                Value::String(string) => {
                    let integer = string.value.parse()?;
                    trace!("hinted {}: {}", ARD_HINT_INTEGER, integer);
                    return Ok(Some(Integer::new(integer).with_meta_clone(&string.meta).into()));
                }

                _ => return Err(ReadError::Hint(format!("malformed {}, not a string", ARD_HINT_INTEGER))),
            },

            ARD_HINT_UNSIGNED_INTEGER => match value {
                Value::String(string) => {
                    let unsigned_integer = string.value.parse()?;
                    trace!("hinted {}: {}", ARD_HINT_UNSIGNED_INTEGER, unsigned_integer);
                    return Ok(Some(UnsignedInteger::new(unsigned_integer).with_meta_clone(&string.meta).into()));
                }

                _ => return Err(ReadError::Hint(format!("malformed {}, not a string", ARD_HINT_UNSIGNED_INTEGER))),
            },

            ARD_HINT_BYTES => match value {
                Value::String(string) => {
                    let bytes = BASE64_STANDARD.decode(&string.value)?;
                    trace!("hinted {}: {} bytes", ARD_HINT_BYTES, bytes.len());
                    return Ok(Some(Bytes::new(bytes).with_meta_clone(&map.meta).into()));
                }

                _ => return Err(ReadError::Hint(format!("malformed {}, not a string", ARD_HINT_BYTES))),
            },

            ARD_HINT_MAP => match value {
                Value::List(list) => {
                    let mut new_map = Map::new();

                    for element in &list.value {
                        match element {
                            Value::List(entry) => {
                                if entry.value.len() == 2 {
                                    let key = entry.value.get(0).unwrap();
                                    let value = entry.value.get(1).unwrap();
                                    new_map.value.insert(key.clone(), value.clone());
                                } else {
                                    return Err(ReadError::Hint(format!(
                                        "malformed {}, entry list length is not 2",
                                        ARD_HINT_MAP
                                    )));
                                }
                            }

                            _ => return Err(ReadError::Hint(format!("malformed {}, entry not a list", ARD_HINT_MAP))),
                        }
                    }

                    trace!("hinted {}: {}", ARD_HINT_MAP, new_map.value.len());
                    return Ok(Some(new_map.with_meta_clone(&map.meta).into()));
                }

                _ => return Err(ReadError::Hint(format!("malformed {}, not a list", ARD_HINT_MAP))),
            },

            _ => {}
        }
    }

    Ok(None)
}
