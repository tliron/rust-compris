use super::{super::*, errors::*};

use tracing::trace;

/// Attempts to convert a map to a hinted value.
///
/// Also converts escaped hints to their unescaped values.
pub fn to_hinted_value(map: &Map) -> Result<Option<Value>, ReadError> {
    if map.value.len() != 1 {
        return Ok(None);
    }

    let (key, value) = map.value.iter().next().unwrap();

    if let Value::String(hint) = key {
        match hint.value.as_str() {
            XJSON_HINT_INTEGER => match value {
                Value::String(string) => {
                    let integer = string.value.parse()?;
                    trace!("hinted {}: {}", XJSON_HINT_INTEGER, integer);
                    return Ok(Some(Integer::new(integer).with_meta(&string.meta).into()));
                }

                _ => return Err(ReadError::Hint(format!("malformed {}, not a string", XJSON_HINT_INTEGER))),
            },

            XJSON_HINT_UNSIGNED_INTEGER => match value {
                Value::String(string) => {
                    let unsigned_integer = string.value.parse()?;
                    trace!("hinted {}: {}", XJSON_HINT_UNSIGNED_INTEGER, unsigned_integer);
                    return Ok(Some(UnsignedInteger::new(unsigned_integer).with_meta(&string.meta).into()));
                }

                _ => return Err(ReadError::Hint(format!("malformed {}, not a string", XJSON_HINT_UNSIGNED_INTEGER))),
            },

            XJSON_HINT_BYTES => match value {
                Value::String(string) => {
                    let bytes = Bytes::new_from_base64(&string.value)?;
                    trace!("hinted {}: {} bytes", XJSON_HINT_BYTES, bytes.value.len());
                    return Ok(Some(bytes.with_meta(&string.meta).into()));
                }

                _ => return Err(ReadError::Hint(format!("malformed {}, not a string", XJSON_HINT_BYTES))),
            },

            XJSON_HINT_MAP => match value {
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
                                        XJSON_HINT_MAP
                                    )));
                                }
                            }

                            _ => return Err(ReadError::Hint(format!("malformed {}, not a list", XJSON_HINT_MAP))),
                        }
                    }

                    trace!("hinted {}: {}", XJSON_HINT_MAP, new_map.value.len());
                    return Ok(Some(new_map.with_meta(&map.meta).into()));
                }

                _ => return Err(ReadError::Hint(format!("malformed {}, not a list", XJSON_HINT_MAP))),
            },

            XJSON_HINT_INTEGER_ESCAPED => {
                trace!("escaped hint: {}", XJSON_HINT_INTEGER);
                let mut new_map = Map::new();
                new_map.value.insert(String::new(XJSON_HINT_INTEGER.into()).into(), value.clone());
                return Ok(Some(new_map.with_meta(&map.meta).into()));
            }

            XJSON_HINT_UNSIGNED_INTEGER_ESCAPED => {
                trace!("escaped hint: {}", XJSON_HINT_UNSIGNED_INTEGER);
                let mut new_map = Map::new();
                new_map.value.insert(String::new(XJSON_HINT_UNSIGNED_INTEGER.into()).into(), value.clone());
                return Ok(Some(new_map.with_meta(&map.meta).into()));
            }

            XJSON_HINT_BYTES_ESCAPED => {
                trace!("escaped hint: {}", XJSON_HINT_BYTES);
                let mut new_map = Map::new();
                new_map.value.insert(String::new(XJSON_HINT_BYTES.into()).into(), value.clone());
                return Ok(Some(new_map.with_meta(&map.meta).into()));
            }

            XJSON_HINT_MAP_ESCAPED => {
                trace!("escaped hint: {}", XJSON_HINT_MAP);
                let mut new_map = Map::new();
                new_map.value.insert(String::new(XJSON_HINT_MAP.into()).into(), value.clone());
                return Ok(Some(new_map.with_meta(&map.meta).into()));
            }

            _ => {}
        }
    }

    Ok(None)
}
