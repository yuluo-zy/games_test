//! Recovered save-file codec, mutable shadow and incremental write consumers,
//! plus the asynchronous capture thread. The original game-thread capture
//! system that assembles GameSaveData from ~9 domain resources is not
//! registered here yet (domain owners pending); load_game's on_load replay
//! depends on meta_commander.rs per the coordinator ruling.
pub mod load_completion_checker;
pub mod load_game;
pub mod save_backup_list;
pub mod save_backup_metronome;
pub mod save_file_identity;
pub mod save_game;
pub mod save_thread;
pub mod shadow_file;
pub mod snapshot;
pub mod snapshot_gc;

#[cfg(test)]
mod tests;

/// B `add_save_load_systems` 0x1409443b0：注册 save_game / load_game / save_thread /
/// check_load_completion 等系统。当前域资源（GameProfile/各域 Query）未齐，系统注册
/// 留待域恢复后接入；此函数保留合同签名并记录阻塞面，不注册空系统。
/// on_load 回放链已由 `load_game::advance_load_game`/`load_game_session` 落地。
pub fn add_save_load_systems(_schedule: &mut bevy_ecs::schedule::Schedule) {
    // 阻塞：save_game 本体 ECS 组装（0x140a3f470，~9 资源）依赖
    // CameraRigParametersCache 以外的 TimeOfDay/Sheep/chain_index/snapshots 域资源；
    // capture_game_snapshot（0x1417a1d00 域 Query）；load_game 系统本体接线依赖
    // AppMode/GameProfile/ActiveSession 等宿主资源。commander_update 由
    // `meta_commander::configure_session_switch_systems` 注册。
    // save_thread 是独立线程（`save_thread` 入口），不属于 Schedule。
}

/// B `add_save_load_events_and_resources` 0x1409447e0：注册事件/资源
/// （SaveGameCmd/SavingGameComplete/LoadingGameComplete 等）。
pub fn add_save_load_events_and_resources(world: &mut bevy_ecs::world::World) {
    use bevy_ecs::event::{EventRegistry, Events};
    // Original @1409447e0 constructs both channels and starts Async save
    // before inserting the load/save owners. Do it once at App assembly.
    if !world.contains_resource::<save_game::AutoSaveState>() {
        let mut auto = save_game::AutoSaveState::default();
        let handle = save_thread::SaveThreadHandle::spawn_with_unsaved(auto.unsaved_files.clone());
        auto.attach_command_sender(handle.command_sender.clone());
        world.insert_resource(handle);
        world.insert_resource(auto);
    }
    world.init_resource::<load_game::LoadGameManager>();
    world.init_resource::<load_completion_checker::LoadingCompletionState>();
    world.init_resource::<crate::resources::session::SavedSessionList>();
    save_game::register_save_game_events(world);
    if !world.contains_resource::<Events<save_game::SavingGameComplete>>() {
        EventRegistry::register_event::<save_game::SavingGameComplete>(world);
    }
    if !world.contains_resource::<Events<load_game::LoadingGameComplete>>() {
        EventRegistry::register_event::<load_game::LoadingGameComplete>(world);
    }
    if !world.contains_resource::<Events<load_game::StartupRevealStarted>>() {
        EventRegistry::register_event::<load_game::StartupRevealStarted>(world);
    }
    if !world.contains_resource::<Events<load_game::StartupRevealFinished>>() {
        EventRegistry::register_event::<load_game::StartupRevealFinished>(world);
    }
    if !world.contains_resource::<Events<load_completion_checker::DisplayCorruptedSaveFileWarning>>() {
        EventRegistry::register_event::<load_completion_checker::DisplayCorruptedSaveFileWarning>(world);
    }
    if !world.contains_resource::<Events<crate::resources::session::NewSessionStartedCmd>>() {
        EventRegistry::register_event::<crate::resources::session::NewSessionStartedCmd>(world);
    }
    if !world.contains_resource::<Events<crate::resources::session::SessionLoadedCmd>>() {
        EventRegistry::register_event::<crate::resources::session::SessionLoadedCmd>(world);
    }
}

/// 注册 save 系统（save_game 节拍器/快照触发；load_game 消费 NextSession；save_thread
/// 生产 spawner）。由中央仲裁接入 `startup/run_game.rs::register_core_startup`。
use bevy_ecs::schedule::IntoScheduleConfigs;
pub fn add_save_load_systems_impl(schedule: &mut bevy_ecs::schedule::Schedule) {
    schedule.add_systems((
        save_game::save_game_system
            .after(crate::systems::debug::sheep_controls::sheep_controls),
        // 域内快照捕获系统（O：save_game 0x140a3f470 触发后内联快照采集）——
        // 宿主经 `CaptureSnapshotRequested` 请求额外快照；采集 roofs/walls/terrain/
        // stairs 并发送 AsyncSaveCmd::CaptureSnapshot(9) + record_snapshot。
        save_game::capture_snapshot_system,
        load_game::load_game_system,
        save_thread::init_save_thread,
    ));
}
