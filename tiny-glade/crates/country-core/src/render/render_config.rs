//! 画质 / 显示设置的枚举与小函数（layer A1）。
//!
//! 证据（O，逐条核对机器码，见 `evidence/country-core-audit/L0_NOTES.md`）：
//!  * `FullscreenConfig::resolution`           0x140968ad0：判别式（u32 @+0）== 1 时返回 `Some((u32,u32) @+4)`，否则 None。
//!  * `UpscalingPreset::tgsr_resolution_scale` 0x140968bb0：按 u8 判别式查 f32 表（0x142ab572c）。
//!  * `UpscalingPreset` 的 `Sequence::previous` 0x140968f40：判别式 1..=4 → 判别式-1，0 → None。
//!  * `impl$5::fmt` 0x140968bc0（UpscalingPreset 的显示名，经 `lang_lib::lookup`）、`impl$2::fmt` 0x140968af0
//!    （ImageQualityPreset）、`impl$3::fmt` 0x140968b70（二值枚举）。
//!  * `MonitorMatch` 的 Debug（0x14048f410）给出字段名 `name`、`physical_position`、`physical_size`。
//!  * 默认值：`impl$23::default` 0x140968e50 / `impl$39::default` 0x140968e80 —— 含 1920×1080（0x780, 0x438）和
//!    60.0；整块结构的字段名未恢复，见 `L0_NOTES.md`，这里只落地已确认的函数与枚举。
//!
//! 旧版把 `UpscalingPreset` 写成 `None/TgsrHalf/…` 且数值表是编的，`ImageQualityPreset` 写成 Low/Medium/High/Ultra，
//! `FullscreenConfig` 写成 `{mode, resolution}` 结构体 —— 均与机器码不符。
//!
//! 2026-10-17 unlock-tinyglade-deps 增补（全部 fresh 反编译，B shared 只读）：
//!  * `VideoMode` 0x14003AAA0 serialize：字段顺序/名 `resolution`(array<u32,2> @+0)、
//!    `bit_depth`(u16 @+0xc)、`refresh_rate_millihertz`(u32 @+8)。
//!  * `MonitorMatch` 0x140039C10 serialize：`name` 是 **Option<String>**（不是 String），
//!    `physical_position` [i32;2] @+0x18、`physical_size` [u32;2] @+0x20。
//!  * `DisplayConfig` 0x140039E00 serialize：`fullscreen`(bool @+0x44)、
//!    `allow_exclusive_fullscreen`(bool @+0x45)、`fullscreen_config`(FullscreenConfig @+0x30)、
//!    `windowed_resolution`([u32;2] @+0x28)、`monitor`(Option<MonitorMatch> @+0)。
//!    默认 0x140968E50：fullscreen=true、allow_exclusive_fullscreen=true、
//!    fullscreen_config=0、windowed_resolution=[1920,1080]、monitor=None。
//!  * `VideoSettingsVersion` 由 0x14034F0F0 内联确认按裸 u32 序列化（transparent）。
//!  * `VideoSettings` 0x14003A110 serialize（22 字段）与默认 0x140968E80。
//!  * `GraphicsWorkarounds` 原始路径是 `rhapsody::raw::workarounds`；本 crate 不可改他模块，
//!    这里按 0x140082A00 serialize 的 4 个 bool 落 **合同 seam**，待 rhapsody 落地后改回。

use serde::{Deserialize, Serialize};

/// B startup and app-handoff resource (insert_resource @0x14012e420).
/// Three packed extents are copied from WindowSize @0x1405a2580/25ce/261c;
/// the source names of these three slots are unavailable. The two scales and
/// trailing flag are direct stores @0x1405a26bd..26cd. No native padding or
/// process pointer is represented here.
#[derive(bevy_ecs::resource::Resource, Clone, Copy, Debug, PartialEq)]
pub struct FrameRenderConfig {
    pub extent_0: [u32; 2],
    pub extent_8: [u32; 2],
    pub extent_10: [u32; 2],
    pub world_resolution_scale: f32,
    pub tgsr_resolution_scale: f32,
    pub flag_20: bool,
}

impl FrameRenderConfig {
    pub fn at_startup(window: crate::resources::window::WindowSize, video: &VideoSettings) -> Self {
        Self {
            extent_0: window.as_dims(),
            extent_8: window.as_dims(),
            extent_10: window.as_dims(),
            world_resolution_scale: video.world_resolution_scale.unwrap_or(1.0),
            tgsr_resolution_scale: if video.upscaling_mode == UpscalingMode::Tgsr {
                video.upscaling_preset.tgsr_resolution_scale()
            } else {
                1.0
            },
            flag_20: false,
        }
    }
}
use uuid::Uuid;

