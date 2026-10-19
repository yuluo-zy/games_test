//! B `blueprint.rs` — 装饰蓝图库（D1 消费契约）。
//!
//! 证据：`DecoratorBlueprints::insert` 0x1408258b0（值 = 0x140B 蓝图结构体，
//! +0x50/+0x94/+0x138 字段、+0x120 door stairs 参数指针）、`get` 0x140825ff0
//! （hashbrown 探测查询，未中返回 0）、`add_door_stairs` 0x140825830（分配 0x20B
//! = 8×u32 参数块，替换 +0x120）、`set_interaction_mesh` 0x1408255e0（+0x80 区域
//! {u32@+0x80=0, ptr@+0x84, f32@+0x8c, Arc@+0x90, slice ptr+len@+0xa0/+0xa8}，
//! NaN 校验 + 旧 Arc 释放）。
//!
//! 本模块为 decorator_visual 调用闭合所需的最小真实接线；0x140B 完整字段表与
//! 蓝图主流程归 blueprint.rs 的 D-owner。方法签名按调用点（generate_* 各函数）
//! 与上述反编译 O 事实。

use std::sync::Arc;

use bevy_ecs::resource::Resource;
use country_core::geometry::instanced_wall::BrickSourceId;
use country_core::resources::walls::decorator_storage::DecoratorStorage;
use country_core::resources::walls::decorator_subtype::DecoratorCollision;
use country_core::systems::collision::world::RaycastMeshData;
use glam::{Quat, Vec3};
use hashbrown::HashMap;

/// 交互网格记录（O：0x1408255e0 写入面 +0x80..+0xb0）。
///
/// 机器码布局：`+0x80` 判别 0（Some）、`+0x84` Vec3（xyz）、`+0x90`
/// `Arc<RaycastMeshData>`、`+0xa0` 四元数（16B）。
#[derive(Clone)]
pub struct InteractionMesh {
    /// `+0x84`：位置（三个 f32）。
    pub xyz: Vec3,
    /// `+0x90`：碰撞网格 Arc。
    pub mesh: Arc<RaycastMeshData>,
    /// `+0xa0`：旋转四元数（四个 f32）。
    pub rotation: Quat,
}

impl std::fmt::Debug for InteractionMesh {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InteractionMesh")
            .field("xyz", &self.xyz)
            .field("mesh", &Arc::as_ptr(&self.mesh))
            .field("rotation", &self.rotation)
            .finish()
    }
}

/// B DecoratorBlueprint：装饰蓝图记录（原 0x140B=320B 结构体）。
/// 仅恢复 decorator_visual 消费面字段；其余偏移按 U 保留命名。
#[derive(Clone, Debug, Default)]
pub struct DecoratorBlueprint {
    /// +0x00：u64（旗写面 = {u32 1, u32 0}；O generate_flags 0x1407ff6c0）。
    pub u50: u64,
    /// +0x80..+0xb0：交互网格（set_interaction_mesh 0x1408255e0 写；None=未设）。
    pub interaction: Option<InteractionMesh>,
    /// +0x94：u64（U）。
    pub u94: u64,
    /// +0x120：door stairs 装配参数（8×u32，0x140825830 分配替换）。
    pub door_stairs: Option<Box<[u32; 8]>>,
    /// +0x10/+0x18：旗布基（O 写面 {0.5,-0.7071,0.7071,0.5} = 四元数分量；
    /// generate_flags 0x1407ff6c0）。
    pub flag_cloth_basis: [f32; 4],
    /// +0x20..0x40：变换槽（O 写面 pos/quat；CachedDecoratorTransform 语义 U）。
    pub transform_slots: [u64; 4],
    /// +0x138：u8（U）。
    pub u138: u8,
    /// 碰撞体列表（set_colliders 0x14017a22f0/0x1408ed6f0 消费面；原 +0x0 区域）。
    pub colliders: Vec<DecoratorCollision>,
    /// 生成变换（I：generate_* 写面承载；与 transform_slots 对应）。
    pub transform: Option<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransform>,
    /// 描边网格（set_outline 消费面；I：原 +0x20 区域，U：载荷语义）。
    pub outline: Option<(f32, f32, f32)>,
    /// 烟囱参数（add_chimney 消费面，U：载荷语义）。
    pub chimney: Option<f32>,
    /// 布（add_cloth 消费面，U：载荷语义）。
    pub cloth: Option<u8>,
}

impl DecoratorBlueprint {
    /// new（内联，I）：全零蓝图。
    pub fn new() -> Self {
        Self::default()
    }

    /// 0x14017a22f0（O 结构）/0x1408ed6f0：设置碰撞体列表（替换 +0x0 区域）。
    pub fn set_colliders(&mut self, colliders: Vec<DecoratorCollision>) {
        self.colliders = colliders;
    }

    /// set_interaction_mesh 0x1408255e0（O，逐语句）：写 +0x80 区域。
    ///
    /// fresh 2026-10-17：`xyz` 三个 f32 与 `rotation` 四个 f32 都必须有限，
    /// **否则 panic**（`assert!` → panic_fmt；早期实现"非有限则保持原值"与
    /// 机器码相反，已修）。旧 Arc 在覆盖前释放（Rust `Option` 赋值等价）。
    /// 机器码参数面：`(self, mesh: Arc<RaycastMeshData>, xyz: &Vec3,
    /// rotation: &Quat)`；`+0x80=0` 为 Some 判别。
    pub fn set_interaction_mesh(
        &mut self,
        mesh: Arc<RaycastMeshData>,
        xyz: &Vec3,
        rotation: &Quat,
    ) {
        assert!(xyz.is_finite(), "interaction mesh position must be finite");
        assert!(rotation.is_finite(), "interaction mesh rotation must be finite");
        self.interaction = Some(InteractionMesh {
            xyz: *xyz,
            mesh,
            rotation: *rotation,
        });
    }

