//! 脊线方向 UI 编辑（ui_edit_roof_ridge_dir.rs）。
//!
//! - `ui_edit_roof_ridge_dir`（0x141a386b0）：UI 分发 —— **未实现**（依赖
//!   Decorators::iter_removed_ids / RoofRidgeDims::to_world_space /
//!   box_trigger_volume / UiWidgetContext，工具 widget 框架批次 3 后置）。
//! - `do_ui_edit_roof_ridge_dir`（0x141a39490）：交互执行 —— **未实现**（近似体
//!   清退为纯签名，reviewer FIX-1；真逻辑含方向位翻转 `^1`（=`^0x100`）、
//!   SignifierFactory / slider3d / 触发体拾取链，待 UI 工具框架接线）。
//! - `switch_roof_ridge_direction`（0x141a3a730）：已重建 —— 写 Roof[0x3c]、
//!   标 Changed、RoofGeometryChanged kind 1、WallGableChanged、RoofSound
//!   kind 7（pivot.y+height）。
//! - `record_switch_roof_ridge_history`（0x141a3ac20）：已重建 —— 直写 History
//!   （tag 3，payload[1]=from、payload[2]=to、payload[8..16]=wall_id）。
//!
//! 事件：`SwitchRoofRidgeDirCmd { wall, direction, previous_direction }`
//! （观测载荷：wall@+8、direction@+0x10、previous@+0x11，0x18B 含事件索引）。
//! `RecordRidgeSwitchHistoryCmd` 载荷同布局；`record_switch_roof_ridge_history`
//! 的 from/to 语义（event+0x10→from、+0x11→to）为 decomp O，发射方
//! `do_ui_edit_roof_ridge_dir` 未重建（字段赋值待 UI 批次）。
use country_core::resources::{
    history::History,
    roofs::{Roof, commands::*},
    walls::{WallId, public_walls::PublicWalls},
};
use country_core::systems::{mode_manager::AppMode, ui::resources::ui_state::UiState};
use bevy_ecs::{event::Event, prelude::*};

/// B `system_roof::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd`。
#[derive(Event, Clone, Copy, Debug)]
pub struct SwitchRoofRidgeDirCmd {
    pub wall: WallId,
    pub direction: u8,
    pub previous_direction: u8,
}

/// B `system_roof::ui_edit_roof_ridge_dir::RecordRidgeSwitchHistoryCmd`
/// （do_ui_edit_roof_ridge_dir 分发的历史载荷）。
#[derive(Event, Clone, Copy, Debug)]
pub struct RecordRidgeSwitchHistoryCmd {
    pub wall: WallId,
    pub direction: u8,
    pub previous_direction: u8,
}

/// B 0x141a386b0：屋顶编辑模式 + UI 交互命中 → 写 DoUiEditRoofRidgeDir。
pub fn ui_edit_roof_ridge_dir(
    mode: Res<AppMode>,
    walls: Res<PublicWalls>,
    roofs: Query<&Roof>,
    ui: Res<UiState>,
    mut dispatch: EventWriter<DoUiEditRoofRidgeDir>,
) {
    use country_core::systems::mode_manager::EditSubject;
    let AppMode::Editing(EditSubject::Roof(wall)) = *mode else {
        return;
    };
    let wall = WallId(wall);
    // 交互必须存在于该屋顶的关联实体上（widget 命中）。
    if ui.interaction().widget.0 == 0 {
        return;
    }
    let Some(entity) = walls.roof_entity(wall) else {
        return;
    };
    if roofs.get(entity).is_err() {
        return;
    }
    dispatch.write(DoUiEditRoofRidgeDir { wall });
}

/// 交互完成事件（B `DoUiEditRoofRidgeDir`）。
#[derive(Event, Clone, Copy, Debug)]
pub struct DoUiEditRoofRidgeDir {
    pub wall: WallId,
}

/// B 0x141a39490：从 UI 状态读目标方向并分发方向切换命令。
///
/// 未实现（近似体清退，reviewer FIX-1）：真逻辑 = `clicked()` 时发
/// `SwitchRoofRidgeDirCmd` + `RecordRidgeSwitchHistoryCmd`，历史载荷
/// `CONCAT11(old, roof[0x3c]) ^ 0x100`（翻转方向位 = `^1`）。现纯签名不写
/// 真实事件，待 UI 工具框架接线后按 decomp 重建。
pub fn do_ui_edit_roof_ridge_dir(
    _dispatch: EventReader<DoUiEditRoofRidgeDir>,
    _mode: Res<AppMode>,
    _walls: Res<PublicWalls>,
    _roofs: Query<&Roof>,
    _ui: Res<UiState>,
    _switch: EventWriter<SwitchRoofRidgeDirCmd>,
    _history: EventWriter<RecordRidgeSwitchHistoryCmd>,
) {
    // 未实现：真逻辑含 RoofRidgeDims::from/to_world_space、box_trigger_volume、
    // SignifierFactory/UiSignifierStream、slider3d::into_drag_state、方向位翻转
    // `^1`（= `^0x100`）。不写真实 SwitchRoofRidgeDirCmd/RecordRidgeSwitchHistoryCmd。
}

/// B 0x141a3a730：切换脊线方向（Roof[0x3c]），发 geometry kind1 / gable / sound kind7。
/// 同值方向也执行全部写入与通知，重复消息不合并。
pub fn switch_roof_ridge_direction(
    walls: Res<PublicWalls>,
    mut roofs: Query<&mut Roof>,
    mut requests: EventReader<SwitchRoofRidgeDirCmd>,
    mut geometry: EventWriter<RoofGeometryChanged>,
    mut gable: EventWriter<WallGableChanged>,
    mut sound: EventWriter<RoofSound>,
) {
    for request in requests.read() {
        let entity = walls
            .roof_entity(request.wall)
            .expect("switch_roof_ridge_direction requires a roof association");
        let mut roof = roofs
            .get_mut(entity)
            .expect("switch_roof_ridge_direction associated entity has no Roof");
        roof.observed_mut()[0x3c] = request.direction;
        roof.set_changed();
        geometry.write(RoofGeometryChanged {
            entity,
            wall: request.wall,
            kind: 1,
        });
        gable.write(WallGableChanged(request.wall));
        // decomp-0x141a3a730：Roof::pivot_3d() 的 y 分量再 + Roof::height()
        // （屋顶顶端世界 Y = 基底 + 高度）写 RoofSound kind 7 的 pivot。
        let mut pivot = roof.pivot_3d().to_array();
        pivot[1] += roof.height();
        sound.write(RoofSound {
            entity,
            wall: request.wall,
            pivot,
            kind: 7,
            flat_surface: None,
        });
    }
}

/// B 0x141a3ac20：方向切换的历史直写（History tag 3，from/to 双字节）。
pub fn record_switch_roof_ridge_history(
    mut requests: EventReader<RecordRidgeSwitchHistoryCmd>,
    mut history: ResMut<History>,
) {
    for request in requests.read() {
        let edit = country_core::resources::history::building_transactions::RoofHistoryEdit::direction(
            request.wall.0,
            request.direction,
            request.previous_direction,
        );
        let _ = history.add(edit.into());
    }
}
