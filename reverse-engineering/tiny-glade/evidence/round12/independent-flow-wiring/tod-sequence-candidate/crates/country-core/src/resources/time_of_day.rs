//! `time_of_day.rs` — 一天中的小时重映射 + 昼夜序列/光照快照资源
//! (B `country_core::resources::time_of_day`, 0x140a74790 等).
//!
//! 证据（2026-10-17 fresh，只读 `--worker shared`；仅以反编译/机器码为准）：
//! * `remap_hour_to_fmod_hour` 0x140a74790（判等，未改动）。
//! * `TodSequence::snapshot_at_tod` 0x140b067d0（824 B）：按 `time_of_day` 关键帧
//!   在 `Vec<TodSequenceEntry>` 上查找相邻对，`inverse_lerp` 后逐字段插值；
//!   `SunSkyColors::lerp` 处理 12 个颜色分量。数据源为 `assets/tod/default.json`
//!   （启动 `startup_time_of_day::load_default_tod` 读入，属库外启动模块）。
//! * 数据字符串表（binary）：`entries`、`time_of_day`、`snapshot`、`temp_ui_id`、
//!   `sun_sky`、`base_fog_thickness`、`fog_tint`、`speed_bias`、
//!   `sun_xz_rotation`、`latitude_degrees`；`TodSequenceEntry with 3 elements`、
//!   `TodLightingSnapshot with 4 elements`。
//! * 内存布局（0x140b067d0 偏移反推，stride 0x50）：
//!   entry: +0x00 `temp_ui_id` u64、+0x08 `time_of_day` f32、
//!          +0x0c `snapshot`（0x44 B）。
//!   snapshot: +0x00 sun_sky 12×f32、+0x30 base_fog_thickness f32、
//!             +0x34 fog_tint 3×f32、+0x40 speed_bias f32。
//! * `TIME_OF_DAY_SPEED` 全局 @0x14362c0b8 = 0x3ada740d = 1/600。
//!
//! 原 `TimeOfDay` 的序列化字段只有 3 个 f32：`time_of_day` / `sun_xz_rotation` /
//! `latitude_degrees`（0x140b389b0 `to_json_string<TimeOfDay>` 三次
//! `serialize_entry<..., f32>`）。其余字段是运行时/默认值字段。
//!
//! **登记（非本模块实现）**：`assets/tod/default.json` 的解析/装载在
//! `startup::startup_time_of_day::load_default_tod`（库外启动模块）；本模块只承载
//! 已装载的 `TodSequence` 与 `snapshot_at_tod` 纯算法。
use bevy_ecs::resource::Resource;
use glam::Vec3;
use serde::{Deserialize, Serialize};

use crate::render::atmosphere::SunSkyColors;

const X_SUMMER: [f32; 6] = [0.0, 3.5, 9.0, 17.0, 20.0, 24.0];
const Y_SUMMER: [f32; 6] = [0.0, 3.0, 11.0, 17.0, 22.0, 24.0];
const X_WINTER: [f32; 8] = [0.0, 3.5, 4.52, 6.2, 9.0, 17.0, 20.0, 24.0];
const Y_WINTER: [f32; 8] = [0.0, 3.0, 5.0, 9.0, 11.0, 17.0, 22.0, 24.0];

fn lerp(x: f32, x0: f32, x1: f32, y0: f32, y1: f32) -> f32 {
    ((x - x0) / (x1 - x0)) * (y1 - y0) + y0
}

fn remap(hour: f32, xs: &[f32], ys: &[f32]) -> f32 {
    let mut out = hour;
    for (x, y) in xs.windows(2).zip(ys.windows(2)) {
        if hour >= x[0] && hour < x[1] {
            out = lerp(hour, x[0], x[1], y[0], y[1]);
        }
    }
    out
}

/// 把游戏小时（0..24）重映射到日照曲线用的小时。
pub fn remap_hour_to_fmod_hour(hour: f32, winter: bool) -> f32 {
    if winter {
        remap(hour, &X_WINTER, &Y_WINTER)
    } else {
        remap(hour, &X_SUMMER, &Y_SUMMER)
    }
}

