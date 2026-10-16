use super::{
    DeletedAutoClutter,
    debug_spawn_clutter::{
        DebugClutterEntityCounter, DebugSpawnAllClutterCmd, DebugSpawnClutterCmd, DebugTerrainHeight,
        SpawnAutoClutter, debug_spawn_all_clutter,
    },
    decoclutter_show_pins::{PinVisibilityToggle, PinnedDecoClutter, ShowPinnedDecoClutter, decoclutter_show_pins},
    defered_autoclutter_history::{RecordAutoClutterHistory, defered_autoclutter_history},
    edit_autoclutter::{DeleteAutoClutterCmd, delete_auto_clutter},
    rmb_autoclutter::{EvRightClickedAutoClutter, RmbAutoClutterHit, handle_rmb_autoclutter},
    ui_autoclutter::{AutoClutterParentMap, ui_convert_autoclutter_to_decoclutter},
    ui_focus_decoclutter::{DecoClutterFocused, DecoClutterFocusState, ui_focus_decoclutter},
};
use crate::resources::walls::decorator_storage::DecoratorStorage;
use crate::systems::history::{HistoryCpuStage, recording::HistoryRecordingFailed};
use bevy_ecs::prelude::*;

pub fn register_ui_clutter_resources(world: &mut World) {
    world.init_resource::<DeletedAutoClutter>();
    world.init_resource::<Events<DeleteAutoClutterCmd>>();
    world.init_resource::<Events<RecordAutoClutterHistory>>();
    world.init_resource::<Events<HistoryRecordingFailed>>();

    // 拾取/生成可见消费（原 add_ui 挂载面）。
    world.init_resource::<Events<RmbAutoClutterHit>>();
    world.init_resource::<Events<EvRightClickedAutoClutter>>();
    world.init_resource::<Events<SpawnAutoClutter>>();
    world.init_resource::<Events<DebugSpawnClutterCmd>>();
    world.init_resource::<Events<DebugSpawnAllClutterCmd>>();
    world.init_resource::<Events<PinVisibilityToggle>>();
    world.init_resource::<Events<DecoClutterFocused>>();
    world.init_resource::<DebugClutterEntityCounter>();
    world.init_resource::<DebugTerrainHeight>();
    world.init_resource::<AutoClutterParentMap>();
    world.init_resource::<PinnedDecoClutter>();
    world.init_resource::<ShowPinnedDecoClutter>();
    world.init_resource::<DecoClutterFocusState>();
    // 装饰权威存储（ui_focus_decoclutter 主读路径消费）。
    world.init_resource::<DecoratorStorage>();
}

/// Local resolved-ID edit/record order; caller maintains events and supplies
/// History<S>. The global original UI schedule has not been reconstructed.
pub fn add_ui_clutter_cpu_systems<S: Send + Sync + 'static>(schedule: &mut Schedule) {
    schedule.add_systems(
        (delete_auto_clutter::<S>, defered_autoclutter_history::<S>)
            .chain()
            .in_set(HistoryCpuStage::Record),
    );
    // 原 add_ui 挂载的 6 个 clutter 系统（debug_spawn/decoclutter_show_pins/defered/
    // rmb/ui_autoclutter/ui_focus_decoclutter；宿主提供实体机制）。
    schedule.add_systems((
        handle_rmb_autoclutter,
        ui_convert_autoclutter_to_decoclutter,
        debug_spawn_all_clutter,
        decoclutter_show_pins,
        ui_focus_decoclutter,
    ));
}
