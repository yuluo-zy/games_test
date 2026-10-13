use anyhow::Result;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Mutex;
use yaml_rust2::YamlLoader;

static MISSING: Lazy<Mutex<HashMap<String, &'static str>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

/// Translations and locale are deliberately leaked like B, so returned text remains
/// valid across ArcSwap replacement; containers themselves retain normal ownership.
#[derive(Debug)]
pub struct LanguageLibrary {
    locale: &'static str,
    entries: HashMap<String, &'static str>,
}
impl LanguageLibrary {
    /// Original inline construction preserves the caller's locale and collected
    /// map; validation and fallback belong to the loader, not this constructor.
    /// Ownership here deliberately uses the existing safe reconstruction API.
    pub fn new(locale: String, entries: HashMap<String, &'static str>) -> Self {
        Self {
            locale: Box::leak(locale.into_boxed_str()),
            entries,
        }
    }
    pub fn try_from_yml(path: impl AsRef<Path>, locale: String) -> Result<Self> {
        let text = std::fs::read_to_string(path)?;
        Self::parse_yml(&text, locale)
    }
    /// Synthetic pure entry, sharing the original parser policy for validation.
    pub fn parse_yml(text: &str, locale: String) -> Result<Self> {
        let documents = YamlLoader::load_from_str(text)?;
        let entries = if documents.is_empty() {
            HashMap::new()
        } else {
            // Original intentionally unwraps the first document's map and strings.
            documents[0]
                .as_hash()
                .unwrap()
                .iter()
                .map(|(key, value)| {
                    let key = key.as_str().unwrap().to_owned();
                    let value: &'static str =
                        Box::leak(value.as_str().unwrap().to_owned().into_boxed_str());
                    (key, value)
                })
                .collect()
        };
        Ok(Self::new(locale, entries))
    }
    pub fn locale(&self) -> &'static str {
        self.locale
    }
    pub fn try_get(&self, key: &str) -> Option<&'static str> {
        self.entries.get(key).copied()
    }
    pub fn get(&self, key: &str) -> &'static str {
        if let Some(value) = self.try_get(key) {
            return value;
        }
        let mut cache = MISSING.lock().unwrap();
        // The original probes the missing-key map before allocating its owned key.
        // Missing translations are queried repeatedly by UI consumers.
        if let Some(value) = cache.get(key) {
            return value;
        }
        log::warn!("LanguageLibrary: missing key {key}");
        let value: &'static str = Box::leak(format!("?_{key}").into_boxed_str());
        cache.insert(key.to_owned(), value);
        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn constructor_preserves_supplied_locale_and_entries() {
        let entries = HashMap::from([("key".to_owned(), "value"), ("empty".to_owned(), "")]);
        let library = LanguageLibrary::new("custom-locale".into(), entries);
        assert_eq!(library.locale(), "custom-locale");
        assert_eq!(library.try_get("key"), Some("value"));
        assert_eq!(library.try_get("empty"), Some(""));
        assert_eq!(library.try_get("absent"), None);
        let empty = LanguageLibrary::new("".into(), HashMap::new());
        assert_eq!(empty.locale(), "");
        assert_eq!(empty.try_get("key"), None);
    }
    #[test]
    fn flat_keys_first_document_and_missing_cache() {
        let lib = LanguageLibrary::parse_yml(
            "Menu.Close: Close\nquoted: \"123\"\n---\nMenu.Close: ignored",
            "test".into(),
        )
        .unwrap();
        assert_eq!(lib.get("Menu.Close"), "Close");
        assert_eq!(lib.get("quoted"), "123");
        assert_eq!(lib.get("missing"), "?_missing");
        assert!(std::ptr::eq(lib.get("missing"), lib.get("missing")));
        let other = LanguageLibrary::parse_yml("missing: present\n", "other".into()).unwrap();
        assert_eq!(other.get("missing"), "present");
        let empty = LanguageLibrary::parse_yml("", "empty".into()).unwrap();
        assert!(std::ptr::eq(lib.get("missing"), empty.get("missing")));
        assert!(
            LanguageLibrary::parse_yml("", "empty".into())
                .unwrap()
                .try_get("x")
                .is_none()
        );
    }
    #[test]
    fn schema_mismatch_panics_but_syntax_failure_returns_error() {
        assert!(LanguageLibrary::parse_yml("bad: [", "bad".into()).is_err());
        assert!(
            std::panic::catch_unwind(|| LanguageLibrary::parse_yml("bad: 123", "bad".into()))
                .is_err()
        );
        assert!(
            std::panic::catch_unwind(|| LanguageLibrary::parse_yml("- sequence", "bad".into()))
                .is_err()
        );
    }
}
