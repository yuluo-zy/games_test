//! ui_color_gates.rs — 门颜色 UI（B 0x1422afe70，2600B；restorer-state-color
//! 2026-10-17 fresh 反编译）。
//!
//! **未实现**（分歧点）：真实控制流 = 模式 12 守卫 → 稀疏墙实体位集（每 0x20B
//! 一组 16×16）迭代 → `PublicWalls`（stride 0x14）→ `AssetMeshLibrary::get` →
//! `Mesh::positions`（顶点/3=三角形数）→ `fold` 产出 `Triangle` 列表（拾取用）→
//! `format!("color_gates_"+WallId)` → 控件构建（WidgetParams + `UiState::add
//! <String>`）→ `is_active` 时 `SignifierFactory::new` + `UiSignifierStream::write`
//! （mesh 顶点数据）→ 点击写命令（tag=5）。core mesh/signifier/widget 面未对齐。

use bevy_ecs::prelude::*;

use country_core::resources::mouse_terrain_raycast::CursorPositionSS;
use country_core::resources::walls::wooden_gates::Gates;
use country_core::systems::mode_manager::AppMode;
use country_core::systems::ui::resources::ui_state::UiState;
use country_core::systems::ui::signifiers::signifier_create::UiSignifierStream;

use crate::{HideColorMenuCmd, ShowColorMenuCmd};

/// `ui_color_gates` @0x1422afe70（2600B，未实现）。
#[allow(clippy::too_many_arguments)]
pub fn ui_color_gates(
    mode: Res<AppMode>,
    _ui: Res<UiState>,
    _signifiers: ResMut<UiSignifierStream>,
    _gates: Res<Gates>,
    _mesh_library: Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,
    _shows: EventWriter<ShowColorMenuCmd>,
    _hides: EventWriter<HideColorMenuCmd>,
    _cursor: Res<CursorPositionSS>,
) {
    // O：模式 12 守卫。
    if mode.native_tag() == 0xc {
        // O：稀疏墙位集 → mesh 三角形 → 点击写 tag-5 命令（widget/signifier
        // 面 GAP，未恢复）。
    }
}
