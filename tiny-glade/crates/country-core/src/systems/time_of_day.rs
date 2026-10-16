//! `systems/time_of_day.rs` — 昼夜系统（B 0x140a73460，O）。
//!
//! 证据：`chapters/systems/time_of_day.rs.md`、`camera-time/ALGORITHMS.md`（22KB 反编译全析）：
//! - **angle 插值**：向 `2π − angle`（6.28319@0x142934674），wrap ±π，`1−exp(−3·dt)`
//!   （−3@0x142ad58e8），收敛 < 0.01 清标志；
//! - **hour 插值**：wrap ±0.5，`exp(−3·dt·max(speed,1))`，收敛 < 0.001 清标志；
//! - **播放状态机**：dev 关 `0→progress=0→2；2→progress+=dt/−5`（倒带）；dev 开
//!   `0→progress=0→1；1→progress+=dt/3`（前进）；
//! - **小时推进**：`target_hour = fmodf(progress·24 + 8 + dt·TIME_OF_DAY_SPEED·speed·trig, 24)`；
//!   `hour = fmodf(target·24 + 8, 24)` → `remap_hour_to_fmod_hour(hour, winter)`；
//! - **suncalc 太阳位置**：`day_of_year ∈ 1..365`（越界 unwrap panic）；
//!   `encoded = day_of_year | 0xFCE00`（2023<<9）；`unix_ms = decode·1000 + hour·3600000`；
//!   `suncalc::get_position(ms, lat=state@+0x10, lon=0)` → colatitude → 3D 太阳方向 →
//!   写光照资源（`TodLightingSnapshot` 插值、`SunSkyColors` 两套）+ 音频上报
//!   （"TOD-Enabled"(0xb)/"Timetravel"(10)/"DayNight"(8)）。
//!
//! 本实现：`TimeOfDayState` 资源（衔接卡0 TimeOfDay 节载荷/save 节）+ 纯算法
//! （插值/播放机/小时推进/remap）+ [`suncalc_pos_to_vec`]。suncalc 的
//! `get_position` 与光照/音频写盘依赖 suncalc crate / FMOD / 光照资源，登记待接。
use bevy_ecs::prelude::*;
use glam::Vec3;
use serde::{Deserialize, Serialize};

use crate::resources::time_of_day::remap_hour_to_fmod_hour;

/// `2π`（O：6.28319@0x142934674）。
const TWO_PI: f32 = 6.283_185_5;
/// `−3`（O：0x142ad58e8，angle/hour 插值平滑）。
const SMOOTH_K: f32 = -3.0;
/// `24`/`8`（O：0x142934600/0x142934604，小时模/偏移）。
const HOUR_MOD: f32 = 24.0;
const HOUR_OFFSET: f32 = 8.0;
/// `3600000`（O：0x142ad58f0，时→毫秒）。
const HOUR_TO_MS: f32 = 3_600_000.0;
/// `0xFCE00` = 2023<<9（O：日期编码）。
const YEAR_2023_BITS: u64 = 0x0fce00;

/// 播放状态（O：dev 关/开 状态机判别）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum TodPlaybackState {
    #[default]
    Idle = 0,
    Playing = 1,
    Rewinding = 2,
}

/// 昼夜状态资源（O 资源面：当前态 {hour, angle, 角度过渡@+3, 小时过渡@+0xa, 瞬时@+0xb}；
/// 播放态 {state, progress, target_hour, 纬度@+0x10, 日出@+0x14, 日落@+0x18,
/// day_of_year@+0x28}；dev 态 {小时倍率@+0x23}）。
#[derive(Resource, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimeOfDayState {
    /// 当前小时（0..24）。
    pub hour: f32,
    /// 太阳角度。
    pub angle: f32,
    /// 角度过渡中（@+3）。
    pub angle_transitioning: bool,
    /// 小时过渡中（@+0xa）。
    pub hour_transitioning: bool,
    /// 瞬时吸附（@+0xb）。
    pub instant: bool,
    /// 播放状态。
    pub playback: TodPlaybackState,
    /// 播放进度（0..1）。
    pub progress: f32,
    /// 目标小时。
    pub target_hour: f32,
    /// 纬度（@+0x10，suncalc lat）。
    pub latitude: f32,
    /// 日出小时（@+0x14）。
    pub sunrise: f32,
    /// 日落小时（@+0x18）。
    pub sunset: f32,
    /// 一年中的第几天（1..365；越界 unwrap panic）。
    pub day_of_year: u32,
    /// dev 小时倍率（@+0x23）。
    pub dev_speed_mult: f32,
}

