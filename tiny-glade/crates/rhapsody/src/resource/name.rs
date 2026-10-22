//! Interned names for resources and passes.
//!
//! A graph is declared every frame, and each declaration used to allocate a
//! `String`/`Arc<str>` per resource and per pass, then compare them byte by
//! byte against the cached plan. An interned name is one pointer: creating it
//! for a name seen before allocates nothing, copying it is free, and equality
//! and hashing are pointer operations.
//!
//! The intern table only grows. Names must therefore come from a bounded set
//! (static pass and resource names); never put a frame counter, an address or a
//! random value into one.
use std::{fmt, ops::Deref, sync::Arc};
use ustr::Ustr;

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ResourceName(Ustr);

impl ResourceName {
    pub fn new(name: &str) -> Self {
        Self(ustr::ustr(name))
    }
    pub fn as_str(&self) -> &'static str {
        self.0.as_str()
    }
    /// Number of distinct names interned so far (process-wide). Steady-state
    /// rendering must not grow this: it only rises when a *new* string appears.
    pub fn interned_count() -> usize {
        ustr::num_entries()
    }
    /// The interned name for `name`, or `None` if it was never interned.
    /// Lookups by string must use this, not `Borrow<str>`: the hash of an
    /// interned name is its precomputed pointer hash, not the string's.
    pub fn from_existing(name: &str) -> Option<Self> {
        Ustr::from_existing(name).map(Self)
    }
}
impl Deref for ResourceName {
    type Target = str;
    fn deref(&self) -> &str {
        self.0.as_str()
    }
}
impl AsRef<str> for ResourceName {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
impl From<&str> for ResourceName {
    fn from(name: &str) -> Self {
        Self::new(name)
    }
}
impl From<&String> for ResourceName {
    fn from(name: &String) -> Self {
        Self::new(name)
    }
}
impl From<String> for ResourceName {
    fn from(name: String) -> Self {
        Self::new(&name)
    }
}
impl From<Arc<str>> for ResourceName {
    fn from(name: Arc<str>) -> Self {
        Self::new(&name)
    }
}
impl From<&Arc<str>> for ResourceName {
    fn from(name: &Arc<str>) -> Self {
        Self::new(name)
    }
}
impl PartialEq<str> for ResourceName {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}
impl PartialEq<&str> for ResourceName {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}
impl PartialEq<String> for ResourceName {
    fn eq(&self, other: &String) -> bool {
        self.as_str() == other
    }
}
impl PartialEq<ResourceName> for str {
    fn eq(&self, other: &ResourceName) -> bool {
        self == other.as_str()
    }
}
impl PartialEq<ResourceName> for &str {
    fn eq(&self, other: &ResourceName) -> bool {
        *self == other.as_str()
    }
}
impl PartialEq<ResourceName> for String {
    fn eq(&self, other: &ResourceName) -> bool {
        self.as_str() == other.as_str()
    }
}
impl fmt::Display for ResourceName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
impl fmt::Debug for ResourceName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self.as_str(), f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn equal_strings_share_one_identity_and_different_strings_do_not() {
        let a = ResourceName::new("main_color");
        let b = ResourceName::from(String::from("main_color"));
        let c = ResourceName::from(Arc::<str>::from("main_color_2"));
        assert_eq!(a, b);
        assert_ne!(a, c);
        assert!(std::ptr::eq(a.as_str(), b.as_str()), "interned once");
        assert_eq!(a, "main_color");
        assert_eq!(format!("{a}"), "main_color");
        assert_eq!(format!("{a:?}"), "\"main_color\"");
        assert_eq!(a.len(), 10, "derefs to str");
    }
    #[test]
    fn hashing_and_ordering_follow_the_string() {
        use std::collections::{BTreeSet, HashSet};
        let names = ["b", "a", "c", "a"].map(ResourceName::new);
        assert_eq!(names.iter().copied().collect::<HashSet<_>>().len(), 3);
        let sorted: Vec<_> = names.iter().copied().collect::<BTreeSet<_>>().into_iter().collect();
        assert_eq!(sorted, ["a", "b", "c"]);
    }
}
