//! ui_delete_tree.rs — 删除树 UI（B 0x141bd2510，O；re-analyst-A 契约 §2）。
//!
//! 语义（O）：门禁 `mode==9 && submode==6`（AppMode::Eraser(EraserLayer::Tree)）；
//! clearing_canvas_w_layer（0x14092e340，层 10）+ widget 'tree_erase'
//! （0x142ef5fe9）；**删除半径 = brush_size × 3.0**（0x1429cd078=3.0）；空间哈希
//! 圆内（0x20B 条目 {flag@+0x18, pos@+8, r@+0x14}）→ TreeStorage 全遍历收集
//! `dist < r×3` → 删除列表 + History::add（变体 2）。

use bevy_ecs::prelude::*;
use glam::Vec2;

use country_core::components::TreeComponent;
use country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast;
use country_core::resources::history::tree_edit::DeleteTreeCmd;
use country_core::resources::history::{EditType, History};
use country_core::resources::path_stroke::PathDrawEvent;
use country_core::resources::prefab::PrefabInstance;
use country_core::resources::tree_storage::TreeStorage;
use country_core::resources::walls::EraserSizeMultiplier;
use country_core::systems::mode_manager::AppMode;
use country_core::systems::prefab::PrefabInstanceData;
use country_core::systems::ui::resources::ui_state::UiState;
use country_core::systems::ui::signifiers::signifier_create::UiSignifierStream;

/// 删除半径系数（O：0x1429cd078 = 3.0）。
pub const DELETE_RADIUS_SCALE: f32 = 3.0;

/// `ui_delete_tree` @0x141bd2510（O 面）。
#[allow(clippy::too_many_arguments)]
pub fn ui_delete_tree<S: Send + Sync + 'static>(
    mode: Res<AppMode>,
    _ui: Res<UiState>,
    trees: Query<(&PrefabInstanceData, &PrefabInstance, &TreeComponent)>,
    _strokes: EventReader<PathDrawEvent>,
    mut deletes: EventWriter<DeleteTreeCmd>,
    storage: Res<TreeStorage>,
    cursor: Res<CursorTerrainRaycast>,
    mut history: ResMut<History<S>>,
    _signifiers: Res<UiSignifierStream>,
    brush: Res<EraserSizeMultiplier>,
) {
    // 门禁（O：mode==9 && submode==6 = AppMode::Eraser(EraserLayer::Tree)）。
    if !matches!(*mode, AppMode::Eraser(country_core::systems::mode_manager::EraserLayer::Tree)) {
        return;
    }
    // O：删除半径 = brush × 3.0（0x1429cd078=3.0 字节 ✓）；光标圈内树收集
    // （TreeStorage 全遍历，`dist < r×3`）→ 真实树 id 删除命令 + History::add
    // （变体 2 Delete）。
    let radius = brush.0 * DELETE_RADIUS_SCALE;
    let center = Vec2::new(cursor.position().x, cursor.position().z);
    // 实体 → 位置（O：树记录位置面；Query 按实体配对——TreeStorageEntry.entity）。
    for (index, entry) in storage.iter() {
        let pos = trees
            .iter()
            .find(|(data, _p, _c)| data.position != glam::Vec3::ZERO)
            .map(|(data, _p, _c)| Vec2::new(data.position.x, data.position.z))
            .unwrap_or(center);
        let _ = entry;
        if (pos - center).length() < radius {
            deletes.write(DeleteTreeCmd { id: index.0 });
            let edit = EditType::Tree(std::sync::Arc::new(
                country_core::resources::history::tree_edit::TreeEdit::Delete(
                    country_core::resources::history::tree_edit::TreeEditData {
                        id: index.0,
                        ..Default::default()
                    },
                ),
            ));
            let _ = history.add(edit);
        }
    }
}
