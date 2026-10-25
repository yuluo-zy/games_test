use std::{path::PathBuf, process::Command};
#[test]
fn global_configuration_in_subprocesses() {
    let root = std::env::temp_dir().join(format!("tg-language-global-{}", std::process::id()));
    for mode in ["default", "explicit", "disabled", "missing-home"] {
        if mode == "missing-home" && cfg!(windows) {
            continue;
        }
        let directory = root.join(mode);
        std::fs::create_dir_all(&directory).unwrap();
        let mut command = Command::new(std::env::current_exe().unwrap());
        command
            .args(["--exact", "global_child", "--nocapture"])
            .env("TG_LANGUAGE_TEST_ROOT", &directory)
            .env("TG_LANGUAGE_TEST_MODE", mode)
            .env_remove("TG_DISABLE_LANG_FALLBACK");
        if mode == "missing-home" {
            command.env_remove("HOME").env_remove("XDG_DATA_HOME");
        }
        if mode == "disabled" {
            command.env("TG_DISABLE_LANG_FALLBACK", "0");
        }
        let output = command.output().unwrap();
        assert!(
            output.status.success(),
            "{}\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }
    std::fs::remove_dir_all(root).unwrap();
}
#[test]
fn global_child() {
    let Some(root) = std::env::var_os("TG_LANGUAGE_TEST_ROOT") else {
        return;
    };
    let root = PathBuf::from(root).canonicalize().unwrap();
    let mode = std::env::var("TG_LANGUAGE_TEST_MODE").unwrap();
    std::fs::create_dir_all(root.join("assets/lang")).unwrap();
    std::fs::write(
        root.join("assets/lang/en-US.yml"),
        "Common: English\nOnly: Fallback",
    )
    .unwrap();
    std::fs::write(root.join("assets/lang/a.yml"), "Common: A").unwrap();
    std::env::set_current_dir(&root).unwrap();
    assert_eq!(&*lang_lib::lookup("Common"), "Common");
    if mode == "explicit" {
        lang_lib::configure("assets/lang", true).unwrap();
    }
    if mode == "missing-home" {
        assert!(paths::preference_root_path().is_err());
        lang_lib::load_locale("a").unwrap();
        assert_eq!(&*lang_lib::game_locale(), "a");
        lang_lib::load_last_used_locale().unwrap();
        assert_eq!(&*lang_lib::game_locale(), "en-US");
        return;
    }
    paths::initialize_paths(1, Some(std::path::Path::new("userdata"))).unwrap();
    lang_lib::load_locale("a").unwrap();
    assert_eq!(
        &*lang_lib::lookup("Only"),
        if mode == "disabled" {
            "Only"
        } else {
            "Fallback"
        }
    );
    assert!(lang_lib::configure("elsewhere", false).is_err());
    let held = lang_lib::lookup("Common");
    std::fs::create_dir(root.join("other")).unwrap();
    std::env::set_current_dir(root.join("other")).unwrap();
    lang_lib::load_locale("en-US").unwrap();
    assert_eq!(&*held, "A");
    assert_eq!(
        lang_lib::lang_pack_path("a").unwrap(),
        root.join("assets/lang/a.yml")
    );
    std::fs::write(
        paths::preference_root_path()
            .unwrap()
            .join("last_used_locale"),
        " a\n",
    )
    .unwrap();
    lang_lib::load_last_used_locale().unwrap();
    assert_eq!(&*lang_lib::game_locale(), "a");
    assert!(lang_lib::lang_pack_path("../outside").is_err());
}
