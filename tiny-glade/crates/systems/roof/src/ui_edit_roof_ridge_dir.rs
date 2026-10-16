//! 脊线方向 UI 编辑（ui_edit_roof_ridge_dir.rs）。
//!
//! - `ui_edit_roof_ridge_dir`（0x141a386b0）：UI 分发。
//! - `do_ui_edit_roof_ridge_dir`（0x141a39490）：交互执行。
//! - `switch_roof_ridge_direction`（0x141a3a730）：写 Roof[0x3c]、标 Changed、
//!   发 geometry kind1（Entity/WallId）、gable WallId、sound kind7（pivot+height）。
//! - `record_switch_roof_ridge_history`（0x141a3ac20）：直写 History（tag 3）。
//!
//! 事件：`SwitchRoofRidgeDirCmd { wall, direction, previous_direction }`
//! （观测载荷：wall@+8、direction@+0x10、previous@+0x11，0x18B 含事件索引）。
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
pub fn do_ui_edit_roof_ridge_dir(
    mut dispatch: EventReader<DoUiEditRoofRidgeDir>,
    mode: Res<AppMode>,
    walls: Res<PublicWalls>,
    roofs: Query<&Roof>,
    ui: Res<UiState>,
    mut switch: EventWriter<SwitchRoofRidgeDirCmd>,
    mut history: EventWriter<RecordRidgeSwitchHistoryCmd>,
) {
    for request in dispatch.read() {
        let Some(roof) = walls
            .roof_entity(request.wall)
            .and_then(|entity| roofs.get(entity).ok())
        else {
            continue;
        };
        let direction = roof.observed()[0x3c] ^ 2;
        switch.write(SwitchRoofRidgeDirCmd {
            wall: request.wall,
            direction,
            previous_direction: roof.observed()[0x3c],
        });
        history.write(RecordRidgeSwitchHistoryCmd {
            wall: request.wall,
            direction,
            previous_direction: roof.observed()[0x3c],
        });
        let _ = (mode.native_tag(), ui.interaction().native_phase);
    }
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
        sound.write(RoofSound {
            entity,
            wall: request.wall,
            pivot: roof.pivot_3d().to_array(),
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
        let edit = country_core::recovered_building::building_history_transactions::RoofHistoryEdit::direction(
            request.wall.0,
            request.direction,
            request.previous_direction,
        );
        let _ = history.add(edit.into());
    }
}
