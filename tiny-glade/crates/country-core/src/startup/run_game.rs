//! B `startup/run_game` — 宿主可调的核心启动链（order 8 收口）。
//!
//! 原版 run_game_thread 0x1405a3440（39KB）装配整个游戏线程；本文件给出库内可接的
//! `register_core_startup`（会话/存档/历史/命令器/音频/会话切换注册），供宿主（tiny-glade）调用。
//! 域系统（墙/屋顶/地形/水/常春藤…）由各域恢复后经 `add_game_startup_systems` 并入；
//! 未注册不冒充原 28-系统顺序。
//!
//! 装配顺序按 ① T2 契约 round4/contracts/t2-run_game_thread-register_core_startup-CONTRACT.md §1
//! （run_game_thread 359 直接调用序 O）对齐本地可落地部分：
//! 会话流 → MetaCommander → GameConfig → 事件注册 → 域资源/系统 → 后台线程（init_save_thread）。

use crate::{
    game_config::GameConfig,
    meta_commander::{MetaCommand, MetaCommander},
    resources::session::SessionFlowPlugin,
    startup::startup_session,
    systems::{
        audio::{
            add_audio_events, add_audio_systems, music::music_system::music_audio_init_system,
        },
        backend::input::{add_input_cpu_consumers, register_input_events},
        debug::sheep_controls::{sheep_controls, SheepControl},
        garden::{add_garden_cpu_systems, register_garden_events},
        history::{add_history_cpu_systems, register_history_events},
        ivy::registration::{add_ivy_systems, register_ivy_storage},
        save::{
            add_save_load_events_and_resources, add_save_load_systems_impl,
            save_game::{sheep_save_section_system, SheepSaveSectionValue},
        },
        terrain::{add_terrain_cpu_systems, add_terrain_tool_systems, register_terrain_events},
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
/// - 命令器：MetaCommander 资源 + configure_session_switch_systems（会话切换链，卡2）
/// - GameConfig（native 0x140522ad0 载入；crate 语义结构 Default 注册）
/// - 事件：add_save_load_events_and_resources（存档/读档事件）、register_input_events
///   （输入游标族）、register_history_events（历史事件集）、add_audio_events（16 音频事件）
/// - 音频：music_audio_init_system<B> + add_audio_systems::<S, B>（S = 宿主 Studio /
///   B = 宿主 MusicBackend，feature native-audio 提供 FmodMusicBackend）
/// - 会话切换：startup_session::load_game_session（13-param，NewSessionStartedCmd /
///   SessionLoadedCmd / DisplayCorruptedSaveFileWarning 生产者）
/// - 存档系统：add_save_load_systems_impl（save_game 节拍器 + load_game + init_save_thread
///   后台线程生产）
/// - 地形：register_terrain_events + add_terrain_cpu_systems + add_terrain_tool_systems
/// - **A 块（⑤轮，③ 基线盘点 F2 闭合）孤儿系统并入生产 Schedule**：
///   ivy（register_ivy_storage + add_ivy_systems 7 系统链；原 22 系统 0x140944070 序
///   待①契约）、garden（register_garden_events + add_garden_cpu_systems::<(),()>）、
///   input（add_input_cpu_consumers）、sheep（SheepControl 资源 + sheep_controls）、
///   audio（add_audio_events + add_audio_systems）。宿主按各域资源面注入运行期资源。
pub fn register_core_startup<B, S>(world: &mut World, schedule: &mut Schedule)
where
    B: crate::systems::audio::music::music_player::MusicBackend + Resource + 'static,
    S: crate::resources::audio_resources::Studio + Send + Sync + 'static,
{
    // ① T2 契约 §1 装配顺序（run_game_thread 359 调用序 O，本地可落地部分）：
    // 会话流 → MetaCommander → GameConfig → 事件注册 → 域资源/系统 → 后台线程。
    //
    // 1. 会话流资源（SavedSessionList/ActiveSession/NextSession/AutoSaveState + 会话事件）。
    SessionFlowPlugin.build(world);
    // 2. 命令器资源 + 会话切换系统（commander_update case 0 写 NextSession）。
    world.init_resource::<MetaCommander>();
    crate::meta_commander::configure_session_switch_systems(schedule);
    // 3. GameConfig（native 0x140522ad0 从配置文件载入；crate 语义结构，Default 注册）。
    world.init_resource::<GameConfig>();
    // 4. 事件注册（存档/读档 + 输入游标族 + 历史 + 音频）。
    add_save_load_events_and_resources(world);
    register_input_events(world);
    register_history_events(world);
    add_audio_events(world);
    // 5. 域资源/系统：音频 → 历史回放链 → 会话切换 → 存档 → 地形 →
    //    A 块孤儿系统（ivy/garden/input/sheep）。
    schedule.add_systems(music_audio_init_system::<B>);
    add_audio_systems::<S, B>(schedule);
    add_history_cpu_systems::<()>(schedule);
    schedule.add_systems(startup_session::load_game_session);
    add_save_load_systems_impl(schedule);
    // 地形 CPU 系统 + 工具 UI/历史生产（terrain 域；宿主注入 AppMode/History/SSBO）。
    register_terrain_events(world);
    add_terrain_cpu_systems(schedule);
    add_terrain_tool_systems::<()>(schedule);
    // A 块：ivy（资源注册 + 7 系统链；原 22 系统 0x140944070 序待①契约）。
    register_ivy_storage(world);
    add_ivy_systems(schedule);
    // A 块：garden（build 0x140b6d290 CPU 生产面：事件 + Draw/Record 链）。
    register_garden_events(world);
    add_garden_cpu_systems::<(), ()>(schedule);
    // A 块：input（translate_input→cursor_moved→delta→reset + mod_key +
    //   update_cursor_position + 回放族）。
    add_input_cpu_consumers(schedule);
    // A 块：sheep（debug 域 sheep_controls；仲裁批准落地；宿主注入控制值）。
    world.init_resource::<SheepControl>();
    schedule.add_systems(sheep_controls);
    // C 块（⑤轮，① c-snapshot-apply-session-consumers 契约 §2）：会话切换消费端——
    // time_of_day 系统（crate 恢复的 3 参子集；完整 11-param 面待①）。
    world.init_resource::<crate::systems::time_of_day::TimeOfDayState>();
    world.init_resource::<crate::systems::time_of_day::TimeOfDayLightingSink>();
    // ⑥轮收官（① r6-28-system-names §2）：time_of_day 用 `resource_exists<NextSession>`
    // run 条件（public-session-names entry7，O）。
    schedule.add_systems(
        crate::systems::time_of_day::time_of_day.run_if(bevy_ecs::schedule::common_conditions::resource_exists::<crate::resources::session::NextSession>),
    );
    // ⑦轮链3（① c-snapshot-apply-session-consumers + r6-28-system-names §2，宿主接线面）：
    // 会话切换消费者完整链 = load_game_session（生产 NewSessionStartedCmd/SessionLoadedCmd）
    // → 库内消费端（reset_input_state 用 on_event<NewSessionStartedCmd>、
    //   time_of_day/input_record_replay 用 resource_exists<NextSession>，已接线）
    // → **库外 3 消费系统**（add_preplaced_autoclutter [system_clutter] /
    //   detect_overhang_support_segments [system_wall_constructor] /
    //   sheep_unstuck [system_critters]，各带 ER<NewSessionStartedCmd>/ER<SessionLoadedCmd>）
    //   宿主注册点——crate 只保证事件已注册 + 资源在场，不实现库外系统（round5 c 契约 §2）。
    // B 块（⑤轮，① b-sheep-write-path 契约）：Sheep 节生产链——
    // sheep_controls 捕获活羊 → SheepControl → sheep_save_section_system →
    // SheepSaveSectionValue（宿主 assemble 从该资源取 SaveFileSection::Sheep 真实值）。
    world.init_resource::<SheepSaveSectionValue>();
    schedule.add_systems(sheep_save_section_system);
}
