//! Process isolation prevents global initialization and cwd from leaking between tests.
use std::{path::PathBuf, process::Command};
#[test]
fn global_initialization_in_subprocess() {
    let root = std::env::temp_dir().join(format!("tg-path-global-{}", std::process::id()));
    std::fs::create_dir_all(&root).unwrap();
    let output = Command::new(std::env::current_exe().unwrap())
        .args(["--exact", "global_child", "--nocapture"])
        .env("TG_PATH_TEST_ROOT", &root)
        .output()
        .unwrap();
    let _ = std::fs::remove_dir_all(root);
    assert!(
        output.status.success(),
        "{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}
#[test]
fn global_child() {
    let Some(root) = std::env::var_os("TG_PATH_TEST_ROOT") else {
        return;
    };
    let root = PathBuf::from(root).canonicalize().unwrap();
    assert!(paths::try_paths().is_err());
    assert!(std::panic::catch_unwind(paths::config).is_err());
    let blocked = root.join("blocked/Tiny Glade/Steam/1");
    std::fs::create_dir_all(&blocked).unwrap();
    std::fs::write(blocked.join("beta-saves"), b"blocked").unwrap();
    assert!(paths::initialize_paths(1, Some(&root.join("blocked"))).is_err());
    assert!(paths::try_paths().is_err());
    std::env::set_current_dir(&root).unwrap();
    let barrier = std::sync::Arc::new(std::sync::Barrier::new(8));
    let joins: Vec<_> = (0..8)
        .map(|id| {
            let barrier = barrier.clone();
            std::thread::spawn(move || {
                barrier.wait();
                paths::initialize_paths(id, Some(&PathBuf::from(format!("race/{id}")))).is_ok()
            })
        })
        .collect();
    assert_eq!(
        joins
            .into_iter()
            .map(|j| j.join().unwrap())
            .filter(|ok| *ok)
            .count(),
        1
    );
    assert_eq!(std::fs::read_dir(root.join("race")).unwrap().count(), 1);
    let before = paths::config().to_owned();
    assert!(before.is_absolute());
    std::fs::create_dir(root.join("elsewhere")).unwrap();
    std::env::set_current_dir(root.join("elsewhere")).unwrap();
    assert_eq!(paths::config(), before);
    assert_eq!(
        paths::preference_root_path().unwrap(),
        paths::standard_app_root_path()
    );
    assert_eq!(
        paths::installed(),
        std::env::current_exe().unwrap().parent().unwrap()
    );
    assert!(paths::initialize_paths(999, Some(&root.join("unexpected"))).is_err());
    assert!(!root.join("unexpected").exists());
}
