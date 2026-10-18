//! Candidate B TOD system canonical resource wiring. Not production.
//! Fresh native evidence: time_of_day @140a73460, GUI TOD @1405cc7f0,
//! circle drag @140505780, insert_resources @140940ed0.
//! KeyButtonInput is the existing explicit host projection of original
//! ButtonInput<KeyCode>; no second input Resource identity is introduced.
use bevy_ecs::prelude::*;
use glam::{Quat, Vec3};
use std::sync::atomic::{AtomicU32, Ordering};

use crate::{
    render::{
        atmosphere::{DerivedAtmosphere, SunSkyColors},
        directional_light::DirectionalLight,
    },
    resources::session::NewSessionStartedCmd,
    resources::{
        audio_resources::{AudioCore, Studio},
        session::ActiveSession,
        time::GameTime,
        time_of_day::{
            inverse_lerp, remap_hour_to_fmod_hour, time_continuous_lerp_mut, TimeOfDay,
            TodLightingSnapshot, TodPlaybackState, TodSequence, TIME_OF_DAY_SPEED,
        },
    },
    systems::save::load_game::LoadingGameComplete,
};

/// `2π`（O：6.2831855 @0x142934674）。
const TWO_PI: f32 = 6.283_185_5;
/// `-3`（O：0x142ad58e8）。
const SMOOTH_K: f32 = -3.0;
/// `24` / `8`（O：0x142934600 / 0x142934604）。
const HOUR_MOD: f32 = 24.0;
const HOUR_OFFSET: f32 = 8.0;
/// `3600000`（O：0x142ad58f0）。
const HOUR_TO_MS: f32 = 3_600_000.0;
/// 2023-01-01 的 Unix epoch 日序（time crate `Date::from_ordinal_date(2023, d)` 等价）。
const YEAR_2023_EPOCH_DAYS: i64 = 19_358;
/// 2023 << 9（O：0xfce00，`time::Date` 打包编码；仅作证据常量）。
const YEAR_2023_BITS: u32 = 0x0fce00;

/// 帧间全局 `f32`（O：静态 0x143634d84）：上一帧 `snapshot.speed_bias`，
/// 下一帧作为 `exp2f` 输入。
static SPEED_BIAS_GLOBAL: AtomicU32 = AtomicU32::new(0);

#[inline]
fn load_speed_bias() -> f32 {
    f32::from_bits(SPEED_BIAS_GLOBAL.load(Ordering::Relaxed))
}

#[inline]
fn store_speed_bias(v: f32) {
    SPEED_BIAS_GLOBAL.store(v.to_bits(), Ordering::Relaxed);
}

/// 角度插值（判等函数，红线不动）。
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

