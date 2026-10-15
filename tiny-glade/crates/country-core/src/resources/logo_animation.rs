//! `logo_animation.rs` — 启动 logo 的五阶段进度（B `resources::logo_animation`）。
//!
//! 证据（RNE 汇编 + `session-save/decomp/decomp-0x140b1a6b0.txt`）：
//! - 布局：`progress[5]` f32 @+0x00..0x14、`durations[5]` f32 @+0x14..0x28（O；此前合同的
//!   `_f0/_f1/wait_progress/_f2/_stages` 布局不符）。
//! - `build` 0x140b19d40：`progress` 全 0，`durations = [1.15, 0.15, 0.65, 1.0e10, 0.5]`
//!   （O：常量 0x142ae8fa0 的 4 个 f32 + `+0x24 = 0.5`）。阶段 3 的 `1e10` 是"等待加载"，只有 `finish_wait` 能结束它。
//! - `play` 0x140b19d00：`progress` 全清零（名字来自 PDB，I）。
//! - `is_waiting` 0x140b19d10 = `progress[3] > 0.0`；`is_finished` 0x140b19d20 = `progress[4] >= 1.0`（1.0 @0x142925904）；
//!   `finish_wait` 0x140b19d30 = `progress[3] = 1.0`。
//! - `animate_logo` 0x140b1a6b0：取第一个 `progress[i] < 1.0` 的阶段（NaN 也算未完成），
//!   `progress[i] = clamp_below_1(max(f32::MIN_POSITIVE, dt / durations[i] + progress[i]))`
//!   （下界常量 0x142a10904 = `f32::MIN_POSITIVE`，所以首帧之后进度不再恰为 0）；
//!   阶段 0 与 2 在进度**恰为 0.0** 时各播一次音效。
//!
//! 未做：音效引擎调用（返回 [`LogoAudioEvent`]，由调用方播放）、`startup_animated_logo` 的资源注册（0x140b19d90）。

/// 阶段数。
pub const LOGO_STAGES: usize = 5;
/// `build` 写入的各阶段时长（秒）。
pub const DEFAULT_DURATIONS: [f32; LOGO_STAGES] = [1.15, 0.15, 0.65, 1.0e10, 0.5];
/// "等待加载"阶段（`is_waiting`/`finish_wait` 读写的槽）。
pub const WAIT_STAGE: usize = 3;

/// 阶段起点触发的 FMOD 事件（O：串长均为 0x17）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LogoAudioEvent {
    /// 阶段 0：`event:/UI/Logo/Scribble`。
    Scribble,
    /// 阶段 2：`event:/UI/Logo/Branches`。
    Branches,
}

impl LogoAudioEvent {
    pub fn path(self) -> &'static str {
        match self {
            Self::Scribble => "event:/UI/Logo/Scribble",
            Self::Branches => "event:/UI/Logo/Branches",
        }
    }
}

/// logo 动画资源。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LogoAnimation {
    pub progress: [f32; LOGO_STAGES],
    pub durations: [f32; LOGO_STAGES],
}

impl Default for LogoAnimation {
    /// `build`（0x140b19d40）。
    fn default() -> Self {
        Self {
            progress: [0.0; LOGO_STAGES],
            durations: DEFAULT_DURATIONS,
        }
    }
}

impl LogoAnimation {
    /// 0x140b19d00：进度全部清零。
    pub fn play(&mut self) {
        self.progress = [0.0; LOGO_STAGES];
    }

    /// 0x140b19d10。
    pub fn is_waiting(&self) -> bool {
        self.progress[WAIT_STAGE] > 0.0
    }

    /// 0x140b19d20。
    pub fn is_finished(&self) -> bool {
        self.progress[LOGO_STAGES - 1] >= 1.0
    }

    /// 0x140b19d30。
    pub fn finish_wait(&mut self) {
        self.progress[WAIT_STAGE] = 1.0;
    }

    /// `animate_logo`（0x140b1a6b0）。`dt` 是帧增量（O：`[time + 0x58]`）。
    /// 返回本帧需要播放的音效（阶段 0/2 进度恰为 0.0 时）。
    pub fn animate(&mut self, dt: f32) -> Option<LogoAudioEvent> {
        let stage = self.progress.iter().position(|p| p.is_nan() || *p < 1.0)?;
        let current = self.progress[stage];
        let event = if current == 0.0 {
            match stage {
                0 => Some(LogoAudioEvent::Scribble),
                2 => Some(LogoAudioEvent::Branches),
                _ => None,
            }
        } else {
            None
        };
        let advanced = dt / self.durations[stage] + current;
        let floored = if f32::MIN_POSITIVE <= advanced {
            advanced
        } else {
            f32::MIN_POSITIVE
        };
        self.progress[stage] = if floored <= 1.0 { floored } else { 1.0 };
        event
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_matches_build_constants() {
        let logo = LogoAnimation::default();
        assert_eq!(logo.progress, [0.0; 5]);
        assert_eq!(logo.durations, [1.15, 0.15, 0.65, 1.0e10, 0.5]);
        assert!(!logo.is_waiting());
        assert!(!logo.is_finished());
    }

    #[test]
    fn first_frame_plays_scribble_exactly_once() {
        let mut logo = LogoAnimation::default();
        assert_eq!(logo.animate(0.1), Some(LogoAudioEvent::Scribble));
        assert_eq!(logo.animate(0.1), None);
        assert!((logo.progress[0] - 0.2 / 1.15).abs() < 1e-6);
    }

    #[test]
    fn zero_dt_still_leaves_the_floor_value_so_audio_is_not_repeated() {
        let mut logo = LogoAnimation::default();
        assert_eq!(logo.animate(0.0), Some(LogoAudioEvent::Scribble));
        assert_eq!(logo.progress[0], f32::MIN_POSITIVE);
        assert_eq!(logo.animate(0.0), None);
    }

    #[test]
    fn stages_run_in_order_and_branches_plays_at_stage_two() {
        let mut logo = LogoAnimation::default();
        logo.progress[0] = 1.0;
        logo.progress[1] = 1.0;
        assert_eq!(logo.animate(0.1), Some(LogoAudioEvent::Branches));
        assert_eq!(logo.animate(10.0), None);
        assert_eq!(logo.progress[2], 1.0);
        assert_eq!(logo.progress[3], 0.0);
    }

    #[test]
    fn wait_stage_only_ends_through_finish_wait() {
        let mut logo = LogoAnimation::default();
        logo.progress[..3].fill(1.0);
        logo.animate(1000.0);
        assert!(logo.is_waiting());
        assert!(logo.progress[3] < 1.0);
        logo.finish_wait();
        assert_eq!(logo.animate(1.0), None);
        assert_eq!(logo.progress[4], 1.0);
        assert!(logo.is_finished());
        assert_eq!(logo.animate(1.0), None);
    }

    #[test]
    fn nan_progress_counts_as_unfinished_and_recovers_to_the_floor() {
        let mut logo = LogoAnimation::default();
        logo.progress[0] = f32::NAN;
        logo.animate(0.1);
        assert_eq!(logo.progress[0], f32::MIN_POSITIVE);
    }

    #[test]
    fn play_resets_every_stage() {
        let mut logo = LogoAnimation {
            progress: [1.0, 1.0, 1.0, 0.5, 0.2],
            ..Default::default()
        };
        logo.play();
        assert_eq!(logo.progress, [0.0; 5]);
    }

    #[test]
    fn audio_paths_have_the_original_length() {
        assert_eq!(LogoAudioEvent::Scribble.path().len(), 0x17);
        assert_eq!(LogoAudioEvent::Branches.path().len(), 0x17);
    }
}
