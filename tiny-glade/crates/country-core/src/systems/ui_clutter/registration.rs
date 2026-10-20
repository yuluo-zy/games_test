use super::{
    AutoClutterEntityIndex, DeletedAutoClutter,
    debug_spawn_clutter::{
        DebugClutterEntityCounter, DebugSpawnAllClutterCmd, DebugSpawnClutterCmd, SpawnAutoClutter,
        debug_spawn_all_clutter,
    },
    decoclutter_show_pins::{
        PinAnimationTable, PinHitDedup, PinVisibilityToggle, PinnedDecoClutter,
        PinnedDecoClutterBuffers, PinnedDecoGeometryTable, ShowPinnedDecoClutter,
        decoclutter_show_pins,
    },
    defered_autoclutter_history::{RecordAutoClutterHistory, defered_autoclutter_history},
    edit_autoclutter::{DeleteAutoClutterCmd, delete_auto_clutter},
    rmb_autoclutter::{
        EvRightClickedAutoClutter, RmbAutoClutterDirect, RmbAutoClutterHit,
        handle_rmb_autoclutter,
    },
    ui_autoclutter::{AutoClutterChildTable, StartPlaceDecoClutterInteraction, ui_convert_autoclutter_to_decoclutter},
    ui_focus_decoclutter::{
        DecoClutterFocusState, DecoClutterFocusUi, RequestMoveClutter, RequestRotateDecorator,
        ui_focus_decoclutter,
    },
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
    world.init_resource::<Events<super::ui_autoclutter::InstaCreateDeco>>();
    world.init_resource::<Events<super::ui_autoclutter::InstaCreateStairPoint>>();
    world.init_resource::<Events<SpawnAutoClutter>>();
    world.init_resource::<Events<DebugSpawnClutterCmd>>();
    world.init_resource::<Events<DebugSpawnAllClutterCmd>>();
    world.init_resource::<Events<PinVisibilityToggle>>();
    world.init_resource::<Events<RequestRotateDecorator>>();
    world.init_resource::<Events<RequestMoveClutter>>();
    world.init_resource::<Events<StartPlaceDecoClutterInteraction>>();
    world.init_resource::<DebugClutterEntityCounter>();
    // 转换维护的 parent 映射（原名 DecoratorAutoClutterChildren）。
    world.init_resource::<crate::resources::clutter_meshes::DecoratorAutoClutterChildren>();
    // 宿主记录表适配（0xa0B 子表/0x60 父表/0x30 变换表）。
    world.init_resource::<AutoClutterChildTable>();
    world.init_resource::<PinnedDecoClutter>();
    world.init_resource::<PinnedDecoGeometryTable>();
    world.init_resource::<PinAnimationTable>();
    world.init_resource::<PinnedDecoClutterBuffers>();
    world.init_resource::<PinHitDedup>();
    world.init_resource::<Events<ShowPinnedDecoClutter>>();
    world.init_resource::<DecoClutterFocusState>();
    world.init_resource::<DecoClutterFocusUi>();
    // 右键 probe 目标表（rmb handler 消费）+ 入口直达路径记录。
    world.init_resource::<AutoClutterEntityIndex>();
    world.init_resource::<RmbAutoClutterDirect>();
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
    schedule.add_systems((ui_convert_autoclutter_to_decoclutter, handle_rmb_autoclutter::<S>)
        .chain().before(HistoryCpuStage::Record));
    schedule.add_systems((
        debug_spawn_all_clutter::<S>,
        decoclutter_show_pins,
        ui_focus_decoclutter,
    ));
}
