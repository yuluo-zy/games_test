//! `weather_system.rs` — 冬季降雪强度（B 0x1408e5b00，全证）。
//!
//! 只在 `glade_type == 4`（冬）时写；非冬季连变更 tick 都不回写。强度按 10 分钟正弦的符号涨落：
//! `intensity = clamp(intensity ± dt * 0.1, 0, 1)`，`sin(game_secs / 300 * π) > 0` 取 `+`。
//! 常量：1e9 @0x1429a7640、300 @0x142aa5140、π @0x1429edba4、0.1 @0x142925984、1.0 @0x142925904。
use core::time::Duration;

/// `glade_type` 的冬季取值（O：`byte [glade+0x58] == 4`）。
pub const GLADE_TYPE_WINTER: u8 = 4;

/// 降雪强度（O：`+0xc`）。前 0xc 字节未恢复。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SnowIntensity {
    pub intensity: f32,
}

/// 返回 `true` 表示发生了写入（原版此时才回写 `Mut` 的变更 tick）。
///
/// `game_time` 是 `GameTime` 的累计 `Duration`（O：`+0x0` secs、`+0x8` nanos），`delta` 是其 f32 帧增量（`+0x28`）。
pub fn weather_system(
    snow: &mut SnowIntensity,
    game_time: Duration,
    delta: f32,
    glade_type: u8,
) -> bool {
    if glade_type != GLADE_TYPE_WINTER {
        return false;
    }
    let secs = game_time.as_secs() as f32;
    let nanos = game_time.subsec_nanos() as i32 as f32;
    let phase = ((nanos / 1e9_f32 + secs) / 300.0_f32) * core::f32::consts::PI;
    let step = delta * 0.1_f32;
    let signed = if 0.0 < phase.sin() { step } else { -step };
    let next = signed + snow.intensity;
    let floored = if 0.0 <= next { next } else { 0.0 };
    snow.intensity = if floored <= 1.0 { floored } else { 1.0 };
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn non_winter_never_writes() {
        let mut snow = SnowIntensity { intensity: 0.3 };
        assert!(!weather_system(&mut snow, Duration::from_secs(10), 1.0, 3));
        assert_eq!(snow.intensity, 0.3);
    }

    #[test]
    fn positive_half_of_the_sine_snows_more_and_negative_half_less() {
        let mut up = SnowIntensity { intensity: 0.5 };
        assert!(weather_system(
            &mut up,
            Duration::from_secs(150),
            1.0,
            GLADE_TYPE_WINTER
        ));
        assert!((up.intensity - 0.6).abs() < 1e-6);
        let mut down = SnowIntensity { intensity: 0.5 };
        weather_system(&mut down, Duration::from_secs(450), 1.0, GLADE_TYPE_WINTER);
        assert!((down.intensity - 0.4).abs() < 1e-6);
    }

    #[test]
    fn intensity_is_clamped_to_unit_range() {
        let mut full = SnowIntensity { intensity: 0.99 };
        weather_system(&mut full, Duration::from_secs(150), 10.0, GLADE_TYPE_WINTER);
        assert_eq!(full.intensity, 1.0);
        let mut empty = SnowIntensity { intensity: 0.01 };
        weather_system(
            &mut empty,
            Duration::from_secs(450),
            10.0,
            GLADE_TYPE_WINTER,
        );
        assert_eq!(empty.intensity, 0.0);
    }

    #[test]
    fn exactly_zero_sine_counts_as_the_decreasing_branch() {
        let mut snow = SnowIntensity { intensity: 0.5 };
        weather_system(&mut snow, Duration::ZERO, 1.0, GLADE_TYPE_WINTER);
        assert!((snow.intensity - 0.4).abs() < 1e-6);
    }
}
