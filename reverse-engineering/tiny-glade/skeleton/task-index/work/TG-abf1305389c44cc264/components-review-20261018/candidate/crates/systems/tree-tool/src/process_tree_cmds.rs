//! process_tree_cmds.rs — 树命令处理器（B 0x142267430 process_tree_cmds /
//! 0x1422686d0 despawn_trees，O；2026-10-17 fresh 反编译 + wave3 落库）。
//!
//! - `process_tree_cmds`：消费 MoveTreeCmd/DeleteTreeCmd/InsertTreeCmd。
//! - `despawn_trees`：`Query<(Entity, &TreeAnimation, &TreeMarkedForDeletion)>`，
//!   α **严格小于** 0.01（0x142960674 = 0x3c23d70a；`<=0.01 && !=0.01`）→
//!   `DespawnPrefabCmd{entity}`。
//!
//! ## wave3（2026-10-17）落库
//! - **Move 分支**：MoveTreeCmd 0x20 载荷（coregap-1 已落库）→ 全实现。逐事件
//!   `TreeStorage::get(index@+8)`；None → 日志 "MoveTreeCmd references a
//!   non-existing tree"（decomp 0x142267430 行 247-270）；Some → 定位
//!   PrefabInstanceData 写新位置 → 水面采样（bilinear_sample_at_world_xz）→
//!   `PlayTreeSoundCmd{kind=1, position}`（真实槽 stride 0x20 另有
//!   in_water@+9 / inverse_lerp@+0xc——共享类型缺这两个字段，见分歧 T1）。
//! - **Delete 分支**：补 `Entities::contains(entity)` 门控（decomp 行 835）——
//!   原 `Commands::entity` 无条件入队是近似（分歧 18），现以
//!   `entities.contains` 判等。
//! - **Insert 分支**：2026-10-17 consume-types 按 fresh 反编译重建（名称门 /
//!   deny_load 门 / α / spawn 四组件 / prefab 兜底 / storage.insert / 水面 /
//!   PlayTreeSoundCmd kind=0 / 冲突 DespawnPrefabCmd）。事件槽真实 stride 0x48
//!   （event base 布局见函数内注释）；共享 `InsertTreeCmd` 仅承载
//!   name/position/scale/id，rotation/variant/flags/height 与部分 SpawnPrefabCmd
//!   输入仍未落库 → 分歧 T2/T4 显式登记；SpawnPrefabCmd 0x48 载荷已落（T5 闭合）。

use bevy_ecs::entity::Entities;
use bevy_ecs::prelude::*;

use country_core::components::{
    TreeAnimation, TreeComponent, TreeMarkedForDeletion, TreeMoveBend,
};
use country_core::resources::history::tree_edit::{DeleteTreeCmd, InsertTreeCmd};
use country_core::resources::session::ActiveSession;
use country_core::resources::time_of_day::inverse_lerp;
use country_core::resources::tree_storage::{
    MoveTreeCmd, TreeStorage, TreeStorageEntry, TreeStorageIndex,
};
use country_core::resources::world_raster_defs::WaterRaster;
use country_core::systems::audio::tree_audio::TreeAmbience;
use country_core::systems::audio::PlayTreeSoundCmd;
use country_core::systems::prefab::{DespawnPrefabCmd, SpawnPrefabCmd};
use country_core::systems::save::load_completion_checker::LoadingCompletionState;
use country_core::utils::components::transform::Transform;

use crate::ui_place_tree::random_tree_prefab;

/// despawn α 阈值（O：0x142960674 = 0x3c23d70a = 0.01）。判等：despawn_trees
/// 反编译行 75/122 `*pf <= 0.01 && *pf != 0.01` —— **严格小于**（排除 NaN）。
pub const DESPAWN_ALPHA_THRESHOLD: f32 = 0.01;