use rhapsody::raw::init::GraphicsInstanceLayerAllowlist;

/// 全屏配置。判别式 1 携带分辨率（O）；0 的含义为 I（无需分辨率的模式）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u32)]
pub enum FullscreenConfig {
    /// 判别式 0（I）。
    Borderless = 0,
    /// 判别式 1：独占全屏，分辨率 @+4。
    Exclusive { width: u32, height: u32 } = 1,
}

impl FullscreenConfig {
    /// O 0x140968ad0。
    pub fn resolution(&self) -> Option<(u32, u32)> {
        match *self {
            FullscreenConfig::Exclusive { width, height } => Some((width, height)),
            _ => None,
        }
    }
}

/// TGSR 上采样预设（5 个判别式，顺序由比例表 + `impl$5::fmt` 跳转表互证）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum UpscalingPreset {
    UltraPerformance = 0,
    Performance = 1,
    Balanced = 2,
    Quality = 3,
    UltraQuality = 4,
}

impl UpscalingPreset {
    pub const ALL: [UpscalingPreset; 5] = [
        UpscalingPreset::UltraPerformance,
        UpscalingPreset::Performance,
        UpscalingPreset::Balanced,
        UpscalingPreset::Quality,
        UpscalingPreset::UltraQuality,
    ];

    /// O 0x140968bb0：f32 表，内部分辨率相对输出的比例（1/3, 1/2, 25/43, 2/3, 10/13）。
    pub fn tgsr_resolution_scale(&self) -> f32 {
        const SCALE: [f32; 5] = [
            f32::from_bits(0x3eaa_aaab), // 0.33333334
            f32::from_bits(0x3f00_0000), // 0.5
            f32::from_bits(0x3f14_d653), // 0.5813953
            f32::from_bits(0x3f2a_aaab), // 0.6666667
            f32::from_bits(0x3f44_ec4f), // 0.7692308
        ];
        SCALE[*self as usize]
    }

    /// `enum_iterator::Sequence::previous`（O 0x140968f40）。
    pub fn previous(&self) -> Option<Self> {
        (*self as usize).checked_sub(1).map(|i| Self::ALL[i])
    }

    /// `impl$5::fmt` 的本地化键（O：`lang_lib::lookup(key)`）。
    pub fn lang_key(&self) -> &'static str {
        match self {
            Self::UltraPerformance => "Quality.UltraPerformance",
            Self::Performance => "Quality.Performance",
            Self::Balanced => "Quality.Balanced",
            Self::Quality => "Quality.Quality",
            Self::UltraQuality => "Quality.UltraQuality",
        }
    }
}

/// 画质预设（`impl$2::fmt` 跳转表：0 Performance、1 Balanced、2 Quality、3 UltraQuality、4 Custom）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum ImageQualityPreset {
    Performance = 0,
    Balanced = 1,
    Quality = 2,
    UltraQuality = 3,
    Custom = 4,
}

impl ImageQualityPreset {
    /// `impl$2::fmt` 的本地化键。
    pub fn lang_key(&self) -> &'static str {
        match self {
            Self::Performance => "Quality.Performance",
            Self::Balanced => "Quality.Balanced",
            Self::Quality => "Quality.Quality",
            Self::UltraQuality => "Quality.UltraQuality",
            Self::Custom => "Quality.Custom",
        }
    }
}

/// 二值枚举（`impl$3::fmt`：0 → 本地化键 "Tweaker.No"，1 → 字面量 "TGSR"）。名称 I：TGSR 开关。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum UpscalingMode {
    Off = 0,
    Tgsr = 1,
}

impl UpscalingMode {
    /// `Ok(Left(key))` 需经 `lang_lib::lookup`；`Right` 为直接输出的字面量。
    pub fn display(&self) -> Result<&'static str, &'static str> {
        match self {
            Self::Off => Err("Tweaker.No"),
            Self::Tgsr => Ok("TGSR"),
        }
    }
}

/// `MonitorMatch`（fresh 0x140039C10 serialize；字段名/类型 O）：
/// `name: Option<String> @+0x00`、`physical_position: [i32;2] @+0x18`、
/// `physical_size: [u32;2] @+0x20`，总 40 B（`monitor::from_winit` 0x140175450 回填）。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct MonitorMatch {
    pub name: Option<String>,
    pub physical_position: [i32; 2],
    pub physical_size: [u32; 2],
}

