//! tree_animation.rs — 树动画（B `handplaced_tree_animation` 0x142279500 /
//! `forest_tree_animation` 0x14227a0d0，O；2026-10-17 fresh 反编译 + 反汇编复核）。
//!
//! - `handplaced_tree_animation`：AppMode 跳表分派（0x143038050：标签
//!   {6,7,8,11,12,15,17,18,19} 加、{9,10,13,14,16,20} 减、兜底 tag 0/3 减其余加）
//!   → 全局 canopy 生长状态按 `± delta` 推进并做 smoothstep（`t²(3-2t)`，常量 3.0 =
//!   0x1429cd078）→ 相机眼点/朝向射线 × 音频触发盒（`AudioTriggerCollection::boxes` +
//!   `Aabb3::intersect_segment`；fresh 复核盒扩展 = min+(-2.5,-1.0,-2.5)、
//!   max+(2.5,1.0,2.5)）→ 逐树进度/淡出推进（`delta / 自定时长` 或
//!   `delta / -0.4`（0x142a7fcb0））→ `pack_2x16f_uint`（0x140c90df0）写
//!   `PrefabInstanceData+0x20`。
//! - `forest_tree_animation`：逐树一阶生长/收缩（标记树 `delta / -0.3`，普通树带延迟计时器
//!   门控：timer>0 递减、≤0 以 2×delta 加速）→ `clamp(progress+step, 0, 1)` → 进度变化时
//!   `EasingFunction::CubicOut.apply(progress)` × 幅度写 `PrefabInstanceData[+0xc]` → 衰减到
//!   0 发 `DespawnPrefabCmd`（payload = 打包 Entity）并清放置记录 `+0x44` 有树标志。
//!
//! ## fresh 复核（2026-10-17 restorer-tree-forest-remaining）
//! - AppMode 跳表 0x143038050 16 项解码：{6,7,8,11,12,15,17,18,19}→加、
//!   {9,10,13,14,16,20}→减、index0（兜底）→0x142279837（tag==0||3 减，否则加）。
//!   Rust 判定集合判等。
//! - `TreeAnimation` 行 stride **0x10**：时长@0、进度@+4、淡出@+8、标志@+0xc；
//!   `PrefabInstanceData` 行 stride **0x30**：位置 x@0/y@+4/z@+8，写 `+0xc`/`+0x20`。
//! - 全局 canopy 状态由第 7 参 `ResMut<TreeCanopyMaxCullDistance>` 的值承载
//!   （`{progress@0, smoothstep@4}`）——core 该资源仅 `f32`，本实现以
//!   `Local<HandplacedCanopyState>` 承载（登记 I）。
//! - `forest_tree_animation` 的 progress 钳制在机器码是**分支**（`0<=x?x:0`、
//!   `r<=1?r:1`），NaN 时得 0（Rust `f32::clamp` 会保留 NaN）→ 已按分支复刻。
//! - DespawnPrefabCmd 原生载荷仅 8 B（打包 Entity）；core 类型多一个 `_gap`，
//!   原生不写 → 本实现写 0（不再伪造 tree.id）。
//!
//! ## ⚠ 与当前 country_core 类型的分歧（未实现面，详见 RESTORE.md）
//!
//! `TreeAnimation` 真实 stride 0x10 {+0 自定时长, +4 进度, +8 淡出, +0xc 标志}；当前
//! `{alpha f32, _gap u64}` 缺 `+4` 进度位。`PrefabInstanceData` 真实 stride 0x30（本函数写
//! `+0xc` / `+0x20`）；当前 0x58 Vec 布局。`InteractableForestTree` 真实 stride 0x18
//! {+0x8 幅度, +0xc 进度, +0x10 延迟计时器}——当前 `position: Vec3` 的 x/y/z 偏移恰好对齐
//! （+0x8/+0xc/+0x10），语义名分歧（以 position 承载）。`Aabb3::intersect_segment` 原为
//! 3D AABB 线段求交（0x141240570），当前 utils 无此方法（本地等效实现落地并标注）。

use bevy_ecs::prelude::*;
use glam::Vec3;

use country_core::asset_libraries::audio_trigger_library::{
    AudioTriggerKind, AudioTriggerLibrary,
};
use country_core::components::{PrefabInstanceData, TreeAnimation, TreeMarkedForDeletion};
use country_core::render::nani::instance_updates::pack_2x16f_uint;
use country_core::resources::time::GameTime;
use country_core::resources::tree_storage::{InteractableForestPlacements, InteractableForestTree};
use country_core::systems::camera_rig::main_camera::MainCamera;
use country_core::systems::mode_manager::AppMode;
use country_core::systems::prefab::DespawnPrefabCmd;
use country_core::systems::ui::signifiers::signifier_create::UiSignifierStream;
use utils::easing_functions::EasingFunction;

/// 手放树逐树进度跟踪（分歧 I：真实进度存于 `TreeAnimation[+4]`；当前 country_core 类型缺
/// 该位，以 `Local<Vec<f32>>` 按查询迭代序承载——树集稳定时行为等价）。
#[derive(Default)]
pub struct HandplacedTreeProgress {
    /// 每树进度（按查询迭代序索引）。
    pub per_tree: Vec<f32>,
}

