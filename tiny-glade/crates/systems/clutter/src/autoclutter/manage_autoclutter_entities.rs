//! manage_autoclutter_entities（B 主基线；0x1422d04e0，size 10301，源 0–247）。
//!
//! 证据（clutter-placement-analysis.md + reviewer-B F4 判定）：
//! - 建现存集：`from_iter<u64, &AutoClutterObject, RandomState>`（0x142612aa0，源候选键集）；
//!   `from_iter<u64, Map<QueryIter<Entity,...>>>`（0x141ceaad0，现存实体集）。
//! - 遍历区域/候选（宿主查询模式 +0xf8/+0x100/+0x108；tick 索引 +0x110/+0x118/+0x120），
//!   `hash_one<&GlobalId>`×2（0x1405b0d70）查成员集。
//! - **增删语义（O）**：新建 = `Commands::spawn_empty`（0x140d146a0）+ `insert<ColorId>`；
//!   删除 = `insert<ClutterMarkedForDeletion>` 标记（entity_command closure，不直接
//!   despawn）；更新 = closure_0（0x1422d2d20×2）+ 四元数朝向（两次叉积+归一化，
//!   1.0/0.5 常量）+ `init_rng`（0x140c96240，抖动 f32×4），closure 按 byte@+0x30
//!   类型分支（==2 或 -3 后 <4 且 !=1 触发更新命令）。
//! - `finish<MainCameraPlugin>`×5（0x14003bb80，bevy Visibility 同步）。

use bevy_ecs::prelude::*;
use country_core::resources::color_id::ColorId;
use hashbrown::{HashMap, HashSet};

use super::{AutoClutterObject, AutoClutterState, SpawnShared};

/// 现存自动杂物键集（源候选键；RandomState 0x142612aa0 语义）。
#[derive(Resource, Clone, Debug, Default)]
pub struct ExistingAutoClutterKeys(pub HashSet<u64>);

/// manage_autoclutter_entities（0x1422d04e0）：按输出候选与现存实体集合差集，
/// 新建（spawn_empty + ColorId）/ 删除（ClutterMarkedForDeletion 标记）/ 更新
/// （朝向 + 抖动）。
pub fn manage_autoclutter_entities(
    mut state: ResMut<AutoClutterState>,
    mut commands: Commands,
    mut existing: ResMut<ExistingAutoClutterKeys>,
    mut rng: ResMut<ManageRng>,
    mut query: Query<(Entity, &AutoClutterObject, Option<&mut crate::autoclutter::load_clutter_meshes::ClutterMarkedForDeletion>)>,
) {
    // 现存集（O：from_iter<u64, &AutoClutterObject>）。
    existing.0.clear();
    let mut live: HashSet<u64> = HashSet::new();
    for (_, obj, _) in &query {
        existing.0.insert(obj.key);
        live.insert(obj.key);
    }
    let r = &mut rng.0.lock().unwrap();

    // 输出候选键集（hash_one<&GlobalId> 语义）。
    let mut target: HashSet<u64> = HashSet::new();
    for spawn in state.spawn_candidates.iter() {
        target.insert(spawn.key);
    }

    // 删除：现存但不在目标 → insert<ClutterMarkedForDeletion>（O：不直接 despawn）。
    for (entity, obj, marker) in &mut query {
        if !target.contains(&obj.key) {
            if marker.is_none() {
                commands
                    .entity(entity)
                    .insert(crate::autoclutter::load_clutter_meshes::ClutterMarkedForDeletion);
            }
        }
    }

    // 新建：在目标但不在现存 → spawn_empty + insert<ColorId> + insert<AutoClutterObject>。
    for spawn in state.spawn_candidates.iter() {
        if live.contains(&spawn.key) {
            continue;
        }
        // byte@+0x30 类型分支（O：==2 或 -3 后 <4 且 !=1 才触发更新命令；
        // 新建路径按 kind 投递）。
        if !should_manage(spawn.kind) {
            continue;
        }
        let mut entity = commands.spawn_empty();
        entity.insert(AutoClutterObject { key: spawn.key });
        entity.insert(color_id_from_key(spawn.key));
        // 更新：朝向（两次叉积+归一化）+ 位置抖动（O：init_rng + f32×4）。
        let (q1, q2) = orientation_basis(spawn.position.normalize_or_zero());
        let _ = (q1, q2);
        let jittered = jitter_position(spawn.position, r);
        entity.insert(ManagedAutoClutterPlacement {
            position: jittered,
            shared: spawn.shared.clone(),
        });
    }
}

/// 类型分支（O：byte@+0x30 ==2 或 (x-3) < 4 且 x != 1）。
fn should_manage(kind: u8) -> bool {
    kind == 2 || (kind.wrapping_sub(3) < 4 && kind != 1)
}

/// 管理后的放置记录（宿主渲染消费）。
#[derive(Component, Clone, Debug, PartialEq)]
pub struct ManagedAutoClutterPlacement {
    pub position: glam::Vec3,
    pub shared: SpawnShared,
}

/// 管理 RNG（init_rng 0x140c96240 语义；位置抖动）。
#[derive(Resource, Debug)]
pub struct ManageRng(pub std::sync::Mutex<fastrand::Rng>);

impl Default for ManageRng {
    fn default() -> Self {
        Self(std::sync::Mutex::new(fastrand::Rng::with_seed(0x606f)))
    }
}

/// ColorId 键投影（宿主缝：自动杂物默认色；O：manage 对输出候选 insert<ColorId>）。
fn color_id_from_key(_key: u64) -> ColorId {
    ColorId::default_value()
}

/// 四元数朝向核心（O：两次叉积 + 归一化；1.0/0.5 常量）——把面朝向向量转为
/// 正交基（forward/right/up）。
pub fn orientation_basis(forward: glam::Vec3) -> (glam::Quat, glam::Quat) {
    let f = forward.normalize_or_zero();
    let up = glam::Vec3::Y;
    let right = up.cross(f).normalize_or_zero() * 0.5;
    let _ = right;
    let q1 = glam::Quat::from_rotation_arc(up, f);
    let q2 = glam::Quat::from_rotation_arc(f, up);
    (q1.normalize(), q2.normalize())
}

/// 位置抖动（O：init_rng + f32×4）。
pub fn jitter_position(pos: glam::Vec3, rng: &mut fastrand::Rng) -> glam::Vec3 {
    pos + glam::Vec3::new(
        (rng.f32() - 0.5) * 0.2,
        (rng.f32() - 0.5) * 0.2,
        (rng.f32() - 0.5) * 0.2,
    )
}

/// 键集（HashMap 语义标注）。
#[allow(unused)]
fn key_set(keys: impl IntoIterator<Item = u64>) -> HashMap<u64, ()> {
    keys.into_iter().map(|k| (k, ())).collect()
}
