//! B `startup/run_game` — 宿主可调的核心启动链（order 8 收口）。
//!
//! 原版 run_game_thread 0x1405a3440（39KB）装配整个游戏线程；本文件给出库内可接的
//! `register_core_startup`（会话/存档/历史/命令器/音频注册），供宿主（tiny-glade）调用。
//! 域系统（墙/屋顶/地形/水/常春藤…）由各域恢复后经 `add_game_startup_systems` 并入；
//! 未注册不冒充原 28-系统顺序。

use crate::{
    meta_commander::{MetaCommand, MetaCommander},
    resources::session::SessionFlowPlugin,
    systems::{
        audio::music::music_system::music_audio_init_system,
        history::{add_history_cpu_systems, register_history_events},
        save::{add_save_load_events_and_resources, load_game::LoadGameManager},
    },
};
use bevy_ecs::prelude::*;

/// `run_game_thread` 新会话初始命令集（X7，RE-X5X7 §2，RNE 原字节）。
/// 数组 3 × 0x38B（cmd0@+0xb00/cmd1@+0xb38/cmd2@+0xb70）；cmd2 = **变体 8**
/// （0x8000000000000008，0x1405a340c 直写），+0x08 = [rbx+0x2230] 16B、
/// +0x18 = [rbx+0x2240] 16B、+0x28 = rdi。cmd0/cmd1 的 tag/逐字段载荷需 ReVa
/// p-code 定点（残留），由调用方按已定位源字段传入。
pub fn new_session_on_load_commands(
    cmd0: MetaCommand,
    cmd1: MetaCommand,
    cmd2_payload_a: [u64; 2],
    cmd2_payload_b: [u64; 2],
    cmd2_payload_c: u64,
) -> [MetaCommand; 3] {
    [
        cmd0,
        cmd1,
        MetaCommand {
            tag: 0x8000_0000_0000_0008,
            payloads: [
                cmd2_payload_a[0],
                cmd2_payload_a[1],
                cmd2_payload_b[0],
                cmd2_payload_b[1],
                cmd2_payload_c,
                0,
            ],
        },
    ]
}

/// 新会话 on_load 构建：压入初始命令集 + 回放终止哨兵（run_game_thread 语义）。
pub fn build_new_session_on_load(
    next: &mut crate::resources::session::NextSession,
    commands: [MetaCommand; 3],
) {
    next.extend_on_load(commands);
    next.append_replay_terminator();
}

/// 注册 core 资源/事件与可挂载系统（库内可接部分）。
/// - 会话流：SessionFlowPlugin::build（ActiveSession/SavedSessionList/NextSession/
///   AutoSaveState + 会话事件）
/// - 存档：add_save_load_events_and_resources（AutoSaveState 已有、LoadGameManager、
///   LoadingCompletionState、SavedSessionList、存档/读档事件）
/// - 历史：register_history_events（ReplayBuffer 资源由宿主按 speed 提供；事件集）
/// - 命令器：MetaCommander 资源 + configure_session_switch_systems（会话切换链，卡2）
/// - 音频：music_audio_init_system<B>（B = 宿主 MusicBackend 资源，feature native-audio
///   提供 FmodMusicBackend）
pub fn register_core_startup<B>(world: &mut World, schedule: &mut Schedule)
where
    B: crate::systems::audio::music::music_player::MusicBackend + Resource + 'static,
{
    // 会话流（ActiveSession/SavedSessionList/NextSession + 事件）。
    SessionFlowPlugin.build(world);
    // 存档事件/资源（AutoSaveState/事件；LoadGameManager 由读档系统消费）。
    add_save_load_events_and_resources(world);
    // 命令器资源 + 会话切换系统。
    world.init_resource::<MetaCommander>();
    crate::meta_commander::configure_session_switch_systems(schedule);
    // 历史事件集。
    register_history_events(world);
    // 音频初始化（B 由宿主装配；music_audio_init_system 经 Commands 插入 MusicAudioState）。
    schedule.add_systems(music_audio_init_system::<B>);
    // 历史 CPU 回放链（ReplayBuffer speed 由宿主在进入回放前设置）。
    add_history_cpu_systems::<()>(schedule);
    // 读档管理器资源（load_game 系统消费）。
    world.init_resource::<LoadGameManager>();
}
