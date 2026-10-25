//! ui_move_tree.rs — 移动树 UI（B 0x14226b1a0，O；2026-10-17 fresh 反编译）。
//!
//! 门禁（O，decomp 行 101-110）：`mode ∈ {0x10, 0}`——`lVar14==0xb`（`uVar8==0x10`
//! AppMode::Tree）或 `lVar14==0 && uVar8==0`（AppMode::Editing(None)，tag 0；reviewer-
//! wave3-tg-tr 复核补登记：0=None 亦放行）**且 `!DEMO`**（0x14362c098；wave4 G5）。
//! 流程：候选树 → `format!("move_tree_{id}")`
//! widget（UiSounds）→ `UiWidgetContext::dragged()` 状态 3/4/5 分派 case1/2/3；
//! case2/3 目标 = 光标 + case1 偏移，地形高 `y=max(h,0)`，界内 `MoveTreeCmd`
//! （槽 0x20：id@+0、index u64@+8、position Vec3 @+0x10），case2 界外双事件，
//! case3 界内另加 `History::add`（TreeEdit Move disc=1）。

use bevy_ecs::prelude::*;
use glam::Vec2;

use country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast;
use country_core::resources::glade_border::{
    DisplayOutOfBorder, FeedbackInputOutOfBorder, GladeBorder,
};
use country_core::resources::history::{EditType, History};
use country_core::resources::prefab::PrefabInstance;
use country_core::resources::terrain_height::{TerrainHeightSsbo, TerrainHeightsData};
use country_core::resources::tree_storage::{MoveTreeCmd, TreeStorage};
use country_core::systems::mode_manager::{AppMode, EditSubject};
use country_core::systems::prefab::PrefabInstanceData;
use country_core::systems::ui::resources::ui_state::UiState;
use country_core::systems::ui::resources::widget_params::{
    UiSounds, WidgetParams, WidgetVolumeShape,
};

/// 移动缓存（O：`Local<TreeMoveCache>`；原生只有两项：case1 记录的树原位置
/// 与 `树位置 − 光标` xz 偏移；**无拖动 id 门、无 case3 清除**，wave4 G6）。
#[derive(Default)]
pub struct TreeMoveCache {
    /// case1 记录的树原位置（case3 历史旧位；decomp `in_stack_00000048+0/+8`）。
    pub start: glam::Vec3,
    /// case1 记录的 xz 偏移 `树位置 − 光标`（decomp `local_378`、
    /// `local_374` 写入 Local `+0xc/+0x14`）。
    pub offset: Vec2,
}

