use serde::{Deserialize, Deserializer};
use std::collections::HashMap;
use std::hash::Hash;

/// Missing input and an exhausted default are distinct in the original `into_concrete`.
#[derive(Clone, Debug, Default, PartialEq)]
pub enum Inheritable<T> {
    #[default]
    Inherit,
    MissingDefault,
    Value(T),
}
pub trait Inherit {
    fn inherit_from(&mut self, parent: &Self);
    /// Optional game-owned finalization after merging an enabled optional
    /// object. Primitive/library adapters have no additional default policy.
    fn finish_inheritance(&mut self) {}
}
impl<T: Clone + Inherit> Inherit for Inheritable<T> {
    fn inherit_from(&mut self, parent: &Self) {
        match (&mut *self, parent) {
            (Self::Value(child), Self::Value(parent)) => child.inherit_from(parent),
            (Self::Inherit, _) => *self = parent.clone(),
            _ => {}
        }
    }
}
impl<T> Inheritable<T> {
    /// Borrow the resolved value without cloning or consuming it.
    /// B inlinee 0x7bf7 in load_prefabs_impl distinguishes both unresolved states.
    pub fn as_concrete(&self) -> &T {
        match self {
            Self::Value(value) => value,
            Self::Inherit => panic!(
                "as_concrete() called on an Inheritable value without a prior call to inherit()"
            ),
            Self::MissingDefault => panic!(
                "Unable to resolve: a default value {} was not provided.",
                std::any::type_name::<T>()
            ),
        }
    }

    /// Resolve only the outer missing-input marker; concrete children are atomic.
    /// B scanner inlinee 0x1a669 copies even an unresolved parent marker.
    pub fn inherit_shallow(&mut self, parent: &Self)
    where
        T: Clone,
    {
        if matches!(self, Self::Inherit) {
            *self = parent.clone();
        }
    }

    pub fn into_concrete(self) -> T {
        match self {
            Self::Value(value) => value,
            Self::Inherit => panic!(
                "into_concrete() called on an Inheritable value without a prior call to inherit()"
            ),
            Self::MissingDefault => panic!(
                "Unable to resolve: a default value {} was not provided.",
                std::any::type_name::<T>()
            ),
        }
    }
}
impl<'de, T: Deserialize<'de>> Deserialize<'de> for Inheritable<T> {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        deserialize_inheritable(d)
    }
}
/// An explicit field always becomes a concrete value; missing fields use serde's default.
/// B inlinee 0x20bb7 delegates the typed serde call and propagates its original error.
pub fn deserialize_inheritable<'de, D, T>(d: D) -> Result<Inheritable<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    T::deserialize(d).map(Inheritable::Value)
}
/// Original accepts either a bare value or explicit Some/None (`MaybeOption` in PDB/strings).
pub fn deserialize_inheritable_option<'de, D, T>(d: D) -> Result<Inheritable<Option<T>>, D::Error>
where
    D: Deserializer<'de>,
    T: serde::de::DeserializeOwned,
{
    // The original fork's serde Content type-name check no longer matches Rust 1.97's
    // lifetime suffix. Preserve its RON input rules by delegating typed parsing directly
    // to the same fork; do not discard enum identifiers in untagged Content buffering.
    let raw = Box::<ron::value::RawValue>::deserialize(d)?;
    if ron::from_str::<()>(raw.get_ron()).is_ok() {
        return Ok(Inheritable::Value(None));
    }
    let value = ron::Options::default()
        .with_default_extension(ron::extensions::Extensions::IMPLICIT_SOME)
        .from_str::<Option<T>>(raw.get_ron())
        .map_err(serde::de::Error::custom)?;
    Ok(Inheritable::Value(value))
}
impl<T: Inherit> Inherit for Option<T> {
    fn inherit_from(&mut self, parent: &Self) {
        if let Some(child) = self {
            if let Some(parent) = parent {
                child.inherit_from(parent);
            }
            // Original prefab RT/audio option specializations always finalize
            // a present child, including when the parent option is None.
            child.finish_inheritance();
        }
    }
}
impl<K: Eq + Hash + Clone, V: Inherit + Clone> Inherit for HashMap<K, V> {
    fn inherit_from(&mut self, parent: &Self) {
        for (key, value) in parent {
            match self.get_mut(key) {
                Some(child) => child.inherit_from(value),
                None => {
                    self.insert(key.clone(), value.clone());
                }
            }
        }
    }
}
macro_rules! no_inherit {($($t:ty),*)=>{$(impl Inherit for $t {fn inherit_from(&mut self,_:&Self){}})*}}
no_inherit!(String, bool, u32, f32, [u32; 2], [f32; 3]);