/// `VideoMode`（fresh 0x14003AAA0 serialize；内存布局 O，16 B）：
/// `resolution: [u32;2] @+0x00`、`refresh_rate_millihertz: u32 @+0x08`、
/// `bit_depth: u16 @+0x0c`。声明顺序即 serde 字段顺序。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct VideoMode {
    pub resolution: [u32; 2],
    pub bit_depth: u16,
    pub refresh_rate_millihertz: u32,
}

/// `VideoSettingsVersion`（O：0x14034F0F0 内联确认按裸 u32 序列化；大小 4 B @+0x94）。
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct VideoSettingsVersion(pub u32);

/// `DisplayConfig`（fresh 0x140039E00 serialize；默认 0x140968E50）。
///
/// 声明顺序 = serde 顺序：`fullscreen`, `allow_exclusive_fullscreen`,
/// `fullscreen_config`, `windowed_resolution`, `monitor`。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct DisplayConfig {
    /// @+0x44（默认 true）。
    pub fullscreen: bool,
    /// @+0x45（默认 true）。
    pub allow_exclusive_fullscreen: bool,
    /// @+0x30（默认判别式 0 = Borderless）。
    pub fullscreen_config: FullscreenConfig,
    /// @+0x28（默认 [1920, 1080]）。
    pub windowed_resolution: [u32; 2],
    /// @+0x00（默认 None）。
    pub monitor: Option<MonitorMatch>,
}

impl Default for DisplayConfig {
    fn default() -> Self {
        Self {
            fullscreen: true,
            allow_exclusive_fullscreen: true,
            fullscreen_config: FullscreenConfig::Borderless,
            windowed_resolution: [1920, 1080],
            monitor: None,
        }
    }
}

/// 合同 seam：原类型 `rhapsody::raw::workarounds::GraphicsWorkarounds`
/// （fresh 0x140082A00 serialize：4 个 bool，按序
/// `sync_before_rg_pass` @+0、`sync_before_draw` @+1、
/// `sync_before_dispatch` @+2、`serial_cb_recording` @+3；默认全 false）。
///
/// 解锁条件：rhapsody crate 落地该类型后，本文件改为
/// `pub use rhapsody::raw::workarounds::GraphicsWorkarounds;` 并删除本定义。
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct GraphicsWorkarounds {
    pub sync_before_rg_pass: bool,
    pub sync_before_draw: bool,
    pub sync_before_dispatch: bool,
    pub serial_cb_recording: bool,
}

/// `VideoSettings`（fresh 0x14003A110 serialize，22 字段；默认 0x140968E80）。
///
/// 字段顺序 = serde 顺序；内存偏移为 O 证据（用于 unlock 报告，不是 `repr(C)`）。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct VideoSettings {
    /// @+0x94。
    pub version: VideoSettingsVersion,
    /// @+0x38。
    pub display: DisplayConfig,
    /// @+0x88：`Option<f32>`；0x14003A1E1 `TEST BL,BL` 为 None 时跳过序列化。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub world_resolution_scale: Option<f32>,
    /// @+0xa8（默认 true）。
    pub vsync_enable: bool,
    /// @+0xa9（默认 true）。
    pub dof_enable: bool,
    /// @+0xaa（默认 true）。
    pub photomode_dof_enable: bool,
    /// @+0xab。
    pub upscaling_mode: UpscalingMode,
    /// @+0xc3（默认 3 = Quality）。
    pub upscaling_preset: UpscalingPreset,
    /// @+0x98（默认 60.0）。
    pub fps_limit: f32,
    /// @+0x9c。
    pub sharpen: f32,
    /// @+0xa0。
    pub brightness_shift: f32,
    /// @+0xa4。
    pub gamma_shift: f32,
    /// @+0x00。
    pub last_known_graphics_devices: Vec<LastKnownGraphicsDevice>,
    /// @+0xac。
    pub graphics_device_uuid: Option<Uuid>,
    /// @+0xbd。
    pub graphics_validation: bool,
    /// @+0xbe。
    pub use_aftermath: bool,
    /// @+0xbf。
    pub use_aftermath_checkpoints: bool,
    /// @+0xc0。
    pub graphics_find_crash: bool,
    /// @+0xc1。
    pub color_management: bool,
    /// @+0xc2。
    pub render_upside_down: bool,
    /// @+0x18。
    pub vulkan_layer_allowlist: GraphicsInstanceLayerAllowlist,
    /// @+0x90。
    pub workarounds: GraphicsWorkarounds,
    /// 非 serde 迁移槽（原 +0x80 u32 / +0x84 f32 / +0x88 u32 位 0 / +0x8c f32，
    /// 由 `upgrade` 使用；原默认写 +0x80=0、+0x88=0）。
    #[serde(skip)]
    migration: VideoSettingsMigration,
}

