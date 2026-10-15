//! `audio_utils.rs` — `AudioLoop`：一圈 FMOD 事件实例的循环播放器（B `utils::audio_utils`）。
//!
//! 证据（RNE 汇编逐条核对）：
//! - 布局：`entries: Vec<_>`（元素 **0x30 B**，`cap@0 ptr@8 len@0x10`）、`start: Instant`（`secs@0x18`、`nanos u32@0x20`，
//!   `nanos == 1_000_000_000`（0x3b9aca00）表示"未设置"）、`cursor@0x28`、`threshold_ms u32@0x30`、
//!   三个标志字节 `+0x34`（已启动）、`+0x35`（使用 `loop` 参数）、`+0x36`（停止时显式 `stop`）；
//! - `current` 0x140a593f0：`&entries[cursor]`，越界 panic；
//! - `start_if_not_playing` 0x140a596b0：先把 `start` 置为"未设置"；`+0x34` 已置位则返回；`+0x35` 为真时对当前实例
//!   `set_parameter("loop", 1.0)`；实例为 `Active`（判别 1）才 `start`（失败 panic）；最后 `+0x34 = 1`；
//! - `stop` 0x140a59430：`start` 未设置 → 记下 `now` 并返回；否则 `elapsed_ms = (secs as u32) * 1000 + nanos / 1_000_000`
//!   （全部 u32 回绕），`threshold_ms >= elapsed_ms` 返回；`+0x35` 为真：`set_parameter("loop", 0.0)`，仅当 `+0x36 == 1` 且实例
//!   `Active` 才 `stop(0)`；`+0x35` 为假：实例 `Active` 就 `stop(0)`；`stop` 失败 panic；之后
//!   `cursor = (cursor + 1) % len`、`+0x34 = 0`、`start` 置为未设置。
//!
//! `stop` 的模式参数是数字 0（`xor r8d, r8d`）。此前文档把 `stop(0)` 写成 `STOP_IMMEDIATE`，而 FMOD Studio 的
//! `FMOD_STUDIO_STOP_ALLOWFADEOUT` 是 0、`IMMEDIATE` 是 1，两者矛盾，这里保留数字 0（`StopMode::Zero`）不下结论。
//!
//! 未做：`AudioCore::instantiate_loop`（0x140a59220，由 `AudioLoopDesc` 构造 `entries`，依赖 FMOD 创建事件实例）。
use core::time::Duration;

/// `start` 的"未设置"哨兵（O：`nanos == 0x3b9aca00`）。
pub const UNSET_NANOS: u32 = 1_000_000_000;
/// 参数名（O：0x142ad2f20）。
pub const LOOP_PARAMETER: &str = "loop";

/// `stop` 的模式参数（O：数字 0）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StopMode {
    Zero,
}

/// 循环里的一个事件实例（元素 0x30 B）。
pub trait LoopEntry {
    type Error;
    /// 判别 == 1（`Active`）。
    fn is_active(&self) -> bool;
    fn set_parameter(&mut self, name: &str, value: f32);
    fn start(&mut self) -> Result<(), Self::Error>;
    fn stop(&mut self, mode: StopMode) -> Result<(), Self::Error>;
}

/// `start` 时间戳（原版 `Instant`，这里用相对某个起点的 `Duration`，`None` = 未设置）。
pub type Timestamp = Option<Duration>;

/// 循环播放器。
#[derive(Clone, Debug)]
pub struct AudioLoop<E> {
    pub entries: Vec<E>,
    pub start: Timestamp,
    pub cursor: usize,
    pub threshold_ms: u32,
    /// `+0x34`。
    pub started: bool,
    /// `+0x35`。
    pub uses_loop_parameter: bool,
    /// `+0x36`。
    pub stops_explicitly: bool,
}

impl<E: LoopEntry> AudioLoop<E> {
    pub fn new(
        entries: Vec<E>,
        threshold_ms: u32,
        uses_loop_parameter: bool,
        stops_explicitly: bool,
    ) -> Self {
        Self {
            entries,
            start: None,
            cursor: 0,
            threshold_ms,
            started: false,
            uses_loop_parameter,
            stops_explicitly,
        }
    }

    /// 0x140a593f0。
    pub fn current(&self) -> &E {
        &self.entries[self.cursor]
    }

    /// 0x140a596b0。
    pub fn start_if_not_playing(&mut self) -> Result<(), E::Error> {
        self.start = None;
        if self.started {
            return Ok(());
        }
        if self.uses_loop_parameter {
            self.entries[self.cursor].set_parameter(LOOP_PARAMETER, 1.0);
        }
        let entry = &mut self.entries[self.cursor];
        if entry.is_active() {
            entry.start()?;
        }
        self.started = true;
        Ok(())
    }

