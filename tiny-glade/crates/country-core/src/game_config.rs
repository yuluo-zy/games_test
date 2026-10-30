//! game_config.rs — `country_core::game_config` 游戏配置资源。
//!
//! 2026-10-17 unlock-tinyglade-deps：按 fresh 反编译落库真实字段/serde 布局
//! （Ghidra shared 只读，证据 `work/unlock-tinyglade-deps/`）：
//!
//! | 项 | VA | 证据 |
//! |---|---|---|
//! | `GameConfig` serialize | 0x1402E73B0 | struct "GameConfig"，字段按序 `video`/`audio`/`ui`/`dev`/`game`/`lang_override` |
//! | `GameConfig` FieldVisitor | 0x1402E7DE0 | 名字集合同上；`lang_override` 另有别名 `lang`、`overwrite_lang`（0xD/0xE 长度分支） |
//! | `GameConfig` Default | 0x140914880 | 嵌套默认值（video/audio/ui/dev/game/lang=None） |
//! | `GameConfig::upgrade` | 0x14090DF90 | 内联 `VideoSettings::upgrade` |
//! | `DevSettings` serialize | 0x1402E7650 | 8 bool：`capture_crashes`…`enable_asset_pipeline` @+0..+7 |
//! | `DevSettings` Default | 0x140914830/`GameConfig` @+0xe0 | 0x01000101 |
//! | `GameSettings` serialize | 0x1402E7940 | 11 字段（f32/bool/Option/ScreenshotFormat） |
//! | `GameSettings` Default | 0x140914850 | 逐字段常量 |
//! | `ScreenshotFormat` serialize | 0x1402E7D10 | 判别式 0→"Jpeg"、1→"Png"（0x1429B57A4/0x1429B57A8） |
//!
//! 内存布局（O，用于对照；语义结构不强制 `repr(C)`）：
//! `video @0x00`（0xC8）、`lang_override @0xC8`（Option<String> 0x18）、
//! `dev @0xE0`（8）、`ui @0xE8`（0x10）、`audio @0xF8`（0x10）、
//! `game @0x108`（0x20），总 0x128。

use bevy_ecs::resource::Resource;
use serde::{Deserialize, Serialize};

use crate::render::render_config::VideoSettings;
use crate::resources::audio_settings::AudioSettings;
use crate::resources::ui_settings::UiSettings;
use crate::systems::main_camera::CameraRotationPivot;

/// 游戏配置。serde 字段顺序 = 原序列化顺序。
#[derive(Resource, Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct GameConfig {
    /// VideoSettings @+0x00（0xC8 B）。
    pub video: VideoSettings,
    /// AudioSettings @+0xF8（0x10 B）。
    pub audio: AudioSettings,
    /// UiSettings @+0xE8（0x10 B）。
    pub ui: UiSettings,
    /// DevSettings @+0xE0（8 B）。
    pub dev: DevSettings,
    /// GameSettings @+0x108（0x20 B）。
    pub game: GameSettings,
    /// Option<String> @+0xC8；原字段名 `lang_override`，deserializer 另接受
    /// `lang`、`overwrite_lang`（0x1402E7DE0 长度 5/0xD/0xE 分支）。
    #[serde(alias = "lang", alias = "overwrite_lang")]
    pub lang_override: Option<String>,
    /// **兼容 seam**：原字段不存在；`display_glade_border`（0x141BE0AD0）
    /// 读 `GameConfig+0x121`，即本结构 `game.hide_glade_border_outline`
    /// （GameSettings+0x19）。该消费者尚未改名，暂保留同名镜像字段（不参与 serde）。
    /// 解锁条件：glade-border 改读 `config.game.hide_glade_border_outline` 后删除。
    #[serde(skip)]
    pub border_signifier_enabled: bool,
}

impl GameConfig {
    /// `GameConfig::upgrade` @0x14090DF90 —— 内联 `VideoSettings::upgrade`。
    pub fn upgrade(&mut self) {
        self.video.upgrade();
    }
}

/// `DevSettings`（fresh 0x1402E7650；8 × bool @+0x00..+0x07）。
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct DevSettings {
    pub capture_crashes: bool,
    pub report_crashes: bool,
    pub internal_hotkeys: bool,
    pub day_night_cycle: bool,
    pub skip_splash: bool,
    pub skip_welcome: bool,
    pub demo: bool,
    pub enable_asset_pipeline: bool,
}

impl Default for DevSettings {
    /// `GameConfig::default` 0x140914880 在 dev 槽写 `0x01000101`（8 字节）：
    /// capture=1 / report=1 / internal=0 / day_night=1 / skip_splash=0 /
    /// skip_welcome=0 / demo=0 / enable_asset_pipeline=0。
    fn default() -> Self {
        Self {
            capture_crashes: true,
            report_crashes: true,
            internal_hotkeys: false,
            day_night_cycle: true,
            skip_splash: false,
            skip_welcome: false,
            demo: false,
            enable_asset_pipeline: false,
        }
    }
}

impl DevSettings {
    /// `DevSettings::live_link` @0x14090DF40：`movzx eax,[rcx+7]` → 读 +0x07，
    /// 即 `enable_asset_pipeline`（serde 字段顺序 @+7）。
    pub fn live_link(&self) -> bool {
        self.enable_asset_pipeline
    }
}

/// `ScreenshotFormat`（fresh 0x1402E7D10 serialize：0→"Jpeg"、1→"Png"；
/// Display 0x14090DF50 文案 .rdata 0x142AAB678="JPEG" / 0x142AAB67C="PNG"）。
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum ScreenshotFormat {
    #[default]
    Jpeg = 0,
    Png = 1,
}