#[derive(Debug, Clone, Default)]
struct VideoSettingsMigration {
    legacy_version: u32,
    legacy_scale: f32,
    migrated: u32,
    migrated_scale: f32,
}

impl Default for VideoSettings {
    fn default() -> Self {
        Self {
            version: VideoSettingsVersion(0),
            display: DisplayConfig::default(),
            world_resolution_scale: None,
            vsync_enable: true,
            dof_enable: true,
            photomode_dof_enable: true,
            upscaling_mode: UpscalingMode::Off,
            upscaling_preset: UpscalingPreset::Quality,
            fps_limit: 60.0,
            sharpen: 0.0,
            brightness_shift: 0.0,
            gamma_shift: 0.0,
            last_known_graphics_devices: Vec::new(),
            graphics_device_uuid: None,
            graphics_validation: false,
            use_aftermath: false,
            use_aftermath_checkpoints: false,
            graphics_find_crash: false,
            color_management: false,
            render_upside_down: false,
            vulkan_layer_allowlist: GraphicsInstanceLayerAllowlist::default(),
            workarounds: GraphicsWorkarounds::default(),
            migration: VideoSettingsMigration::default(),
        }
    }
}

impl VideoSettings {
    /// `VideoSettings::upgrade`（O：内联进 `GameConfig::upgrade` 0x14090DF90）。
    ///
    /// 逐句：
    /// ```text
    /// if (*(u8*)(self+0x88) & 1) == 0 {
    ///     let f = *(f32*)(self+0x84);
    ///     let v = *(i32*)(self+0x80);
    ///     *(u32*)(self+0x80) = 0;
    ///     if v != 0 && (f != 1.0 || f.is_nan()) && 0.125 < f {
    ///         *(u32*)(self+0x88) = 1;
    ///         *(f32*)(self+0x8c) = f;
    ///     }
    /// }
    /// ```
    pub fn upgrade(&mut self) {
        if self.migration.migrated & 1 == 0 {
            let scale = self.migration.legacy_scale;
            let version = self.migration.legacy_version;
            self.migration.legacy_version = 0;
            if version != 0 && (scale != 1.0 || scale.is_nan()) && 0.125 < scale {
                self.migration.migrated = 1;
                self.migration.migrated_scale = scale;
            }
        }
    }
}

/// `LastKnownGraphicsDevice`（I：名字来自 `render_config` 的序列化串 0x14292b810 区的 `"LastKnownGraphicsDevice"` /
/// `"uuid"` / `"driver"`；O：0x140968c40 产出 48 B = 两个 `String`，`+0` 是 uuid、`+0x18` 是物理设备的 `Display`）。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LastKnownGraphicsDevice {
    pub uuid: String,
    pub driver: String,
}