    /// 0x140a59430。`now` 是当前时间。
    pub fn stop(&mut self, now: Duration) -> Result<(), E::Error> {
        let Some(start) = self.start else {
            self.start = Some(now);
            return Ok(());
        };
        let elapsed = now.saturating_sub(start);
        let elapsed_ms = (elapsed.as_secs() as u32)
            .wrapping_mul(1000)
            .wrapping_add(elapsed.subsec_millis());
        if self.threshold_ms >= elapsed_ms {
            return Ok(());
        }
        let stops_explicitly = self.stops_explicitly;
        let entry = &mut self.entries[self.cursor];
        if self.uses_loop_parameter {
            entry.set_parameter(LOOP_PARAMETER, 0.0);
            if stops_explicitly && entry.is_active() {
                entry.stop(StopMode::Zero)?;
            }
        } else if entry.is_active() {
            entry.stop(StopMode::Zero)?;
        }
        self.cursor = (self.cursor + 1) % self.entries.len();
        self.started = false;
        self.start = None;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct Fake {
        active: bool,
        fail_stop: bool,
        log: Vec<String>,
    }

    impl LoopEntry for Fake {
        type Error = &'static str;
        fn is_active(&self) -> bool {
            self.active
        }
        fn set_parameter(&mut self, name: &str, value: f32) {
            self.log.push(format!("{name}={value}"));
        }
        fn start(&mut self) -> Result<(), Self::Error> {
            self.log.push("start".into());
            Ok(())
        }
        fn stop(&mut self, _mode: StopMode) -> Result<(), Self::Error> {
            self.log.push("stop".into());
            if self.fail_stop { Err("stop") } else { Ok(()) }
        }
    }

    fn entry(active: bool) -> Fake {
        Fake {
            active,
            ..Default::default()
        }
    }

    fn ms(n: u64) -> Duration {
        Duration::from_millis(n)
    }

    #[test]
    fn start_sets_the_parameter_starts_active_entries_and_is_idempotent() {
        let mut l = AudioLoop::new(vec![entry(true)], 100, true, false);
        l.start_if_not_playing().unwrap();
        assert_eq!(l.entries[0].log, ["loop=1", "start"]);
        assert!(l.started);
        l.start_if_not_playing().unwrap();
        assert_eq!(l.entries[0].log.len(), 2);
    }

    #[test]
    fn inactive_entries_are_not_started_but_the_flag_is_still_set() {
        let mut l = AudioLoop::new(vec![entry(false)], 100, false, false);
        l.start_if_not_playing().unwrap();
        assert!(l.entries[0].log.is_empty());
        assert!(l.started);
    }

    #[test]
    fn start_clears_a_pending_stop_timestamp() {
        let mut l = AudioLoop::new(vec![entry(true)], 100, false, false);
        l.start = Some(ms(5));
        l.start_if_not_playing().unwrap();
        assert_eq!(l.start, None);
    }

    #[test]
    fn first_stop_only_records_the_time() {
        let mut l = AudioLoop::new(vec![entry(true), entry(true)], 100, true, true);
        l.started = true;
        l.stop(ms(1000)).unwrap();
        assert_eq!(l.start, Some(ms(1000)));
        assert!(l.started);
        assert!(l.entries[0].log.is_empty());
    }

    #[test]
    fn stop_waits_until_the_threshold_is_exceeded() {
        let mut l = AudioLoop::new(vec![entry(true)], 100, false, false);
        l.started = true;
        l.stop(ms(0)).unwrap();
        l.stop(ms(100)).unwrap();
        assert!(l.started, "100 ms is not greater than the 100 ms threshold");
        l.stop(ms(101)).unwrap();
        assert!(!l.started);
        assert_eq!(l.entries[0].log, ["stop"]);
    }

    #[test]
    fn stop_with_loop_parameter_stops_the_instance_only_when_asked_to() {
        let mut soft = AudioLoop::new(vec![entry(true)], 0, true, false);
        soft.stop(ms(0)).unwrap();
        soft.stop(ms(10)).unwrap();
        assert_eq!(soft.entries[0].log, ["loop=0"]);
        let mut hard = AudioLoop::new(vec![entry(true)], 0, true, true);
        hard.stop(ms(0)).unwrap();
        hard.stop(ms(10)).unwrap();
        assert_eq!(hard.entries[0].log, ["loop=0", "stop"]);
    }

    #[test]
    fn stop_advances_the_cursor_around_the_ring_and_unsets_the_timestamp() {
        let mut l = AudioLoop::new(
            vec![entry(false), entry(false), entry(false)],
            0,
            false,
            false,
        );
        for round in 0..3 {
            l.stop(ms(0)).unwrap();
            l.stop(ms(10)).unwrap();
            assert_eq!(l.cursor, (round + 1) % 3);
            assert_eq!(l.start, None);
        }
    }

    #[test]
    fn stop_failure_leaves_the_cursor_and_flags_untouched() {
        let mut l = AudioLoop::new(
            vec![Fake {
                active: true,
                fail_stop: true,
                ..Default::default()
            }],
            0,
            false,
            false,
        );
        l.started = true;
        l.stop(ms(0)).unwrap();
        assert_eq!(l.stop(ms(10)), Err("stop"));
        assert!(l.started);
        assert_eq!(l.cursor, 0);
    }

    #[test]
    fn elapsed_milliseconds_wrap_like_the_original_u32_arithmetic() {
        // 4_294_968 s * 1000 ms = 4_294_968_000, which wraps to 704 in u32 (a real elapsed time of ~49 days).
        let wrapped = (4_294_968u64 as u32).wrapping_mul(1000);
        assert_eq!(wrapped, 704);
        let mut below = AudioLoop::new(vec![entry(false), entry(false)], 1000, false, false);
        below.start = Some(Duration::ZERO);
        below.stop(Duration::new(4_294_968, 0)).unwrap();
        assert_eq!(
            below.cursor, 0,
            "wrapped 704 ms <= 1000 ms threshold: early return"
        );
        let mut above = AudioLoop::new(vec![entry(false), entry(false)], 500, false, false);
        above.start = Some(Duration::ZERO);
        above.stop(Duration::new(4_294_968, 0)).unwrap();
        assert_eq!(
            above.cursor, 1,
            "wrapped 704 ms > 500 ms threshold: the ring advances"
        );
    }

    #[test]
    #[should_panic]
    fn current_out_of_bounds_panics() {
        let mut l = AudioLoop::new(vec![entry(true)], 0, false, false);
        l.cursor = 1;
        let _ = l.current();
    }

    #[test]
    fn parameter_name_is_loop() {
        assert_eq!(LOOP_PARAMETER, "loop");
        assert_eq!(UNSET_NANOS, 0x3b9aca00);
    }
}
