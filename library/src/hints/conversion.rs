use super::{
    super::{annotate::*, normal::*, parse::*},
    hints::*,
};

use kutil::std::immutable::*;

impl<AnnotatedT> Variant<AnnotatedT> {
    /// Attempts to convert the [Variant] to a hinted [Variant].
    ///
    /// Also converts escaped hints to their unescaped values.
    pub fn to_hinted_variant(&self, hints: &Hints) -> Result<Option<Variant<AnnotatedT>>, ParseError>
    where
        AnnotatedT: Annotated + Clone + Default,
    {
        if let Some((key, value)) = self.to_key_value_pair()
            && let Variant::Text(hint) = key
        {
            if hint.inner == hints.integer {
                let text = value.validate_hinted_text(&hints.integer)?;
                let integer: i64 = text.inner.parse()?;
                tracing::trace!("hinted {}: {}", hints.integer, integer);
                return Ok(Some(Integer::from(integer).with_annotations_from(text).into()));
            } else if hint.inner == hints.unsigned_integer {
                let text = value.validate_hinted_text(&hints.unsigned_integer)?;
                let unsigned_integer: u64 = text.inner.parse()?;
                tracing::trace!("hinted {}: {}", hints.unsigned_integer, unsigned_integer);
                return Ok(Some(UnsignedInteger::from(unsigned_integer).with_annotations_from(text).into()));
            } else if hint.inner == hints.bytes {
                let text = value.validate_hinted_text(&hints.bytes)?;
                let blob = Blob::new_from_base64(&text.inner)?;
                tracing::trace!("hinted {}: {} bytes", hints.bytes, blob.inner.len());
                return Ok(Some(blob.with_annotations_from(text).into()));
            } else if hint.inner == hints.map {
                let list = value.validate_hinted_list(&hints.map)?;
                let mut new_map = Map::default();

                for item in list {
                    let entry = item.validate_hinted_list(&hints.map)?;
                    if let Some((key, value)) = entry.to_pair() {
                        let key = if let Some(hinted) = key.to_hinted_variant(hints)? { hinted } else { key.clone() };

                        let value =
                            if let Some(hinted) = value.to_hinted_variant(hints)? { hinted } else { value.clone() };

                        new_map.inner.insert(key, value);

                        // Note: we will allow duplicate keys above, because JSON does, too
                        // if new_map.value.insert(key, value).is_some() {
                        //     return Err(ReadError::Hint(format!("malformed {:?}, duplicate key", hints.map)));
                        // }
                    } else {
                        return Err(ParseError::Hint(format!("malformed {:?}, item length is not 2", hints.map)));
                    }
                }

                tracing::trace!("hinted {}: {}", hints.map, new_map.inner.len());
                return Ok(Some(new_map.with_annotations_from(list).into()));
            } else if hint.inner == hints.escaped_integer {
                tracing::trace!("escaped hint: {}", hints.integer);
                return Ok(Some(self.unescape_hint(&hints.integer, key, value)));
            } else if hint.inner == hints.escaped_unsigned_integer {
                tracing::trace!("escaped hint: {}", hints.unsigned_integer);
                return Ok(Some(self.unescape_hint(&hints.unsigned_integer, key, value)));
            } else if hint.inner == hints.escaped_bytes {
                tracing::trace!("escaped hint: {}", hints.bytes);
                return Ok(Some(self.unescape_hint(&hints.bytes, key, value)));
            } else if hint.inner == hints.escaped_map {
                tracing::trace!("escaped hint: {}", hints.map);
                return Ok(Some(self.unescape_hint(&hints.map, key, value)));
            }
        }

        Ok(None)
    }

    fn validate_hinted_text(&self, hint: &str) -> Result<&Text<AnnotatedT>, ParseError> {
        match self {
            Variant::Text(text) => Ok(text),
            _ => Err(ParseError::Hint(format!("malformed {:?}, not text", hint))),
        }
    }

    fn validate_hinted_list(&self, hint: &str) -> Result<&List<AnnotatedT>, ParseError> {
        match self {
            Variant::List(list) => Ok(list),
            _ => Err(ParseError::Hint(format!("malformed {:?}, not a list", hint))),
        }
    }

    fn unescape_hint(&self, new_key: &str, key: &Variant<AnnotatedT>, variant: &Variant<AnnotatedT>) -> Self
    where
        AnnotatedT: Annotated + Clone + Default,
    {
        let mut new_map = Map::default();
        new_map.inner.insert(Variant::from(ByteString::from(new_key)).with_annotations_from(key), variant.clone());
        new_map.with_annotations_from(self).into()
    }
}
