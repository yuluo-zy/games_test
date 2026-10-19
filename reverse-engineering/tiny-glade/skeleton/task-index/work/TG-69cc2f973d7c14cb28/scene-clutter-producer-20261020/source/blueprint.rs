//! B decorator blueprints: retained native insert/clear/copy and clutter output.
//! Evidence: insert@1408258b0/1854B, clear@1408260d0/3222B,
//! copy@140826d70/957B; full visual consumer@140827910/7991B.
//! This candidate recovers the actual shared blueprint state and clutter producer
//! interface. The full consumer and its side-effect resource dependencies remain
//! a separate acceptance gate; legacy generator fields below are not an ABI claim.

use std::sync::Arc;

use bevy_ecs::{
    resource::Resource,
    system::{Res, ResMut},
};
use country_core::geometry::instanced_wall::BrickSourceId;
use country_core::resources::walls::decorator_storage::{DecoratorStorage, DecoratorStorageAddr};
use country_core::resources::walls::decorator_subtype::DecoratorCollision;
use country_core::systems::collision::world::RaycastMeshData;
use glam::{Quat, Vec3};
use hashbrown::HashMap;

use crate::utils::visual_entity_builder::WindowVisualEntityBuilder;

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

/// A positioned mesh preserves original Arc identity as well as ordered pose.
/// Native visual differential equality compares the allocation pointer, not hash.
#[derive(Clone)]
pub struct PositionedDecoratorMesh {
    pub mesh: Arc<RaycastMeshData>,
    pub position: Vec3,
    pub rotation: Quat,
}

impl std::fmt::Debug for PositionedDecoratorMesh {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PositionedDecoratorMesh")
            .field("mesh", &Arc::as_ptr(&self.mesh))
            .field("position", &self.position)
            .field("rotation", &self.rotation)
            .finish()
    }
}

impl PartialEq for PositionedDecoratorMesh {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.mesh, &other.mesh)
            && self.position == other.position
            && self.rotation == other.rotation
    }
}

/// Original tags 0/1/2. The two native consumers handle Unspecified differently:
/// collision skips it; interaction rejects it. Do not collapse this to Option.
#[derive(Clone, Debug, Default, PartialEq)]
pub enum BlueprintMeshSlot {
    Mesh(PositionedDecoratorMesh),
    Disabled,
    #[default]
    Unspecified,
}

/// Recovered clutter fields coexist with older generator interfaces below.
/// This is a semantic host type, not a claim to reproduce the native ABI layout.
#[derive(Clone, Debug, Default)]
pub struct DecoratorBlueprint {
    /// Exact source for actual recovered producers; None marks legacy callers
    /// whose source contract has not yet been recovered, never a default source.
    pub storage_addr: Option<DecoratorStorageAddr>,
    /// Original +0x139, ORed with 4 by interaction-source construction.
    pub interaction_source_flags: u8,
    pub collision_mesh: BlueprintMeshSlot,
    pub interaction_mesh: BlueprintMeshSlot,
    /// Original +0xb0 Vec<WindowVisualEntityBuilder>. Generation never spawns.
    pub visual_entities: Vec<WindowVisualEntityBuilder>,
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
    pub transform: Option<
        country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransform,
    >,
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

    /// B set_colliders@1408252b0. First is shrunken collision geometry; second
    /// is full interaction geometry. Both carry the cached world pose.
    pub fn set_positioned_colliders(
        &mut self,
        collision: Arc<RaycastMeshData>,
        interaction: Arc<RaycastMeshData>,
        position: &Vec3,
        rotation: &Quat,
    ) {
        assert!(position.is_finite(), "collider position must be finite");
        assert!(rotation.is_finite(), "collider rotation must be finite");
        self.collision_mesh = BlueprintMeshSlot::Mesh(PositionedDecoratorMesh {
            mesh: collision,
            position: *position,
            rotation: *rotation,
        });
        self.interaction_mesh = BlueprintMeshSlot::Mesh(PositionedDecoratorMesh {
            mesh: interaction,
            position: *position,
            rotation: *rotation,
        });
    }