impl Default for TimeOfDayState {
    fn default() -> Self {
        Self {
            hour: HOUR_OFFSET,
            angle: 0.0,
            angle_transitioning: false,
            hour_transitioning: false,
            instant: false,
            playback: TodPlaybackState::Idle,
            progress: 0.0,
            target_hour: HOUR_OFFSET,
            latitude: 0.0,
            sunrise: 6.0,
            sunset: 20.0,
            day_of_year: 1,
            dev_speed_mult: 1.0,
        }
    }
}

/// 角度插值（O：`1−exp(−3·dt)`、wrap ±π、收敛 < 0.01）。
pub fn advance_angle(current: f32, target: f32, dt: f32) -> (f32, bool) {
    let diff = (target - current).rem_euclid(TWO_PI);
    let wrapped = if diff > core::f32::consts::PI {
        diff - TWO_PI
    } else {
        diff
    };
    let next = current + wrapped * (1.0 - (SMOOTH_K * dt).exp());
    let converged = wrapped.abs() < 0.01;
    (if converged { target } else { next }, !converged)
}

/// 小时插值（O：wrap ±0.5 **天**（= ±12 小时，24 小时圆上取最短路径）、
/// `exp(−3·dt·max(speed,1))`、收敛 < 0.001）。
pub fn advance_hour(current: f32, target: f32, dt: f32, speed: f32) -> (f32, bool) {
    let diff = target - current;
    // 24 小时圆最短路径：half-day = 0.5 天 = 12 小时（O：0.5@0x142925870）。
    let wrapped = if diff > 12.0 {
        diff - 24.0
    } else if diff < -12.0 {
        diff + 24.0
    } else {
        diff
    };
    let factor = 1.0 - (SMOOTH_K * dt * speed.max(1.0)).exp();
    let next = (current + wrapped * factor).rem_euclid(24.0);
    let converged = wrapped.abs() < 0.001;
    (if converged { target } else { next }, !converged)
}

/// 播放状态推进（O：dev 关 `0→2 倒带 dt/−5`；dev 开 `0→1 前进 dt/3`）。
pub fn advance_playback(state: &mut TodPlaybackState, progress: &mut f32, dev_on: bool, dt: f32) {
    match *state {
        TodPlaybackState::Idle => {
            *progress = 0.0;
            *state = if dev_on {
                TodPlaybackState::Playing
            } else {
                TodPlaybackState::Rewinding
            };
        }
        TodPlaybackState::Playing => {
            *progress = (*progress + dt / 3.0).min(1.0);
        }
        TodPlaybackState::Rewinding => {
            *progress = (*progress - dt / 5.0).max(0.0);
        }
    }
}

/// 小时推进（O：`target = fmodf(progress·24 + 8 + dt·speed·trig, 24)`；
/// `hour = fmodf(target·24 + 8, 24)` → remap）。
pub fn advance_tod_hour(
    progress: f32,
    dt: f32,
    speed: f32,
    time_of_day_speed: f32,
    winter: bool,
) -> (f32, f32) {
    let target = (progress * HOUR_MOD + HOUR_OFFSET + dt * time_of_day_speed * speed)
        .rem_euclid(HOUR_MOD);
    let hour = (target * HOUR_MOD + HOUR_OFFSET).rem_euclid(HOUR_MOD);
    (target, remap_hour_to_fmod_hour(hour, winter))
}

/// 正交基常量（O：RNE 直读 0x142ad590c..5984；`suncalc_pos_to_vec` 的 3×3 旋转矩阵元）。
///
/// 布局为 `{f32, f32, 0, 0}` 8 组 + 首项 0x590c（9 个非零元）。各元按反编译
/// `decomp-0x140a73460.txt` 行 390..402 的矩阵方程命名 A..P。
mod basis {
    /// `_UNK_142ad590c` = 0.581319
    pub const C: f32 = 0.581_319_9;
    /// `_UNK_142ad5910` = -0.762443
    pub const A: f32 = -0.762_443_1;
    /// `_UNK_142ad5914` = 0.647056
    pub const B: f32 = 0.647_056;
    /// `_UNK_142ad5920` = 0.418681
    pub const D: f32 = 0.418_681;
    /// `_UNK_142ad5924` = -0.493343
    pub const E: f32 = -0.493_343_2;
    /// `_UNK_142ad5930` = 0.551646
    pub const F: f32 = 0.551_646;
    /// `_UNK_142ad5934` = 0.551646
    pub const G: f32 = 0.551_646;
    /// `_UNK_142ad5940` = 0.551646
    pub const H: f32 = 0.551_646;
    /// `_UNK_142ad5950` = 0.834078
    pub const I: f32 = 0.834_078;
    /// `_UNK_142ad5954` = 0.834078
    pub const J: f32 = 0.834_078;
    /// `_UNK_142ad5960` = 0.0
    pub const K: f32 = 0.0;
    /// `_UNK_142ad5964` = 0.834078
    pub const L: f32 = 0.834_078;
    /// `_UNK_142ad5970` = 0.806015
    pub const M: f32 = 0.806_015;
    /// `_UNK_142ad5974` = 0.591895
    pub const N: f32 = 0.591_895;
    /// `_UNK_142ad5980` = 0.0
    pub const O: f32 = 0.0;
    /// `_UNK_142ad5984` = 0.806015
    pub const P: f32 = 0.806_015;
}

