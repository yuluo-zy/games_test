//! Bounded per-pack/snapshot missing-key markers with reentrant-safe diagnostics.
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

const MISSING_LIMIT: usize = 1024;
#[derive(Debug, Default)]
pub(crate) struct MissingCache {
    entries: Mutex<HashMap<String, Arc<str>>>,
}
impl MissingCache {
    pub(crate) fn get(&self, key: &str) -> Arc<str> {
        let (value, notify) = {
            let mut cache = self.entries.lock().unwrap_or_else(|e| e.into_inner());
            if let Some(value) = cache.get(key) {
                return Arc::clone(value);
            }
            let value: Arc<str> = format!("?_{key}").into();
            let notify = cache.len() < MISSING_LIMIT;
            if notify {
                cache.insert(key.to_owned(), Arc::clone(&value));
            }
            (value, notify)
        };
        // Insert before notifying, and suppress recursive logger lookups. No cache
        // lock is held while calling external logging code, including on panic.
        if notify {
            crate::diagnostics::warn(|| log::warn!("LanguageLibrary: missing key {key}"));
        }
        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn missing_cache_is_bounded_and_cached_lookups_reuse_allocation() {
        let cache = MissingCache::default();
        let first = cache.get("key");
        assert!(Arc::ptr_eq(&first, &cache.get("key")));
        for i in 0..2000 {
            cache.get(&i.to_string());
        }
        assert_eq!(cache.entries.lock().unwrap().len(), MISSING_LIMIT);
        let value = cache.get("uncached");
        let weak = Arc::downgrade(&value);
        drop(value);
        assert!(weak.upgrade().is_none());
    }
}
