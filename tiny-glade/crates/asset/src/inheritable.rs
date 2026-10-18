use serde::{Deserialize, Deserializer};
use std::collections::HashMap;
use std::hash::Hash;

/// A field awaiting inheritance, a required field without a default, or a value.
/// `Value(None)` explicitly disables an optional field and never inherits it.
#[derive(Clone, Debug, Default, PartialEq)]
pub enum Inheritable<T> {
    #[default]
    Inherit,
    MissingDefault,
    Value(T),
}

/// Why a field could not be resolved after configuration inheritance.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UnresolvedField {
    InheritanceRequired,
    MissingDefault,
}

impl std::fmt::Display for UnresolvedField {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::InheritanceRequired => "field still requires inheritance",
            Self::MissingDefault => "required field has no value or default",
        })
    }
}

impl std::error::Error for UnresolvedField {}

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
    /// Borrow a resolved field; suitable for validating externally supplied data.
    pub fn try_as_concrete(&self) -> Result<&T, UnresolvedField> {
        match self {
            Self::Value(value) => Ok(value),
            Self::Inherit => Err(UnresolvedField::InheritanceRequired),
            Self::MissingDefault => Err(UnresolvedField::MissingDefault),
        }
    }

    /// Consume a resolved field without cloning its contents.
    pub fn try_into_concrete(self) -> Result<T, UnresolvedField> {
        match self {
            Self::Value(value) => Ok(value),
            Self::Inherit => Err(UnresolvedField::InheritanceRequired),
            Self::MissingDefault => Err(UnresolvedField::MissingDefault),
        }
    }

    /// Borrow the resolved value without cloning or consuming it.
    ///
    /// # Panics
    /// Panics if inheritance has not resolved this field. For external data, use
    /// [`Self::try_as_concrete`] and attach the configuration field's name.
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
    pub fn inherit_shallow(&mut self, parent: &Self)
    where
        T: Clone,
    {
        if matches!(self, Self::Inherit) {
            *self = parent.clone();
        }
    }

    /// Consume a resolved field.
    ///
    /// # Panics
    /// Panics if the field is unresolved; use [`Self::try_into_concrete`] at input boundaries.
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
pub fn deserialize_inheritable<'de, D, T>(d: D) -> Result<Inheritable<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    T::deserialize(d).map(Inheritable::Value)
}
/// Accept a bare value, explicit `Some`/`None`, or `()` as an explicit `None`.
pub fn deserialize_inheritable_option<'de, D, T>(d: D) -> Result<Inheritable<Option<T>>, D::Error>
where
    D: Deserializer<'de>,
    T: serde::de::DeserializeOwned,
{
    // Parse typed RON directly so enum identifiers survive optional-value buffering.
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
            // An enabled child still needs its own defaults when its parent is disabled.
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