/// `TIME_OF_DAY_SPEED` 全局（O：`resources::time_of_day::TIME_OF_DAY_SPEED`，
/// 地址 0x14362c0b8，位模式 0x3ada740d = 1/600）。
pub const TIME_OF_DAY_SPEED: f32 = 0.001_666_666_6;

/// `utils::inverse_lerp(a, b, t) = (t - a) / (b - a)`（O 0x140c904a0，5 条指令，
/// 无 clamp）。`utils` crate 现有源码未恢复该函数；按机器码等价落地。
#[inline]
pub fn inverse_lerp(a: f32, b: f32, t: f32) -> f32 {
    (t - a) / (b - a)
}

/// `utils::time_continuous_lerp_mut(value, target, dt, speed)`（O 0x140c90670）：
/// `value = (target - value) * (1 - exp(-dt * speed)) + value`。
#[inline]
pub fn time_continuous_lerp_mut(value: &mut f32, target: f32, dt: f32, speed: f32) -> f32 {
    let f = (-dt * speed).exp();
    *value = (target - *value) * (1.0 - f) + *value;
    *value
}

/// 播放状态（O：TimeOfDay +0x00，`0=Idle / 1=Playing / 2=Rewinding`）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u32)]
pub enum TodPlaybackState {
    #[default]
    Idle = 0,
    Playing = 1,
    Rewinding = 2,
}

/// `TimeOfDay` 资源（O 布局，逐字段来自 0x140a73460 的偏移读写）。
///
/// 序列化面（save，O）：仅 `time_of_day` / `sun_xz_rotation` / `latitude_degrees`
/// 三个 f32（0x140b389b0）。字段名 O；默认值为 schema 重建（无原生 default body
/// 证据）：`time_of_day=0.0`、`sun_xz_rotation=0.0`、`latitude_degrees=0.0`、
/// `sunrise=0.25`、`sunset=0.75`、`day_of_year=1`。
#[derive(Resource, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TimeOfDay {
    /// +0x00：播放状态。
    #[serde(skip)]
    pub state: TodPlaybackState,
    /// +0x04：播放进度（0..1）。
    #[serde(skip)]
    pub progress: f32,
    /// +0x08：一天内相位 `[0,1)`（`hour = fmod(t*24 + 8, 24)`）。
    pub time_of_day: f32,
    /// +0x0c：太阳绕 XZ 旋转角（弧度）。
    pub sun_xz_rotation: f32,
    /// +0x10：suncalc 纬度（度，f32）。
    pub latitude_degrees: f32,
    /// +0x14：昼段下界（`time_of_day` 相位，`sunrise <= t < sunset`）。
    #[serde(skip)]
    pub sunrise: f32,
    /// +0x18：昼段上界。
    #[serde(skip)]
    pub sunset: f32,
    /// +0x1c：上一帧连续昼夜值（只读消费；本函数不写，见 REBUILD 登记）。
    #[serde(skip)]
    pub day_night_continuous_prev: f32,
    /// +0x20：连续昼夜值 `[0,1]`（`time_continuous_lerp_mut` 目标 0/1）。
    #[serde(skip)]
    pub day_night_continuous: f32,
    /// +0x24：`inverse_lerp(sunrise, sunset, t)` 结果（不在昼段时为 0）。
    #[serde(skip)]
    pub day_night_factor: f32,
    /// +0x28：一年中的第几天（u16；`1..=365` 越界 unwrap panic）。
    #[serde(skip)]
    pub day_of_year: u16,
}

impl Default for TimeOfDay {
    fn default() -> Self {
        // Original insert_resources @140940ed0..f00. Only runtime caches are
        // initialized locally where native writes them before first read.
        Self {
            state: TodPlaybackState::Idle,
            progress: 0.0,
            time_of_day: f32::from_bits(0x3e99999a),
            sun_xz_rotation: 0.0,
            latitude_degrees: f32::from_bits(0x426d5134),
            sunrise: f32::from_bits(0x3f0147ae),
            sunset: f32::from_bits(0x3f50a3d7),
            day_night_continuous_prev: f32::from_bits(0x3e19999a),
            day_night_continuous: 0.0,
            day_night_factor: 0.0,
            day_of_year: 207,
        }
    }
}

