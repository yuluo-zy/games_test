//! 环形音效播放器：持续播放请求取消停止计时，停止成功后再轮换实例。
//!
//! 通过 LoopEntry 适配实际音频后端；计时使用 Duration，不把真实时间压缩为 u32 毫秒。
use core::time::Duration;
use std::{sync::OnceLock, time::Instant};

/// 循环事件使用的音频参数名。
pub const LOOP_PARAMETER: &str = "loop";

/// 后端的停止策略。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StopMode {
    /// 允许音效自然淡出。
    AllowFadeOut,
}

/// 音频后端适配接口；实例创建由 AudioCore 负责。
pub trait LoopEntry {
    type Error;
    fn is_active(&self) -> bool;
    fn set_parameter(&mut self, name: &str, value: f32);
    fn start(&mut self) -> Result<(), Self::Error>;
    fn stop(&mut self, mode: StopMode) -> Result<(), Self::Error>;
}

/// 已有系统按字段访问实例；保留字段名以兼容调用方。
#[derive(Clone, Debug)]
pub struct AudioLoop<E> {
    /// 非空的循环实例队列。
    pub entries: Vec<E>,
    /// 首次停止请求的时间，None 表示尚未请求。
    pub start: Option<Duration>,
    /// 当前实例在队列中的索引。
    pub cursor: usize,
    /// 连续没有播放请求多久之后允许停止，单位毫秒。
    pub threshold_ms: u32,
    pub started: bool,
    pub uses_loop_parameter: bool,
    pub stops_explicitly: bool,
}

impl<E: LoopEntry> AudioLoop<E> {
    pub fn new(
        entries: Vec<E>,
        threshold_ms: u32,
        uses_loop_parameter: bool,
        stops_explicitly: bool,
    ) -> Self {
        assert!(!entries.is_empty(), "音效循环至少需要一个实例");
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

    /// Original noloop adaptor sets AudioLoop+36. Keep native parameter
    /// detection; this additionally requests explicit stop when a loop exists.
    pub fn noloop(mut self) -> Self {
        self.stops_explicitly = true;
        self
    }

    /// 调用方必须维持非空实例队列及有效 cursor。
    pub fn current(&self) -> &E {
        &self.entries[self.cursor]
    }

    pub fn current_mut(&mut self) -> &mut E {
        &mut self.entries[self.cursor]
    }

    /// 取消待停止请求；后端失败时不把实例标记为已启动，允许下次重试。
    pub fn start_if_not_playing(&mut self) -> Result<(), E::Error> {
        self.start = None;
        if self.started {
            return Ok(());
        }
        let uses_loop_parameter = self.uses_loop_parameter;
        let entry = self.current_mut();
        if uses_loop_parameter {
            entry.set_parameter(LOOP_PARAMETER, 1.0);
        }
        if entry.is_active() {
            entry.start()?;
        }
        self.started = true;
        Ok(())
    }

    /// 使用单调实时时钟请求停止；显式时钟调用方仍可使用 [`Self::stop`]。
    pub fn stop_now(&mut self) -> Result<(), E::Error> {
        static CLOCK_ORIGIN: OnceLock<Instant> = OnceLock::new();
        self.stop(CLOCK_ORIGIN.get_or_init(Instant::now).elapsed())
    }

    /// 首次调用记录时间，严格超过阈值后停止；时间倒退按零耗时处理。
    pub fn stop(&mut self, now: Duration) -> Result<(), E::Error> {
        let requested_at = *self.start.get_or_insert(now);
        if now.saturating_sub(requested_at) <= Duration::from_millis(u64::from(self.threshold_ms)) {
            return Ok(());
        }
        let uses_loop_parameter = self.uses_loop_parameter;
        let stop_backend = !uses_loop_parameter || self.stops_explicitly;
        let entry = self.current_mut();
        if uses_loop_parameter {
            entry.set_parameter(LOOP_PARAMETER, 0.0);
        }
        if stop_backend && entry.is_active() {
            entry.stop(StopMode::AllowFadeOut)?;
        }
        self.cursor = if self.cursor + 1 == self.entries.len() {
            0
        } else {
            self.cursor + 1
        };
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
    fn live_stop_clock_advances_and_releases_a_started_loop() {
        let mut player = AudioLoop::new(vec![entry(true), entry(true)], 0, false, false);
        player.start_if_not_playing().unwrap();
        player.stop_now().unwrap();
        assert!(player.start.is_some());
        assert!(player.started);
        assert_eq!(player.entries[0].log, ["start"]);

        // A zero delay still requires a later instant, just like stop(now).
        std::thread::sleep(ms(2));
        player.stop_now().unwrap();
        assert_eq!(player.entries[0].log, ["start", "stop"]);
        assert_eq!(player.cursor, 1);
        assert!(!player.started);
        assert_eq!(player.start, None);
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
    fn long_running_audio_does_not_wrap_the_stop_delay() {
        let mut player = AudioLoop::new(vec![entry(false), entry(false)], 1000, false, false);
        player.start = Some(Duration::ZERO);
        player.stop(Duration::new(4_294_968, 0)).unwrap();
        assert_eq!(player.cursor, 1);
        let mut player = AudioLoop::new(vec![entry(false)], 100, false, false);
        player.start = Some(ms(0));
        player.stop(Duration::from_micros(100_001)).unwrap();
        assert_eq!(player.start, None);
    }

    #[test]
    #[should_panic(expected = "音效循环至少需要一个实例")]
    fn empty_audio_rings_are_rejected_at_construction() {
        AudioLoop::<Fake>::new(Vec::new(), 0, false, false);
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
    }
}
