use super::*;
use crate::global::GlobalPaths;
use crate::platform::{choose_unix_root, choose_windows_root};
use std::ffi::OsString;
static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
fn root() -> PathBuf {
    std::env::temp_dir().join(format!(
        "tg-path-test-{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    ))
}
#[test]
fn platform_policies_and_missing_environment() {
    let home = std::env::temp_dir().join("test-home");
    assert_eq!(
        choose_windows_root(Some(home.clone()), || panic!("must not query profile")).unwrap(),
        home.join("Tiny Glade")
    );
    assert_eq!(
        choose_windows_root(None, || Some(home.clone().into_os_string())).unwrap(),
        home.join("Saved Games/Tiny Glade")
    );
    assert!(choose_windows_root(None, || None).is_err());
    assert_eq!(
        choose_unix_root(Some(home.clone().into_os_string()), None, true).unwrap(),
        home.join("Library/Application Support/Tiny Glade")
    );
    assert_eq!(
        choose_unix_root(
            Some(home.clone().into_os_string()),
            Some("relative".into()),
            false
        )
        .unwrap(),
        home.join(".local/share/Tiny Glade")
    );
    assert_eq!(
        choose_unix_root(None, Some(home.clone().into_os_string()), false).unwrap(),
        home.join("Tiny Glade")
    );
    assert!(choose_unix_root(None, None, false).is_err());
    assert!(choose_unix_root(Some("".into()), None, true).is_err());
}
#[test]
#[cfg(unix)]
fn non_unicode_home_is_supported() {
    use std::os::unix::ffi::OsStringExt;
    let home = std::env::temp_dir().join(OsString::from_vec(vec![b'h', 255]));
    assert_eq!(
        choose_unix_root(Some(home.clone().into_os_string()), None, false).unwrap(),
        home.join(".local/share/Tiny Glade")
    );
}
#[test]
#[cfg(windows)]
fn non_unicode_windows_profile_is_supported() {
    use std::os::windows::ffi::OsStringExt;
    let home = std::env::temp_dir().join(OsString::from_wide(&[b'h' as u16, 0xd800]));
    assert_eq!(
        choose_windows_root(None, || Some(home.clone().into_os_string())).unwrap(),
        home.join("Saved Games/Tiny Glade")
    );
}
#[test]
fn invalid_injected_path_fails_before_creating_directories() {
    let root = root();
    assert!(Paths::initialize_at(1, root.clone(), PathBuf::new()).is_err());
    assert!(!root.exists());
}
#[test]
fn account_paths_and_partial_failure() {
    let root = root();
    let app = root.join("Tiny Glade");
    let p = Paths::initialize_at(7, app.clone(), PathBuf::from("installation")).unwrap();
    assert_eq!(p.config(), app.join("Steam/7"));
    assert!(p.installed().is_absolute());
    for name in [
        "saves",
        "beta-saves",
        "tmp",
        "deleted-saves",
        "backup-saves",
    ] {
        assert!(p.config().join(name).is_dir());
    }
    let bad = app.join("Steam/8");
    std::fs::create_dir_all(&bad).unwrap();
    std::fs::write(bad.join("beta-saves"), b"file blocks folder").unwrap();
    assert!(Paths::initialize_at(8, app, root.clone()).is_err());
    assert!(bad.join("saves").is_dir());
    assert!(!bad.join("tmp").exists());
    std::fs::remove_dir_all(root).unwrap();
}
#[test]
fn failed_initialization_can_retry_and_reinitialization_has_no_side_effects() {
    let storage = GlobalPaths::default();
    assert!(
        storage
            .initialize(|| anyhow::bail!("injected failure"))
            .is_err()
    );
    let root = root();
    storage
        .initialize(|| Paths::initialize_at(1, root.clone(), root.clone()))
        .unwrap();
    assert!(
        storage
            .initialize(|| panic!("second builder must not run"))
            .is_err()
    );
    std::fs::remove_dir_all(root).unwrap();
}
#[test]
fn concurrent_initialization_runs_one_builder() {
    let storage = std::sync::Arc::new(GlobalPaths::default());
    let root = root();
    let barrier = std::sync::Arc::new(std::sync::Barrier::new(8));
    let joins: Vec<_> = (0..8)
        .map(|id| {
            let storage = storage.clone();
            let root = root.clone();
            let barrier = barrier.clone();
            std::thread::spawn(move || {
                barrier.wait();
                storage
                    .initialize(|| {
                        Paths::initialize_at(id, root.join(id.to_string()), root.clone())
                    })
                    .is_ok()
            })
        })
        .collect();
    assert_eq!(
        joins
            .into_iter()
            .map(|join| join.join().unwrap())
            .filter(|ok| *ok)
            .count(),
        1
    );
    assert_eq!(std::fs::read_dir(&root).unwrap().count(), 1);
    std::fs::remove_dir_all(root).unwrap();
}