/// 兼容别名（本域旧接口名）。
pub type TimeOfDayState = TimeOfDay;

/// `TimeOfDayScreen` 的时相/过渡槽（O：`resources::gui::TimeOfDayScreen`
/// 在 0x140a73460 中被读写的字段）。
///
/// **跨域登记**：原类型在 `resources/gui.rs`（当前 crate 只建模 `visible`）。
/// 本资源仅承载系统使用的字段，偏移 O：
/// `+0x00 time_of_day: f32`、`+0x04 sun_xz_rotation: f32`、
/// `+0x0a hour_transitioning: u8`、`+0x0b instant: u8`、
/// `+0x0c angle_transitioning: u8`。
/// `visible` 字段不在本系统使用；合并到 gui::TimeOfDayScreen 的解锁条件：
/// 该类型补齐上述字段后把本资源替换为 `ResMut<gui::TimeOfDayScreen>`。
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct TodScreen {
    pub time_of_day: f32,
    pub sun_xz_rotation: f32,
    pub hour_transitioning: bool,
    pub instant: bool,
    pub angle_transitioning: bool,
}

/// dev 配置槽（O：`ResMut<GameConfig>` 中被本系统读写的两个字段）。
///
/// **跨域登记**：原字段在 `game_config::GameConfig`：
/// `+0xe3 dev_hotkey: u8`（本系统 `^= 1`）、`+0x118 dev_speed_mult: f32`（读）。
/// 当前 `GameConfig` 未建模这两个字段；解锁条件：GameConfig 补齐后改回
/// `ResMut<GameConfig>` 并删除本 seam。
#[derive(Resource, Clone, Copy, Debug, PartialEq)]
pub struct TodDevConfig {
    /// `+0xe3`：dev 时间方向开关（热键翻转；0 = 恒倒带，1 = 恒前进）。
    pub dev_hotkey: bool,
    /// `+0x118`：dev 速度倍率。
    pub dev_speed_mult: f32,
}

impl Default for TodDevConfig {
    fn default() -> Self {
        Self {
            dev_hotkey: false,
            dev_speed_mult: 1.0,
        }
    }
}

/// `ButtonInput<KeyCode>::just_pressed` 的 seam（O：`Res<ButtonInput<KeyCode>>`）。
///
/// 机器码：`ButtonInput +0x20` 的 `HashSet<KeyCode>` 查询判别值 `0xa9`
/// （Bevy 0.16 KeyCode 顺序下为 `KeyCode::F10`；variant 顺序与 0.19 一致：
/// 160=F1 … 169=F10）。`DEVELOPER_HOTKEYS_ENABLED` 全局另由
/// `game_config::developer_hotkeys_enabled()` 承载。
/// 解锁条件：仅需宿主每帧写入 F10 的 just_pressed 状态；接入 bevy_input 后
/// 改为真实 `Res<ButtonInput<KeyCode>>`。
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TodInputSeam {
    /// `ButtonInput::just_pressed(KeyCode::F10)`。
    pub f10_just_pressed: bool,
}

/// `Weather` 的 seam（O：`Res<Weather>` 只读取 +0x00 的 f32）。
///
/// 该值在 DerivedAtmosphere 写盘时加到其中一个浮点槽（见 REBUILD 偏移表）。
/// 解锁条件：`resources/weather.rs` 类型落地后替换为 `Res<Weather>`。
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct TodWeatherSeam {
    /// `Weather +0x00` f32。
    pub value: f32,
}

/// 帧间全局 `f32` 槽（O：静态 0x143634d84）。
///
/// 每帧由 `TodSequence::snapshot_at_tod` 输出的 `speed_bias` 写入；下一帧
/// `advance_tod_hour` 用 `exp2f(该值)` 作为速度曲线。初始 0.0。
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct TodSpeedBiasGlobal {
    pub speed_bias: f32,
}

