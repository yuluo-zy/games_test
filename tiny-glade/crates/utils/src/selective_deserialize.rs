//! Wrapper policy observed in both B deserializer monomorphizations: execute a
//! selected field visitor, discard its error, and default the output if the
//! visitor never produced a value. The field selector is an explicit type input.
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
/// Original B save-file selectors; the outer visitor delegates to the nested
/// selector, so completed inner values survive a later outer parse error.
pub enum HistoryField {}
impl FieldSelector for HistoryField {
    const FIELD: &'static str = "History";
}
pub enum SnapshotsField {}
impl FieldSelector for SnapshotsField {
    const FIELD: &'static str = "snapshots";
}
/// Original save-GC nesting, with the snapshot payload type supplied by caller.
/// This alias describes the recovered selector contract, not the author's name.
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
mod review_tests {
    use super::*;
    #[test]
    fn borrowed_key_selection_preserves_nested_partial_success() {
        let input=br#"{"unknown":{"deep":[1,2]},"History":{"snap\u0073hots":[7,9],"snapshots":"bad"},"broken": }"#;
        let mut deserializer = serde_json::Deserializer::from_slice(input);
        let value = SaveHistorySnapshots::<Vec<u64>>::deserialize(&mut deserializer).unwrap();
        assert_eq!(value.value.value, vec![7, 9]);
        let key =
            serde::de::value::BorrowedStrDeserializer::<serde::de::value::Error>::new("History");
        assert!(SelectedKey::<HistoryField>::deserialize(key).unwrap().0);
    }
}
