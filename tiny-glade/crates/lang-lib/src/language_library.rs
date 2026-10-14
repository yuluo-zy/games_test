//! Owned language packs with fallible schema validation and bounded missing-key caches.
use anyhow::{Context, Result, ensure};
use std::{
    cell::Cell,
    collections::HashMap,
    path::Path,
    sync::{Arc, Mutex},
};
use yaml_rust2::YamlLoader;

const MISSING_LIMIT: usize = 1024;
thread_local! { static LOGGING_MISSING: Cell<bool> = const { Cell::new(false) }; }
#[derive(Debug, Default)]
pub(crate) struct MissingCache(Mutex<HashMap<String, Arc<str>>>);
impl MissingCache {
    pub(crate) fn get(&self, key: &str) -> Arc<str> {
        let (value, notify) = {
            let mut cache = self.0.lock().unwrap_or_else(|e| e.into_inner());
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
            LOGGING_MISSING.with(|logging| {
                if !logging.replace(true) {
                    struct Reset<'a>(&'a Cell<bool>);
                    impl Drop for Reset<'_> {
                        fn drop(&mut self) {
                            self.0.set(false);
                        }
                    }
                    let _reset = Reset(logging);
                    log::warn!("LanguageLibrary: missing key {key}");
                }
            });
        }
        value
    }
}

pub(crate) fn validate_locale(locale: &str) -> Result<()> {
    ensure!(
        !locale.is_empty()
            && locale
                .bytes()
                .all(|c| c.is_ascii_alphanumeric() || c == b'-' || c == b'_'),
        "Invalid locale identifier: {locale:?}"
    );
    Ok(())
}
#[derive(Debug)]
pub struct LanguageLibrary {
    locale: Arc<str>,
    entries: HashMap<String, Arc<str>>,
    missing: MissingCache,
}
impl LanguageLibrary {
    /// Construct an owned library from already validated application data.
    pub fn new(locale: String, entries: HashMap<String, Arc<str>>) -> Self {
        Self {
            locale: locale.into(),
            entries,
            missing: MissingCache::default(),
        }
    }
    pub fn try_from_yml(path: impl AsRef<Path>, locale: String) -> Result<Self> {
        let path = path.as_ref();
        let text = std::fs::read_to_string(path)
            .with_context(|| format!("Reading language pack {}", path.display()))?;
        Self::parse_yml(&text, locale)
            .with_context(|| format!("Parsing language pack {}", path.display()))
    }
    /// Accept an empty file or one flat string-to-string mapping.
    pub fn parse_yml(text: &str, locale: String) -> Result<Self> {
        validate_locale(&locale)?;
        let documents = YamlLoader::load_from_str(text).context("Parsing YAML")?;
        ensure!(
            documents.len() <= 1,
            "Language pack must contain at most one YAML document"
        );
        let mut entries = HashMap::new();
        if let Some(document) = documents.first() {
            let mapping = document
                .as_hash()
                .context("Language pack root must be a mapping")?;
            for (index, (key, value)) in mapping.iter().enumerate() {
                let key = key.as_str().with_context(|| {
                    format!("Language entry {} has a non-string key", index + 1)
                })?;
                let value = value
                    .as_str()
                    .with_context(|| format!("Language entry {key:?} must have a string value"))?;
                entries.insert(key.to_owned(), Arc::from(value));
            }
        }
        Ok(Self::new(locale, entries))
    }
    pub fn locale(&self) -> Arc<str> {
        Arc::clone(&self.locale)
    }
    pub fn try_get(&self, key: &str) -> Option<Arc<str>> {
        self.entries.get(key).map(Arc::clone)
    }
    pub fn get(&self, key: &str) -> Arc<str> {
        self.try_get(key).unwrap_or_else(|| self.missing.get(key))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn flat_keys_and_owned_missing_cache() {
        let lib = LanguageLibrary::parse_yml("Menu.Close: Close\nquoted: \"123\"", "test".into())
            .unwrap();
        assert_eq!(&*lib.get("Menu.Close"), "Close");
        assert_eq!(&*lib.get("quoted"), "123");
        let missing = lib.get("missing");
        assert_eq!(&*missing, "?_missing");
        assert!(Arc::ptr_eq(&missing, &lib.get("missing")));
        let weak = Arc::downgrade(&missing);
        drop(missing);
        drop(lib);
        assert!(weak.upgrade().is_none());
    }
    #[test]
    fn schema_errors_return_errors() {
        for text in [
            "bad: [",
            "bad: 123",
            "- sequence",
            "123: text",
            "a: text\n---\na: another",
            "null",
        ] {
            assert!(
                LanguageLibrary::parse_yml(text, "test".into()).is_err(),
                "{text}"
            );
        }
        assert!(
            LanguageLibrary::parse_yml("", "test".into())
                .unwrap()
                .try_get("x")
                .is_none()
        );
        for locale in ["", "../en-US", "/en-US", "a\\b", "a.b"] {
            assert!(LanguageLibrary::parse_yml("", locale.into()).is_err());
        }
    }
    #[test]
    fn missing_cache_is_bounded_and_cached_lookups_reuse_allocation() {
        let cache = MissingCache::default();
        let first = cache.get("key");
        assert!(Arc::ptr_eq(&first, &cache.get("key")));
        for i in 0..2000 {
            cache.get(&i.to_string());
        }
        assert_eq!(cache.0.lock().unwrap().len(), MISSING_LIMIT);
        let value = cache.get("uncached");
        let weak = Arc::downgrade(&value);
        drop(value);
        assert!(weak.upgrade().is_none());
    }
    #[test]
    fn pack_strings_are_reclaimed_after_last_reader() {
        let lib = LanguageLibrary::parse_yml("key: value", "test".into()).unwrap();
        let value = lib.get("key");
        let weak = Arc::downgrade(&value);
        let locale = Arc::downgrade(&lib.locale());
        drop(lib);
        assert_eq!(&*value, "value");
        assert!(locale.upgrade().is_none());
        drop(value);
        assert!(weak.upgrade().is_none());
    }
}
