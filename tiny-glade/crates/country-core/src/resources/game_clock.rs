//! `game_clock.rs` — `time_update`（B 0x140922c30）写入的 0x38 B 游戏时钟。
//!
//! 证据：反编译 `camera-time/decomp/decomp-0x140922c30.txt`，常量 1/60 @0x142aad4a8（0x3c888889）、
//! 60.0 @0x1429d18cc、1e9 @0x1429a7640。
//!
//! 布局（O，按写入偏移）：`elapsed` Duration（secs u64 @+0x00、nanos u32 @+0x08）、`total_secs` f64 @+0x10、
//! `ticks_60` u64 @+0x18、`frac` f32 @+0x20、`delta` f32 @+0x28、`frame_count` u32 @+0x2c、`ran` u8 @+0x30、
//! `paused` u8 @+0x31（只读）。此前文档把 `nanos` 写成 `+0x4`、`flag` 写成 `+0x6`，与写入偏移不符。
//!
//! 未恢复：`time_update` 在重放时从 `InputRecordReplayState` 取帧的下标运算（调用方把结果作为
//! [`DeltaSource::Replay`] 传入）。
use core::time::Duration;

/// 前两帧对 `delta` 的上限（O：0x142aad4a8 = 0x3c888889）。
pub const FIRST_FRAMES_DELTA_CAP: f32 = 0.016_666_668;

/// 时间步来源。
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DeltaSource {
    /// 重放：帧里的 f32 增量（O：`frame + 0xc8`）。
    Replay { frame_delta: f32 },
    /// 实时：上游时间资源的 `Duration`（O：`+0x10` secs、`+0x18` nanos）。
    Live { secs: u64, nanos: u32 },
}

/// `time_update` 的入口门控输入（O：0x140922c35..0x140922d17）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ClockGate {
    /// `*param_2 == 0`（门控资源首个 u32 为 0）。
    pub state_is_zero: bool,
    /// `EscMenuState` 标签（tag 10 / 7 / 2 参与判定）。
    pub mode_tag: u32,
    /// `EscMenuState` 载荷首字节（tag 10 要求为 0，tag 7 要求为 1）。
    pub mode_flag: u8,
    /// 时钟 `+0x31`：暂停标志。
    pub paused: bool,
}

impl ClockGate {
    /// Consume the same owned menu resource as LUT rendering and menu advancement.
    pub fn from_menu(
        state_is_zero: bool,
        menu: &crate::resources::gui::EscMenuState,
        paused: bool,
    ) -> Self {
        let view = menu.view();
        Self {
            state_is_zero,
            mode_tag: view.tag,
            mode_flag: view.flag,
            paused,
        }
    }

    /// O：`bVar14`，时钟是否推进（影响 `Live` 的步长是否为 0 以及写入 `ran`）。
    pub fn runs(self) -> bool {
        if !self.state_is_zero {
            return false;
        }
        let mode_ok = match self.mode_tag {
            10 => self.mode_flag == 0,
            7 => self.mode_flag == 1,
            2 => true,
            _ => false,
        };
        mode_ok && !self.paused
    }
}

/// 原版两处 panic。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GameClockError {
    /// `Duration::try_from_secs_f32` 失败（负、NaN 或过大，0x142aad5e0，长 0x3b）。
    InvalidDelta,
    /// "overflow when adding durations"（0x142aad928，长 0x1e）。
    DurationOverflow,
}

impl core::fmt::Display for GameClockError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidDelta => f.write_str("cannot convert float seconds to Duration"),
            Self::DurationOverflow => f.write_str("overflow when adding durations"),
        }
    }
}

impl std::error::Error for GameClockError {}

/// 游戏时钟资源。
#[derive(bevy_ecs::resource::Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct GameClock {
    pub elapsed: Duration,
    pub total_secs: f64,
    pub ticks_60: u64,
    pub frac: f32,
    pub delta: f32,
    pub frame_count: u32,
    pub ran: bool,
}