/// `process_tree_cmds` @0x142267430（O 面；fresh 反编译 2026-10-17，wave3 落库
/// Move/Delete 分支）。
///
/// 分支：Move（槽 stride 0x20）→ Insert（槽 stride 0x48）→ Delete（槽 stride
/// 0x10），与源码行 43-74 / 74-176 / 182-201 一致。
///
/// ## 分歧（2026-10-17 consume-types 复核后）
/// - T1（已闭合）：`PlayTreeSoundCmd` 现承载 kind/selector/extra/value/position
///   （槽 stride 0x20 逐偏移对应），Move/Insert 两分支均按机器码写满。
/// - T2：`InsertTreeCmd` 实际 0x48 槽 = name/position/scale/rotation/variant/
///   flags/id/height；共享型只落 name/position/scale/id → rotation/variant/
///   flags/height 写面缺失（Insert 分支仅这四字段不可观察）。
/// - T4：Insert 的 `is_corrupted` 宿主资源容器未取证（以 false 登记）；
///   `random_tree_prefab` 的 drag_started 输入同理。selector 已按机器码取
///   `cmd.id & 1`（wave4 G4 修正，非缺口）。
/// - T5（已闭合）：`SpawnPrefabCmd` 0x48 载荷已落 core（prefab/entity/pos/
///   scale/rotation/variant/1.0/flags/mode），Insert 分支逐字段写入；rotation/
///   variant 因 T2 以 0 登记。
#[allow(clippy::too_many_arguments)]
pub fn process_tree_cmds<S: Send + Sync + 'static>(
    mut storage: ResMut<TreeStorage>,
    mut transforms: Query<&mut Transform>,
    mut moves: EventReader<MoveTreeCmd>,
    mut deletes: EventReader<DeleteTreeCmd>,
    mut inserts: EventReader<InsertTreeCmd>,
    mut spawns: EventWriter<SpawnPrefabCmd>,
    mut commands: Commands,
    mut sounds: EventWriter<PlayTreeSoundCmd>,
    session: Res<ActiveSession>,
    water: Res<WaterRaster>,
    mut despawns: EventWriter<DespawnPrefabCmd>,
    entities: &Entities,
    readiness: Option<Res<LoadingCompletionState>>,
) {
    // ---- Move 分支（O：槽 stride 0x20；decomp 行 187-325）。wave3 落库。
    for cmd in moves.read() {
        if let Some(entry) = storage.get(TreeStorageIndex(cmd.index)) {
            // O：实体定位实例变换（decomp 行 272-299 经 decorator/wall 表寻
            // 0x30 B 变换记录；语义等价 = 按 entry.entity 查 `Transform`）→ 写新
            // 位置。原 `unwrap_failed` panic 路径（实体失效/未注册）以 get_mut
            // Ok 兜底登记（分歧：graceful vs panic）。
            let mut size_ratio = 0.0;
            if let Ok(mut data) = transforms.get_mut(entry.entity) {
                // O：写 `位置@+0..0xc`（asm 14226785e/142267863）。
                data.translation = cmd.position;
                // O：`inverse_lerp(0.5, 0.9, 变换记录 scale@+0xc)`
                // （asm 14226786a-142267877；常量 0x142925870/0x142a7c88c）。
                size_ratio = inverse_lerp(0.5, 0.9, data.scale.x);
            }
            // O：水面采样（decomp 行 300-303）。结果 `0.0 < level` 是
            // PlayTreeSoundCmd selector@+9；value@+0xc = inverse_lerp 结果。
            let pos = cmd.position;
            let water_level = (**water)
                .as_ref()
                .bilinear_sample_at_world_xz(pos.x, pos.z, 130.0, 130.0);
            // O：PlayTreeSoundCmd kind=1（decomp 行 314-323 / asm
            // 1422678ab-1422678e3：kind@+8=1、selector@+9=in_water、value@+0xc）。
            sounds.write(PlayTreeSoundCmd {
                kind: 1,
                selector: (water_level > 0.0) as u8,
                extra: 0,
                value: size_ratio,
                position: pos,
            });
        } else {
            // O：`TreeStorage::get` None → 日志 "MoveTreeCmd references a
            // non-existing tree"（decomp 行 247-270，log level 门控）。
            log::warn!("MoveTreeCmd references a non-existing tree");
        }
    }
    // ---- Delete 分支（O：槽 stride 0x10；decomp 行 819-896）。wave3 补
    // `Entities::contains` 门控（行 835）。
    for cmd in deletes.read() {
        if let Some(entry) = storage.remove(TreeStorageIndex(cmd.id)) {
            // O：`Entities::contains(entity)` 门控（decomp 行 835）——实体仍存在
            // 才推入 TreeMarkedForDeletion；否则跳过（原以 Commands::entity 直接
            // 排队是近似，分歧 18，已判等修复）。
            if entities.contains(entry.entity) {
                commands
                    .entity(entry.entity)
                    .insert(TreeMarkedForDeletion);
                // O：kind=2 + 树变换记录位置/scale（decomp 行 877-896：
                // inverse_lerp(0.5,0.9,scale) → kind@+8=2、value@+0xc、
                // position@+0x10/+0x18）。
                if let Ok(data) = transforms.get(entry.entity) {
                    sounds.write(PlayTreeSoundCmd {
                        kind: 2,
                        selector: 0,
                        extra: 0,
                        value: inverse_lerp(0.5, 0.9, data.scale.x),
                        position: data.translation,
                    });
                }
            }
        }
    }
    // ---- Insert 分支（O：槽 stride 0x48；decomp 0x142267430 行 326-785）。
    //
    // 2026-10-17 consume-types：按 fresh 反编译逐语句重建。事件字段锚（element
    // base = 事件 base + 8 的 Bevy 分叉 EventInstance 头）：
    // `name String@+0`、`position x@+0x10/y@+0x14/z@+0x18`、`scale@+0x1c`、
    // `rotation@+0x20`、`variant@+0x24`、`flags@+0x28`、`id@+0x30`、`height@+0x38`。
    // 共享 `InsertTreeCmd` 仅承载 name/position/scale/id → rotation/variant/
    // flags/height 四处 type gap 登记（T2），下方以显式注释标出。
    //
    // 名称门（decomp 行 341-521；字符串直读 0x143034d30/d50/d60/f28/f4f）：
    // `naked = name ∈ {clearing/naked_tree/large(25)、…/naked1(26)、…/naked2(26)、
    // …/naked1_no_mushrooms(39)、…/naked2_no_mushrooms(39)}`。
    for cmd in inserts.read() {
        let name = cmd.prefab_name.as_str();
        let naked = matches!(
            name,
            "clearing/naked_tree/large"
                | "clearing/naked_tree/naked1"
                | "clearing/naked_tree/naked2"
                | "clearing/naked_tree/naked1_no_mushrooms"
                | "clearing/naked_tree/naked2_no_mushrooms"
        );
        // O：`SessionLoadReadiness::deny_load`（0x140a9fa70）与
        // `BackupSaveFileKind::is_corrupted`（0x140ae8320）。readiness 取值面
        // 已接线；is_corrupted 的宿主资源容器未取证 → 以 false 登记（分歧 T4）。
        let deny_load = readiness
            .as_deref()
            .map(|state| state.readiness.deny_load())
            .unwrap_or(false);
        let is_corrupted = false;
        // O：`MAXSS(scale, 0.7)` 仅在 `naked && deny_load` 时生效（decomp 行
        // 526-534 / asm 142267be3-142267c08）；否则保持事件 scale。
        let scale = if naked && deny_load {
            cmd.scale.max(0.7)
        } else {
            cmd.scale
        };
        // O：动画 α = inverse_lerp(0.5, 0.9, scale)*0.75 + (1−r)*0.55
        // （常量 0x142925870=0.5、0x142a7c88c=0.9、0x1429edb90=0.75、
        // 0x142a877a0=0.55）。
        let r = inverse_lerp(0.5, 0.9, cmd.scale);
        let alpha = r * 0.75 + (1.0 - r) * 0.55;
        let pos = cmd.position;
        // O：`Commands::spawn_empty` + 逐条 insert（decomp 行 540-609，四段
        // 命令队列，顺序 TreeComponent → TreeAnimation → TreeMoveBend →
        // TreeAmbience）。
        let entity = commands.spawn_empty().id();
        // 当前 InsertTreeCmd 尚无原版 is_preplaced 字段；这条手放入口写 false。
        commands.entity(entity).insert(TreeComponent {
            id: cmd.id,
            is_preplaced: false,
        });
        commands.entity(entity).insert(TreeAnimation::new(alpha));
        // B：初始缓动位置等于树的世界 XZ 位置。
        commands.entity(entity).insert(TreeMoveBend {
            position: glam::Vec2::new(pos.x, pos.z),
        });
        // O：TreeAmbience 位置 = (x, y + scale*10, z)（常量 0x142a95db0 =
        // {0.0, 10.0} 广播乘 scale；decomp 行 487-503）。
        commands.entity(entity).insert(TreeAmbience {
            position: glam::Vec3::new(pos.x, pos.y + scale * 10.0, pos.z),
        });
        // O：prefab = 事件名；名 == "MISSING_TREE_PREFAB_NAME"(24) →
        // `random_tree_prefab`（decomp 行 611-682；四常量比对见 asm
        // 142267e06-142267e2c）。selector = 元素+0x38 qword（= 事件 id@+0x30）
        // 的低字节（asm 142268146 `MOV RCX,[RDI]`，RDI=LEA[R12+0x38]；
        // 0x14226a170 内 `in_CL & 1` 取变体位）→ `(cmd.id & 1)`。
        // drag_started 取 `DragState::is_started` 的资源面未取证（T4）→ false。
        // T5 已落：prefab 名写入 SpawnPrefabCmd 0x48 载荷首字段。
        let prefab_name = if name == country_core::resources::history::tree_edit::missing_prefab() {
            random_tree_prefab(
                (cmd.id & 1) as u8,
                deny_load,
                session.meta.is_halloween_any(),
                is_corrupted,
                false,
            )
            .to_string()
        } else {
            cmd.prefab_name.clone()
        };
        // O：`SpawnPrefabCmd`（事件槽 stride 0x50 = 头 8B + 载荷 0x48；
        // decomp 行 683-719 / asm 142267e55-142267ec8 逐字段）：
        // prefab@+0、entity@+0x10、position@+0x18、scale@+0x24、rotation@+0x28、
        // variant@+0x2c、常量 1.0@+0x30、flags[16]@+0x34、mode@+0x44。
        // T2：rotation/variant 原生取事件 +0x20/+0x24，共享槽缺 → 0 登记。
        // mode（iVar22）：naked&&deny → 3；否则非拖动路径 = 1；拖动/flags 门
        // （ActiveWidget::is_dragging + 事件 flags 字节）未接线 → 登记。
        let mode = if naked && deny_load { 3 } else { 1 };
        spawns.write(SpawnPrefabCmd {
            prefab_name: Some(country_core::resources::render::mesh_atlas_library::PrefabName::new(
                &prefab_name,
            )),
            entity: Some(entity),
            position: pos,
            scale,
            rotation: 0.0,
            variant: 0,
            one: 1.0,
            flags: [0; 0x10],
            mode,
        });
        // O：`TreeStorage::insert`（0x140a38560）；Some(previous) → 日志 +
        // DespawnPrefabCmd{entity: previous.entity}（decomp 行 721 / 747-785；
        // 串 "Tree {} already exists. Replacing it with a new tree"）。
        let previous: Option<TreeStorageEntry> = storage.insert(
            TreeStorageIndex(cmd.id),
            TreeStorageEntry {
                entity,
                position: pos,
            },
        );
        if let Some(previous) = previous {
            log::warn!("Tree {} already exists. Replacing it with a new tree", cmd.id);
            despawns.write(DespawnPrefabCmd {
                entity: Some(previous.entity),
                _gap: 0,
            });
        }
        // O：水面采样（decomp 行 722-726）→ `PlayTreeSoundCmd`：kind=0、
        // selector=0.0<水样、extra=naked && (deny||corrupted)、value=r、
        // position（decomp 行 727-746；slot stride 0x20 与共享型逐一对应）。
        let water_level = (**water)
            .as_ref()
            .bilinear_sample_at_world_xz(pos.x, pos.z, 130.0, 130.0);
        sounds.write(PlayTreeSoundCmd {
            kind: 0,
            selector: (water_level > 0.0) as u8,
            extra: (naked && (deny_load || is_corrupted)) as u8,
            value: r,
            position: pos,
        });
    }
}

/// `despawn_trees` @0x1422686d0（O：标记树 α<0.01 回收——`DespawnPrefabCmd`）。
pub fn despawn_trees(
    mut trees: Query<(Entity, &TreeAnimation, &TreeMarkedForDeletion)>,
    mut despawns: EventWriter<DespawnPrefabCmd>,
) {
    for (entity, animation, _marked) in trees.iter_mut() {
        // O：α 严格小于 0.01（`<=0.01 && !=0.01`，排除 NaN；decomp 行 75/122）。
        // 读取生长进度 +4；+0 是时长，不能用于删除判断。
        if animation.progress < DESPAWN_ALPHA_THRESHOLD {
            despawns.write(DespawnPrefabCmd {
                entity: Some(entity),
                _gap: 0,
            });
        }
    }
}