/// `TodLightingSnapshot`（O，`struct TodLightingSnapshot with 4 elements`）。
///
/// 布局（0x140b067d0）：sun_sky 12×f32 @+0x00、base_fog_thickness f32 @+0x30、
/// fog_tint 3×f32 @+0x34、speed_bias f32 @+0x40。
///
/// **跨域登记**：原 `sun_sky` 的 serde 字段为 4 个 Vec3：
/// `sun_tint_color` / `moon_tint_color` / `sky_low_color` / `sky_high_color`
/// （`assets/tod/default.json`）。当前 `render::atmosphere::SunSkyColors`
/// 是 3×Vec4 的 12-f32 容器；`lerp` 逐分量一致，故本域复用其 12-f32 存储。
/// 解锁条件：render 域把 `SunSkyColors` 改为 4×Vec3 命名字段后，本类型自动对齐。
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct TodLightingSnapshot {
    /// +0x00：12×f32 颜色（SunSkyColors::lerp）。
    pub sun_sky: SunSkyColors,
    /// +0x30：`prev + (next - prev) * t`。
    pub base_fog_thickness: f32,
    /// +0x34：3×f32，`next * t + prev * (1 - t)`。
    pub fog_tint: Vec3,
    /// +0x40：`(next - prev) * t + prev`（也作为下一帧 exp2 输入）。
    pub speed_bias: f32,
}

impl Default for TodLightingSnapshot {
    fn default() -> Self {
        Self {
            sun_sky: SunSkyColors {
                a: glam::Vec4::ZERO,
                b: glam::Vec4::ZERO,
                c: glam::Vec4::ZERO,
            },
            base_fog_thickness: 0.0,
            fog_tint: Vec3::ZERO,
            speed_bias: 0.0,
        }
    }
}

impl TodLightingSnapshot {
    /// `TodLightingSnapshot::lerp`（O：0x140b067d0 内联两处，source line 145）。
    pub fn lerp(&self, other: &Self, t: f32) -> Self {
        let w = 1.0 - t;
        Self {
            sun_sky: self.sun_sky.lerp(&other.sun_sky, t),
            base_fog_thickness: (other.base_fog_thickness - self.base_fog_thickness) * t
                + self.base_fog_thickness,
            fog_tint: other.fog_tint * t + self.fog_tint * w,
            speed_bias: (other.speed_bias - self.speed_bias) * t + self.speed_bias,
        }
    }
}

/// `TodSequenceEntry`（O，`with 3 elements`，stride 0x50）。
///
/// 内存：`temp_ui_id` u64 @0x00、`time_of_day` f32 @0x08、`snapshot` @0x0c。
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct TodSequenceEntry {
    /// serde `temp_ui_id`（O）。
    pub temp_ui_id: u64,
    /// serde `time_of_day`：关键帧（0..1）。
    pub time_of_day: f32,
    /// serde `snapshot`。
    pub snapshot: TodLightingSnapshot,
}

/// `TodSequence`（O：`struct TodSequence with 1 element`，`entries: Vec<...>`）。
#[derive(Resource, Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TodSequence {
    pub entries: Vec<TodSequenceEntry>,
}

impl Default for TodSequenceEntry {
    fn default() -> Self {
        Self {
            temp_ui_id: fastrand::usize(0..=usize::MAX) as u64,
            time_of_day: 0.0,
            snapshot: TodLightingSnapshot::default(),
        }
    }
}