/// 手放树 canopy 全局生长状态（分歧 I：真实为 `ResMut<TreeCanopyMaxCullDistance>` 的
/// **值结构** `{progress f32 @+0, smoothstep f32 @+4}`（第 7 参 change-detection 包装）；
/// country_core 该资源仅 `f32`，以 forest-local 承载并登记）。
#[derive(Default)]
pub struct HandplacedCanopyState {
    /// 全局生长进度（O：`[state] += ±delta`，clamp[0,1]）。
    pub progress: f32,
    /// smoothstep 输出（O：`[state+4] = t²(3-2t)`）。
    pub smoothstep: f32,
}

/// `should_remove_tree_canopy_from_view`（源行 152，内联；O 名——canopy 可见性判定）。
///
/// ⚠ 内联判定条件（阈值/距离来源）未单独恢复（U）；此处保留合同名，
/// 以 canopy 淡出值是否为 0 落地（登记，不作为真实判定）。
pub fn should_remove_tree_canopy_from_view(fade: f32) -> bool {
    fade == 0.0
}

/// 3D AABB 线段求交（分歧 I：原 `Aabb3::intersect_segment` 0x141240570 在 utils 重建中
/// 未落地；此实现为 segment-vs-AABB 相交测试，语义 = 线段任一端点入盒或穿盒）。
fn aabb3_intersect_segment(min: Vec3, max: Vec3, start: Vec3, end: Vec3) -> bool {
    let dir = end - start;
    let tmin = (min - start) / dir;
    let tmax = (max - start) / dir;
    let lo = tmin.min(tmax);
    let hi = tmax.max(tmin);
    let t0 = lo.max_element();
    let t1 = hi.min_element();
    t0 <= t1 && t1 >= 0.0
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
    mut canopy: Local<HandplacedCanopyState>,
    mut per_tree: Local<HandplacedTreeProgress>,
) {
    let delta = time.delta;

    // ---------------------------------------------------------------- AppMode 分派（O）
    // 跳表 0x143038050 16 项：加集 {6,7,8,11,12,15,17,18,19}、减集
    // {9,10,13,14,16,20}；兜底项 tag==0||3 → 减、否则加。
    let subtract = matches!(mode.native_tag(), 0 | 3 | 9 | 10 | 13 | 14 | 16 | 20);

    // 全局 canopy：`progress = clamp(progress ± delta, 0, 1)`；`smoothstep = t²(3-2t)`。
    let mut progress = canopy.progress;
    if subtract {
        progress -= delta;
    } else {
        progress += delta;
    }
    progress = progress.clamp(0.0, 1.0);
    let smooth = progress * progress * (3.0 - 2.0 * progress);
    canopy.progress = progress;
    canopy.smoothstep = smooth;

    // ---------------------------------------------------------------- 相机射线 × 音频触发盒
    // O：eye_position（0x140c96d40）+ eye_direction（0x140c96d80）→ 线段端点
    //   eye + dir * 4.5（dir.xy ×3.0×1.5、dir.z ×3.0×1.5）。
    let eye = camera.eye_position();
    let dir = camera.eye_forward();
    let ray_end = eye + dir * 4.5;
    // O：逐盒（原生记录 stride 0x1c）：`tag@+0x18` bit0 置位跳过、tag==0 提前结束；
    //   `Aabb3` 扩展 min+(-2.5,-1.0,-2.5)、max+(2.5,1.0,2.5)，再
    //   `Aabb3::intersect_segment`。core `boxes()` 为 slotmap 迭代、无 tag 位，
    //   过滤面登记 U；扩展常量按 fresh 反汇编修正。
    let mut audio_hit = false;
    'triggers: for (_handle, bbox) in audio.get_triggers(AudioTriggerKind::Primary).boxes() {
        if aabb3_intersect_segment(
            bbox.min + Vec3::new(-2.5, -1.0, -2.5),
            bbox.max + Vec3::new(2.5, 1.0, 2.5),
            eye,
            ray_end,
        ) {
            audio_hit = true;
            break 'triggers;
        }
    }

    // ---------------------------------------------------------------- 逐树循环（O）
    // TreeAnimation 真实 stride 0x10 {+0 时长, +4 进度, +8 淡出, +0xc 标志字节}；
    // core 型 {alpha f32 @0, _gap u64 @8} 缺 +4 → 进度以 `Local<Vec<f32>>` 承载
    // （分歧 I），`+8` 淡出以 `_gap` 低 32 位承载、`+0xc` 标志以 `_gap` 高字节承载。
    let mut idx = 0usize;
    for (mut animation, _instance, marked) in trees.iter_mut() {
        // O：step = 标记树或 `+0xc` 标志非 0 → `delta / -0.4`（0x142a7fcb0）；
        //      否则 `delta / [anim+0]`（每树自定时长）。
        let step = if marked.is_some() || (animation._gap >> 32) as u8 != 0 {
            delta / -0.4
        } else {
            delta / animation.alpha
        };
        // O：`progress = clamp(progress + step, 0, 1)`（写回真实 `[anim+4]`；此处写 Local）。
        let old_progress = per_tree.per_tree.get(idx).copied().unwrap_or(0.0);
        let new_progress = (old_progress + step).clamp(0.0, 1.0);
        if per_tree.per_tree.len() <= idx {
            per_tree.per_tree.resize(idx + 1, 0.0);
        }
        per_tree.per_tree[idx] = new_progress;
        // O：`at_max = (progress == 1.0)`（UCOMISS vs 1.0；SETZ/SETNP）。
        let at_max = new_progress == 1.0;
        // O：淡出分支（0x142279d75/d90）：`!(subtract & !audio_hit) & at_max`
        //   → 淡出 += delta（min 1）；否则淡出 -= delta（clamp[0,1]）。
        let old_fade = f32::from_bits(animation._gap as u32).clamp(0.0, 1.0);
        let fade = if !(subtract && !audio_hit) && at_max {
            (old_fade + delta).min(1.0)
        } else {
            (old_fade - delta).clamp(0.0, 1.0)
        };
        animation._gap = u64::from(f32::to_bits(fade));
        // O：变化检测（progress/fade 与旧值比，含 NaN）→
        // `pack_2x16f_uint(progress, 1-(1-fade)²)` 写 `PrefabInstanceData[+0x20]`（真实）。
        // ⚠ 权威布局修正后 core `PrefabInstanceData` 无该字段 → **未实现
        // （core-type-gap，登记）**；打包值仍按原式计算。
        if new_progress != old_progress || fade != old_fade {
            let _packed = pack_2x16f_uint(new_progress, 1.0 - (1.0 - fade) * (1.0 - fade));
        }
        // O：写 `[anim+0xc] = 0` 清标志（由 `_gap` 高 32 位写 0 隐含）。
        idx += 1;
    }
}