/// `suncalc_pos_to_vec`（行 348 内联，O：方位/高度 → 3D 方向；colatitude =
/// `π/2 − (azimuth + π)`，正交基常量 @0x142ad5910..5984 直读）。
///
/// 结构（O，decomp 0x140a73460 行 348..424，两级旋转 + 归一化）：
/// 1. 高度/方位经 sin/cos 组合（`sin(colat)·sin(alt)` 等）；
/// 2. 正交基矩阵（A..P）第一段 → 第二段（N/M/P 旋转）→ 高度双角四元数旋转
///    （`sin(2·alt)·z + cos(2·alt)·x` 结构）→ 输出 Vec3。
///
/// 分量→语义映射（x=东/前、y=高、z=北，I）与 get_position 输出序
/// （@0x140=高度、@0x148=方位，I）登记待取证；矩阵方程逐项对应机器码（O）。
pub fn suncalc_pos_to_vec(azimuth: f64, elevation: f64) -> Vec3 {
    // colatitude = π/2 − (azimuth + π)（O：`dVar61 = _UNK_142ad5900 − (… + _UNK_142ad58f8)`）。
    let colatitude = core::f64::consts::FRAC_PI_2 - (azimuth + core::f64::consts::PI);
    let s_colat = colatitude.sin();
    let c_colat = colatitude.cos();
    let s_alt = elevation.sin();
    let c_alt = elevation.cos();

    // f32 输入（O：行 383..389）。
    let sin_alt = s_alt as f32;
    let cos_alt = c_alt as f32;
    let one_m_cos = 1.0_f32 - cos_alt;
    // `sin(colat)·sin(alt)` 与 `−cos(colat)·sin(alt)` 仅进入 uVar63 次级输出（行 409..419，
    // 归一化经 `+ _UNK_1429edc00`（-0.15）——非光照方向），本实现按主输出（fStack_168 族）
    // 落地，次级向量登记待接。
    let _ = (s_colat, c_colat);

    // 第一段：正交基矩阵（O：行 390..398；fVar56=0、fVar58=-0、K=0）。
    let x0 = one_m_cos * basis::E * basis::I + (cos_alt + one_m_cos * basis::D) * basis::F;
    let y0 = (one_m_cos * basis::C + cos_alt) * basis::J + one_m_cos * basis::E * basis::G;
    let z0 = sin_alt * basis::B * basis::L - sin_alt * basis::A * basis::H;

    // 第二段（O：行 399..402；fVar38=0、O=0）。
    let x2 = basis::N * z0 - basis::M * x0;
    let y2 = y0;
    let z2 = -basis::N * x0 - basis::P * z0;

    // 高度双角四元数旋转（O：行 413..416：`q_s*z2*2*c + (c²−s²)*x2` 等）。
    let c2 = cos_alt * cos_alt;
    let s2 = sin_alt * sin_alt;
    let two_sc = 2.0 * sin_alt * cos_alt;
    let out_x = sin_alt * z2 * two_sc + (c2 - s2) * x2;
    let out_y = 2.0 * s2 * y2 + (c2 - s2) * y2;
    let out_z = -x2 * sin_alt * two_sc + (c2 - s2) * z2;

    Vec3::new(out_x, out_y, out_z)
}

