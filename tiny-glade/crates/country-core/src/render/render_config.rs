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

use serde::{Deserialize, Serialize};

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

/// `MonitorMatch`（Debug 字段名 O；类型 I：`physical_position`/`physical_size` 各 8 B）。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MonitorMatch {
    pub name: String,
    pub physical_position: [i32; 2],
    pub physical_size: [u32; 2],
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
    fn display_keys() {
        assert_eq!(ImageQualityPreset::Custom.lang_key(), "Quality.Custom");
        assert_eq!(UpscalingMode::Off.display(), Err("Tweaker.No"));
        assert_eq!(UpscalingMode::Tgsr.display(), Ok("TGSR"));
    }
}