/// `forest_tree_animation` @0x14227a0d0（O；4 参 = Query + GameTime + DespawnPrefabCmd +
/// placements）。
pub fn forest_tree_animation(
    mut trees: Query<(
        Entity,
        &mut InteractableForestTree,
        &mut PrefabInstanceData,
        Option<&TreeMarkedForDeletion>,
    )>,
    time: Res<GameTime>,
    mut despawns: EventWriter<DespawnPrefabCmd>,
    mut placements: ResMut<InteractableForestPlacements>,
) {
    let delta = time.delta;
    for (entity, mut tree, _instance, marked) in trees.iter_mut() {
        // ⚠ InteractableForestTree 真实 stride 0x18 {+0x8 幅度, +0xc 进度, +0x10 计时器}；
        // 当前 core 以 `position: Vec3` 的 x/y/z 承载——偏移恰好对齐（+0x8/+0xc/+0x10），
        // 语义名分歧（进度/计时器语义 I，见 RESTORE.md §3）。
        let amplitude = tree.position.x;
        let timer = tree.position.z;

        // O：step 与 growing：
        //   标记树 → `step = delta / -0.3`（0x1429258fc = -0.3），growing=false；
        //   否则 timer>0 → `timer -= delta`，step=0，growing=true；
        //        timer≤0 → `step = delta + delta`（2×delta），growing=true。
        let (step, growing) = if marked.is_some() {
            (delta / -0.3, false)
        } else if timer > 0.0 {
            tree.position.z = timer - delta;
            (0.0, true)
        } else {
            (delta + delta, true)
        };

        let old_progress = tree.position.y;
        // O：`progress = clamp(progress + step, 0, 1)` —— 原机器码为**分支**
        //   `if 0.0 <= x { x } else { 0.0 }`、`if r <= 1.0 { r } else { 1.0 }`
        //   （NaN 得 0），与 `f32::clamp`（NaN 传播）不同；按分支复刻。
        let x = tree.position.y + step;
        let r = if 0.0 <= x { x } else { 0.0 };
        tree.position.y = if r <= 1.0 { r } else { 1.0 };

        if tree.position.y != old_progress {
            // O：`EasingFunction::apply(progress)`（self = 静态 0x14303802c 判别值
            // 0x06 = CubicOut，0x140c95ef0）× 幅度 → 写 `PrefabInstanceData[+0xc]`。
            // ⚠ 权威布局修正后 `PrefabInstanceData` 无该承载位 → **未实现
            // （core-type-gap，登记）**；缓动值仍按原式计算。
            let _eased = EasingFunction::CubicOut.apply(tree.position.y) * amplitude;
        }

        // O：`!growing && progress == 0.0`（NaN 不命中）→ `DespawnPrefabCmd`
        // payload = 打包 Entity（8 B；核心类型多出的 `_gap` 原生不写 → 0）+ 清记录 +0x44。
        if !growing && tree.position.y == 0.0 {
            despawns.write(DespawnPrefabCmd {
                entity: Some(entity),
                _gap: 0,
            });
            // ⚠ 真实：`placements.records[tree.id].+0x44 = 0`（越界 panic）；当前
            // country_core 记录无 `+0x44` 位 → 未实现面（保留结构、不伪造字段）。
            let _ = &mut placements;
        }
    }
}
