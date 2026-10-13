//! Integration probe against the original B language assets; no game assets are changed.
use anyhow::{Context, Result};
use lang_lib::{LanguageLibrary, game_locale, load_locale, lookup};
use std::path::PathBuf;

fn main() -> Result<()> {
    let mut args = std::env::args_os().skip(1);
    let game = PathBuf::from(args.next().context("game directory argument")?);
    let owned_probe = PathBuf::from(
        args.next()
            .context("explicit probe output directory argument")?,
    );
    std::env::set_current_dir(&game)?;
    // Synthetic account in the explicitly supplied probe directory only.
    paths::initialize_paths(123456789, Some(&owned_probe))?;
    let before = lookup("AreYouSure");
    anyhow::ensure!(before == "AreYouSure", "preinit lookup changed the key");
    anyhow::ensure!(game_locale() == "en-US", "preinit locale");
    let mut locales = Vec::new();
    for entry in std::fs::read_dir(game.join("assets/lang"))? {
        let path = entry?.path();
        if path.extension().and_then(|x| x.to_str()) != Some("yml") {
            continue;
        }
        let locale = path.file_stem().unwrap().to_str().unwrap().to_owned();
        let library = LanguageLibrary::try_from_yml(&path, locale.clone())?;
        anyhow::ensure!(
            library.try_get("AreYouSure").is_some(),
            "missing required real asset key in {locale}"
        );
        locales.push(locale);
    }
    locales.sort();
    load_locale("zh-CN")?;
    anyhow::ensure!(game_locale() == "zh-CN", "selected locale");
    let translated = lookup("AreYouSure");
    anyhow::ensure!(translated != "AreYouSure", "real Chinese translation");
    load_locale("en-US")?;
    anyhow::ensure!(
        lookup("AreYouSure") == "Are you sure?",
        "real English translation"
    );
    // Held static text remains valid after replacement.
    anyhow::ensure!(!translated.is_empty(), "text survival across replacement");
    load_locale("__missing_probe_locale__")?;
    anyhow::ensure!(game_locale() == "en-US", "missing pack fallback");
    anyhow::ensure!(
        std::fs::read_to_string(paths::standard_app_root_path().join("last_used_locale"))?
            == "__missing_probe_locale__",
        "requested locale persistence"
    );
    if std::env::var("TG_DISABLE_LANG_FALLBACK").is_err() {
        anyhow::ensure!(
            lookup("__missing_probe_key__") == "?___missing_probe_key__",
            "missing key fallback marker"
        );
    } else {
        anyhow::ensure!(
            lookup("__missing_probe_key__") == "__missing_probe_key__",
            "fallback disabled"
        );
    }
    println!("verified locales: {}", locales.join(","));
    println!("candidate asset integration passed; original native behavior not executed");
    Ok(())
}