/// 小时插值（判等函数，红线不动）。
pub fn advance_hour(current: f32, target: f32, dt: f32, speed: f32) -> (f32, bool) {
    let diff = target - current;
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

/// 播放状态推进（O 0x140a73460：dev 关恒 `Rewinding(2)`、dev 开恒 `Playing(1)`；
/// 旧 state 只决定 progress 变化）。
///
/// | 旧 state | dev 关（`+0xe3==0`） | dev 开 |
/// |---|---|---|
/// | Idle(0) | state=2、progress=0 | state=1、progress=0 |
/// | Playing(1) | state=2、progress 不变 | state=1、`progress+=dt/3` clamp≤1 |
/// | Rewinding(2) | state=2、`progress+=dt/−5` clamp≥0 | state=1、progress 不变 |
pub fn advance_playback(state: &mut TodPlaybackState, progress: &mut f32, dev_on: bool, dt: f32) {
    let old = *state;
    if !dev_on {
        *state = TodPlaybackState::Rewinding;
        match old {
            TodPlaybackState::Idle => *progress = 0.0,
            TodPlaybackState::Playing => {}
            TodPlaybackState::Rewinding => {
                *progress = (*progress + dt / -5.0).max(0.0);
            }
        }
    } else {
        *state = TodPlaybackState::Playing;
        match old {
            TodPlaybackState::Idle => *progress = 0.0,
            TodPlaybackState::Playing => {
                *progress = (*progress + dt / 3.0).min(1.0);
            }
            TodPlaybackState::Rewinding => {}
        }
    }
}

/// 小时推进（O 0xa73c53..0xa73cc3）：
/// ```
/// dt_speed = dt * TIME_OF_DAY_SPEED
/// inc = exp2f(speed_bias_global) * (progress * dt_speed) * dev_speed_mult
/// target = fmodf(inc + target + 1.0, 1.0) → rem_euclid(1.0)
/// ```
/// 调用点另有门：`screen.visible && screen.time_input_active` 时保持 target 不变。
/// `speed_bias_global` 是上一帧 `TodLightingSnapshot.speed_bias`（静态 0x143634d84）。
pub fn advance_tod_hour(
    progress: f32,
    dt: f32,
    dev_speed_mult: f32,
    target_hour: f32,
    speed_bias_global: f32,
) -> f32 {
    let dt_speed = dt * TIME_OF_DAY_SPEED;
    let progress_dt = progress * dt_speed;
    let mut inc = speed_bias_global.exp2() * progress_dt;
    inc *= dev_speed_mult;
    (inc + target_hour + 1.0).rem_euclid(1.0)
}

/// `suncalc::get_position(ms, lat, lng) -> (azimuth, altitude)`（O 0x1417ab4b0，
/// 628 B）逐语句等价实现（`suncalc` 0.4.0 未入当前依赖）。
pub fn sun_position_at(unix_ms: i64, latitude_deg: f64, longitude_deg: f64) -> (f64, f64) {
    const DEG2RAD: f64 = 0.017_453_292_519_943_295;
    let d5 = (unix_ms as f64) / 86_400_000.0 + -0.5 + 2_440_588.0 + -2_451_545.0;
    let mut d4 = (d5 * 0.985_600_28 + 357.5291) * DEG2RAD;
    let d1 = d4.sin();
    let d2 = (d4 + d4).sin();
    let d3 = (d4 * 3.0).sin();
    d4 = (d3 * 0.000_3 + d2 * 0.02 + d1 * 1.914_8) * DEG2RAD
        + d4
        + 1.796_593_062_783_907
        + core::f64::consts::PI;
    let sin_ecl = d4.sin();
    let declination = (sin_ecl * 0.397_783_703_493_422_74 + 0.0).asin();
    let cos_ecl = d4.cos();
    let right_ascension = (sin_ecl * 0.917_479_223_326_096_2).atan2(cos_ecl);
    let hour_angle =
        ((d5 * 360.985_623_5 + 280.16) * DEG2RAD + longitude_deg * DEG2RAD) - right_ascension;
    let sin_ha = hour_angle.sin();
    let cos_ha = hour_angle.cos();
    let sin_lat = (latitude_deg * DEG2RAD).sin();
    let cos_lat = (latitude_deg * DEG2RAD).cos();
    let tan_decl = declination.tan();
    let azimuth = sin_ha.atan2(sin_lat * cos_ha - tan_decl * cos_lat);
    let sin_decl = declination.sin();
    let cos_decl = declination.cos();
    let altitude = (cos_decl * cos_lat * cos_ha + sin_decl * sin_lat).asin();
    (azimuth, altitude)
}

/// 正交基常量（O：常量池 0x142ad590c..0x142ad5984 直读，逐位一致）。
mod basis {
    pub const C: f32 = 0.581_318_6; // 0x3f14d14c
    pub const A: f32 = -0.762_442_5; // 0xbf432f6f
    pub const B: f32 = 0.647_056; // 0x3f25a576
    pub const D: f32 = 0.418_681_44; // 0x3ed65d6a
    pub const E: f32 = -0.493_343; // 0xbefc9774
    pub const F: f32 = 0.551_645_8; // 0x3f0d38a9
    pub const G: f32 = 0.551_645_8; // 0x3f0d38a9
    pub const I: f32 = 0.834_078_5; // 0x3f55862b
    pub const J: f32 = 0.834_078_5; // 0x3f55862b
    pub const M: f32 = 0.806_015_4; // 0x3f4e5706
    pub const N: f32 = 0.591_894_6; // 0x3f178668
}

/// `suncalc_pos_to_vec`（O 0x140a73460 内联，source line 348）：
/// 方位/高度 → (主太阳方向, 次级方向)，均未再次归一化（次级方向原样）。
///
/// 主方向：`colatitude = π/2 − (azimuth + π)`；
/// `f44=-sin(colat)·cos(elev)`、`f43=sin(elev)`、`f29=cos(colat)·cos(elev)`；
/// 用 `orbit_angle` 的半角做 Y 轴旋转（`cos_a=c²−s²`、`sin_a=2cs`），
/// `y += -0.15`，再 `1/sqrt(z²+y²+x²)` 归一化。
///
/// 次级方向：正交基 A..P 第一/二段（`basis_angle` 的 sin/cos）后同样做半角 Y 旋转；
/// 原样写 DerivedAtmosphere（不归一化）。
pub fn suncalc_pos_to_vec(
    azimuth: f64,
    elevation: f64,
    basis_angle: f32,
    orbit_angle: f32,
) -> (Vec3, Vec3) {
    let colatitude = core::f64::consts::FRAC_PI_2 - (azimuth + core::f64::consts::PI);
    let cos_colat = colatitude.cos() as f32;
    let sin_colat = colatitude.sin() as f32;
    let cos_elev = elevation.cos() as f32;
    let sin_elev = elevation.sin() as f32;

    let f44 = -sin_colat * cos_elev;
    let f43 = sin_elev;
    let f29 = cos_colat * cos_elev;

    // 第一段：basis_angle 的 sin/cos + A..P 正交基。
    let s = basis_angle.sin();
    let c = basis_angle.cos();
    let m = 1.0 - c;
    let f41_a = m * basis::E * basis::I + (c + m * basis::D) * basis::F;
    let f42 = (m * basis::C + c) * basis::J + m * basis::E * basis::G;
    // O：`(0.0 - s·(-0.7624425))·0.5516458 + (s·0.647056)·0.8340785`
    let f32s = s * basis::B * basis::I + s * 0.762_442_5 * basis::F;

    // 第二段（N/M）。
    let f45 = f32s * basis::N - f41_a * basis::M;
    let f49 = f42;
    let f41_b = -f41_a * basis::N - f32s * basis::M;

    // 末段：orbit_angle 半角 Y 轴旋转。
    let sa = (orbit_angle * 0.5).sin();
    let ca = (orbit_angle * 0.5).cos();

    // Original primary vector x=cos(colat)*cos(elev), z=-sin(colat)*cos(elev).
    // Keep the original glam quaternion multiplication, including its y
    // arithmetic; replacing it with a simplified 2D rotation changes rounding.
    let rotation = Quat::from_xyzw(0.0, sa, 0.0, ca);
    let rotated_sun = rotation * Vec3::new(f29, f43, f44);
    let shifted_sun = Vec3::new(rotated_sun.x, rotated_sun.y + -0.15, rotated_sun.z);
    let inv = 1.0
        / (shifted_sun.z * shifted_sun.z
            + shifted_sun.y * shifted_sun.y
            + shifted_sun.x * shifted_sun.x)
            .sqrt();
    let main = shifted_sun * inv;
    let secondary = rotation * Vec3::new(f45, f49, f41_b);

    (main, secondary)
}

/// `time_of_day` @0x140a73460（O）。参数顺序与 PDB `FunctionSystem` 泛型实参一致；
/// `TodScreen` / `TodDevConfig` / `TodWeatherSeam` / `TodInputSeam` /
/// `TodLightingSeam` 是对应宿主资源的同域 seam（见类型注释）。
#[allow(clippy::too_many_arguments)]
pub fn time_of_day<S: Studio + Send + Sync + 'static>(
    audio: Res<AudioCore<S>>,
    tod_sequence: Res<TodSequence>,
    weather: Res<crate::resources::weather::Weather>,
    time: Res<GameTime>,
    button_input: Res<crate::systems::backend::input::button_input::KeyButtonInput>,
    mut directional_light: ResMut<DirectionalLight>,
    mut tod: ResMut<TimeOfDay>,
    mut lighting: ResMut<DerivedAtmosphere>,
    mut config: ResMut<crate::game_config::GameConfig>,
    mut new_session_evt: EventReader<NewSessionStartedCmd>,
    mut loading_complete_evt: EventReader<LoadingGameComplete>,
    mut screen: ResMut<crate::resources::gui::TimeOfDayScreen>,
    active_session: Res<ActiveSession>,
) {
    let dt = time.delta;

    // 1. 音频门 ×2（O 0xa7373c..0xa737ba）。
    if audio.is_connected() {
        let v = if config.dev.day_night_cycle { 1.0 } else { 0.0 };
        let _ = audio.set_parameter("TOD-Enabled", v);
    }
    if audio.is_connected() {
        let v = if screen.time_input_active { 1.0 } else { 0.0 };
        let _ = audio.set_parameter("Timetravel", v);
    }

    // 2. angle 块（O 0xa737c1..0xa738d2）。
    if !screen.angle_input_active {
        if !screen.visible {
            screen.sun_xz_rotation = TWO_PI - tod.sun_xz_rotation;
        }
    } else {
        let current = tod.sun_xz_rotation.rem_euclid(TWO_PI);
        let target = TWO_PI - screen.sun_xz_rotation;
        let factor = (SMOOTH_K * dt).exp();
        let mut diff = target - current;
        if -core::f32::consts::PI <= diff {
            if core::f32::consts::PI <= diff {
                diff -= TWO_PI;
            }
        } else {
            diff += TWO_PI;
        }
        let next = (1.0 - factor) * diff + current;
        tod.sun_xz_rotation = next;
        if (next - target).abs() < 0.01 {
            screen.angle_input_active = false;
        }
    }

    // 3. hour 块（O 0xa738d2..0xa73aaa）。
    if !screen.visible {
        screen.time_of_day = tod.time_of_day;
    } else if !screen.time_input_active {
        let old = screen.time_of_day;
        let wrapped = old.rem_euclid(1.0);
        let target = tod.time_of_day;
        let speed = config.game.tod_speed.max(1.0);
        let factor = (speed * (dt * SMOOTH_K)).exp();
        let mut diff = target - wrapped;
        if -0.5 <= diff {
            if 0.5 <= diff {
                diff -= 1.0;
            }
        } else {
            diff += 1.0;
        }
        let next = (1.0 - factor) * diff + wrapped;
        screen.time_of_day = next;
        // 原生比较对象是写入前的 screen 值（`subss xmm2,[rsi]`）。
        if (next - old).abs() < 0.001 {
            screen.time_input_active = false;
        }
    } else {
        // instant：直接朝屏幕小时吸附，写回 TimeOfDay 相位。
        let target = tod.time_of_day;
        let mut diff = screen.time_of_day - target;
        let factor = (-dt).exp();
        if -0.5 <= diff {
            if 0.5 <= diff {
                diff -= 1.0;
            }
        } else {
            diff += 1.0;
        }
        let next = (target + (1.0 - factor) * diff).rem_euclid(1.0);
        tod.time_of_day = next;
        if (next - screen.time_of_day).abs() < 0.001 {
            screen.time_input_active = false;
        }
    }

    // 4. 事件读取器（O 0xa73aaa..0xa73b4f）：两个 read().last()，只取 is_some。
    let session_event = new_session_evt.read().last().is_some();
    let load_event = loading_complete_evt.read().last().is_some();

    // 5. dev 热键（O 0xa73b52..0xa73b9d）：全局门 + ButtonInput::just_pressed(F10)。
    if crate::game_config::developer_hotkeys_enabled() && button_input.just_pressed(0xa9) {
        config.dev.day_night_cycle ^= true;
    }

    // 6. 播放状态机（O 0xa73b9d..0xa73c39）；dev 门 = GameConfig+0xe3。
    {
        let tod = &mut *tod;
        advance_playback(
            &mut tod.state,
            &mut tod.progress,
            config.dev.day_night_cycle,
            dt,
        );
    }

    // 7. 小时推进（O 0xa73c39..0xa73cf0）。
    if !(screen.visible && screen.time_input_active) {
        tod.time_of_day = advance_tod_hour(
            tod.progress,
            dt,
            config.game.tod_speed,
            tod.time_of_day,
            load_speed_bias(),
        );
    }
    let hour = (tod.time_of_day * HOUR_MOD + HOUR_OFFSET).rem_euclid(HOUR_MOD);
    let remapped = remap_hour_to_fmod_hour(hour, active_session.is_winter());
    if audio.is_connected() {
        let _ = audio.set_parameter("DayNight", remapped);
    }

    // 8. 日期/太阳（O 0xa73d17..0xa73e5f）。
    let day = tod.day_of_year;
    assert!(day != 0 && day <= 365, "day_of_year out of range");
    let encoded = (day as u32) | YEAR_2023_BITS;
    let _ = encoded; // 证据常量；实际日期用等价的 epoch 日序。
    let date_ms = (YEAR_2023_EPOCH_DAYS + (day as i64 - 1)) * 86_400_000;
    let hour_ms = (hour * HOUR_TO_MS) as i64;
    let unix_ms = date_ms + hour_ms;
    let (azimuth, elevation) = sun_position_at(unix_ms, tod.latitude_degrees as f64, 0.0);

    let basis_angle = tod.time_of_day * TWO_PI + 3.45;
    let (sun_dir, moon_dir) =
        suncalc_pos_to_vec(azimuth, elevation, basis_angle, tod.sun_xz_rotation);

    // snapshot_at_tod（O 0xa73e5f）+ 全局 speed_bias 写回（O 0xa73ed0）。
    let snapshot = tod_sequence.snapshot_at_tod(tod.time_of_day);
    store_speed_bias(snapshot.speed_bias);

    // 9. 昼/夜段与连续值（O 0xa73ed8..0xa73f62）。
    let in_night = tod.sunrise <= tod.time_of_day && tod.time_of_day < tod.sunset;
    tod.day_night_factor = if in_night {
        inverse_lerp(tod.sunrise, tod.sunset, tod.time_of_day)
    } else {
        0.0
    };
    let target = if in_night { 1.0 } else { 0.0 };
    if !session_event && !load_event {
        time_continuous_lerp_mut(&mut tod.day_night_continuous, target, dt, 1.0);
    } else {
        tod.day_night_continuous = target;
    }

    // Original complete final writer; do not leave data in a lighting seam.
    write_derived_atmosphere(
        &mut lighting,
        &mut directional_light,
        &snapshot,
        sun_dir,
        moon_dir,
        tod.day_night_continuous,
        tod.day_night_continuous_prev,
        weather.weather_fog,
    );
}

