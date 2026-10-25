//! ui_delete_tree.rs — 删除树 UI（B 0x141bd2510，O；2026-10-17 fresh 反编译 +
//! 反汇编复核）。
//!
//! 控制流（O）：
//! 1. 门禁：`DEV_OVERRIDE_IS_DEMO_MODE != 0 → return`；
//!    `AppMode == Eraser(Tree)`（tag 9 + 子字节 6）。
//! 2. `UiState::clearing_canvas_w_layer_and_sound("tree_erase", 0x0a, 3)`
//!    （0x14092e340；name 实读 0x142ef5fe9 长度 10 = `"tree_erase"`；
//!    argument_79=0x0a、layer=3）→ `slider3d::into_drag_state`（0x14092df10）：
//!    `state != 2` 才用 `CursorTerrainRaycast::position()` 作初值；
//!    `state == 2`（未拖动）且无路径事件时不处理。
//! 3. 半径初值 = `EraserSizeMultiplier * 3.0`（0x1429cd078）。
//! 4. `PathDrawEvent`（stride 0x20 / payload 0x18）：`observed_flag`（payload
//!    `+0x10`，事件基址 +0x18）== false 时取**首个**事件：
//!    center = `position.xz`（+8/+0x10）、radius = `2 * radius`（+0x14），
//!    并置 active=true；否则保留光标中心。
//! 5. 逐 `TreeStorage::iter()` 条目（**不是** `iter_removed_ids`；0x140a1b100
//!    为 ICF 折叠 hashmap iter）：`sqrt(dx²+dz²) < radius` →
//!    `DeleteTreeCmd{id}` + `History::add(TreeEdit::Delete(TreeEditData{..}))`。
//!
//! ## 记录（I/U）
//! - native 位置源 = Query 的 0x30 B 放置记录 `+0/+8`；core `PrefabInstanceData`
//!   （0x58 Vec 布局）无该字段 → 本实现对位置用 `TreeStorageEntry.position`
//!   （native 记录 0x48 B 的 `+0x10` 打包位置解包面），登记投影。
//! - History `TreeEditData`：native 从放置记录取 prefab 名/scale/rotation；
//!   core 记录没有对应字段 → prefab 名取 `PrefabInstance.name`，
//!   scale/rotation 显式保持默认（0）并登记（不是近似业务分支）。

use bevy_ecs::prelude::*;
use glam::Vec2;

use country_core::components::TreeComponent;
use country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast;
use country_core::resources::history::tree_edit::{DeleteTreeCmd, TreeEdit, TreeEditData};
use country_core::resources::history::{EditType, History};
use country_core::resources::path_stroke::PathDrawEvent;
use country_core::resources::prefab::PrefabInstance;
use country_core::resources::tree_storage::TreeStorage;
use country_core::resources::walls::EraserSizeMultiplier;
use country_core::systems::mode_manager::{AppMode, EraserLayer};
use country_core::systems::prefab::PrefabInstanceData;
use country_core::systems::ui::resources::ui_state::UiState;
use country_core::systems::ui::signifiers::signifier_create::UiSignifierStream;

use crate::ui_place_tree::DEV_OVERRIDE_IS_DEMO_MODE;

/// 删除半径系数（O：0x1429cd078 = 3.0）。
pub const DELETE_RADIUS_SCALE: f32 = 3.0;

/// 清除画布 widget 名（O：0x142ef5fe9 实读 10 B `"tree_erase"`）。
pub const TREE_ERASE_WIDGET: &str = "tree_erase";
/// `clearing_canvas_w_layer_and_sound` 的 `argument_79`（O：R9B = 0x0a）。
pub const TREE_ERASE_ARGUMENT_79: u8 = 0x0a;
/// 清除画布 layer（O：栈参 byte = 3）。
pub const TREE_ERASE_LAYER: u8 = 3;

/// `ui_delete_tree` @0x141bd2510（O 面；见模块文档）。
#[allow(clippy::too_many_arguments)]
pub fn ui_delete_tree<S: Send + Sync + 'static>(
    mode: Res<AppMode>,
    ui: Res<UiState>,
    trees: Query<(&PrefabInstanceData, &PrefabInstance, &TreeComponent)>,
    mut strokes: EventReader<PathDrawEvent>,
    mut deletes: EventWriter<DeleteTreeCmd>,
    storage: Res<TreeStorage>,
    cursor: Res<CursorTerrainRaycast>,
    mut history: ResMut<History<S>>,
    _signifiers: Res<UiSignifierStream>,
    brush: Res<EraserSizeMultiplier>,
) {
    // O：demo 门禁 + Eraser(Tree)（native tag 9 / 子字节 6）。
    if DEV_OVERRIDE_IS_DEMO_MODE || !matches!(*mode, AppMode::Eraser(EraserLayer::Tree)) {
        return;
    }

    // O：clearing canvas（0x14092e340）+ into_drag_state（0x14092df10）。
    // `dragged() == 2` 表示未拖动态；native 只在 state != 2 时取光标位置。
    let ctx = ui.clearing_canvas_w_layer_and_sound(
        TREE_ERASE_WIDGET,
        TREE_ERASE_ARGUMENT_79,
        TREE_ERASE_LAYER,
    );
    let drag_state = ctx.dragged();

    let mut active = drag_state != 2;
    let mut center = Vec2::ZERO;
    let mut radius = 0.0;
    if active {
        let p = cursor.position();
        center = Vec2::new(p.x, p.z);
        radius = brush.0 * DELETE_RADIUS_SCALE;
    }

    // O：PathDrawEvent 首个 `observed_flag == false` 事件替换中心/半径并置 active。
    for ev in strokes.read() {
        if !ev.observed_flag {
            center = Vec2::new(ev.position.x, ev.position.z);
            radius = ev.radius * 2.0;
            active = true;
            break;
        }
    }
    if !active {
        return;
    }

    // O：逐 TreeStorage 条目；`sqrt(dx²+dz²) < radius` → DeleteTreeCmd + History。
    for (index, entry) in storage.iter() {
        let pos = Vec2::new(entry.position.x, entry.position.z);
        if (pos - center).length() >= radius {
            continue;
        }
        let prefab_name = trees
            .get(entry.entity)
            .ok()
            .and_then(|(_data, prefab, _tree)| prefab.name.as_ref().map(|n| (**n).clone()))
            .unwrap_or_default();
        deletes.write(DeleteTreeCmd { id: index.0 });
        // History：native 用放置记录 0x30 B 填 position/scale/rotation/prefab；
        // core 仅能取 position（TreeStorage 投影）与 prefab 名，scale/rotation
        // 保持默认并登记（见模块文档）。
        let edit = EditType::Tree(std::sync::Arc::new(TreeEdit::Delete(TreeEditData {
            id: index.0 as i32,
            position: entry.position,
            prefab_name,
            ..Default::default()
        })));
        let _ = history.add(edit);
    }
}
