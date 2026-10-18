//! manage_autoclutter_entities（B 主基线；0x1422d04e0，size 8178）。
//!
//! 2026-10-17 restorer-clutter-remaining：按 fresh 反编译重建
//! （`evidence/decomp/0x1422d04e0_*.c`）。
//!
//! 控制流（O）：
//! 1. 现存集 = `from_iter<u64, &AutoClutterObject>`（排除 `ClutterMarkedForDeletion`）。
//! 2. 目标集 = 区域候选 + 输出候选键集（`hash_one<&GlobalId>`）。
//! 3. 现存∉目标 → `insert<ClutterMarkedForDeletion>`（不直接 despawn）。
//! 4. 目标∉现存 → `spawn_empty` + 插入 `AutoClutter` 组件：
//!    `{shared, scale = a+b, flags, key, kind_flag, bool}`；缩放抖动按 kind 与
//!    目标数量 count 分派（O）：
//!    - kind==4 或 kind>=7：`(1-r)*0 + r*0.1`；
//!    - kind==6：`(1-r)*0.2 + r`；
//!    - 其它（kind 3/5 等）：count>4 → `(1-r)*0.3 + r*0.8`；否则
//!      `((1-r)*0.1 + r)*0.3`。
//! 5. 组件附加（O）：
//!    - `(kind & 6) != 4 && kind > 2` → `AutoClutterConvertableToDecoClutter`；
//!    - 记录 +0x45==9 → 按 kind 插 `NaniBundle<Plant/SolidVertexColor/SolidVertexColorWindow>`；
//!      否则按 NameHash 四次比较 → `NaniBundle<SolidAlbedoMapEmissive/SolidAlbedoMap>`；
//!    - kind∉{2,(3..6 除 1)} → `WindowShutter` + `ColorId`。
//! 6. 朝向四元数：由候选 +0x54/+0x58/+0x5c 经
//!    `Quat::from_rotation_arc(Vec3::Z, v)`（两次叉积 + 归一化 + 1.0/0.5 常量，
//!    0x1422d11e3 分支即 shortest-arc 的 d<=0 分支）。
//!
//! U 登记：NaniBundle 实例数据选择（mesh selector/subset kind）与
//! `AutoClutterConvertableToDecoClutter` 类型在 core 未建模 → 不伪造插入，登记为
//! core-type-gap。动画状态表（stride 0x18）由 [`crate::autoclutter::autoclutter_animation`]
//! 的组件承载（U）。

use bevy_ecs::prelude::*;
use country_core::resources::color_id::ColorId;
use glam::{Quat, Vec3};
use hashbrown::{HashMap, HashSet};

use super::{AutoClutterObject, AutoClutterState};

/// 现存自动杂物键集（源候选键；RandomState 0x142612aa0 语义）。
#[derive(Resource, Clone, Debug, Default)]
pub struct ExistingAutoClutterKeys(pub HashSet<u64>);

/// 管理后的放置记录（宿主渲染消费）。`base_quat` 为候选反射向量的朝向四元数。
#[derive(Component, Clone, Debug, PartialEq)]
pub struct ManagedAutoClutterPlacement {
    pub position: Vec3,
    pub key: u64,
    pub shared: u64,
    pub scale: f32,
    pub kind: u8,
    pub base_quat: Quat,
}

/// 窗百叶窗组件（O：`WindowShutter`；kind + 值）。
#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct WindowShutter {
    pub kind: u8,
    pub value: u64,
}

/// 管理 RNG（init_rng 0x140c96240 语义；缩放抖动）。
#[derive(Resource, Debug)]
pub struct ManageRng(pub std::sync::Mutex<fastrand::Rng>);

impl Default for ManageRng {
    fn default() -> Self {
        Self(std::sync::Mutex::new(fastrand::Rng::with_seed(0x606f)))
    }
}

/// 缩放抖动（O 0x1422d1af0 分派）。
fn manage_scale(kind: u8, count: usize, rng: &mut fastrand::Rng) -> f32 {
    let b = kind.wrapping_sub(3);
    if b == 1 || 3 < b {
        // kind==4 或 kind>=7。
        let r = rng.f32();
        (1.0 - r) * 0.0 + r * 0.1
    } else if kind == 6 {
        let r = rng.f32();
        (1.0 - r) * 0.2 + r
    } else if count > 4 {
        let r = rng.f32();
        (1.0 - r) * 0.3 + r * 0.8
    } else {
        let r = rng.f32();
        ((1.0 - r) * 0.1 + r) * 0.3
    }
}