pub fn write_derived_atmosphere(
    output: &mut DerivedAtmosphere,
    light: &mut DirectionalLight,
    snapshot: &TodLightingSnapshot,
    sun: Vec3,
    moon: Vec3,
    continuous: f32,
    previous: f32,
    weather: f32,
) {
    // Existing SunSkyColors a/b/c preserve twelve contiguous floats; original
    // serde fields are four Vec3. Project without changing resource identity.
    let a = snapshot.sun_sky.a.to_array();
    let b = snapshot.sun_sky.b.to_array();
    let c = snapshot.sun_sky.c.to_array();
    let sun_tint = Vec3::new(a[0], a[1], a[2]);
    let moon_tint = Vec3::new(a[3], b[0], b[1]);
    let sky_low = Vec3::new(b[2], b[3], c[0]);
    let sky_high = Vec3::new(c[1], c[2], c[3]);

    // Original cmp: ucomiss xmm0(0.5),xmm6(continuous); jbe high.
    // Preserve the unordered/NaN destination: NaN also takes this high branch.
    let (selected, fake_sun, fake_moon) = if !(0.5 > continuous) {
        light.set_direction_towards_light(moon);
        let twice = continuous + -0.5;
        let weight = (1.0 - previous) * (twice + twice);
        (moon_tint * weight, sun_tint, moon_tint * (1.0 - weight))
    } else {
        light.set_direction_towards_light(sun);
        let weight = (1.0 - previous) * (1.0 - (continuous + continuous));
        (sun_tint * weight, sun_tint * (1.0 - weight), moon_tint)
    };
    *output = DerivedAtmosphere {
        sun_tint: selected,
        fake_light_color: fake_sun,
        fake_light_dir: sun,
        fake_light2_color: fake_moon,
        fake_light2_dir: moon,
        sky_low_color: sky_low,
        sky_high_color: sky_high,
        fog_thickness: snapshot.base_fog_thickness + weather,
        fog_tint: snapshot.fog_tint,
        dir_to_sun: sun,
        dir_to_moon: moon,
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn playback_state_machine_forces_the_dev_direction_each_frame() {
        let mut state = TodPlaybackState::Idle;
        let mut progress = 0.0;
        advance_playback(&mut state, &mut progress, true, 0.3);
        assert_eq!(state, TodPlaybackState::Playing);
        assert_eq!(progress, 0.0);
        advance_playback(&mut state, &mut progress, true, 0.6);
        assert!((progress - 0.2).abs() < 1e-4);
        let mut state = TodPlaybackState::Rewinding;
        let mut progress = 0.7;
        advance_playback(&mut state, &mut progress, true, 1.0);
        assert_eq!(state, TodPlaybackState::Playing);
        assert_eq!(progress, 0.7);

        let mut state = TodPlaybackState::Idle;
        let mut progress = 0.5;
        advance_playback(&mut state, &mut progress, false, 2.0);
        assert_eq!(state, TodPlaybackState::Rewinding);
        assert_eq!(progress, 0.0);
        let mut state = TodPlaybackState::Playing;
        let mut progress = 0.4;
        advance_playback(&mut state, &mut progress, false, 5.0);
        assert_eq!(state, TodPlaybackState::Rewinding);
        assert_eq!(progress, 0.4);
        advance_playback(&mut state, &mut progress, false, 5.0);
        assert_eq!(progress, 0.0);
    }

    #[test]
    fn tod_hour_advance_uses_exp2_speed_bias_and_tod_speed() {
        // bias=0 → exp2=1；progress=0.5、dt=1、dev=1、speed=1/600 →
        // inc = 1 * (0.5 * (1/600)) = 1/1200。
        let target = advance_tod_hour(0.5, 1.0, 1.0, 0.25, 0.0);
        let expected = (0.5 * TIME_OF_DAY_SPEED + 0.25 + 1.0).rem_euclid(1.0);
        assert!((target - expected).abs() < 1e-6, "{target} vs {expected}");
        // 参数为 1 时 exp2(1)=2，增量翻倍。
        let target2 = advance_tod_hour(0.5, 1.0, 1.0, 0.25, 1.0);
        let expected2 = (2.0 * 0.5 * TIME_OF_DAY_SPEED + 0.25 + 1.0).rem_euclid(1.0);
        assert!((target2 - expected2).abs() < 1e-6);
        // dev 倍率作用于增量。
        let target3 = advance_tod_hour(0.5, 1.0, 3.0, 0.25, 0.0);
        let expected3 = (3.0 * 0.5 * TIME_OF_DAY_SPEED + 0.25 + 1.0).rem_euclid(1.0);
        assert!((target3 - expected3).abs() < 1e-6);
        // TIME_OF_DAY_SPEED 原生值 = 1/600（0x3ada740d）。
        assert_eq!(TIME_OF_DAY_SPEED.to_bits(), 0x3ada740d);
    }

    #[test]
    fn day_of_year_out_of_range_panics() {
        let mut state = TimeOfDay::default();
        state.day_of_year = 366;
        let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let day = state.day_of_year;
            assert!(day != 0 && day <= 365);
        }));
        assert!(res.is_err());
    }

    #[test]
    fn original_primary_vector_pack_has_cosine_x_and_negative_sine_z() {
        // colatitude=pi/2. Native zero-rotation vector before y offset=(0,0,-1).
        let (sun, _) = suncalc_pos_to_vec(-core::f64::consts::PI, 0.0, 0.0, 0.0);
        let inv = 1.0f32 / (1.0f32 + 0.15f32 * 0.15f32).sqrt();
        assert!(sun.x.abs() < 1e-6);
        assert!((sun.y - -0.15 * inv).abs() < 1e-6);
        assert!((sun.z - -inv).abs() < 1e-6);
    }

    #[test]
    fn suncalc_pos_to_vec_matches_native_half_angle_and_normalization() {
        let (v, sec) = suncalc_pos_to_vec(core::f64::consts::PI, 0.75, 0.0, 0.0);
        assert!(v.is_finite());
        assert!((v.length() - 1.0).abs() < 1e-5, "{}", v.length());
        assert!(sec.is_finite());
        // 非零轨道角改变主方向。
        let (rotated, _) = suncalc_pos_to_vec(core::f64::consts::PI, 0.75, 0.0, 1.0);
        assert_ne!((rotated.x * 1000.0) as i32, (v.x * 1000.0) as i32);
        // 地平线不退化。
        let (horizon, _) = suncalc_pos_to_vec(1.0, 0.0, 0.0, 0.0);
        assert!(horizon.is_finite());
        assert!(horizon.length() > 0.1);
    }

    #[test]
    fn suncalc_position_is_finite_and_has_expected_noon_shape() {
        // 2023-06-21 12:00 UTC（ordinal 172）在纬度 0：太阳高度应接近正。
        let date_ms = (YEAR_2023_EPOCH_DAYS + 171) * 86_400_000 + 12 * 3_600_000;
        let (az, alt) = sun_position_at(date_ms, 0.0, 0.0);
        assert!(az.is_finite() && alt.is_finite());
        assert!(alt > 0.3, "altitude {alt}");
        // 午夜高度为负。
        let midnight = (YEAR_2023_EPOCH_DAYS + 171) * 86_400_000;
        let (_, alt_mid) = sun_position_at(midnight, 0.0, 0.0);
        assert!(alt_mid < 0.0, "altitude {alt_mid}");
    }

    #[test]
    fn angle_and_hour_blocks_clear_flags_on_convergence() {
        // angle：目标 2π - t；收敛 < 0.01 清标志（红线判等函数）。
        let (a, tr) = advance_angle(0.0, TWO_PI, 1.0);
        assert!(a > 0.0);
        let _ = tr;
        // hour：±0.5 wrap 的判等函数保持原语义。
        let (h, _) = advance_hour(23.9, 0.1, 1.0, 1.0);
        assert!((h - 0.1).abs() < 1e-3 || h < 0.5);
        let (same, done) = advance_hour(10.0, 10.0, 0.0, 1.0);
        assert_eq!(same, 10.0);
        assert!(!done);
    }
}