impl LastKnownGraphicsDevice {
    /// `impl$7::from(&GraphicsDevice)`（O 0x140968c40，只有这一个调用点的小自定义 `From`，不是 derive）。
    ///
    /// 原版先 `format!("{}", device.physical_device)`（源 `+0x58`，rhapsody `physical_device::impl$3::fmt`）写进第一个
    /// `String`，**再** `device.uuid.to_string()`（源 `+0x78`，`uuid::fmt::impl$1::fmt`）写进第二个；写入失败 → panic
    /// `"a Display implementation returned an error unexpectedly"`（`ToString` 的行为，这里自然继承）。
    /// 结果里 `+0` 放 uuid、`+0x18` 放物理设备串。
    ///
    /// 这里的 `GraphicsDevice`（`rhapsody_contract`）不带这两个字段，所以直接收它们的 `Display` 实现；求值顺序保持原版。
    pub fn from_display(
        physical_device: &impl core::fmt::Display,
        uuid: &impl core::fmt::Display,
    ) -> Self {
        let driver = physical_device.to_string();
        let uuid = uuid.to_string();
        Self { uuid, driver }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolution_only_for_discriminant_one() {
        assert_eq!(FullscreenConfig::Borderless.resolution(), None);
        assert_eq!(
            FullscreenConfig::Exclusive {
                width: 2560,
                height: 1440
            }
            .resolution(),
            Some((2560, 1440))
        );
        // 判别式即布局：u32 @+0。
        assert_eq!(core::mem::size_of::<FullscreenConfig>(), 12);
    }

    #[test]
    fn scale_table_matches_binary_bytes() {
        // 0x142ab572c 起的 5 个 f32：ab aa aa 3e | 00 00 00 3f | … 与二进制逐字节一致。
        let s: Vec<f32> = UpscalingPreset::ALL
            .iter()
            .map(|p| p.tgsr_resolution_scale())
            .collect();
        assert_eq!(s[0].to_bits(), 0x3eaaaaab);
        assert_eq!(s[1], 0.5);
        assert!((s[2] - 25.0 / 43.0).abs() < 1e-6);
        assert!((s[3] - 2.0 / 3.0).abs() < 1e-6);
        assert!((s[4] - 10.0 / 13.0).abs() < 1e-6);
        assert!(s.windows(2).all(|w| w[0] < w[1]));
    }

    #[test]
    fn previous_walks_down_and_stops_at_zero() {
        assert_eq!(
            UpscalingPreset::UltraQuality.previous(),
            Some(UpscalingPreset::Quality)
        );
        assert_eq!(
            UpscalingPreset::Performance.previous(),
            Some(UpscalingPreset::UltraPerformance)
        );
        assert_eq!(UpscalingPreset::UltraPerformance.previous(), None);
    }

    #[test]
    fn last_known_graphics_device_formats_physical_device_first_then_uuid() {
        use core::cell::RefCell;
        struct Tracer<'a>(&'a RefCell<Vec<&'static str>>, &'static str, &'static str);
        impl core::fmt::Display for Tracer<'_> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                self.0.borrow_mut().push(self.1);
                f.write_str(self.2)
            }
        }
        let order = RefCell::new(Vec::new());
        let d = LastKnownGraphicsDevice::from_display(
            &Tracer(&order, "physical_device", "Intel(R) UHD Graphics 630"),
            &Tracer(&order, "uuid", "00112233-4455-6677-8899-aabbccddeeff"),
        );
        assert_eq!(*order.borrow(), ["physical_device", "uuid"]);
        assert_eq!(d.driver, "Intel(R) UHD Graphics 630");
        assert_eq!(d.uuid, "00112233-4455-6677-8899-aabbccddeeff");
    }

    #[test]
    fn display_keys() {
        assert_eq!(ImageQualityPreset::Custom.lang_key(), "Quality.Custom");
        assert_eq!(UpscalingMode::Off.display(), Err("Tweaker.No"));
        assert_eq!(UpscalingMode::Tgsr.display(), Ok("TGSR"));
    }

    #[test]
    fn video_mode_layout_and_serde_names_match_b() {
        // 0x14003AAA0：resolution @+0、refresh @+8、bit_depth @+0xc，共 16 B。
        assert_eq!(core::mem::size_of::<VideoMode>(), 16);
        let vm = VideoMode {
            resolution: [1920, 1080],
            bit_depth: 32,
            refresh_rate_millihertz: 60_000,
        };
        let text = ron::ser::to_string(&vm).expect("serialize");
        let r = text.find("resolution").unwrap();
        let b = text.find("bit_depth").unwrap();
        let h = text.find("refresh_rate_millihertz").unwrap();
        assert!(r < b && b < h, "{text}");
    }

    #[test]
    fn monitor_match_layout_and_optional_name() {
        // 0x140039C10：name Option<String> @+0、position @+0x18、size @+0x20 → 40 B。
        assert_eq!(core::mem::size_of::<MonitorMatch>(), 40);
        let m = MonitorMatch {
            name: None,
            physical_position: [0, 0],
            physical_size: [1920, 1080],
        };
        let text = ron::ser::to_string(&m).expect("serialize");
        assert!(text.contains("name:") && text.contains("None"), "{text}");
        assert!(text.contains("physical_position"));
        assert!(text.contains("physical_size"));
    }

    #[test]
    fn display_config_defaults_match_0x140968e50() {
        let d = DisplayConfig::default();
        assert!(d.fullscreen && d.allow_exclusive_fullscreen);
        assert_eq!(d.windowed_resolution, [1920, 1080]);
        assert_eq!(d.fullscreen_config, FullscreenConfig::Borderless);
        assert!(d.monitor.is_none());
    }

    #[test]
    fn video_settings_defaults_match_0x140968e80() {
        let v = VideoSettings::default();
        assert_eq!(v.version, VideoSettingsVersion(0));
        assert!(v.vsync_enable && v.dof_enable && v.photomode_dof_enable);
        assert_eq!(v.upscaling_mode, UpscalingMode::Off);
        assert_eq!(v.upscaling_preset, UpscalingPreset::Quality);
        assert_eq!(v.fps_limit, 60.0);
        assert_eq!(v.world_resolution_scale, None);
        assert!(v.last_known_graphics_devices.is_empty());
        assert_eq!(v.graphics_device_uuid, None);
        assert!(!v.use_aftermath && !v.graphics_find_crash);
    }
}
