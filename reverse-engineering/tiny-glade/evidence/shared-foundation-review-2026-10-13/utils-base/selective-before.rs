//! Wrapper policy observed in both B deserializer monomorphizations: execute a
//! selected field visitor, discard its error, and default the output if the
//! visitor never produced a value. The field selector is an explicit type input.
use serde::de::{Deserialize, Deserializer, IgnoredAny, MapAccess, Visitor};
use std::{fmt, marker::PhantomData};
pub trait FieldSelector {
    const FIELD: &'static str;
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
                while let Some(key) = map.next_key::<String>()? {
                    if key == F::FIELD {
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