/// `ui_move_tree` @0x14226b1a0（O 面；2026-10-17 consume-types 重建状态机）。
///
/// ## 判等/分歧
/// - **状态机判等**：decomp 行 293 的 `slider3d::impl$1::into_drag_state` 在 fresh
///   asm（14226b8c1）实为 `UiWidgetContext::dragged`（0x14092df10）；`AL-2` 后
///   `>2 → 3`（即 **state≥5 全部钳到 case3**），0=无动作、1/2/3=case1/2/3。
///   widget 名 `move_tree_{id}` 由格式串 0x143035501 直读。
/// - **门禁判等**：mode ∈ {Tree, Editing(None)} **且 `!DEMO`**（0x14362c098；
///   wave4 G5）。
/// - **case2 界外双事件判等**（decomp 行 318-341）：DisplayOutOfBorder +
///   FeedbackInputOutOfBorder。
/// - **无 id 门/无 case3 清除（wave4 G6）**：原生 Local 只有旧位置+偏移；
///   case2/3 直接用 Local 偏移，不在 case3 清空。
/// - 分歧 9 维持：候选源 `Decorators::iter_removed_ids` 以 `storage.iter()` 代理。
/// - **History 旧位（T3 已闭合）**：Move 载荷 `previous_position = cache.start`；
/// - WidgetParams 的 UiSounds 槽（hover/drag start/drag release/click）与
///   prefab 名匹配得到的 2.5/1.0 widget 偏移面未在共享 WidgetParams 承载，
///   以 `UiSounds::empty()` + 默认 volume 登记（不影响移动业务逻辑）。
#[allow(clippy::too_many_arguments)]
pub fn ui_move_tree<S: Send + Sync + 'static>(
    mode: Res<AppMode>,
    ui: Res<UiState>,
    storage: Res<TreeStorage>,
    _trees: Query<(&PrefabInstance, &PrefabInstanceData)>,
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
    // O：门禁（decomp 行 101-110）：`mode ∈ {0x10, 0}`——`lVar14==0xb`（uVar8==0x10
    // Tree）或 `lVar14==0 && uVar8==0`（AppMode::Editing(None)，tag 0；reviewer-wave3
    // 复核补登记——0=None 亦放行，非仅 Tree）。
    let tag = mode.native_tag();
    let tree_tag = AppMode::Tree.native_tag();
    let none_tag = AppMode::Editing(EditSubject::None).native_tag();
    if tag != tree_tag && tag != none_tag {
        return;
    }
    // O：`!DEMO` 门（fresh asm：`MOV RAX,[0x14362c098]; CMP byte[RAX],0; JZ
    // continue`；wave4 G5）。与 ui_place_tree 共用同一进程全局覆盖常量。
    if crate::ui_place_tree::DEV_OVERRIDE_IS_DEMO_MODE {
        return;
    }
    // O：`CursorTerrainRaycast::position()` + 水下 y 重映射（decomp 行 288-292：
    // `if (y<0) y = (y/-2.5)*-1.2 + (1 - y/-2.5)*0.0`；常量
    // DIVSS -2.5 / MULSS -1.2）。
    let mut position = cursor.position();
    if position.y < 0.0 {
        position.y = (position.y / -2.5) * -1.2 + (1.0 - position.y / -2.5) * 0.0;
    }
    // O：候选迭代（decomp 行 112-211：`Decorators::iter_removed_ids` 位图 →
    // wall/decorator 表定位；分歧 9，本处以 `storage.iter()` 代理——状态机按
    // widget id 门控，非活动候选不产生动作，行为等价面见 RESTORE.md）。
    for (index, entry) in storage.iter() {
        let id = index.0;
        // O：widget 名 = `format!("move_tree_{}", id)`（格式串 0x143035501 +
        // 0x1430354a0 直读），`UiState::add<String>` 后取 `dragged()`。
        //
        // decomp 行 293 记作 `slider3d::impl$1::into_drag_state`，fresh asm
        // 14226b8c1 实调 0x14092df10 = `dragged`；`AL-2` 超 3 钳 3 → 状态
        // 3/4/5 = case1/2/3，2（空闲）= 无动作。
        let params =
            WidgetParams::new(WidgetVolumeShape::default(), [0; 2]).sounds(UiSounds::empty());
        let state = ui.add(&format!("move_tree_{}", id), params).dragged();
        // O：case2/3 目标 = 光标 xz + case1 记录的偏移（decomp 行 313-314 /
        // 368-369：`local[+0xc]/[+0x14]`）。
        let target_x = position.x + cache.offset.x;
        let target_z = position.z + cache.offset.y;
        // O：`AL-2`；`CMP AL,3; CMOVNC EAX,3`——**≥5 全部钳到 3**（wave4 G6）。
        // 跳表 0x143035538：0（空闲 2）= 无动作、1=case1、2=case2、3=case3。
        let case = state.wrapping_sub(2);
        let case = if case > 2 { 3 } else { case };
        match case {
            1 => {
                // case1（Started）：记录树原位置 + `树位置 − 光标` 偏移
                // （decomp 行 301-311）。原生无 id 门，每轮直接写 Local。
                cache.start = entry.position;
                cache.offset =
                    Vec2::new(entry.position.x - position.x, entry.position.z - position.z);
            }
            2 => {
                // case2（Dragging）：地形高 `y=max(h,0)`；界内 → MoveTreeCmd
                // （载荷 0x18：index@+0、position Vec3@+8）；界外 →
                // **双事件** DisplayOutOfBorder + FeedbackInputOutOfBorder
                // （decomp 行 312-366）。原生无 `dragging==id` 门。
                let h = heights.sample_at_world_xz(target_x, target_z);
                let y = if h <= 0.0 { 0.0 } else { h };
                if border.0.is_pos_inside(Vec2::new(target_x, target_z)) {
                    moves.write(MoveTreeCmd {
                        index: id,
                        position: glam::Vec3::new(target_x, y, target_z),
                    });
                } else {
                    border.1.write(DisplayOutOfBorder);
                    border.2.write(FeedbackInputOutOfBorder);
                }
            }
            3 => {
                // case3（Released）：界内 → MoveTreeCmd + History::add
                // （TreeEdit Move disc=1：旧位 `cache.start` → previous_position、
                // 新位光标+偏移、id；decomp 行 367-417）。
                // 原生**不清 Local**（wave4 G6）。
                let h = heights.sample_at_world_xz(target_x, target_z);
                let y = if h <= 0.0 { 0.0 } else { h };
                let final_pos = glam::Vec3::new(target_x, y, target_z);
                if border.0.is_pos_inside(Vec2::new(target_x, target_z)) {
                    moves.write(MoveTreeCmd {
                        index: id,
                        position: final_pos,
                    });
                    let edit = EditType::Tree(std::sync::Arc::new(
                        country_core::resources::history::tree_edit::TreeEdit::Move(
                            country_core::resources::history::tree_edit::TreeEditData::for_move(
                                country_core::resources::tree_storage::TreeStorageIndex(id),
                                cache.start,
                                final_pos,
                            ),
                        ),
                    ));
                    let _ = history.add(edit);
                }
            }
            _ => {}
        }
    }
}
