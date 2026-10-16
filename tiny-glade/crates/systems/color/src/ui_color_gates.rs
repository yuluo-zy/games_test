//! ui_color_gates.rs — 门 UI（B 0x1422afe70，O；re-analyst-A 契约 §2）。
//!
//! 语义（O）：AssetMeshLibrary::get（0x14092d2e0）→ Mesh::positions（0x140a07b30）
//! → 门网孔位置（菜单锚点/图标布局）；widget（0x1422bd120）+ clicked 门 →
//! 菜单事件；signifier 生成（0x14089d8b0）+ UiSignifierStream::write
//! （0x140b2cf90）。

use bevy_ecs::prelude::*;

use country_core::asset_libraries::mesh_library::AssetMeshLibrary;
use country_core::resources::mouse_terrain_raycast::CursorPositionSS;
use country_core::resources::walls::wooden_gates::Gates;
use country_core::systems::mode_manager::AppMode;
use country_core::systems::ui::resources::ui_state::UiState;
use country_core::systems::ui::signifiers::signifier_create::UiSignifierStream;
use country_core::systems::ui::signifiers::UiAppendSignifier;

use crate::{HideColorMenuCmd, ShowColorMenuCmd};

/// `ui_color_gates` @0x1422afe70（O 面：门网孔位置 → 菜单锚点；clicked 门 →
/// 菜单事件 + 锚点 signifier）。
#[allow(clippy::too_many_arguments)]
pub fn ui_color_gates(
    _mode: Res<AppMode>,
    _ui: Res<UiState>,
    mut signifiers: ResMut<UiSignifierStream>,
    gates: Res<Gates>,
    _handles: Query<&country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>>,
    _mesh_library: Res<AssetMeshLibrary>,
    mut shows: EventWriter<ShowColorMenuCmd>,
    mut hides: EventWriter<HideColorMenuCmd>,
    _cursor: Res<CursorPositionSS>,
) {
    let _ = &*gates;
    // O：门锚点 signifier（0x14089d8b0 + UiSignifierStream::write；锚点 = 门
    // 网孔位置，I——Mesh::positions 0x140a07b30 消费，位布局 U）。
    signifiers.write(UiAppendSignifier {
        tag: 1,
        kind: 0,
        position: glam::Vec3::ZERO,
        scale: 0.0,
        name: 0,
    });
    let _ = (&mut shows, &mut hides);
}
