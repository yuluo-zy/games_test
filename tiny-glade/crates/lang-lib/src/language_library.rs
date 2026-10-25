//! Owned language packs with fallible, flat string-to-string YAML validation.
use crate::missing_cache::MissingCache;
use anyhow::{Context, Result, ensure};
use std::{collections::HashMap, path::Path, sync::Arc};
use yaml_rust2::{
    YamlLoader,
    parser::{Event, MarkedEventReceiver, Parser},
    scanner::Marker,
};

// YamlLoader overwrites duplicate mapping keys. Count the root entries before
// that information is lost, while leaving YAML typing and aliases to the loader.
#[derive(Default)]
struct PackLoader {
    yaml: YamlLoader,
    depth: usize,
    expecting_value: bool,
    entries: usize,
}

impl MarkedEventReceiver for PackLoader {
    fn on_event(&mut self, event: Event, marker: Marker) {
        match &event {
            Event::Scalar(..)
            | Event::Alias(..)
            | Event::MappingStart(..)
            | Event::SequenceStart(..)
                if self.depth == 1 =>
            {
                if !self.expecting_value {
                    self.entries += 1;
                }
                self.expecting_value = !self.expecting_value;
            }
            _ => {}
        }
        match &event {
            Event::MappingStart(..) | Event::SequenceStart(..) => self.depth += 1,
            Event::MappingEnd | Event::SequenceEnd => self.depth -= 1,
            _ => {}
        }
        self.yaml.on_event(event, marker);
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
/// An owned locale and translation table. Lookups clone only the string's `Arc`.
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
    /// Read a UTF-8 pack, reporting its file path on I/O or schema errors.
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
        let mut loader = PackLoader::default();
        Parser::new_from_str(text)
            .load(&mut loader, true)
            .context("Parsing YAML")?;
        let documents = loader.yaml.documents();
        ensure!(
            documents.len() <= 1,
            "Language pack must contain at most one YAML document"
        );
        let mut entries = HashMap::new();
        if let Some(document) = documents.first() {
            let mapping = document
                .as_hash()
                .context("Language pack root must be a mapping")?;
            ensure!(
                mapping.len() == loader.entries,
                "Language pack contains duplicate mapping keys or invalid entries"
            );
            entries.reserve(mapping.len());
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
    /// Return a shared owned value without generating missing-key diagnostics.
    pub fn try_get(&self, key: &str) -> Option<Arc<str>> {
        self.entries.get(key).map(Arc::clone)
    }
    /// Return a translation or a bounded, cached `?_<key>` marker.
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
        let aliases =
            LanguageLibrary::parse_yml("anchor: &key name\n? *key\n: value", "test".into())
                .unwrap();
        assert_eq!(&*aliases.get("name"), "value");
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
            "key: first\nkey: second",
            "key: 123\nkey: second",
            "key: {nested: value}\nkey: second",
            "key: first\n\"key\": second",
            "anchor: &key name\nname: first\n? *key\n: second",
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
