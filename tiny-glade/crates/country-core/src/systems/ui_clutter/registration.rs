use super::{
    DeletedAutoClutter,
    debug_spawn_clutter::{
        DebugClutterEntityCounter, DebugSpawnAllClutterCmd, DebugSpawnClutterCmd, SpawnAutoClutter,
        debug_spawn_all_clutter,
    },
    decoclutter_show_pins::{
        ShowPinnedDecoClutter, decoclutter_show_pins,
    },
    defered_autoclutter_history::{RecordAutoClutterHistory, defered_autoclutter_history},
    edit_autoclutter::{DeleteAutoClutterCmd, delete_auto_clutter},
    rmb_autoclutter::EvRightClickedAutoClutter,
    ui_autoclutter::{StartPlaceDecoClutterInteraction, ui_convert_autoclutter_to_decoclutter},
    ui_focus_decoclutter::{
        RequestMoveClutter, RequestRotateDecorator,
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
    world.init_resource::<Events<EvRightClickedAutoClutter>>();
    world.init_resource::<Events<super::ui_autoclutter::InstaCreateDeco>>();
    world.init_resource::<Events<super::ui_autoclutter::InstaCreateStairPoint>>();
    world.init_resource::<Events<SpawnAutoClutter>>();
    world.init_resource::<Events<DebugSpawnClutterCmd>>();
    world.init_resource::<Events<DebugSpawnAllClutterCmd>>();
    world.init_resource::<Events<RequestRotateDecorator>>();
    world.init_resource::<Events<RequestMoveClutter>>();
    world.init_resource::<Events<StartPlaceDecoClutterInteraction>>();
    world.init_resource::<DebugClutterEntityCounter>();
    // 转换维护的 parent 映射（原名 DecoratorAutoClutterChildren）。
    world.init_resource::<crate::resources::clutter_meshes::DecoratorAutoClutterChildren>();
    world.init_resource::<crate::resources::time::GameTime>();
    world.init_resource::<Events<ShowPinnedDecoClutter>>();
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
    // 原转换函数内部消费真实Query并调用RMB业务；不可再挂人工索引表reader。
    schedule.add_systems(ui_convert_autoclutter_to_decoclutter::<S>.before(HistoryCpuStage::Record));
    schedule.add_systems((
        debug_spawn_all_clutter::<S>,
        decoclutter_show_pins,
    ));
}
