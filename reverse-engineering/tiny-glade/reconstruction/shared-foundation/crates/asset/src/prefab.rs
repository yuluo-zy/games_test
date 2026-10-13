pub mod config;
pub mod scan;
use std::sync::Arc;
use std::{borrow::Borrow, ops::Deref};

/// Borrowed name view. Original B getters preserve string bytes and length;
/// PrefabLibrary compares that length and delegates byte equality to the runtime.
#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct PrefabNameRef(str);
impl PrefabNameRef {
    /// Port adapter for the borrowed nominal view; no allocation or normalization.
    pub fn from_str_ref(name: &str) -> &Self {
        // SAFETY: this transparent single-field type has str's layout/metadata,
        // and introduces no invariant beyond its existing valid UTF-8 contents.
        unsafe { &*(name as *const str as *const Self) }
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
impl AsRef<PrefabNameRef> for PrefabNameRef {
    fn as_ref(&self) -> &PrefabNameRef {
        self
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PrefabName(Arc<str>);
impl PrefabName {
    pub fn new(name: impl AsRef<str>) -> Self {
        Self(Arc::from(name.as_ref()))
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
    pub fn as_name_ref(&self) -> &PrefabNameRef {
        PrefabNameRef::from_str_ref(self.as_str())
    }
}
impl AsRef<str> for PrefabName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl AsRef<PrefabNameRef> for PrefabName {
    fn as_ref(&self) -> &PrefabNameRef {
        self.as_name_ref()
    }
}
impl Borrow<PrefabNameRef> for PrefabName {
    fn borrow(&self) -> &PrefabNameRef {
        self.as_name_ref()
    }
}
impl Deref for PrefabName {
    type Target = PrefabNameRef;
    fn deref(&self) -> &Self::Target {
        self.as_name_ref()
    }
}
impl From<&str> for PrefabName {
    fn from(name: &str) -> Self {
        Self::new(name)
    }
}
