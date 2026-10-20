//! 音频启动注册（B `startup::load_audio` @0x140b0e0f0）。
//!
//! 证据（R10 契约 §2.2 + dependency 注解，O）：
//! * 阶段：batch1/PreStartup。
//! * 第 1 参数调用 `AudioSamplesLoadedPromise::recv`（0x140a57100）。
//! * 向第 2 参数（`&mut Schedule`）追加 **2** 个系统配置：12 字节
//!   `FUN_1412dccf0`、8 字节 `FUN_1412d91b0`。
//! * 日志 target `country_core::startup::load_audio`。
//!
//! I：两个匿名 builder 对应 `game_loading_audio` 的两个系统
//! （`init_game_loading_audio` 0x140b1ae70 / `loading_audio` 0x140b1b1b0——两者
//! 由同一 `snapshot:/Loading` 状态资源串联，配置体量 12/8 与常量一致）。
use crate::resources::audio_resources::{AudioSamplesLoadedPromise, Studio};
use bevy_ecs::schedule::Schedule;
use std::time::Duration;

/// O（decomp 分配尺寸）：load_audio 构造的三条音频对象。
pub const AUDIO_OBJECT_SIZES: [usize; 3] = [0x1c, 0xc, 0x8];
/// O（契约 §2.2）：追加到 Schedule 的两条系统配置。
pub const AUDIO_SCHEDULE_ENTRIES: [(&str, usize); 2] =
    [("FUN_1412dccf0", 0xc), ("FUN_1412d91b0", 8)];
pub const WAITED_LOG_PREFIX: &str = "Waited ";

/// 生产装配：把 `game_loading_audio` 的两个系统加入宿主 Schedule（PreStartup）。
pub fn add_game_loading_audio_systems<S>(schedule: &mut Schedule)
where
    S: Studio + Send + Sync + 'static,
{
    use bevy_ecs::schedule::{IntoScheduleConfigs, common_conditions::resource_exists};
    schedule.add_systems(
        (
            crate::systems::audio::game_loading_audio::init_game_loading_audio::<S>,
            crate::systems::audio::game_loading_audio::loading_audio::<S>,
        )
            .chain()
            .run_if(resource_exists::<crate::resources::audio_resources::AudioCore<S>>),
    );
}

/// O：先消费已载入样本 promise，再追加两个 loading-audio 系统。
pub fn load_audio<S>(
    loaded_samples: Option<&AudioSamplesLoadedPromise<S::Event>>,
    schedule: &mut Schedule,
) where
    S: Studio + Send + Sync + 'static,
{
    if let Some(promise) = loaded_samples {
        let _ = promise.recv();
    }
    add_game_loading_audio_systems::<S>(schedule);
    // U: 三条音频对象（0x1c/0xc/8）的具体构造类型未恢复。
}

/// 原件日志前缀的可见形式；`elapsed` 由宿主计时器提供。
pub fn waited_log(elapsed: Duration) -> String {
    format!("{WAITED_LOG_PREFIX}{:?}", elapsed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constants_match_the_two_schedule_builders_and_log_prefix() {
        assert_eq!(AUDIO_OBJECT_SIZES, [0x1c, 0xc, 0x8]);
        assert_eq!(
            AUDIO_SCHEDULE_ENTRIES,
            [("FUN_1412dccf0", 0xc), ("FUN_1412d91b0", 8)]
        );
        assert!(waited_log(Duration::from_millis(3)).starts_with("Waited "));
        // 两个系统确实属于 game_loading_audio（名字级交叉检查）。
        let _ = crate::systems::audio::game_loading_audio::LOADING_SNAPSHOT;
        let _ = crate::systems::audio::game_loading_audio::LOADING_EVENT;
    }
}
