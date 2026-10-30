//! 核心资源初始化、一次性启动、会话请求和每帧更新的分阶段装配。
//! 各阶段只复用现有业务系统，宿主负责运行顺序和图形/音频设备资源。

use crate::{
    game_config::GameConfig,
    meta_commander::{MetaCommand, MetaCommander},
    resources::history::History,
    startup::startup_session,
    systems::{
        audio::{
            add_audio_events, add_audio_systems, music::music_system::music_audio_init_system,
        },
        backend::input::{add_input_cpu_consumers, register_input_events},
        debug::sheep_controls::sheep_controls,
        garden::{add_garden_cpu_systems, register_garden_events},
        history::{add_history_cpu_systems, register_history_events},
        ivy::registration::{add_ivy_systems, register_ivy_storage},
        save::{
            add_save_load_events_and_resources, add_save_load_systems_impl,
            save_game::{SheepSaveSectionValue, sheep_save_section_system},
        },
        terrain::{add_terrain_cpu_systems, add_terrain_tool_systems, register_terrain_events},
        wall::registration::{add_wall_cpu_systems, register_wall_cpu_resources},
    },
};
use bevy_ecs::prelude::*;

/// 组合三条初始命令。最后一条的业务类型尚未确认，保持完整载荷由调用方解释。
pub fn new_session_on_load_commands(
    first_command: MetaCommand,
    second_command: MetaCommand,
    final_payload_header: [u64; 2],
    final_payload_body: [u64; 2],
    final_payload_tail: u64,
) -> [MetaCommand; 3] {
    [
        first_command,
        second_command,
        MetaCommand {
            tag: 0x8000_0000_0000_0008,
            payloads: [
                final_payload_header[0],
                final_payload_header[1],
                final_payload_body[0],
                final_payload_body[1],
                final_payload_tail,
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

/// 分别装配一次性启动、会话请求和每帧更新，宿主按此顺序运行三个调度表。
/// 对 Bevy App 分别挂载 Startup、PreUpdate、Update，避免初始化反复覆盖运行状态。
/// 这里只注册已存在的业务系统，设备、相机等域资源仍由宿主提供。
pub fn register_core_startup<B, S>(
    world: &mut World,
    startup_schedule: &mut Schedule,
    session_schedule: &mut Schedule,
    update_schedule: &mut Schedule,
) where
    B: crate::systems::audio::music::music_player::MusicBackend + Resource + 'static,
    S: crate::resources::audio_resources::Studio + Send + Sync + 'static,
{
    initialize_core_resources(world);
    let demo = world.resource::<GameConfig>().dev.demo;
    crate::systems::mode_manager::configure_mode_manager::<
        crate::resources::history::EditType,
        crate::resources::history::MaybePendingSnapshotId,
    >(world, update_schedule, demo);
    update_schedule.add_systems(
        crate::systems::mode_manager::synchronize_mode_manager_ui_context
            .after(crate::systems::backend::input::InputSystemSet::Derive)
            .before(crate::systems::mode_manager::mode_manager::<
                crate::resources::history::EditType,
                (),
            >),
    );
    register_startup_systems::<B>(startup_schedule);
    // The host runs the real sample-completion resource commands at
    // PreStartup; load_audio does not register Schedule systems.
    register_session_systems(session_schedule);
    register_update_systems::<B, S>(update_schedule);
}

/// 初始化共享资源；已由宿主载入的配置、会话列表和请求不会被默认值覆盖。
pub fn initialize_core_resources(world: &mut World) {
    use crate::resources::session::{
        NextSession, NextSessionKind, NextSessionTail, NextSessionTemplate, SavedSessionList,
        SessionId,
    };
    world.init_resource::<SavedSessionList>();
    if !world.contains_resource::<NextSession>() {
        world.insert_resource(NextSession::create(
            &NextSessionTemplate::new(SessionId(uuid::Uuid::nil()), NextSessionKind::LoadSavedGame),
            &NextSessionTail::EMPTY,
        ));
    }
    world.init_resource::<MetaCommander>();
    world.init_resource::<GameConfig>();
    world.init_resource::<crate::resources::time_of_day::TodSequence>();
    // Reuse the UI's real constructor and projection. Preserve a host's live
    // UI; the local CPU entry needs the same mode consumer as the full host.
    if !world.contains_resource::<crate::systems::ui::resources::ui_state::UiState>() {
        world.insert_resource(crate::systems::ui::resources::ui_state::UiState::at_startup());
    }
    let context = crate::systems::mode_manager::ModeManagerUiContext::from_ui_state(
        world.resource::<crate::systems::ui::resources::ui_state::UiState>(),
    );
    world.insert_resource(context);
    world.init_resource::<crate::resources::gui::TimeOfDayScreen>();
    add_save_load_events_and_resources(world);
    register_input_events(world);
    register_history_events(world);
    add_audio_events(world);
    register_terrain_events(world);
    // The published-copy mailbox is part of the gameplay World from startup.
    // The render thread is its real producer; until the first submitted frame
    // it contains no values and consumers must observe a miss.
    world.init_resource::<crate::render::gpu_readback::GpuReadbacks>();
    register_wall_cpu_resources(world);
    register_ivy_storage(world);
    register_garden_events(world);
    if !world.contains_resource::<History>() {
        world.insert_resource(
            History::<crate::resources::history::MaybePendingSnapshotId>::empty_glade(),
        );
    }
    world.init_resource::<SheepSaveSectionValue>();
    world.init_resource::<Events<crate::systems::save::save_game::DeleteSessionCmd>>();
    world.init_resource::<Events<crate::resources::session::NewSessionStartedCmd>>();
    world.init_resource::<Events<crate::resources::session::SessionLoadedCmd>>();
    world.init_resource::<Events<crate::systems::save::load_game::LoadingGameComplete>>();
}

/// 只在应用启动阶段执行；额外保护音乐状态，重复运行启动表也不会将其覆盖。
pub fn register_startup_systems<B>(schedule: &mut Schedule)
where
    B: crate::systems::audio::music::music_player::MusicBackend + Resource,
{
    use crate::systems::audio::music::music_system::MusicAudioState;
    use bevy_ecs::schedule::common_conditions::{not, resource_exists, run_once};
    schedule.add_systems((
        crate::resources::session::refresh_session_list_system.run_if(run_once),
        music_audio_init_system::<B>.run_if(not(resource_exists::<MusicAudioState<B::Event>>)),
        super::startup_time_of_day::startup_time_of_day.run_if(run_once),
    ));
}

/// 每帧先处理命令，再消费本帧新请求；chain 保证 Commands 在消费者之前提交。
/// 契约 §1.4：本表对应 OnEnter(AppLoadingState::Game)；batch2/batch3/batch4
/// 共用同一 label。设备宿主负责完整的世界资产产者，不登记 CPU mesh-only 分支。
pub fn register_session_systems(schedule: &mut Schedule) {
    use bevy_ecs::schedule::common_conditions::resource_exists;
    schedule.add_systems(
        (
            crate::meta_commander::commander_update_system,
            startup_session::load_game_session
                .run_if(resource_exists::<crate::resources::session::NextSession>),
        )
            .chain(),
    );
}

/// 每帧系统不再包含磁盘初始扫描、音乐重建和启动网格加载。
pub fn register_update_systems<B, S>(schedule: &mut Schedule)
where
    B: crate::systems::audio::music::music_player::MusicBackend + Resource + 'static,
    S: crate::resources::audio_resources::Studio + Send + Sync + 'static,
{
    use crate::systems::save::save_game::{DeleteSessionCmd, SavingGameComplete};
    use bevy_ecs::schedule::{Condition, common_conditions::on_event};
    schedule.add_systems(
        (
            crate::resources::session::delete_session_system.run_if(on_event::<DeleteSessionCmd>),
            crate::resources::session::refresh_session_list_system
                .run_if(on_event::<DeleteSessionCmd>.or(on_event::<SavingGameComplete>)),
        )
            .chain(),
    );
    add_audio_systems::<S, B>(schedule);
    add_history_cpu_systems::<crate::resources::history::MaybePendingSnapshotId>(schedule);
    add_save_load_systems_impl(schedule);
    add_terrain_cpu_systems(schedule);
    add_terrain_tool_systems::<crate::resources::history::MaybePendingSnapshotId>(schedule);
    add_wall_cpu_systems(schedule);
    crate::systems::wall::registration::add_wall_drawing_cpu_systems::<
        crate::resources::history::MaybePendingSnapshotId,
    >(schedule);
    add_ivy_systems(schedule);
    add_garden_cpu_systems::<(), crate::resources::history::MaybePendingSnapshotId>(schedule);
    add_input_cpu_consumers(schedule);
    schedule.add_systems((
        sheep_controls.after(crate::systems::history::finish_replay::finish_replay),
        sheep_save_section_system.after(sheep_controls),
        crate::systems::time_of_day::time_of_day::<S>,
    ));
    // The complete controller requires the real collision world. CPU-only
    // builds do not install a substitute camera controller.
    #[cfg(feature = "native-physics")]
    {
        use crate::systems::main_camera::{
            camera_snapshot, main_camera_update, paused_main_camera_update, reset_camera,
        };
        use bevy_ecs::schedule::common_conditions::{not, on_event, resource_exists};
        fn active(
            clock: Option<Res<crate::resources::time::GameTime>>,
            state: Option<Res<bevy_state::state::State<crate::app_loading_state::AppLoadingState>>>,
        ) -> bool {
            clock.is_some_and(|clock| clock.ran)
                && state.is_some_and(|state| {
                    *state.get() == crate::app_loading_state::AppLoadingState::Game
                })
        }
        schedule.add_systems(
            reset_camera
                .before(camera_snapshot)
                .run_if(on_event::<crate::resources::session::NewSessionStartedCmd>)
                .run_if(resource_exists::<crate::systems::camera_rig::MainCamera>),
        );
        schedule.add_systems(
            (
                camera_snapshot.run_if(active),
                (
                    main_camera_update.run_if(active),
                    paused_main_camera_update.run_if(not(active)),
                ),
            )
                .chain_ignore_deferred()
                .after(crate::systems::backend::input::InputSystemSet::Derive)
                .run_if(resource_exists::<crate::systems::camera_rig::MainCamera>),
        );
    }
}