impl GameClock {
    /// `time_update` 的计时部分。写入顺序与机器码一致：先写 `delta`，再做可能 panic 的转换与加法，
    /// 成功后才更新 `elapsed`/`ran`/`frame_count`/`total_secs`/`ticks_60`/`frac`。
    pub fn update(&mut self, gate: ClockGate, source: DeltaSource) -> Result<(), GameClockError> {
        let runs = gate.runs();
        let delta = match source {
            DeltaSource::Replay { frame_delta } => frame_delta,
            DeltaSource::Live { secs, nanos } => {
                let live = if runs {
                    nanos as i32 as f32 / 1e9_f32 + secs as f32
                } else {
                    0.0
                };
                if self.frame_count < 2 && FIRST_FRAMES_DELTA_CAP <= live {
                    FIRST_FRAMES_DELTA_CAP
                } else {
                    live
                }
            }
        };
        self.delta = delta;
        if delta < 0.0 {
            return Err(GameClockError::InvalidDelta);
        }
        let step = Duration::try_from_secs_f32(delta).map_err(|_| GameClockError::InvalidDelta)?;
        self.elapsed = self
            .elapsed
            .checked_add(step)
            .ok_or(GameClockError::DurationOverflow)?;
        self.ran = runs;
        self.frame_count = self.frame_count.wrapping_add(1);
        self.total_secs += f64::from(delta);
        let scaled = (delta + self.frac) * 60.0;
        let ticks = scaled as u64;
        self.ticks_60 = ticks;
        let rest = (delta + self.frac) - ticks as f32 / 60.0;
        self.frac = if rest <= 0.0 { 0.0 } else { rest };
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn running() -> ClockGate {
        ClockGate {
            state_is_zero: true,
            mode_tag: 2,
            mode_flag: 0,
            paused: false,
        }
    }

    #[test]
    fn gate_follows_the_tag_10_7_2_rules() {
        let g = |tag, flag| ClockGate {
            mode_tag: tag,
            mode_flag: flag,
            ..running()
        };
        assert!(g(2, 9).runs());
        assert!(g(10, 0).runs());
        assert!(!g(10, 1).runs());
        assert!(g(7, 1).runs());
        assert!(!g(7, 0).runs());
        assert!(!g(3, 0).runs());
        assert!(!ClockGate {
            state_is_zero: false,
            ..running()
        }
        .runs());
        assert!(!ClockGate {
            paused: true,
            ..running()
        }
        .runs());
    }

    #[test]
    fn live_delta_is_secs_plus_nanos_and_accumulates() {
        let mut clock = GameClock {
            frame_count: 5,
            ..Default::default()
        };
        clock
            .update(
                running(),
                DeltaSource::Live {
                    secs: 0,
                    nanos: 500_000_000,
                },
            )
            .unwrap();
        assert_eq!(clock.delta, 0.5);
        assert_eq!(clock.elapsed, Duration::from_millis(500));
        assert_eq!(clock.total_secs, 0.5);
        assert_eq!(clock.frame_count, 6);
        assert!(clock.ran);
        assert_eq!(clock.ticks_60, 30);
        assert_eq!(clock.frac, 0.0);
    }

    #[test]
    fn first_two_frames_cap_the_live_delta_but_replay_is_exempt() {
        let mut clock = GameClock::default();
        clock
            .update(running(), DeltaSource::Live { secs: 1, nanos: 0 })
            .unwrap();
        assert_eq!(clock.delta, FIRST_FRAMES_DELTA_CAP);
        clock
            .update(running(), DeltaSource::Live { secs: 1, nanos: 0 })
            .unwrap();
        assert_eq!(clock.delta, FIRST_FRAMES_DELTA_CAP);
        clock
            .update(running(), DeltaSource::Live { secs: 1, nanos: 0 })
            .unwrap();
        assert_eq!(clock.delta, 1.0);
        let mut replay = GameClock::default();
        replay
            .update(running(), DeltaSource::Replay { frame_delta: 2.0 })
            .unwrap();
        assert_eq!(replay.delta, 2.0);
    }

    #[test]
    fn paused_live_clock_has_zero_delta_and_ran_false() {
        let mut clock = GameClock {
            frame_count: 9,
            ..Default::default()
        };
        let gate = ClockGate {
            paused: true,
            ..running()
        };
        clock
            .update(gate, DeltaSource::Live { secs: 3, nanos: 0 })
            .unwrap();
        assert_eq!(
            (clock.delta, clock.ran, clock.frame_count),
            (0.0, false, 10)
        );
    }

    #[test]
    fn sixtieths_carry_the_remainder_in_frac() {
        let mut clock = GameClock {
            frame_count: 9,
            ..Default::default()
        };
        clock
            .update(running(), DeltaSource::Replay { frame_delta: 0.025 })
            .unwrap();
        assert_eq!(clock.ticks_60, 1);
        assert!((clock.frac - (0.025 - 1.0 / 60.0)).abs() < 1e-6);
        let carried = clock.frac;
        clock
            .update(running(), DeltaSource::Replay { frame_delta: 0.0 })
            .unwrap();
        assert_eq!(clock.ticks_60, 0);
        assert!((clock.frac - carried).abs() < 1e-6);
    }

    #[test]
    fn negative_replay_delta_and_overflow_are_the_original_panics() {
        let mut clock = GameClock::default();
        assert_eq!(
            clock.update(running(), DeltaSource::Replay { frame_delta: -0.1 }),
            Err(GameClockError::InvalidDelta)
        );
        assert_eq!(clock.delta, -0.1);
        assert_eq!(clock.frame_count, 0);
        let mut full = GameClock {
            elapsed: Duration::new(u64::MAX, 999_999_999),
            ..Default::default()
        };
        assert_eq!(
            full.update(running(), DeltaSource::Replay { frame_delta: 1.0 }),
            Err(GameClockError::DurationOverflow)
        );
    }
}
