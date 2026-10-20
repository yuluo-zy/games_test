//! B `system_water::WaterPlugin::build`（0x1406501d0）。
//!
//! 原 `build` 的机器码顺序（PyGhidra 逐指令核对）：
//! 1. `in_set(GameSystemSet(0x10))`，单独 `add_systems(Update)` —— `record_water_history`；
//! 2. `in_set(GameSystemSet(0x22))` → `run_if(Option<Res<WaterAudioGrid>>.is_some())` →
//!    `in_set(GameSystemSet(0x24))` → `add_systems(Update)` —— `update_water_audio_grid`；
//! 3. `insert_resource<WaterRaster>`（`DirtyTilePublisher::unique()` + `new`，常量折叠后符号名为
//!    `GardenRaster::new`，类型由 `insert_resource` 泛型决定）；
//! 4. 三次 `register_event`：`RecordWaterEditToHistoryCmd`、`WaterDrawEvent`、`WaterEraseEvent`。
//!
//! `draw_water` 不由该插件注册。`WaterRaster` 与 `history_runtime::install` 共用同一保护式构造，
//! 不重复插入。
use bevy_app::{App, Update};
use bevy_ecs::prelude::*;
use country_core::{
    resources::{
        history::MaybePendingSnapshotId,
        water::{WaterDrawEvent, WaterEraseEvent},
        world_raster::DirtyTilePublisher,
        world_raster_defs::WaterRaster,
    },
    systems::audio::water_ambience_audio::WaterAudioGrid,
};
use system_water::{
    record_water_history::{RecordWaterEditToHistoryCmd, record_water_history},
    water_audio_grid::update_water_audio_grid,
};

use crate::run_game::GameSystemSet;

pub fn build(app: &mut App) {
    app.add_systems(
        Update,
        record_water_history::<MaybePendingSnapshotId>.in_set(GameSystemSet(0x10)),
    );
    app.add_systems(
        Update,
        update_water_audio_grid
            .in_set(GameSystemSet(0x22))
            .run_if(|grid: Option<Res<WaterAudioGrid>>| grid.is_some())
            .in_set(GameSystemSet(0x24)),
    );
    if !app.world().contains_resource::<WaterRaster>() {
        app.insert_resource(WaterRaster::new(DirtyTilePublisher::unique()));
    }
    app.add_event::<RecordWaterEditToHistoryCmd>();
    app.add_event::<WaterDrawEvent>();
    app.add_event::<WaterEraseEvent>();
}
