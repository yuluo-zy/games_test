//! `audio_instance.rs` — 音频实例的已恢复判定与循环播放器（B `resources::audio_resources` / `wall_audio_utils`）。
//!
//! 证据：
//! - `start_if_not_playing` 0x140a57300（汇编）：实例为 `Active` 时取 FMOD 播放状态，`bt 0xb, state; jb 跳过`，
//!   即状态属于 `{0 Playing, 1 Sustaining, 3 Starting}` 时不启动，其余（`Stopped`/`Stopping`）才启动。
//!   `bt` 以 32 位取模，状态值按 `& 31` 位移；
//! - `AudioLoopPlayer::request_play` 0x1409285a0（反编译）：先置 `should_play = true`；`current < instances.len()`
//!   否则越界 panic；对 `instances[current]` 依次 `set_position`、`set_parameter("editing_speed", 参数)`，
//!   实例为 `Active` 时再 `set_volume(音量)`，失败 → panic。实例步长是 **0x30**（此前文档写 0x40）。
//!
//! FMOD 调用经 [`LoopInstance`] 注入，不在这里重造 FMOD 绑定。

/// FMOD `EventInstance::get_playback_state` 的取值（FMOD Studio 的枚举序）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PlaybackState {
    Playing = 0,
    Sustaining = 1,
    Stopped = 2,
    Starting = 3,
    Stopping = 4,
}

/// `start_if_not_playing` 的判定（O：`(0xb >> (state & 31)) & 1 == 0`）。
pub fn should_start_if_not_playing(state: u32) -> bool {
    (0xb_u32 >> (state & 31)) & 1 == 0
}

impl PlaybackState {
    pub fn should_start(self) -> bool {
        should_start_if_not_playing(self as u32)
    }
}

/// 循环播放器用到的实例接口（wall_audio_utils `AudioLoopPlayer` 的注入面）。
pub trait LoopInstance {
    type Error;
    /// 实例是否为 `Active`（判别 == 1）；`Disabled`/`Err` 时跳过 `set_volume`。
    fn is_active(&self) -> bool;
    /// `set_position`（0x140a57da0）：3D 属性，forward=(0,0,-1)/up=(0,1,0)。
    fn set_position(&mut self, position: [f32; 3]);
    /// `set_parameter`（0x140a57670）：实例参数面。
    fn set_parameter(&mut self, name: &str, value: f32);
    fn set_volume(&mut self, volume: f32) -> Result<(), Self::Error>;
    /// 帧尾 `start_if_not_playing`（0x140a596b0 的实例面）：当前实例启动。
    fn start(&mut self) -> Result<(), Self::Error>;
    /// 帧尾 `stop`（0x140a59430 的实例面）：当前实例立即停止。
    fn stop(&mut self) -> Result<(), Self::Error>;
}

/// 参数名（O：0x142aaf450，长 0xd）。
pub const EDITING_SPEED_PARAMETER: &str = "editing_speed";

/// 循环播放器：一组实例加当前下标与"应播放"标志（O：`current usize @+0x28`、`should_play u8 @+0x38`）。
#[derive(Clone, Debug, Default)]
pub struct AudioLoopPlayer<I> {
    pub instances: Vec<I>,
    pub current: usize,
    pub should_play: bool,
}

impl<I: LoopInstance> AudioLoopPlayer<I> {
    pub fn new(instances: Vec<I>) -> Self {
        Self {
            instances,
            current: 0,
            should_play: false,
        }
    }

    /// 当前实例（`AudioLoopPlayer::resolve`，0x140a592xx 语义；越界 panic 同原版）。
    pub fn current(&self) -> &I {
        &self.instances[self.current]
    }

    /// `request_play`（0x1409285a0）。`current` 越界与 `set_volume` 失败对应原版 panic：越界 panic，音量失败以 `Err` 返回。
    pub fn request_play(
        &mut self,
        position: [f32; 3],
        volume: f32,
        editing_speed: f32,
    ) -> Result<(), I::Error> {
        self.should_play = true;
        let instance = &mut self.instances[self.current];
        instance.set_position(position);
        instance.set_parameter(EDITING_SPEED_PARAMETER, editing_speed);
        if instance.is_active() {
            instance.set_volume(volume)?;
        }
        Ok(())
    }

