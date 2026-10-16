//! tree_animation.rs — 树动画（B 0x142279500 handplaced / 0x14227a0d0 forest，O；
//! re-analyst-A 契约 `systems-recovery/groupA/forest/RE-ANALYSIS.md` §2）。
//!
//! - `handplaced_tree_animation`：手放树 canopy 相机距离淡出——相机视线与树
//!   canopy AABB 求交（`Aabb3::intersect_segment` 0x141240570），命中按距离写
//!   canopy 可见性（`pack_2x16f_uint` 0x140c90df0）+ `should_remove_tree_canopy_from_view`
//!   （行 152，内联）；AudioTriggerLibrary 触发（树风音频，0x14157ce90/40）。
//! - `forest_tree_animation`：森林树 sway/bend——`EasingFunction::apply`
//!   （0x140c95ef0 easing 表）+ 时间参数（1.0/0.3 常量）；`TreeMarkedForDeletion`
//!   树 → `DespawnPrefabCmd` + placements 修剪。

use bevy_ecs::prelude::*;

use country_core::components::{PrefabInstanceData, TreeAnimation, TreeMarkedForDeletion};
use country_core::resources::time::GameTime;
use country_core::resources::tree_storage::{InteractableForestPlacements, InteractableForestTree};
use country_core::systems::camera_rig::main_camera::MainCamera;
use country_core::systems::mode_manager::AppMode;
use country_core::systems::prefab::DespawnPrefabCmd;
use country_core::systems::ui::signifiers::signifier_create::UiSignifierStream;

/// 手放树 canopy 淡出距离（I：机器码距离阈值常量未显式解出；1.0/0.3 时间常量
/// 见 forest_tree_animation——handplaced 的阈值族待 re-analyst 补证）。

/// `should_remove_tree_canopy_from_view`（行 152，内联；O 名——canopy 距离判定）。
pub fn should_remove_tree_canopy_from_view(canopy_max_cull: f32, eye_to_tree: f32) -> bool {
    eye_to_tree > canopy_max_cull
}

/// `handplaced_tree_animation` @0x142279500（O 面：机器码 = puffin 域 + AppMode
/// 跳表分派（0x143038050）+ 相机-树 canopy 判定；canopy 实体切换/淡出帧的字段
/// 明细 U——`TreeAnimation._gap` 承载，宿主接线）。
#[allow(clippy::too_many_arguments)]
pub fn handplaced_tree_animation(
    mut trees: Query<(
        &mut TreeAnimation,
        &mut PrefabInstanceData,
        Option<&TreeMarkedForDeletion>,
    )>,
    _time: Res<GameTime>,
    _mode: Res<AppMode>,
    camera: Res<MainCamera>,
    _audio: Res<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>,
    _signifiers: Res<UiSignifierStream>,
    mut cull: ResMut<country_core::components::TreeCanopyMaxCullDistance>,
) {
    let eye = camera.eye_position();
    for (_animation, instance, marked) in trees.iter_mut() {
        if marked.is_some() {
            continue;
        }
        // O：相机距离 → canopy 剔除（Aabb3::intersect_segment 判定面 + 距离
        // 淡出；canopy 可见性写面 U）。
        let distance = (instance.position - eye).length();
        if should_remove_tree_canopy_from_view(cull.0, distance) {
            // 记录当前最大剔除距离（I：cull 为 f32 载荷，见 tree_edit 定义）。
            cull.0 = cull.0.max(distance);
        }
    }
}

/// `forest_tree_animation` @0x14227a0d0（O 面：4 参 = Query + GameTime +
/// DespawnPrefabCmd + placements；easing sway 驱动 + 标记树回收）。
pub fn forest_tree_animation(
    mut trees: Query<(
        Entity,
        &mut InteractableForestTree,
        &mut PrefabInstanceData,
        Option<&TreeMarkedForDeletion>,
    )>,
    _time: Res<GameTime>,
    mut despawns: EventWriter<DespawnPrefabCmd>,
    mut placements: ResMut<InteractableForestPlacements>,
) {
    for (entity, mut tree, _instance, marked) in trees.iter_mut() {
        if marked.is_some() {
            // O：标记树 → DespawnPrefabCmd + placements 修剪（机器码
            // 0x142279500 的回收族；实体映射 = TreeStorageEntry）。
            despawns.write(DespawnPrefabCmd {
                entity: Some(entity),
                _gap: tree.id,
            });
            placements.records.retain(|r| r.id != tree.id);
            continue;
        }
        // O：sway/bend 动画推进（EasingFunction::apply 0x140c95ef0 + 时间常量
        // 1.0/0.3；`InteractableForestTree` 相位字段 U——以 _gap 时间累加器落地，I）。
        tree._gap = tree._gap.wrapping_add(1);
    }
}
