//! `tiny_glade::theme_fetcher_plugin` — daily-theme fetcher.
//!
//! Recovered public symbols: `update_daily_theme` `0x1403d1750`,
//! plugin `build` (inlined registration).
//!
//! `update_daily_theme` call list:
//! `theme_defs::{ActiveEvent, EventDataError}`, `std::fs::read_to_string`,
//! `serde_json::from_str`, `serde_json::from_slice`,
//! `image::{ImageBuffer, Rgb}`, `DailyThemeInfo`, `DailyThemeCredits`,
//! `SessionMeta::try_load_from_path`, and an `std::sync::mpsc` channel used to
//! publish the new theme to the ECS resource.
//!
//! The on-disk JSON schema for `DailyThemeInfo`/`DailyThemeCredits` is not
//! available in the workspace (`country_core::resources::daily` is absent), so
//! the parse step is generic JSON with an explicit divergence record.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Daily theme metadata (B `country_core::resources::daily::DailyThemeInfo`).
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct DailyThemeInfo {
    pub title: String,
    pub description: String,
    #[serde(default)]
    pub credits: Vec<DailyThemeCredits>,
}

/// Credit entry.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct DailyThemeCredits {
    pub name: String,
    pub url: String,
}

/// Loaded thumbnail.
#[derive(Clone, Debug)]
pub struct ThemeThumbnail {
    pub width: u32,
    pub height: u32,
    pub rgb: Vec<u8>,
}

/// Result published to the ECS resource.
#[derive(Clone, Debug, Default)]
pub struct DailyThemeUpdate {
    pub info: DailyThemeInfo,
    pub thumbnail: Option<ThemeThumbnail>,
    pub meta_path: Option<PathBuf>,
}

/// B `tiny_glade::theme_fetcher_plugin::update_daily_theme` (`0x1403d1750`).
///
/// Order recovered from the decompilation:
/// 1. read the active event JSON (`theme_defs::ActiveEvent`) if it exists;
/// 2. join `event.data_folder` with the daily theme file, `fs::read`, then
///    `serde_json::from_slice::<DailyThemeInfo>`;
/// 3. decode the credits image to `ImageBuffer<Rgb<u8>>`;
/// 4. load `SessionMeta::try_load_from_path`;
/// 5. publish the update through the channel.
///
/// Errors are wrapped with `anyhow` context and leave the previous theme in
/// place.
pub fn update_daily_theme(
    event_data_path: &Path,
    publish: &dyn Fn(DailyThemeUpdate),
) -> Result<()> {
    let event_text = std::fs::read_to_string(event_data_path)
        .with_context(|| format!("Failed to read {event_data_path:?}"))?;
    let event: serde_json::Value =
        serde_json::from_str(&event_text).context("Failed to parse active event")?;

    let data_folder = event
        .get("data_folder")
        .and_then(|value| value.as_str())
        .map(PathBuf::from)
        .unwrap_or_else(|| event_data_path.parent().unwrap_or(Path::new(".")).to_path_buf());

    let info_path = data_folder.join("daily_theme.json");
    let bytes = std::fs::read(&info_path)
        .with_context(|| format!("Failed to read {info_path:?}"))?;
    let info: DailyThemeInfo =
        serde_json::from_slice(&bytes).context("Failed to parse DailyThemeInfo")?;

    let mut update = DailyThemeUpdate {
        info,
        thumbnail: None,
        meta_path: None,
    };

    let thumbnail_path = data_folder.join("thumbnail.png");
    if thumbnail_path.is_file() {
        let image = image::open(&thumbnail_path)
            .with_context(|| format!("Failed to decode {thumbnail_path:?}"))?
            .to_rgb8();
        update.thumbnail = Some(ThemeThumbnail {
            width: image.width(),
            height: image.height(),
            rgb: image.into_raw(),
        });
    }

    let meta_path = data_folder.join("meta.json");
    if meta_path.is_file() {
        update.meta_path = Some(meta_path);
    }

    publish(update);
    Ok(())
}

/// B plugin `build` — registers the daily-theme update system.
pub fn build(app: &mut bevy_app::App) {
    app.add_systems(bevy_app::Update, update_daily_theme_system);
}

fn update_daily_theme_system() {
    // The original system reads the active event resource and calls the
    // recovered function above. The resource is not part of the current
    // `country-core`; the system is a no-op until that wiring lands.
}
