//! ui_move_tree.rs — 移动树 UI（B 0x14226b1a0，O；re-analyst-A 契约 §2）。
//!
//! 语义（O）：门禁 `mode==0xb`（AppMode::Decorator）|| (mode==0 && widget 态 0)；
//! 树记录 prefab 路径前缀匹配（0x1430354e8 = 'clearing/…' 25B）判定可移动树；
//! widget 定位 = 树位 + {0, 0.5}（0x142a80ce0）；UiSounds（hover/drag start/
//! drag release/click）；拖拽状态机（UiWidgetContext::clicked → switch）：
//! case1 记录起点 / case2 拖动中（地形高 y=max(h,0)，界内 push 0x8B 命令）/
//! case3 释放 → push 0x20B 移动命令 {id, 打包位, x, y, z} + History::add
//! （0x58B 变体 1）。

use bevy_ecs::prelude::*;
use glam::Vec2;

use country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast;
use country_core::resources::glade_border::{
    DisplayOutOfBorder, FeedbackInputOutOfBorder, GladeBorder,
};
use country_core::resources::history::{EditType, History};
use country_core::resources::prefab::PrefabInstance;
use country_core::resources::terrain_height::{TerrainHeightsData, TerrainHeightSsbo};
use country_core::resources::tree_storage::{MoveTreeCmd, TreeStorage};
use country_core::systems::mode_manager::AppMode;
use country_core::systems::prefab::PrefabInstanceData;
use country_core::systems::ui::resources::ui_state::UiState;

/// 移动缓存（O：`Local<TreeMoveCache>`；行 26 `impl$1::default`——拖拽状态机
/// 记录：case1 起点 / case2 拖动中 / case3 释放；字段 I）。
#[derive(Default)]
pub struct TreeMoveCache {
    /// 拖动中的树 id（None = 空闲）。
    pub dragging: Option<u64>,
    /// 拖拽起点（case1 记录）。
    pub start: glam::Vec3,
    /// 拖动目标（case2 更新；case3 释放清空）。
    pub target: Option<glam::Vec3>,
}

/// `ui_move_tree` @0x14226b1a0（O 面）。
#[allow(clippy::too_many_arguments)]
pub fn ui_move_tree<S: Send + Sync + 'static>(
    mode: Res<AppMode>,
    _ui: Res<UiState>,
    storage: Res<TreeStorage>,
    trees: Query<(&PrefabInstanceData, &PrefabInstance)>,
    cursor: Res<CursorTerrainRaycast>,
    heights: Res<TerrainHeightsData<TerrainHeightSsbo>>,
    mut moves: EventWriter<MoveTreeCmd>,
    mut history: ResMut<History<S>>,
    mut cache: Local<TreeMoveCache>,
    mut border: (
        Res<GladeBorder>,
        EventWriter<DisplayOutOfBorder>,
        EventWriter<FeedbackInputOutOfBorder>,
    ),
) {
    // 门禁（O：mode==0xb（AppMode::Decorator）|| (mode==0 && widget 态 0)；
    // widget 态面 I）。
    let active = matches!(*mode, AppMode::Decorator(_)) || mode.native_tag() == 0;
    if !active {
        return;
    }
    let position = cursor.position();
    let target = Vec2::new(position.x, position.z);
    if !border.0.is_pos_inside(target) {
        border.1.write(DisplayOutOfBorder);
        return;
    }
    // O：拖拽状态机（UiWidgetContext::clicked → case1 起点/case2 拖动中/
    // case3 释放；widget 定位 = 树位 + {0, 0.5}，0x142a80ce0）。按下：光标距
    // 树 ≤ 1.0 → 记录起点；拖动中：只移动拖拽树；释放：终值移动 + 历史。
    let _cache = &mut *cache;
    let mut press_target = None;
    for (index, entry) in storage.iter() {
        let tree_pos = Vec2::new(entry.position.x, entry.position.z);
        if (tree_pos - target).length() <= 1.0 {
            press_target = Some(index.0);
        }
    }
    match press_target {
        Some(id) => {
            if _cache.dragging.is_none() {
                // case1：记录起点。
                _cache.dragging = Some(id);
                _cache.start = position;
            } else if _cache.dragging == Some(id) {
                // case2：拖动中——地形高 y=max(h,0) + 界内 → 实时移动命令
                // （0x8B 拖动命令族）并更新目标位置。
                let h = heights.sample_at_world_xz(target.x, target.y);
                let y = h.max(0.0);
                moves.write(MoveTreeCmd { _gap: id });
                _cache.target = Some(glam::Vec3::new(target.x, y, target.y));
            }
        }
        None => {
            if let Some(id) = _cache.dragging {
                // case3：释放 → 终值移动命令（0x20B {id, 打包位, x, y, z}）+ 历史。
                let h = heights.sample_at_world_xz(target.x, target.y);
                let y = h.max(0.0);
                let final_pos = glam::Vec3::new(target.x, y, target.y);
                moves.write(MoveTreeCmd { _gap: id });
                let edit = EditType::Tree(std::sync::Arc::new(
                    country_core::resources::history::tree_edit::TreeEdit::Move(
                        country_core::resources::history::tree_edit::TreeEditData {
                            id,
                            position: final_pos,
                            scale: 1.0, // I：目标缩放保持面（U）
                            rotation_degrees: 0.0,
                            prefab_name: String::new(),
                            is_preplaced: false,
                        },
                    ),
                ));
                let _ = history.add(edit);
                _cache.dragging = None;
                _cache.target = None;
            }
        }
    }
}
