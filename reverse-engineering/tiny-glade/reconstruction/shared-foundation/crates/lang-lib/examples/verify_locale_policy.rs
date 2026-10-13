//! Isolated, single-threaded candidate policy probes. Original game is not run.
use anyhow::{Context, Result, ensure};
use lang_lib::{game_locale, load_locale, lookup};
use std::path::{Path, PathBuf};
use std::process::Command;

fn child(root: &Path, mode: &str) -> Result<()> {
    std::fs::create_dir_all(root.join("assets/lang"))?;
    std::env::set_current_dir(root)?;
    paths::initialize_paths(42, Some(&root.join("user-data")))?;
    std::fs::write("assets/lang/en-US.yml", "Common: English\nEnglishOnly: Fallback\n")?;
    std::fs::write("assets/lang/primary.yml", "Common: Primary\n")?;
    ensure!(lookup("Common") == "Common" && game_locale() == "en-US", "preinit");
    load_locale("primary")?;
    let held = lookup("Common");
    ensure!(held == "Primary" && game_locale() == "primary", "selected locale");
    let enabled = mode == "enabled" || mode == "non-unicode" || mode == "disable-after-load";
    ensure!(lookup("EnglishOnly") == if enabled {"Fallback"} else {"EnglishOnly"}, "secondary fallback");
    let missing = lookup("NotInEither");
    ensure!(missing == if enabled {"?_NotInEither"} else {"NotInEither"}, "missing policy");
    if mode == "disable-after-load" {
        // This standalone probe has no threads, and no code reads the environment
        // concurrently. Rust 2024 marks process-global mutation unsafe.
        unsafe { std::env::set_var("TG_DISABLE_LANG_FALLBACK", "0") };
    }
    std::fs::write("assets/lang/en-US.yml", "Common: English2\nEnglishOnly: Refreshed\n")?;
    load_locale("primary")?;
    let expected = if mode == "disable-after-load" { "Fallback" }
        else if enabled { "Refreshed" } else { "EnglishOnly" };
    ensure!(lookup("EnglishOnly") == expected, "fallback replacement/preservation");
    ensure!(held == "Primary", "held text survives replacement");
    if enabled { ensure!(std::ptr::eq(missing, lookup("NotInEither")), "global missing cache"); }
    load_locale("unknown")?;
    ensure!(game_locale() == "en-US" && lookup("Common") == "English2", "pack fallback");
    ensure!(std::fs::read_to_string(paths::standard_app_root_path().join("last_used_locale"))? == "unknown", "requested locale saved");
    println!("policy mode {mode}: passed");
    Ok(())
}

fn main() -> Result<()> {
    let mut args = std::env::args_os().skip(1);
    let root = PathBuf::from(args.next().context("explicit owned probe directory")?);
    if let Some(mode) = args.next() {
        return child(&root, mode.to_str().context("mode UTF-8")?);
    }
    std::fs::create_dir_all(&root)?;
    for mode in ["enabled", "empty", "zero", "disable-after-load", "non-unicode"] {
        let mut command = Command::new(std::env::current_exe()?);
        command.arg(root.join(format!("{mode}-{}", std::process::id()))).arg(mode);
        command.env_remove("TG_DISABLE_LANG_FALLBACK");
        match mode {
            "empty" => { command.env("TG_DISABLE_LANG_FALLBACK", ""); }
            "zero" => { command.env("TG_DISABLE_LANG_FALLBACK", "0"); }
            "non-unicode" => {
                #[cfg(windows)] {
                    use std::os::windows::ffi::OsStringExt;
                    command.env("TG_DISABLE_LANG_FALLBACK", std::ffi::OsString::from_wide(&[0xd800]));
                }
                #[cfg(not(windows))] { anyhow::bail!("this B probe requires Windows"); }
            }
            _ => {}
        }
        ensure!(command.status()?.success(), "policy child {mode} failed");
    }
    println!("five isolated candidate modes passed; native behavior not executed");
    Ok(())
}
