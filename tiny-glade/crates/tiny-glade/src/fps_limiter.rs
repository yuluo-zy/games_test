//! B `crates/tiny-glade/src/fps_limiter.rs` —— 帧率限制器。
//!
//! 恢复依据：fresh 反编译 `tiny_glade::fps_limiter::FpsLimiter::frame`
//! VA `0x140270850`（Ghidra shared，2026-10-17；证据见
//! `recovery/agent-team/work/restorer-batch4-A/0x140270850_frame.c`）。
//!
//! ## 结构布局（O，机器码读写）
//! - `+0x00` `Option<Instant>`：`{ secs: u64 @+0, nanos: u32 @+8 }`，`nanos ==
//!   1_000_000_000` 是 `None` 的 niche（反编译 `cmp [rcx+8], 0x3b9aca00`）。
//! - `+0x10` `f32`：帧预算累计值（反编译 `[rcx+0x10]` 参与加/减/clamp）。
//!
//! ## 控制流（逐分支）
//! ```text
//! now = Instant::now()
//! if 0.0 < target_fps:
//!     profile_scope!("fps limiter")
//!     if last_frame.is_some():
//!         elapsed   = now.duration_since(last)
//!         frame_time = 1.0 / target_fps
//!         used      = min(elapsed.as_secs_f32(), 2 * frame_time)
//!         acc       = acc + frame_time - used
//!         acc       = clamp(acc, -frame_time, frame_time)
//!         if acc > 0.0: spin_sleep::sleep(Duration::from_secs_f32(acc))
//! last_frame = Some(now)      // 即使 target_fps <= 0 也更新（O：函数尾部无条件存储）
//! ```
//!
//! 判等说明：
//! - `min` 用两个比较实现（`elapsed <= 2*ft` 取 elapsed，否则取 `2*ft`），
//!   `f32::min` 的 NaN 选择与之一致（NaN 时取另一侧）。
//! - `clamp` 的原版是两次比较（`-ft <= acc`、`acc <= ft`），对 `ft = +inf`
//!   仍有界；`f32::clamp` 在 `min > max` 时 panic，而这里 `-ft <= ft` 恒成立，
//!   故等价。
//! - 反编译中 `Duration::from_secs_f32` 的 panic 冷路径（0x140270cd4）由
//!   `spin_sleep::sleep(Duration::from_secs_f32(..))` 保留。
//! - `puffin::profile_scope!` 对应 0x1402709xx 的 `puffin::are_scopes_on` +
//!   `ThreadProfiler::begin_scope/end_scope`，作用域名 `"fps limiter"`
//!   （O：0x1429a7d37 起 11 字节）。
//! - `spin_sleep` 在 Windows 上内联了 `timeBeginPeriod/timeEndPeriod`
//!   （O：0x1402708f6/0x140270a25 调 0x140270xxx），直接复用依赖即可。

use std::time::{Duration, Instant};

/// B `tiny_glade::fps_limiter::FpsLimiter`（字段偏移见模块注释）。
#[derive(Debug, Default)]
pub struct FpsLimiter {
    /// `+0x00`，`None` 由 `nanos == 1e9` 表示（std `Option<Instant>` niche）。
    last_frame: Option<Instant>,
    /// `+0x10` f32 帧预算累计。
    accumulated: f32,
}

impl FpsLimiter {
    /// 构造：原版 PDB 未给 `FpsLimiter::new`（该类型由 `Default`/字面量构造）。
    pub fn new() -> Self {
        Self::default()
    }

    /// `FpsLimiter::frame` `0x140270850`（B）。
    ///
    /// `target_fps <= 0.0`（含 NaN：`0.0 < NaN` 为假）时只更新时间戳，
    /// 与反编译 `if (0.0 < xmm1)` 一致。
    pub fn frame(&mut self, target_fps: f32) {
        let now = Instant::now();
        if 0.0 < target_fps {
            puffin::profile_scope!("fps limiter");
            if let Some(last_frame) = self.last_frame {
                let elapsed = now.duration_since(last_frame);
                let frame_time = 1.0 / target_fps;
                let elapsed = elapsed.as_secs_f32();
                let used = if elapsed <= frame_time * 2.0 {
                    elapsed
                } else {
                    frame_time * 2.0
                };
                self.accumulated = (self.accumulated + frame_time) - used;
                self.accumulated = if -frame_time <= self.accumulated {
                    self.accumulated
                } else {
                    -frame_time
                };
                self.accumulated = if self.accumulated <= frame_time {
                    self.accumulated
                } else {
                    frame_time
                };
                if self.accumulated > 0.0 {
                    spin_sleep::sleep(Duration::from_secs_f32(self.accumulated));
                }
            }
        }
        self.last_frame = Some(now);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_frame_only_records_timestamp() {
        let mut limiter = FpsLimiter::new();
        assert!(limiter.last_frame.is_none());
        limiter.frame(60.0);
        assert!(limiter.last_frame.is_some());
        assert_eq!(limiter.accumulated, 0.0);
    }

    #[test]
    fn non_positive_target_still_updates_timestamp() {
        let mut limiter = FpsLimiter::new();
        limiter.frame(0.0);
        assert!(limiter.last_frame.is_some());
        assert_eq!(limiter.accumulated, 0.0);
    }

    #[test]
    fn nan_target_is_ignored_like_non_positive() {
        let mut limiter = FpsLimiter::new();
        limiter.frame(f32::NAN);
        assert!(limiter.last_frame.is_some());
        assert_eq!(limiter.accumulated, 0.0);
    }
}
