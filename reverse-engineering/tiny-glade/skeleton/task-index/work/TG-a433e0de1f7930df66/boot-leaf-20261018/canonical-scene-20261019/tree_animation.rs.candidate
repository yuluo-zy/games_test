//! Original B handplaced142279500/3017B and forest14227a0d0/2228B.
//! Both systems update canonical Placement48. Shared canopy progress/smoothstep
//! are a Resource consumed by the actual tree draw callbacks. The tag9 branch
//! subtracts only for EraserLayer::Tree; segment intersection is bounded to [0,1].

use bevy_ecs::prelude::*;
use glam::Vec3;

use country_core::asset_libraries::audio_trigger_library::{AudioTriggerKind, AudioTriggerLibrary};
use country_core::components::{TreeAnimation, TreeMarkedForDeletion};
use country_core::render::nani::instance_updates::pack_2x16f_uint;
use country_core::resources::time::GameTime;
use country_core::resources::tree_storage::{InteractableForestPlacements, InteractableForestTree};
use country_core::systems::camera_rig::main_camera::MainCamera;
use country_core::systems::mode_manager::AppMode;
use country_core::systems::prefab::DespawnPrefabCmd;
use country_core::systems::prefab::PrefabInstanceData;
use country_core::systems::ui::signifiers::signifier_create::UiSignifierStream;
use utils::easing_functions::EasingFunction;

/// Compatibility name for the one canonical resource; no Local surrogate.
pub use country_core::resources::history::tree_edit::TreeCanopyMaxCullDistance
    as HandplacedCanopyState;

/// `should_remove_tree_canopy_from_view`（源行 152，内联；O 名——canopy 可见性判定）。
///
/// ⚠ 内联判定条件（阈值/距离来源）未单独恢复（U）；此处保留合同名，
/// 以 canopy 淡出值是否为 0 落地（登记，不作为真实判定）。
pub fn should_remove_tree_canopy_from_view(fade: f32) -> bool {
    fade == 0.0
}

/// `handplaced_tree_animation` @0x142279500（O；8 参面：7 个 PDB 参数 + 第 8 个
/// change-detection 状态由 native 通过 `ResMut<TreeCanopyMaxCullDistance>` 承载）。
#[allow(clippy::too_many_arguments)]
pub fn handplaced_tree_animation(
    mut trees: Query<(
        &mut TreeAnimation,
        &mut PrefabInstanceData,
        Option<&TreeMarkedForDeletion>,
    )>,
    time: Res<GameTime>,
    mode: Res<AppMode>,
    camera: Res<MainCamera>,
    audio: Res<AudioTriggerLibrary>,
    _signifiers: Res<UiSignifierStream>,
    mut canopy: ResMut<HandplacedCanopyState>,
) {
    let delta = time.delta;

    // ---------------------------------------------------------------- AppMode 分派（O）
    // 跳表 0x143038050 16 项：加集 {6,7,8,11,12,15,17,18,19}、减集
    // {9,10,13,14,16,20}；兜底项 tag==0||3 → 减、否则加。
    let subtract = matches!(mode.native_tag(), 0 | 3 | 10 | 13 | 14 | 16 | 20)
        || matches!(*mode, AppMode::Eraser(
            country_core::systems::mode_manager::EraserLayer::Tree));

    // 全局 canopy：`progress = clamp(progress ± delta, 0, 1)`；`smoothstep = t²(3-2t)`。
    let mut progress = canopy.progress;
    if subtract {
        progress -= delta;
    } else {
        progress += delta;
    }
    // MAXSS(0, progress), MINSS(1, progress): an unordered source is
    // retained by each instruction; do not change this to f32::clamp.
    let low = if 0.0 > progress { 0.0 } else { progress };
    progress = if 1.0 < low { 1.0 } else { low };
    let smooth = (progress * progress) * (3.0 - (progress + progress));
    canopy.progress = progress;
    canopy.smoothstep = smooth;

    // ---------------------------------------------------------------- 相机射线 × 音频触发盒
    // O：eye_position（0x140c96d40）+ eye_direction（0x140c96d80）→ 线段端点
    //   eye + dir * 4.5（dir.xy ×3.0×1.5、dir.z ×3.0×1.5）。
    let eye = camera.eye_position();
    let dir = camera.eye_forward();
    let ray_end = eye + (dir * 3.0) * 1.5;
    // O：逐盒（原生记录 stride 0x1c）：`tag@+0x18` bit0 置位跳过、tag==0 提前结束；
    //   `Aabb3` 扩展 min+(-2.5,-1.0,-2.5)、max+(2.5,1.0,2.5)，再
    //   `Aabb3::intersect_segment`。core `boxes()` 为 slotmap 迭代、无 tag 位，
    //   过滤面登记 U；扩展常量按 fresh 反汇编修正。
    let mut audio_hit = false;
    'triggers: for (_handle, bbox) in audio.get_triggers(AudioTriggerKind::Primary).boxes() {
        if (utils::geometry::Aabb3 {
            min: bbox.min + Vec3::new(-2.5, -1.0, -2.5),
            max: bbox.max + Vec3::new(2.5, 1.0, 2.5),
        }).intersect_segment(eye, ray_end).is_some() {
            audio_hit = true;
            break 'triggers;
        }
    }

    // ---------------------------------------------------------------- 逐树循环（O）
    // 进度属于实体；查询顺序改变、实体删除或切换 archetype 不会串用别的树状态。
    for (mut animation, mut instance, marked) in trees.iter_mut() {
        let changed = animation.advance(delta, marked.is_some(), !(subtract && !audio_hit));
        if changed {
            let fade = animation.canopy_visibility;
            instance.0.anim_t_packed =
                pack_2x16f_uint(animation.progress, 1.0 - (1.0 - fade) * (1.0 - fade));
        }
    }
}

/// `forest_tree_animation` @0x14227a0d0（O；4 参 = Query + GameTime + DespawnPrefabCmd +
/// placements）。
pub fn forest_tree_animation(
    mut trees:Query<(Entity,&mut InteractableForestTree,&mut PrefabInstanceData,Option<&TreeMarkedForDeletion>)>,
    time:Res<GameTime>,mut despawns:EventWriter<DespawnPrefabCmd>,
    mut placements:ResMut<InteractableForestPlacements>,
) {
    for (entity,mut tree,mut instance,marked) in &mut trees {
        let (step,growing)=if marked.is_some() {(time.delta / -0.3,false)}
        else if tree.delay>0.0 {tree.delay-=time.delta;(0.0,true)}
        else {(time.delta+time.delta,true)};
        let previous=tree.progress;
        // Native14227a68a..69f MAXSS 0,raw / MINSS 1,result retains NaN/-0.
        let raw=step+tree.progress;
        let low=if 0.0>raw {0.0}else{raw};
        tree.progress=if 1.0<low {1.0}else{low};
        if tree.progress!=previous {
            instance.0.scale=EasingFunction::CubicOut.apply(tree.progress)*tree.amplitude;
        }
        if !growing && tree.progress==0.0 {
            despawns.write(DespawnPrefabCmd{entity:Some(entity),_gap:0});
            // Original writes the record Entity's generation@44=0. The same
            // indexed owner must be released before the next manage pass.
            placements.records[tree.placement_index].entity=None;
        }
    }
}