/// 朝向四元数（O：candidate +0x54/+0x58/+0x5c → `from_rotation_arc(Z, v)`）。
fn orientation_from_candidate(spawn: &super::AutoClutterSpawn) -> Quat {
    let v = Vec3::new(
        f32::from_bits(spawn.f54),
        f32::from_bits(spawn.f58),
        f32::from_bits(spawn.f5c),
    );
    if v.length_squared() == 0.0 {
        Quat::IDENTITY
    } else {
        Quat::from_rotation_arc(Vec3::Z, v.normalize())
    }
}

/// 类型分支（O：byte@+0x30 ==2 或 (x-3) < 4 且 x != 1）。
fn should_manage(kind: u8) -> bool {
    kind == 2 || (kind.wrapping_sub(3) < 4 && kind != 1)
}

/// manage_autoclutter_entities（0x1422d04e0）。
pub fn manage_autoclutter_entities(
    mut state: ResMut<AutoClutterState>,
    mut commands: Commands,
    mut existing: ResMut<ExistingAutoClutterKeys>,
    mut rng: ResMut<ManageRng>,
    mut query: Query<(
        Entity,
        &AutoClutterObject,
        Option<&crate::autoclutter::load_clutter_meshes::ClutterMarkedForDeletion>,
    )>,
) {
    // 1. 现存集（O：from_iter<u64, &AutoClutterObject>；含删除标记者不进入目标判定）。
    existing.0.clear();
    let mut live: HashSet<u64> = HashSet::new();
    let mut by_key: HashMap<u64, Entity> = HashMap::new();
    for (entity, obj, marked) in &query {
        if marked.is_none() {
            existing.0.insert(obj.key);
            live.insert(obj.key);
            by_key.insert(obj.key, entity);
        }
    }

    // 2. 目标集（输出候选键）。
    let mut target: HashSet<u64> = HashSet::new();
    for spawn in state.spawn_candidates.iter() {
        target.insert(spawn.key);
    }

    // 3. 删除：现存但不在目标 → insert<ClutterMarkedForDeletion>（O：不 despawn）。
    for (_, obj, marked) in &mut query {
        if !target.contains(&obj.key) && marked.is_none() {
            if let Some(entity) = by_key.get(&obj.key) {
                commands
                    .entity(*entity)
                    .insert(crate::autoclutter::load_clutter_meshes::ClutterMarkedForDeletion);
            }
        }
    }

    // 4. 新建：目标但不在现存。
    let r = &mut rng.0.lock().unwrap();
    let count = state.spawn_candidates.len();
    for spawn in state.spawn_candidates.iter() {
        if live.contains(&spawn.key) {
            continue;
        }
        let kind = spawn.kind;
        if !should_manage(kind) {
            continue;
        }
        let scale = manage_scale(kind, count, r);
        let base_quat = orientation_from_candidate(spawn);
        let mut entity = commands.spawn_empty();
        entity.insert(AutoClutterObject { key: spawn.key });
        entity.insert(ManagedAutoClutterPlacement {
            position: spawn.position,
            key: spawn.key,
            shared: match &spawn.shared {
                super::SpawnShared::ArcStrong(v) => *v,
                super::SpawnShared::Prefab(_) => 0,
            },
            scale,
            kind,
            base_quat,
        });
        // 5a. `(kind & 6) != 4 && kind > 2` → AutoClutterConvertableToDecoClutter
        // （U：类型未落库，core-type-gap）。
        // 5b. WindowShutter + ColorId（O：kind∉{2,3..6\1}）。
        let needs_shutter = kind != 2 && (kind.wrapping_sub(3) >= 4 || kind.wrapping_sub(3) == 1);
        if needs_shutter {
            entity.insert(WindowShutter {
                kind,
                value: spawn.host_id,
            });
            entity.insert(ColorId::default_value());
        }
        // 5c. NaniBundle 实例选择（Plant/SolidVertexColor/SolidVertexColorWindow/
        //     SolidAlbedoMap[Emissive]）依赖 mesh selector/subset kind 宿主数据（U），
        //     不伪造插入。
    }
}
