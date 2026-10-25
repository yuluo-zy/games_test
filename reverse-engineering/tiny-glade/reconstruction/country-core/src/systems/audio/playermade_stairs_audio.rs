//! systems/audio/playermade_stairs_audio.rs — 玩家楼梯音频（PlayStairsAudioCmd）。
//!
//! 反汇编证据（O，audio-events-analysis §5）：
//! - `stairs_audio_init_system`（0x140f878d0）：6×`AudioCore::instantiate_loop`
//!   `event:/SFX/Stairs/{StairMove,StairChangeHeight,StairChangeWidth,LadderMove,
//!   LadderChangeHeight,LadderChangeWidth}`；`insert_resource<StairsAudioState>`。
//! - `stairs_audio_system`（0x140f880a0）：帧首 6 loop stop；`PlayStairsAudioCmd`（stride 0x38）
//!   kind≤18 跳表；瞬发串 `event:/SFX/Stairs/{StairPlace,StairDelete,StairsMerge,
//!   StairsMergeConfirm,LadderAppear,LadderPlace,StepsAppear,WalkwayAppear,StairSupportAppear,
//!   StairPillarAppear}`；参数 `stair_width size editing_speed extend_amount h_offset
//!   avg_arch_supports height_above_ground in_water stair_edge_length is_stub_preview move_all`。

#![allow(dead_code, unused_imports)]

use bevy_ecs::prelude::{EventReader, Res, ResMut};
use bevy_ecs::resource::Resource;

use crate::audio_events::PlayStairsAudioCmd;
use crate::resources::audio_resources::AudioCore;
use crate::typed_gap::Gap;
use crate::utils::audio_utils::{AudioLoop, AudioLoopDesc};

/// `StairsAudioState`（O 名；6×AudioLoop stride 0x38：+0/+0x38/+0x70/+0xa8/+0xe0/+0x118）。
#[derive(Resource, Debug, Default)]
pub struct StairsAudioState {
    /// 6 个循环 loop（O 数量；布局 U）。
    pub loops: Vec<AudioLoop>,
    /// 其余（U）。
    pub _gap: Gap,
}

/// `stairs_audio_init_system`（0x140f878d0，O）。
pub fn stairs_audio_init_system(core: Res<AudioCore>, mut commands: bevy_ecs::system::Commands) {
    let names: [&'static str; 6] = [
        "event:/SFX/Stairs/StairMove",
        "event:/SFX/Stairs/StairChangeHeight",
        "event:/SFX/Stairs/StairChangeWidth",
        "event:/SFX/Stairs/LadderMove",
        "event:/SFX/Stairs/LadderChangeHeight",
        "event:/SFX/Stairs/LadderChangeWidth",
    ];
    let mut loops = Vec::new();
    for name in names {
        let desc = AudioLoopDesc::new(name, Some("editing_speed"), 3, 0x32);
        if let Ok(loop_) = core.instantiate_loop(&desc) {
            loops.push(loop_);
        }
    }
    commands.insert_resource(StairsAudioState { loops, _gap: Gap });
}

/// `stairs_audio_system`（0x140f880a0，O）。
pub fn stairs_audio_system(
    mut _state: ResMut<StairsAudioState>,
    mut _events: EventReader<PlayStairsAudioCmd>,
) {
    // 帧首 6 loop stop（O）；kind 跳表 ≤18：瞬发（instantiate_audio + set_position +
    // set_parameter(stair_width/size/…) + start_if_not_playing）或循环（start_if_not_playing）。
}