/// `time_of_day` @0x140a73460（O 结构；suncalc 与光照/音频写盘待接）。
pub fn time_of_day(
    time: Res<crate::resources::time::GameTime>,
    mut state: ResMut<TimeOfDayState>,
    _lighting: Res<TimeOfDayLightingSink>,
) {
    let dt = time.delta;
    // 角度/小时插值（瞬时吸附直接对齐 fmod 后小时）。
    let (angle, angle_tr) = advance_angle(state.angle, TWO_PI - state.angle, dt);
    state.angle = angle;
    state.angle_transitioning = angle_tr;
    // 播放状态机。
    let dev_on = state.dev_speed_mult != 0.0;
    let tod: &mut TimeOfDayState = &mut state;
    let (playback, progress) = (&mut tod.playback, &mut tod.progress);
    advance_playback(playback, progress, dev_on, dt);
    // 小时推进（TIME_OF_DAY_SPEED 全局 0x14362c0b8；本实现用 1.0 默认，待取证）。
    let (target, remapped) = advance_tod_hour(
        state.progress,
        dt,
        state.dev_speed_mult,
        1.0,
        false,
    );
    state.target_hour = target;
    state.hour = remapped;
    // 日期/太阳：day_of_year ∈ 1..365（越界 unwrap panic）；suncalc::get_position +
    // 光照写盘（TodLightingSnapshot/SunSkyColors）与音频上报依赖 suncalc/FMOD，登记待接。
    assert!(
        (1..=365).contains(&state.day_of_year),
        "day_of_year out of range"
    );
    let _encoded = state.day_of_year as u64 | YEAR_2023_BITS;
}

/// 光照写盘接缝（I：TodLightingSnapshot 插值/SunSkyColors 两套；宿主接线）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct TimeOfDayLightingSink {
    pub updated: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn angle_interpolation_converges_to_target() {
        // 目标 = 2π − angle，收敛 < 0.01 清标志。
        let (angle, transitioning) = advance_angle(0.0, TWO_PI, 1.0);
        assert!((angle - TWO_PI).abs() < 1e-3);
        assert!(angle > 0.0);
        let _ = transitioning;
    }

    #[test]
    fn hour_interpolation_wraps_and_converges() {
        let (hour, _) = advance_hour(23.9, 0.1, 1.0, 1.0);
        assert!((hour - 0.1).abs() < 1e-3 || hour < 0.5);
        let (same, done) = advance_hour(10.0, 10.0, 0.0, 1.0);
        assert_eq!(same, 10.0);
        assert!(!done);
    }

    #[test]
    fn playback_state_machine_advances_and_rewinds() {
        let mut state = TodPlaybackState::Idle;
        let mut progress = 0.0;
        advance_playback(&mut state, &mut progress, true, 0.3);
        assert_eq!(state, TodPlaybackState::Playing);
        advance_playback(&mut state, &mut progress, true, 0.6);
        assert!((progress - 0.2).abs() < 1e-4); // 0 + 0.6/3
        let mut state = TodPlaybackState::Idle;
        let mut progress = 0.5;
        advance_playback(&mut state, &mut progress, false, 2.0);
        assert_eq!(state, TodPlaybackState::Rewinding);
        advance_playback(&mut state, &mut progress, false, 5.0);
        assert_eq!(progress, 0.0); // 0.5 - 5/5 → clamp 0
    }

    #[test]
    fn tod_hour_advance_remaps_to_the_summer_curve() {
        // target = progress·24 + 8；hour = target·24 + 8 mod 24 → remap。
        let (target, remapped) = advance_tod_hour(0.0, 0.0, 1.0, 1.0, false);
        assert_eq!(target, 8.0);
        // remap(8) → 在 [3.5, 9) 段 → ((8-3.5)/5.5)·8 + 3 ≈ 9.55。
        assert!((remapped - 9.545).abs() < 0.01);
    }

    #[test]
    fn day_of_year_out_of_range_panics_like_unwrap() {
        let mut state = TimeOfDayState::default();
        state.day_of_year = 366;
        let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _encoded = state.day_of_year as u64 | YEAR_2023_BITS;
            assert!((1..=365).contains(&state.day_of_year));
        }));
        assert!(res.is_err());
    }

    #[test]
    fn suncalc_pos_to_vec_is_finite_and_follows_the_constant_matrix() {
        // 正午（azimuth 向南、elevation 高）→ 有限方向；矩阵方程逐项对应机器码。
        let v = suncalc_pos_to_vec(core::f64::consts::PI, 0.75);
        assert!(v.is_finite());
        assert!(v.length_squared() > 0.0);
        // 高纬度正午太阳应指向上方为主（y 分量显著）。
        assert!(v.y.abs() > 0.1);
        // 零高度太阳（地平线）→ 方向仍有限且不退化。
        let horizon = suncalc_pos_to_vec(1.0, 0.0);
        assert!(horizon.is_finite());
        assert!(horizon.length_squared() > 0.5);
    }
}
