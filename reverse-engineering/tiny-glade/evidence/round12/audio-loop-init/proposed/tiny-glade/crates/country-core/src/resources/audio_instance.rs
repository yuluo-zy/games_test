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

/// Original player owns one complete AudioLoop plus its should-play byte.
/// Ring timing, parameter detection, current index and entries have one owner.
#[derive(Clone, Debug)]
pub struct AudioLoopPlayer<I> {
    pub audio: crate::utils::audio_utils::AudioLoop<I>,
    pub should_play: bool,
}
impl<I> AudioLoopPlayer<I> {
    pub fn from_audio(audio: crate::utils::audio_utils::AudioLoop<I>) -> Self {
        Self { audio, should_play: false }
    }
}
impl<I> AudioLoopPlayer<I>
where I: LoopInstance + crate::utils::audio_utils::LoopEntry<Error=<I as LoopInstance>::Error> {
    /// Explicit low-level host adapter. Game startup uses from_audio on the
    /// fully initialized canonical producer, rather than discarding its state.
    pub fn new(instances: Vec<I>) -> Self {
        Self::from_audio(crate::utils::audio_utils::AudioLoop::new(instances,0,false,false))
    }
    pub fn current(&self) -> &I { self.audio.current() }
    pub fn request_play(&mut self, position:[f32;3], volume:f32, editing_speed:f32) -> Result<(), <I as LoopInstance>::Error> {
        self.should_play = true;
        let instance = self.audio.current_mut();
        LoopInstance::set_position(instance,position);
        LoopInstance::set_parameter(instance,EDITING_SPEED_PARAMETER,editing_speed);
        if LoopInstance::is_active(instance) { LoopInstance::set_volume(instance,volume)?; }
        Ok(())
    }
    fn frame_end_with_stop(&mut self, stop: impl FnOnce(&mut crate::utils::audio_utils::AudioLoop<I>) -> Result<(),<I as LoopInstance>::Error>) -> Result<(),<I as LoopInstance>::Error> {
        let play = std::mem::take(&mut self.should_play);
        if play { self.audio.start_if_not_playing() } else { stop(&mut self.audio) }
    }
    pub fn frame_end(&mut self) -> Result<(),<I as LoopInstance>::Error> {
        self.frame_end_with_stop(crate::utils::audio_utils::AudioLoop::stop_now)
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

    impl crate::utils::audio_utils::LoopEntry for Fake {
        type Error = &'static str;
        fn is_active(&self) -> bool { LoopInstance::is_active(self) }
        fn set_parameter(&mut self,name:&str,value:f32) { LoopInstance::set_parameter(self,name,value); }
        fn start(&mut self) -> Result<(),Self::Error> { LoopInstance::start(self) }
        fn stop(&mut self,_:crate::utils::audio_utils::StopMode) -> Result<(),Self::Error> { LoopInstance::stop(self) }
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
            player.audio.entries[0].calls,
            ["position", "editing_speed=2", "volume=0.5"]
        );
    }

    #[test]
    fn inactive_instance_skips_the_volume_call() {
        let mut player = AudioLoopPlayer::new(vec![Fake::default()]);
        player.request_play([1.0, 2.0, 3.0], 0.5, 1.0).unwrap();
        assert_eq!(player.audio.entries[0].calls, ["position", "editing_speed=1"]);
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
    fn player_retains_native_delayed_stop_and_rotation_from_its_ring() {
        use core::time::Duration;
        let audio = crate::utils::audio_utils::AudioLoop::new(vec![Fake {active:true,..Default::default()},Fake::default()],50,false,false);
        let mut player = AudioLoopPlayer::from_audio(audio);
        player.request_play([0.;3],1.,1.).unwrap();
        player.frame_end_with_stop(|ring|ring.stop(Duration::ZERO)).unwrap();
        assert!(player.audio.started);
        assert_eq!(player.audio.entries[0].calls,["position","editing_speed=1","volume=1","start"]);
        player.frame_end_with_stop(|ring|ring.stop(Duration::ZERO)).unwrap();
        player.frame_end_with_stop(|ring|ring.stop(Duration::from_millis(50))).unwrap();
        assert!(player.audio.started);
        player.frame_end_with_stop(|ring|ring.stop(Duration::from_millis(51))).unwrap();
        assert!(!player.audio.started);
        assert_eq!(player.audio.cursor,1);
        assert_eq!(player.audio.entries[0].calls.last().unwrap(),"stop");
        assert!(!player.should_play);
    }

    #[test]
    #[should_panic]
    fn current_out_of_bounds_panics_like_the_original() {
        let mut player = AudioLoopPlayer::new(vec![Fake::default()]);
        player.audio.cursor = 1;
        let _ = player.request_play([0.0, 0.0, 0.0], 1.0, 1.0);
    }

    #[test]
    fn parameter_name_has_the_original_length() {
        assert_eq!(EDITING_SPEED_PARAMETER.len(), 0xd);
    }
}
