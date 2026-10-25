//! ui_color_decorator.rs — 窗/旗装饰物颜色 UI（B 0x1422b0e00，2168B；
//! restorer-state-color 2026-10-17 fresh 反编译）。
//!
//! **未实现**（分歧点）：真实控制流 = 模式 12 守卫 → `RaycastWorld::
//! legacy_raycast_w_params()` 拾取 → 命中类型 `local_200`：
//! - 0xd/0xe：`DecoratorStorage::lookup_decorator_info` + `stairs_assembly_params::
//!   from` + hash 集（`hash_one<WidgetId>`）判定楼梯标志 `bVar21/bVar19`。
//! - 4：`decorator_info` → 子类型（+0x46-0x38）；`StairsState::graph()` →
//!   `IndexMap::get` 遍历连接节点 `StairSegment::new`。
//! 构建控件 `"color_decorator_"+index`；`is_active`：楼梯 '?' → `OutlineStairsCmd`，
//!   否则 outline 显示；点击 → `write<HideColorMenuCmd>` + 按子类型
//!   '8'/'9'/':'→3、';'→10、'<'→9、'='→6、'?'→`bVar21+0xb` → `write<ShowColorMenuCmd>`
//!   `(cVar9, cVar20, index)`。core legacy_raycast/StairsState graph/outline 面未对齐。

use bevy_ecs::prelude::*;

use country_core::resources::mouse_terrain_raycast::{CameraCursorRay, CursorPositionSS};
use country_core::resources::stairs::stairs_state::StairsState;
use country_core::resources::walls::decorator_storage::DecoratorStorage;
use country_core::systems::mode_manager::AppMode;
use country_core::systems::ui::resources::ui_state::UiState;
use country_core::systems::wall::structure_outline::OutlineStairsCmd;

use crate::{HideColorMenuCmd, ShowColorMenuCmd};

/// `ui_color_windows_and_flags` @0x1422b0e00（2168B，未实现）。
#[allow(clippy::too_many_arguments)]
pub fn ui_color_windows_and_flags(
    mode: Res<AppMode>,
    _ui: Res<UiState>,
    _storage: Res<DecoratorStorage>,
    _shows: EventWriter<ShowColorMenuCmd>,
    _hides: EventWriter<HideColorMenuCmd>,
    _cursor: Res<CursorPositionSS>,
    _outlines: EventWriter<OutlineStairsCmd>,
    _stairs: Res<StairsState>,
    _camera_ray: Res<CameraCursorRay>,
) {
    // O：模式 12 守卫。
    if mode.native_tag() == 0xc {
        // O：legacy_raycast 拾取 → 命中类型分派（0xd/0xe/4）→ ShowColorMenuCmd
        // 类别码 {3,6,9,10,11/12}；widget/stair-outline 面 GAP，未恢复。
    }
}