    /// set_outline（内联，I）：描边网格（载荷 U）。
    pub fn set_outline(&mut self, outline: (f32, f32, f32)) {
        self.outline = Some(outline);
    }

    /// 0x140825830（O）：door stairs 装配参数（0x20B = 8×u32）替换 +0x120
    /// （旧块释放）。
    pub fn add_door_stairs(&mut self, params: [u32; 8]) {
        self.door_stairs = Some(Box::new(params));
    }

    /// add_chimney（内联，I）：烟囱参数（载荷 U）。
    pub fn add_chimney(&mut self, v: f32) {
        self.chimney = Some(v);
    }

    /// add_cloth（内联，I）：布（载荷 U）。
    pub fn add_cloth(&mut self, v: u8) {
        self.cloth = Some(v);
    }

    /// unset_collider（内联，I）：清空碰撞体列表。
    pub fn unset_collider(&mut self) {
        self.colliders.clear();
    }

    /// 砖源 id 消费面（generate_* 装配读：BrickSourceId 为墙砖源，U：精确用途）。
    pub fn brick_source(&self) -> Option<BrickSourceId> {
        None
    }
}

/// B DecoratorBlueprints：DecoratorId(u32) → DecoratorBlueprint 蓝图库。
#[derive(Resource, Clone, Debug, Default)]
pub struct DecoratorBlueprints {
    /// 蓝图表。
    pub map: HashMap<u32, DecoratorBlueprint>,
    /// 上一帧蓝图表（原 +0x20/+0x28 的 prev map；差分实例化基线）。
    pub prev: HashMap<u32, DecoratorBlueprint>,
}

impl DecoratorBlueprints {
    /// 0x1408258b0（O）：插入蓝图（键 u32；冲突行为 U，D1 以覆盖语义承载）。
    pub fn insert(&mut self, id: u32, blueprint: DecoratorBlueprint) {
        self.map.insert(id, blueprint);
    }

    /// `clear_blueprints` @0x1408260d0（O 结构）：移除本帧 modified/removed
    /// 装饰与已移除存储桶的蓝图（Arc 归零由 HashMap 释放承载）。
    pub fn clear_blueprints(&mut self, storage: &DecoratorStorage) {
        let mut ids: Vec<u32> = Vec::new();
        for (_, bucket) in storage.iter() {
            ids.extend(bucket.modified_ids.iter().copied());
            ids.extend(bucket.removed_ids.iter().copied());
        }
        for bucket in storage.iter_terrain_decorators() {
            ids.extend(bucket.modified_ids.iter().copied());
            ids.extend(bucket.removed_ids.iter().copied());
        }
        for bucket in storage.iter_stair_decorators() {
            ids.extend(bucket.modified_ids.iter().copied());
            ids.extend(bucket.removed_ids.iter().copied());
        }
        // 已移除存储地址的整个桶（O：removed storages 遍历）。
        for addr in storage.iter_removed_storages() {
            if let Some(bucket) = storage.get(addr) {
                ids.extend(bucket.removed_ids.iter().copied());
                ids.extend(bucket.modified_ids.iter().copied());
            }
        }
        for id in ids {
            self.map.remove(&id);
        }
    }

    /// `copy_blueprints_to_prev` @0x140826d70（O）：当前蓝图表
    /// `clone_from` 到 prev 基线（差分实例化读取 prev）。
    pub fn copy_blueprints_to_prev(&mut self) {
        self.prev.clone_from(&self.map);
    }

    /// 0x140825ff0（O）：查询蓝图，未中返回 None（原返回 0）。
    pub fn get(&self, id: u32) -> Option<&DecoratorBlueprint> {
        self.map.get(&id)
    }

    /// 可变查询（内联，I）。
    pub fn get_mut(&mut self, id: u32) -> Option<&mut DecoratorBlueprint> {
        self.map.get_mut(&id)
    }

    /// 删除（内联，I；移动/移除族消费）。
    pub fn remove(&mut self, id: u32) -> Option<DecoratorBlueprint> {
        self.map.remove(&id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use country_core::resources::walls::decorator_storage::Decorators;
    use country_core::resources::walls::WallId;

    #[test]
    fn copy_blueprints_to_prev_clones_current_map() {
        let mut blueprints = DecoratorBlueprints::default();
        blueprints.insert(1, DecoratorBlueprint::new());
        blueprints.copy_blueprints_to_prev();
        assert_eq!(blueprints.prev.len(), 1);
        assert!(blueprints.prev.contains_key(&1));
    }

    #[test]
    fn clear_blueprints_removes_modified_and_removed_ids() {
        let mut storage = DecoratorStorage::default();
        let mut bucket = Decorators::default();
        bucket.modified_ids.insert(1);
        bucket.removed_ids.insert(2);
        storage.per_wall.insert(WallId(9), bucket);

        let mut blueprints = DecoratorBlueprints::default();
        for id in [1, 2, 3] {
            blueprints.insert(id, DecoratorBlueprint::new());
        }
        blueprints.clear_blueprints(&storage);
        assert_eq!(blueprints.map.len(), 1, "untouched id 3 retained");
        assert!(blueprints.map.contains_key(&3));
    }
}
