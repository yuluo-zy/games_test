//! Best-effort extraction of one selected field from a serialized map.
//! Parse errors are deliberately suppressed: the last completely decoded value
//! survives a later error, and an absent or unreadable field uses its default.
use serde::de::{Deserialize, Deserializer, Error, IgnoredAny, MapAccess, Unexpected, Visitor};
use std::{fmt, marker::PhantomData};
pub trait FieldSelector {
    const FIELD: &'static str;
}
// Ask Serde to decode the key, then compare its borrowed string directly. Save
// GC can skip many thousands of keys; owning a String for every key adds an
// allocation without preserving any extra business state.
struct SelectedKey<F>(bool, PhantomData<F>);
impl<'de, F: FieldSelector> Deserialize<'de> for SelectedKey<F> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct MatchField<F>(PhantomData<F>);
        impl<'de, F: FieldSelector> Visitor<'de> for MatchField<F> {
            type Value = SelectedKey<F>;
            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("a string")
            }
            fn visit_str<E: Error>(self, value: &str) -> Result<Self::Value, E> {
                Ok(SelectedKey(value == F::FIELD, PhantomData))
            }
            fn visit_string<E: Error>(self, value: String) -> Result<Self::Value, E> {
                self.visit_str(&value)
            }
            fn visit_bytes<E: Error>(self, value: &[u8]) -> Result<Self::Value, E> {
                let text = std::str::from_utf8(value)
                    .map_err(|_| E::invalid_value(Unexpected::Bytes(value), &self))?;
                self.visit_str(text)
            }
            fn visit_byte_buf<E: Error>(self, value: Vec<u8>) -> Result<Self::Value, E> {
                self.visit_bytes(&value)
            }
        }
        // Preserve String's deserializer entry and accepted string/UTF-8 byte
        // forms. Format parsing, escaping and validation stay in Serde.
        deserializer.deserialize_string(MatchField(PhantomData))
    }
}
/// Field selecting the history section of a save file.
pub enum HistoryField {}
impl FieldSelector for HistoryField {
    const FIELD: &'static str = "History";
}
pub enum SnapshotsField {}
impl FieldSelector for SnapshotsField {
    const FIELD: &'static str = "snapshots";
}
/// Extracts snapshot payloads nested inside save-file history.
pub type SaveHistorySnapshots<T> =
    SelectiveDeserialize<HistoryField, SelectiveDeserialize<SnapshotsField, T>>;
#[derive(Clone, Debug, PartialEq)]
pub struct SelectiveDeserialize<F, T> {
    pub value: T,
    _field: PhantomData<F>,
}
impl<F, T: Default> Default for SelectiveDeserialize<F, T> {
    fn default() -> Self {
        Self {
            value: T::default(),
            _field: PhantomData,
        }
    }
}
impl<F, T> SelectiveDeserialize<F, T> {
    pub fn into_inner(self) -> T {
        self.value
    }
}
impl<'de, F: FieldSelector, T: Deserialize<'de> + Default> Deserialize<'de>
    for SelectiveDeserialize<F, T>
{
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct SelectField<'a, F, T> {
            result: &'a mut Option<T>,
            marker: PhantomData<F>,
        }
        impl<'de, F: FieldSelector, T: Deserialize<'de>> Visitor<'de> for SelectField<'_, F, T> {
            type Value = ();
            fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
                write!(fmt, "map containing {}", F::FIELD)
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<(), A::Error> {
                while let Some(key) = map.next_key::<SelectedKey<F>>()? {
                    if key.0 {
                        *self.result = Some(map.next_value()?);
                    } else {
                        map.next_value::<IgnoredAny>()?;
                    }
                }
                Ok(())
            }
        }
        let mut result = None;
        let _ = deserializer.deserialize_struct(
            "SelectiveDeserialize",
            &[F::FIELD],
            SelectField {
                result: &mut result,
                marker: PhantomData::<F>,
            },
        );
        Ok(Self {
            value: result.unwrap_or_default(),
            _field: PhantomData,
        })
    }
}

#[cfg(test)]
#[path = "oracle/selective_deserialize_review_tests.rs"]
mod review_tests;
