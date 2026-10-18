use super::*;
use crate::preference::replace_preference;
use std::{
    io::Write,
    sync::{
        OnceLock,
        atomic::{AtomicU64, Ordering},
    },
};
static TEST_ID: AtomicU64 = AtomicU64::new(0);
struct Fixture(PathBuf);
impl Fixture {
    fn new() -> Self {
        let root = std::env::temp_dir().join(format!(
            "tg-localization-test-{}-{}",
            std::process::id(),
            TEST_ID.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir_all(root.join("lang")).unwrap();
        let fixture = Self(root);
        fixture.pack("en-US", "Common: English\nEnglishOnly: Fallback\n");
        fixture.pack("a", "Common: A\n");
        fixture.pack("b", "Common: B\n");
        fixture
    }
    fn pack(&self, locale: &str, text: &str) {
        std::fs::write(self.0.join("lang").join(format!("{locale}.yml")), text).unwrap();
    }
    fn service(&self, fallback: bool) -> Localization {
        Localization::new(self.0.join("lang"), fallback).unwrap()
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}
#[test]
fn transactional_failure_preserves_old_snapshot_and_readers() {
    let fixture = Fixture::new();
    let service = fixture.service(true);
    service.load_locale("a").unwrap();
    let old = service.lookup("Common");
    let weak_text = Arc::downgrade(&old);
    let snapshot = service.snapshot.load_full().unwrap();
    let weak_snapshot = Arc::downgrade(&snapshot);
    drop(snapshot);
    std::fs::remove_file(fixture.0.join("lang/en-US.yml")).unwrap();
    assert!(service.load_locale("b").is_err());
    assert_eq!(&*service.game_locale(), "a");
    assert_eq!(&*service.lookup("Common"), "A");
    fixture.pack("en-US", "Common: English\nEnglishOnly: Fallback");
    service.load_locale("b").unwrap();
    assert!(weak_snapshot.upgrade().is_none());
    assert_eq!(&*old, "A");
    drop(old);
    assert!(weak_text.upgrade().is_none());
}
#[test]
fn malformed_requested_pack_falls_back_but_malformed_default_returns_error() {
    let fixture = Fixture::new();
    fixture.pack("bad", "Common: 123");
    for fallback_enabled in [false, true] {
        let service = fixture.service(fallback_enabled);
        // Whole-pack replacement remains available even when key fallback is off.
        service.load_locale("bad").unwrap();
        assert_eq!(&*service.game_locale(), "en-US");
        let snapshot = service.snapshot.load_full().unwrap();
        if fallback_enabled {
            assert!(Arc::ptr_eq(
                &snapshot.primary,
                snapshot.fallback.as_ref().unwrap()
            ));
        } else {
            assert!(snapshot.fallback.is_none());
        }
        drop(snapshot);
        fixture.pack("en-US", "Common: 123");
        assert!(service.load_locale("bad").is_err());
        assert_eq!(&*service.lookup("Common"), "English");
        for locale in ["../a", "a/b", "a\\b", "", "/a"] {
            assert!(service.load_locale(locale).is_err());
        }
        fixture.pack("en-US", "Common: English\nEnglishOnly: Fallback");
    }
}
#[test]
fn explicit_fallback_policy_and_snapshot_cache_lifetime() {
    let fixture = Fixture::new();
    let enabled = fixture.service(true);
    let disabled = fixture.service(false);
    enabled.load_locale("a").unwrap();
    disabled.load_locale("a").unwrap();
    assert_eq!(&*enabled.lookup("EnglishOnly"), "Fallback");
    assert_eq!(&*disabled.lookup("EnglishOnly"), "EnglishOnly");
    let missing = enabled.lookup("missing");
    assert_eq!(&*missing, "?_missing");
    assert!(Arc::ptr_eq(&missing, &enabled.lookup("missing")));
    let weak = Arc::downgrade(&missing);
    drop(missing);
    enabled.load_locale("b").unwrap();
    assert!(weak.upgrade().is_none());
    assert!(
        disabled
            .snapshot
            .load()
            .as_ref()
            .unwrap()
            .fallback
            .is_none()
    );
}
#[test]
fn preference_roundtrip_trimming_and_invalid_identifiers() {
    let fixture = Fixture::new();
    let root = fixture.0.join("override/Tiny Glade");
    let service = fixture.service(true);
    service.load_locale_and_save("a", &root).unwrap();
    service.load_locale_and_save("b", &root).unwrap();
    let restored = fixture.service(true);
    restored.load_last_used_locale(&root).unwrap();
    assert_eq!(&*restored.game_locale(), "b");
    std::fs::write(root.join("last_used_locale"), " a\n").unwrap();
    restored.load_last_used_locale(&root).unwrap();
    assert_eq!(&*restored.game_locale(), "a");
    std::fs::write(root.join("last_used_locale"), "../outside").unwrap();
    restored.load_last_used_locale(&root).unwrap();
    assert_eq!(&*restored.game_locale(), "en-US");
    assert_eq!(std::fs::read_dir(root).unwrap().count(), 1);
}
#[test]
fn write_and_replacement_errors_preserve_old_file_and_cleanup() {
    let fixture = Fixture::new();
    let target = fixture.0.join("last_used_locale");
    std::fs::write(&target, "a").unwrap();
    let error = || std::io::Error::other("injected failure");
    assert!(
        replace_preference(
            &target,
            |file| {
                file.write_all(b"partial")?;
                Err(error())
            },
            |_, _| panic!("must not replace")
        )
        .is_err()
    );
    assert_eq!(std::fs::read_to_string(&target).unwrap(), "a");
    assert!(replace_preference(&target, |file| file.write_all(b"b"), |_, _| Err(error())).is_err());
    assert_eq!(std::fs::read_to_string(&target).unwrap(), "a");
    assert!(
        !std::fs::read_dir(&fixture.0).unwrap().any(|e| e
            .unwrap()
            .file_name()
            .to_string_lossy()
            .ends_with(".tmp"))
    );
    let blocked = fixture.0.join("blocked");
    std::fs::write(&blocked, b"file").unwrap();
    let service = fixture.service(true);
    service.load_locale_and_save("b", &blocked).unwrap();
    assert_eq!(&*service.game_locale(), "b");
}
#[test]
fn concurrent_loads_commit_complete_snapshots_and_matching_preferences() {
    let fixture = Fixture::new();
    let root = fixture.0.join("preferences");
    let service = Arc::new(fixture.service(true));
    service.load_locale("a").unwrap();
    std::thread::scope(|scope| {
        for locale in ["a", "b"] {
            let service = &service;
            let root = &root;
            scope.spawn(move || {
                for _ in 0..100 {
                    service.load_locale_and_save(locale, root).unwrap();
                }
            });
        }
        let service = &service;
        scope.spawn(move || {
            for _ in 0..1000 {
                let snapshot = service.snapshot.load_full().unwrap();
                let locale = snapshot.primary.locale();
                let value = snapshot.primary.get("Common");
                assert_eq!(&*value, if &*locale == "a" { "A" } else { "B" });
                assert_eq!(
                    &*snapshot.fallback.as_ref().unwrap().get("EnglishOnly"),
                    "Fallback"
                );
            }
        });
    });
    assert_eq!(
        std::fs::read_to_string(root.join("last_used_locale")).unwrap(),
        &*service.game_locale()
    );
}
#[test]
fn logger_can_reenter_lookup_and_load_without_recursion_or_deadlock() {
    static LIBRARY: OnceLock<LanguageLibrary> = OnceLock::new();
    static SERVICE: OnceLock<Localization> = OnceLock::new();
    thread_local! {
        static ACTIVE: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
        static CALLS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    }
    struct ReentrantLogger;
    impl log::Log for ReentrantLogger {
        fn enabled(&self, _: &log::Metadata<'_>) -> bool {
            true
        }
        fn log(&self, _: &log::Record<'_>) {
            if !ACTIVE.get() {
                return;
            }
            let calls = CALLS.get() + 1;
            CALLS.set(calls);
            // Bound the broken case so a regression fails instead of overflowing
            // the stack. This logger deliberately invokes both service surfaces.
            if calls < 4 {
                let _ = LIBRARY.get().unwrap().get("logger-inner");
                SERVICE.get().unwrap().load_locale("unavailable").unwrap();
            }
        }
        fn flush(&self) {}
    }
    static LOGGER: ReentrantLogger = ReentrantLogger;
    LIBRARY
        .set(LanguageLibrary::new("logger".into(), Default::default()))
        .unwrap();
    let fixture = Fixture::new();
    assert!(SERVICE.set(fixture.service(true)).is_ok());
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(log::LevelFilter::Warn);
    ACTIVE.set(true);
    assert_eq!(
        &*LIBRARY.get().unwrap().get("logger-outer"),
        "?_logger-outer"
    );
    assert_eq!(
        &*SERVICE.get().unwrap().lookup("service-outer"),
        "?_service-outer"
    );
    SERVICE.get().unwrap().load_locale("unavailable").unwrap();
    ACTIVE.set(false);
    assert_eq!(
        CALLS.get(),
        3,
        "nested service diagnostics must be suppressed"
    );
}