impl core::fmt::Display for ScreenshotFormat {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Jpeg => f.write_str("JPEG"),
            Self::Png => f.write_str("PNG"),
        }
    }
}

/// `GameSettings`（fresh 0x1402E7940 serialize；默认 0x140914850）。
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GameSettings {
    /// @+0x00，默认 1.0。
    pub camera_rotation_sensitivity: f32,
    /// @+0x04，默认 1.0。
    pub camera_movement_speed: f32,
    /// @+0x16，默认 false。
    pub camera_invert_x: bool,
    /// @+0x17，默认 false。
    pub camera_invert_y: bool,
    /// @+0x14，`Option<CameraRotationPivot>`（niche 6 = None；None 时不序列化）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub camera_rotation_pivot: Option<CameraRotationPivot>,
    /// @+0x15，`Option<CameraRotationPivot>`（同一枚举类型；PDB 无 `CameraZoomPivot`）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub camera_zoom_pivot: Option<CameraRotationPivot>,
    /// @+0x08，默认 0.3。
    pub camera_pivot_filter_size: f32,
    /// @+0x0C，默认 45.0。
    pub camera_fov: f32,
    /// @+0x10，默认 1.0。
    pub tod_speed: f32,
    /// @+0x18，默认 Jpeg。
    pub screenshot_format: ScreenshotFormat,
    /// @+0x19，默认 false。
    pub hide_glade_border_outline: bool,
}

impl Default for GameSettings {
    /// `GameSettings::default` 0x140914850 逐常量：
    /// 1.0 / 1.0 / 0.3 / 45.0 / 1.0 / pivot=None(6) / pivot=None(6) /
    /// invert=false / invert=false / ScreenshotFormat=0 / hide=false。
    fn default() -> Self {
        Self {
            camera_rotation_sensitivity: 1.0,
            camera_movement_speed: 1.0,
            camera_invert_x: false,
            camera_invert_y: false,
            camera_rotation_pivot: None,
            camera_zoom_pivot: None,
            camera_pivot_filter_size: 0.3,
            camera_fov: 45.0,
            tod_speed: 1.0,
            screenshot_format: ScreenshotFormat::Jpeg,
            hide_glade_border_outline: false,
        }
    }
}

/// 原getter14090e070读取143634cd8，由main从dev.internal_hotkeys一次初始化。
/// demo/glade边界读取另一个143634d90，不能共用该热键门。
pub const DEVELOPER_MODE_FLAG_ADDR: u64 = 0x1_4363_4cd8;

/// 原进程级开发者热键门；按启动快照读取，不逐帧镜像配置或demo开关。
pub fn developer_hotkeys_enabled() -> bool {
    DEVELOPER_MODE.load(core::sync::atomic::Ordering::Relaxed)
}
static DEVELOPER_MODE: core::sync::atomic::AtomicBool = core::sync::atomic::AtomicBool::new(false);

/// The original main function writes `GameConfig.dev.internal_hotkeys` to the
/// same byte read by `developer_hotkeys_enabled` (B 0x143634cd8).
/// Atomic storage permits the source host's OS/game threads to share that byte.
pub fn set_developer_hotkeys_enabled(enabled: bool) {
    DEVELOPER_MODE.store(enabled, core::sync::atomic::Ordering::Relaxed);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serde_field_order_matches_original_serializer() {
        let text = ron::ser::to_string(&GameConfig::default()).expect("serialize");
        let order = ["video:", "audio:", "ui:", "dev:", "game:", "lang_override:"];
        let mut cursor = 0;
        for key in order {
            let at = text[cursor..]
                .find(key)
                .unwrap_or_else(|| panic!("missing {key} in {text}"));
            cursor += at;
        }
        // `lang` / `overwrite_lang` 是反序列化别名，序列化必须用 canonical 名。
        assert!(!text.contains("lang:"));
        assert!(!text.contains("overwrite_lang:"));
    }

    #[test]
    fn default_settings_ron_is_tolerated() {
        // B 发行 `default_settings.ron` = `(tutorial: true,)`：未知字段被忽略，
        // 其它字段走 #[serde(default)]。
        let config: GameConfig = ron::from_str("(tutorial: true,)").expect("parse");
        assert_eq!(config.dev, DevSettings::default());
        assert!(config.dev.live_link() == config.dev.enable_asset_pipeline);
    }

    #[test]
    fn dev_defaults_match_game_config_default_bytes() {
        let dev = DevSettings::default();
        assert!(dev.capture_crashes && dev.report_crashes && dev.day_night_cycle);
        assert!(!dev.internal_hotkeys && !dev.demo && !dev.enable_asset_pipeline);
    }

    #[test]
    fn game_settings_defaults_match_0x140914850() {
        let game = GameSettings::default();
        assert_eq!(game.camera_rotation_sensitivity, 1.0);
        assert_eq!(game.camera_movement_speed, 1.0);
        assert_eq!(game.camera_pivot_filter_size, 0.3);
        assert_eq!(game.camera_fov, 45.0);
        assert_eq!(game.tod_speed, 1.0);
        assert_eq!(game.camera_rotation_pivot, None);
        assert_eq!(game.camera_zoom_pivot, None);
        assert_eq!(game.screenshot_format, ScreenshotFormat::Jpeg);
        assert!(!game.hide_glade_border_outline);
    }
}
