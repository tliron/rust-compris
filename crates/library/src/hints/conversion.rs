use super::{
    super::{meta::*, normal::*, parse::*},
    hints::*,
};

use tracing::trace;

//
// Value
//

impl Value {
    /// Attempts to convert a value to a hinted value.
    ///
    /// Also converts escaped hints to their unescaped values.
    pub fn to_hinted_value(&self, hints: &Hints) -> Result<Option<Value>, ParseError> {
        if let Some((key, value)) = self.to_key_value_pair() {
            if let Value::Text(hint) = key {
                if hint.value == hints.integer {
                    let text = value.validate_hinted_text(&hints.integer)?;
                    let integer: i64 = text.value.parse()?;
                    trace!("hinted {}: {}", hints.integer, integer);
                    return Ok(Some(Integer::new(integer).with_meta_from(text).into()));
                } else if hint.value == hints.unsigned_integer {
                    let text = value.validate_hinted_text(&hints.unsigned_integer)?;
                    let unsigned_integer: u64 = text.value.parse()?;
                    trace!("hinted {}: {}", hints.unsigned_integer, unsigned_integer);
                    return Ok(Some(UnsignedInteger::new(unsigned_integer).with_meta_from(text).into()));
                } else if hint.value == hints.bytes {
                    let text = value.validate_hinted_text(&hints.bytes)?;
                    let bytes = Blob::new_from_base64(&text.value)?;
                    trace!("hinted {}: {} bytes", hints.bytes, bytes.value.len());
                    return Ok(Some(bytes.with_meta_from(text).into()));
                } else if hint.value == hints.map {
                    let list = value.validate_hinted_list(&hints.map)?;
                    let mut new_map = Map::new();

                    for item in list {
                        let entry = item.validate_hinted_list(&hints.map)?;
                        if let Some((key, value)) = entry.to_pair() {
                            let key = if let Some(hinted) = key.to_hinted_value(hints)? { hinted } else { key.clone() };

                            let value =
                                if let Some(hinted) = value.to_hinted_value(hints)? { hinted } else { value.clone() };

                            new_map.value.insert(key, value);

                            // Note: we will allow duplicate keys above, because JSON does, too
                            // if new_map.value.insert(key, value).is_some() {
                            //     return Err(ReadError::Hint(format!("malformed {:?}, duplicate key", hints.map)));
                            // }
                        } else {
                            return Err(ParseError::Hint(format!("malformed {:?}, item length is not 2", hints.map)));
                        }
                    }

                    trace!("hinted {}: {}", hints.map, new_map.value.len());
                    return Ok(Some(new_map.with_meta_from(list).into()));
                } else if hint.value == hints.escaped_integer {
                    trace!("escaped hint: {}", hints.integer);
                    return Ok(Some(self.unescape_hint(&hints.integer, key, value)));
                } else if hint.value == hints.escaped_unsigned_integer {
                    trace!("escaped hint: {}", hints.unsigned_integer);
                    return Ok(Some(self.unescape_hint(&hints.unsigned_integer, key, value)));
                } else if hint.value == hints.escaped_bytes {
                    trace!("escaped hint: {}", hints.bytes);
                    return Ok(Some(self.unescape_hint(&hints.bytes, key, value)));
                } else if hint.value == hints.escaped_map {
                    trace!("escaped hint: {}", hints.map);
                    return Ok(Some(self.unescape_hint(&hints.map, key, value)));
                }
            }
        }

        Ok(None)
    }

    fn validate_hinted_text(&self, hint: &str) -> Result<&Text, ParseError> {
        match self {
            Value::Text(text) => Ok(text),
            _ => Err(ParseError::Hint(format!("malformed {:?}, not text", hint))),
        }
    }

    fn validate_hinted_list(&self, hint: &str) -> Result<&List, ParseError> {
        match self {
            Value::List(list) => Ok(list),
            _ => Err(ParseError::Hint(format!("malformed {:?}, not a list", hint))),
        }
    }

    fn unescape_hint(&self, new_key: &str, key: &Value, value: &Value) -> Self {
        let mut new_map = Map::new();
        new_map.value.insert(Text::from(new_key).with_meta_from(key).into(), value.clone());
        new_map.with_meta_from(self).into()
    }
}
