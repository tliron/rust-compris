use super::{super::*, errors::*};

use tracing::trace;

/// Attempts to convert a map to a hinted value.
///
/// Also converts escaped hints to their unescaped values.
pub fn to_hinted_value(map: &Map, hints: &Hints) -> Result<Option<Value>, ReadError> {
    if map.value.len() != 1 {
        return Ok(None);
    }

    let (key, value) = map.value.iter().next().unwrap();

    if let Value::String(hint) = key {
        if hint.value == hints.integer {
            match value {
                Value::String(string) => {
                    let integer: i64 = string.value.parse()?;
                    trace!("hinted {}: {}", hints.integer, integer);
                    return Ok(Some(Integer::new(integer).with_meta(&string.meta).into()));
                }

                _ => return Err(ReadError::Hint(format!("malformed {:?}, not a string", hints.integer))),
            }
        } else if hint.value == hints.unsigned_integer {
            match value {
                Value::String(string) => {
                    let unsigned_integer: u64 = string.value.parse()?;
                    trace!("hinted {}: {}", hints.unsigned_integer, unsigned_integer);
                    return Ok(Some(UnsignedInteger::new(unsigned_integer).with_meta(&string.meta).into()));
                }

                _ => return Err(ReadError::Hint(format!("malformed {:?}, not a string", hints.unsigned_integer))),
            }
        } else if hint.value == hints.bytes {
            match value {
                Value::String(string) => {
                    let bytes = Bytes::new_from_base64(&string.value)?;
                    trace!("hinted {}: {} bytes", hints.bytes, bytes.value.len());
                    return Ok(Some(bytes.with_meta(&string.meta).into()));
                }

                _ => return Err(ReadError::Hint(format!("malformed {:?}, not a string", hints.bytes))),
            }
        } else if hint.value == hints.map {
            match value {
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
                                        "malformed {:?}, element list length is not 2",
                                        hints.map
                                    )));
                                }
                            }

                            _ => return Err(ReadError::Hint(format!("malformed {:?}, element not a list", hints.map))),
                        }
                    }

                    trace!("hinted {}: {}", hints.map, new_map.value.len());
                    return Ok(Some(new_map.with_meta(&map.meta).into()));
                }

                _ => return Err(ReadError::Hint(format!("malformed {:?}, not a list", hints.map))),
            }
        } else if hint.value == hints.escaped_integer {
            trace!("escaped hint: {}", hints.integer);
            let mut new_map = Map::new();
            new_map.value.insert(String::new(hints.integer.clone()).into(), value.clone());
            return Ok(Some(new_map.with_meta(&map.meta).into()));
        } else if hint.value == hints.escaped_unsigned_integer {
            trace!("escaped hint: {}", hints.unsigned_integer);
            let mut new_map = Map::new();
            new_map.value.insert(String::new(hints.unsigned_integer.clone()).into(), value.clone());
            return Ok(Some(new_map.with_meta(&map.meta).into()));
        } else if hint.value == hints.escaped_bytes {
            trace!("escaped hint: {}", hints.bytes);
            let mut new_map = Map::new();
            new_map.value.insert(String::new(hints.bytes.clone()).into(), value.clone());
            return Ok(Some(new_map.with_meta(&map.meta).into()));
        } else if hint.value == hints.escaped_map {
            trace!("escaped hint: {}", hints.map);
            let mut new_map = Map::new();
            new_map.value.insert(String::new(hints.map.clone()).into(), value.clone());
            return Ok(Some(new_map.with_meta(&map.meta).into()));
        }
    }

    Ok(None)
}