impl TodSequence {
    /// B140b067d0/824B: adjacent keys are periodic across midnight.
    /// Before first, interpolate last.time-1 -> first; after the final pair,
    /// interpolate last -> first.time+1. A NaN weight copies the left snapshot
    /// without arithmetic; infinities and finite weights clamp to [0,1].
    pub fn snapshot_at_tod(&self, tod: f32) -> TodLightingSnapshot {
        let entries = &self.entries;
        if entries.len() == 1 {
            return entries[0].snapshot;
        }
        let t = tod.rem_euclid(1.0);
        // Native performs rem_euclid before the empty-vector bounds panic.
        let first = &entries[0];
        let last = entries.last().unwrap();
        let (a, b, lower, upper) = if t < first.time_of_day {
            (last, first, last.time_of_day - 1.0, first.time_of_day)
        } else if let Some(pair) = entries.windows(2).find(|pair| {
            pair[0].time_of_day <= t && t <= pair[1].time_of_day
        }) {
            (&pair[0], &pair[1], pair[0].time_of_day, pair[1].time_of_day)
        } else {
            (last, first, last.time_of_day, first.time_of_day + 1.0)
        };
        let raw = inverse_lerp(lower, upper, t);
        // Native bit-classifies the raw weight. Only NaN bypasses the lerp:
        // finite negatives and -inf reach weight0, +inf reaches weight1.
        if raw.is_nan() {
            return a.snapshot;
        }
        a.snapshot.lerp(&b.snapshot, raw.clamp(0.0, 1.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn native_tod_startup_constants_and_saved_surface() {
        let value = TimeOfDay::default();
        assert_eq!(value.time_of_day.to_bits(), 0x3e99999a);
        assert_eq!(value.latitude_degrees.to_bits(), 0x426d5134);
        assert_eq!(value.sunrise.to_bits(), 0x3f0147ae);
        assert_eq!(value.sunset.to_bits(), 0x3f50a3d7);
        assert_eq!(value.day_of_year, 207);
        let serialized = serde_json::to_value(&value).unwrap();
        assert_eq!(serialized.as_object().unwrap().len(), 3);
        assert!(serialized.get("day_of_year").is_none());
    }

    #[test]
    fn original_sequence_json_uses_four_vec3_and_requires_entry_id() {
        let json = r#"{"entries":[{"time_of_day":0.0,"temp_ui_id":11309646118622107426,"snapshot":{"sun_sky":{"sun_tint_color":[1,2,3],"moon_tint_color":[4,5,6],"sky_low_color":[7,8,9],"sky_high_color":[10,11,12]},"base_fog_thickness":0.046,"fog_tint":[0.5,1,2],"speed_bias":-1}}]}"#;
        let sequence: TodSequence = serde_json::from_str(json).unwrap();
        assert_eq!(sequence.entries[0].temp_ui_id, 11309646118622107426);
        let snapshot = sequence.snapshot_at_tod(0.7);
        assert_eq!(snapshot.sun_sky.a, glam::Vec4::new(1.0, 2.0, 3.0, 4.0));
        assert_eq!(snapshot.sun_sky.b, glam::Vec4::new(5.0, 6.0, 7.0, 8.0));
        assert_eq!(snapshot.sun_sky.c, glam::Vec4::new(9.0, 10.0, 11.0, 12.0));
        let encoded = serde_json::to_string(&sequence).unwrap();
        assert_eq!(
            serde_json::from_str::<TodSequence>(&encoded).unwrap(),
            sequence
        );
        assert!(serde_json::from_str::<TodSequence>(
            r#"{"entries":[{"time_of_day":0,"snapshot":{}}]}"#
        )
        .is_err());
    }

    #[test]
    fn keyframes_map_to_their_y_values() {
        for (x, y) in X_SUMMER.iter().zip(Y_SUMMER) {
            if *x < 24.0 {
                assert_eq!(remap_hour_to_fmod_hour(*x, false), y);
            }
        }
        for (x, y) in X_WINTER.iter().zip(Y_WINTER) {
            if *x < 24.0 {
                assert_eq!(remap_hour_to_fmod_hour(*x, true), y);
            }
        }
    }

    #[test]
    fn midpoints_interpolate_linearly() {
        assert_eq!(remap_hour_to_fmod_hour(6.25, false), 7.0);
        assert_eq!(remap_hour_to_fmod_hour(1.75, false), 1.5);
        assert!((remap_hour_to_fmod_hour(5.36, true) - 7.0).abs() < 1e-5);
    }

    #[test]
    fn out_of_range_hours_pass_through() {
        assert_eq!(remap_hour_to_fmod_hour(24.0, false), 24.0);
        assert_eq!(remap_hour_to_fmod_hour(-1.0, true), -1.0);
    }

    #[test]
    fn winter_differs_from_summer_only_inside_the_inserted_window() {
        assert_eq!(
            remap_hour_to_fmod_hour(2.0, true),
            remap_hour_to_fmod_hour(2.0, false)
        );
        assert_ne!(
            remap_hour_to_fmod_hour(5.0, true),
            remap_hour_to_fmod_hour(5.0, false)
        );
        assert_eq!(
            remap_hour_to_fmod_hour(12.0, true),
            remap_hour_to_fmod_hour(12.0, false)
        );
    }

    fn snap(v: f32) -> TodLightingSnapshot {
        TodLightingSnapshot {
            sun_sky: SunSkyColors {
                a: glam::Vec4::ZERO,
                b: glam::Vec4::ZERO,
                c: glam::Vec4::ZERO,
            },
            base_fog_thickness: v,
            fog_tint: Vec3::splat(v),
            speed_bias: v,
        }
    }

    #[test]
    fn snapshot_single_entry_is_copied() {
        let seq = TodSequence {
            entries: vec![TodSequenceEntry {
                temp_ui_id: 7,
                time_of_day: 0.0,
                snapshot: snap(0.5),
            }],
        };
        assert_eq!(seq.snapshot_at_tod(0.9), snap(0.5));
    }

    #[test]
    fn snapshot_interpolates_between_keyframes() {
        let seq = TodSequence {
            entries: vec![
                TodSequenceEntry {
                    temp_ui_id: 1,
                    time_of_day: 0.0,
                    snapshot: snap(0.0),
                },
                TodSequenceEntry {
                    temp_ui_id: 2,
                    time_of_day: 0.5,
                    snapshot: snap(1.0),
                },
                TodSequenceEntry {
                    temp_ui_id: 3,
                    time_of_day: 1.0,
                    snapshot: snap(2.0),
                },
            ],
        };
        let s = seq.snapshot_at_tod(0.25);
        assert!((s.base_fog_thickness - 0.5).abs() < 1e-6);
        assert!((s.speed_bias - 0.5).abs() < 1e-6);
        // t 在上界恰好命中第二段。
        let s2 = seq.snapshot_at_tod(0.5);
        assert!((s2.base_fog_thickness - 1.0).abs() < 1e-6);
        // t < 首键 → 首 entry。
        let s3 = seq.snapshot_at_tod(0.0);
        assert!((s3.base_fog_thickness - 0.0).abs() < 1e-6);
    }

    #[test]
    fn snapshot_remaps_time_into_unit_range() {
        let seq = TodSequence {
            entries: vec![
                TodSequenceEntry {
                    temp_ui_id: 1,
                    time_of_day: 0.0,
                    snapshot: snap(0.0),
                },
                TodSequenceEntry {
                    temp_ui_id: 2,
                    time_of_day: 1.0,
                    snapshot: snap(1.0),
                },
            ],
        };
        // 1.25 → 0.25。
        let s = seq.snapshot_at_tod(1.25);
        assert!((s.base_fog_thickness - 0.25).abs() < 1e-5);
    }

    #[test]
    fn inverse_lerp_matches_the_native_formula() {
        assert_eq!(inverse_lerp(0.0, 2.0, 0.5), 0.25);
        assert_eq!(inverse_lerp(1.0, 3.0, 2.0), 0.5);
    }

    #[test]
    fn time_continuous_lerp_matches_exp_formula() {
        let mut v = 0.0;
        let out = time_continuous_lerp_mut(&mut v, 1.0, 1.0, 1.0);
        assert!((out - (1.0 - (-1.0_f32).exp())).abs() < 1e-6);
        assert_eq!(out, v);
    }
}