    /// 帧尾：按 `should_play` 标志对当前实例 `start`/`stop` 并清标志
    /// （wall_audio_fmod/building/stair 帧尾，0x140a596b0/0x140a59430 的驱动侧）。
    pub fn frame_end(&mut self) -> Result<(), I::Error> {
        let play = self.should_play;
        self.should_play = false;
        let instance = &mut self.instances[self.current];
        if play {
            instance.start()?;
        } else {
            instance.stop()?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct Fake {
        active: bool,
        fail_volume: bool,
        calls: Vec<String>,
    }

    impl LoopInstance for Fake {
        type Error = &'static str;
        fn is_active(&self) -> bool {
            self.active
        }
        fn set_position(&mut self, _position: [f32; 3]) {
            self.calls.push("position".into());
        }
        fn set_parameter(&mut self, name: &str, value: f32) {
            self.calls.push(format!("{name}={value}"));
        }
        fn set_volume(&mut self, volume: f32) -> Result<(), Self::Error> {
            self.calls.push(format!("volume={volume}"));
            if self.fail_volume {
                Err("volume")
            } else {
                Ok(())
            }
        }
        fn start(&mut self) -> Result<(), Self::Error> {
            self.calls.push("start".into());
            Ok(())
        }
        fn stop(&mut self) -> Result<(), Self::Error> {
            self.calls.push("stop".into());
            Ok(())
        }
    }

    #[test]
    fn start_is_skipped_for_playing_sustaining_and_starting() {
        assert!(!PlaybackState::Playing.should_start());
        assert!(!PlaybackState::Sustaining.should_start());
        assert!(!PlaybackState::Starting.should_start());
        assert!(PlaybackState::Stopped.should_start());
        assert!(PlaybackState::Stopping.should_start());
    }

    #[test]
    fn playback_state_bit_test_wraps_modulo_32() {
        assert!(!should_start_if_not_playing(32));
        assert!(!should_start_if_not_playing(35));
        assert!(should_start_if_not_playing(34));
    }

    #[test]
    fn request_play_sets_position_parameter_then_volume() {
        let mut player = AudioLoopPlayer::new(vec![Fake {
            active: true,
            ..Default::default()
        }]);
        player.request_play([1.0, 2.0, 3.0], 0.5, 2.0).unwrap();
        assert!(player.should_play);
        assert_eq!(
            player.instances[0].calls,
            ["position", "editing_speed=2", "volume=0.5"]
        );
    }

    #[test]
    fn inactive_instance_skips_the_volume_call() {
        let mut player = AudioLoopPlayer::new(vec![Fake::default()]);
        player.request_play([1.0, 2.0, 3.0], 0.5, 1.0).unwrap();
        assert_eq!(player.instances[0].calls, ["position", "editing_speed=1"]);
    }

    #[test]
    fn volume_failure_is_reported_after_the_flag_is_set() {
        let mut player = AudioLoopPlayer::new(vec![Fake {
            active: true,
            fail_volume: true,
            ..Default::default()
        }]);
        assert_eq!(
            player.request_play([1.0, 2.0, 3.0], 1.0, 1.0),
            Err("volume")
        );
        assert!(player.should_play);
    }

    #[test]
    fn frame_end_starts_when_should_play_and_stops_otherwise() {
        let mut player = AudioLoopPlayer::new(vec![Fake::default()]);
        player.frame_end().unwrap();
        assert_eq!(player.instances[0].calls, ["stop"]);
        player.request_play([0.0, 0.0, 0.0], 1.0, 1.0).unwrap();
        player.frame_end().unwrap();
        assert_eq!(
            player.instances[0].calls,
            ["stop", "position", "editing_speed=1", "start"]
        );
        assert!(!player.should_play);
    }

    #[test]
    #[should_panic]
    fn current_out_of_bounds_panics_like_the_original() {
        let mut player = AudioLoopPlayer::new(vec![Fake::default()]);
        player.current = 1;
        let _ = player.request_play([0.0, 0.0, 0.0], 1.0, 1.0);
    }

    #[test]
    fn parameter_name_has_the_original_length() {
        assert_eq!(EDITING_SPEED_PARAMETER.len(), 0xd);
    }
}