    /// Clutter's original transform slot is written exactly once after insert.
    pub fn set_cached_transform(
        &mut self,
        transform: country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransform,
    ) {
        assert!(
            self.transform.is_none(),
            "decorator blueprint already has an outline transform"
        );
        self.transform = Some(transform);
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
        assert!(
            rotation.is_finite(),
            "interaction mesh rotation must be finite"
        );
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

/// Current map and previous map are separate original resources. Snapshotting
/// clones Arcs, preserving the native unchanged-identity fast path.
#[derive(Resource, Clone, Debug, Default)]
pub struct DecoratorBlueprints {
    pub map: HashMap<u32, Arc<DecoratorBlueprint>>,
}

#[derive(Resource, Clone, Debug, Default)]
pub struct PrevDecoratorBlueprints {
    pub map: HashMap<u32, Arc<DecoratorBlueprint>>,
}

impl DecoratorBlueprints {
    /// Compatibility boundary for older generators. Their missing source/mesh
    /// contracts remain explicit rather than being synthesized here.
    pub fn insert(&mut self, id: u32, blueprint: DecoratorBlueprint) {
        self.map.insert(id, Arc::new(blueprint));
    }

    /// B insert@1408258b0: reuse a unique allocation, otherwise allocate fresh.
    /// A shared previous snapshot must never be mutated by generation.
    pub fn insert_for(
        &mut self,
        id: u32,
        storage_addr: DecoratorStorageAddr,
        interaction_source_flags: u8,
    ) -> &mut DecoratorBlueprint {
        let entry = self
            .map
            .entry(id)
            .or_insert_with(|| Arc::new(DecoratorBlueprint::new()));
        if Arc::get_mut(entry).is_none() {
            *entry = Arc::new(DecoratorBlueprint::new());
        }
        let blueprint = Arc::get_mut(entry).expect("fresh blueprint must be unique");
        let mut visuals = std::mem::take(&mut blueprint.visual_entities);
        visuals.clear();
        *blueprint = DecoratorBlueprint::new();
        blueprint.visual_entities = visuals;
        blueprint.storage_addr = Some(storage_addr);
        blueprint.interaction_source_flags = interaction_source_flags;
        blueprint
    }

    /// B clear@1408260d0: all live buckets plus the removed-storage ID sets.
    /// The removed registry is authoritative even when its bucket is gone.
    pub fn clear_blueprints(&mut self, storage: &DecoratorStorage) {
        for bucket in storage
            .per_wall
            .values()
            .chain(storage.terrain.iter())
            .chain(storage.stairs.iter())
        {
            for id in bucket.modified_ids.iter().chain(bucket.removed_ids.iter()) {
                self.map.remove(id);
            }
        }
        for removed_ids in storage.removed.values() {
            for id in removed_ids {
                self.map.remove(id);
            }
        }
    }

    pub fn copy_blueprints_to_prev(&self, previous: &mut PrevDecoratorBlueprints) {
        previous.map.clone_from(&self.map);
    }

    pub fn get(&self, id: u32) -> Option<&DecoratorBlueprint> {
        self.map.get(&id).map(Arc::as_ref)
    }

    pub fn get_mut(&mut self, id: u32) -> Option<&mut DecoratorBlueprint> {
        self.map.get_mut(&id).map(Arc::make_mut)
    }

    pub fn remove(&mut self, id: u32) -> Option<Arc<DecoratorBlueprint>> {
        self.map.remove(&id)
    }
}

/// Actual system interfaces; registration/copy phase must use original evidence.
pub fn clear_blueprints(
    storage: Res<DecoratorStorage>,
    mut blueprints: ResMut<DecoratorBlueprints>,
) {
    blueprints.clear_blueprints(&storage);
}

pub fn copy_blueprints_to_prev(
    blueprints: Res<DecoratorBlueprints>,
    mut previous: ResMut<PrevDecoratorBlueprints>,
) {
    blueprints.copy_blueprints_to_prev(&mut previous);
}

#[cfg(test)]
mod tests {
    use super::*;
    use country_core::resources::walls::{WallId, decorator_storage::Decorators};

    #[test]
    fn snapshot_shares_identity_and_reinsert_cannot_mutate_previous() {
        let mut current = DecoratorBlueprints::default();
        current.insert_for(1, DecoratorStorageAddr::Terrain, 0).u138 = 7;
        let mut previous = PrevDecoratorBlueprints::default();
        current.copy_blueprints_to_prev(&mut previous);
        assert!(Arc::ptr_eq(&current.map[&1], &previous.map[&1]));
        current.insert_for(1, DecoratorStorageAddr::Stair, 0);
        assert!(!Arc::ptr_eq(&current.map[&1], &previous.map[&1]));
        assert_eq!(previous.map[&1].u138, 7);
        assert_eq!(
            current.map[&1].storage_addr,
            Some(DecoratorStorageAddr::Stair)
        );
        assert!(matches!(
            current.map[&1].interaction_mesh,
            BlueprintMeshSlot::Unspecified
        ));
    }

    #[test]
    fn clear_covers_inline_buckets_and_removed_bucket_ids() {
        let mut storage = DecoratorStorage::default();
        let mut wall = Decorators::default();
        wall.modified_ids.insert(1);
        storage.per_wall.insert(WallId(9), wall);
        let mut terrain = Decorators::default();
        terrain.removed_ids.insert(2);
        storage.terrain = Some(terrain);
        let mut stairs = Decorators::default();
        stairs.modified_ids.insert(3);
        storage.stairs = Some(stairs);
        storage
            .removed
            .entry(DecoratorStorageAddr::Wall(WallId(10)))
            .or_default()
            .insert(4);
        let mut blueprints = DecoratorBlueprints::default();
        for id in 1..=5 {
            blueprints.insert(id, DecoratorBlueprint::new());
        }
        blueprints.clear_blueprints(&storage);
        assert_eq!(blueprints.map.keys().copied().collect::<Vec<_>>(), [5]);
    }
}
