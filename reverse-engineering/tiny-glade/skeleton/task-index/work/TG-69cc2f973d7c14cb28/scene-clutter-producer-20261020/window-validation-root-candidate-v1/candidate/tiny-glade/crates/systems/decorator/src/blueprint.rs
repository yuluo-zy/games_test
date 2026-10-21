//! B decorator blueprints: retained native insert/clear/copy and clutter output.
//! Evidence: insert@1408258b0/1854B, clear@1408260d0/3222B,
//! copy@140826d70/957B; full visual consumer@140827910/7991B.
//! Shared blueprint state, actual clutter producer interface and the original
//! fullvisual consumer. Canonical companion interfaces and original scheduling
//! are required; this source is not a typecheck or running-scene claim. Legacy
//! generator fields below are explicitly not a recovered native ABI.

use std::{collections::HashSet, sync::Arc};

use bevy_ecs::{
    event::EventReader,
    resource::Resource,
    system::{Commands, Res, ResMut},
};
use country_core::asset_libraries::{
    mesh_library::AssetMeshLibrary,
    shader_library::SharedAssetShaderLibrary,
    texture_library::AssetTextureLibrary,
};
use country_core::geometry::instanced_wall::BrickSourceId;
use country_core::resources::walls::decorator_subtype::DecoratorCollision;
#[cfg(test)]
use country_core::asset_libraries::shader_library::AssetShaderLibrary;
use country_core::resources::walls::dormers::DormerHoleSsbo;
use country_core::resources::walls::public_walls::OnWallDeleted;
use country_core::resources::walls::wall_holes::{RawWallHole, WallHoles};
use country_core::resources::walls::{
    decorator::DecoratorId,
    decorator_collision::DecoratorPhysicsColliders,
    decorator_storage::{DecoratorStorage, DecoratorStorageAddr},
};
use country_core::systems::collision::world::{
    ColliderUpsert, RaycastGeoSource, RaycastMeshData, RaycastWorld, remove_collider,
    upsert_collider,
};
use country_core::systems::collision::{WindowAutoClutterCandidate, WindowAutoClutterCandidates};
use glam::{Quat, Vec3};
use hashbrown::HashMap;

use crate::decorator_visual::deco_differential_state::{
    ChimneyAssemblyParams, DoorStairsAssemblyParams, WallAttachedDecoTracker,
};
use crate::decorator_visual_state::{ClothInfo, DecoratorVisualState};
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
    /// Original 20-byte wall-hole values; canonical geometry owner.
    pub wall_holes: Vec<RawWallHole>,
    /// Original 48-byte dormer values use the actual core/GPU record owner.
    /// Consumer equality projects its native float/integer words below.
    pub dormer_holes: Vec<DormerHoleSsbo>,
    /// Canonical recurring output. The fullvisual collector appends every frame,
    /// including Arc-identical and value-equal blueprint paths.
    pub window_auto_clutter_candidates: Vec<WindowAutoClutterCandidate>,
    pub cloth_info: Option<Box<ClothInfo>>,
    pub chimney_assembly: Option<Box<ChimneyAssemblyParams>>,
    pub door_stairs_assembly: Option<Box<DoorStairsAssemblyParams>>,
    /// +0x00：u64（旗写面 = {u32 1, u32 0}；O generate_flags 0x1407ff6c0）。
    pub u50: u64,
    /// +0x80..+0xb0：交互网格（set_interaction_mesh 0x1408255e0 写；None=未设）。
    pub interaction: Option<InteractionMesh>,
    /// +0x94：u64（U）。
    pub u94: u64,
    /// +0x10/+0x18：旗布基（O 写面 {0.5,-0.7071,0.7071,0.5} = 四元数分量；
    /// generate_flags 0x1407ff6c0）。
    pub flag_cloth_basis: [f32; 4],
    /// +0x20..0x40：变换槽（O 写面 pos/quat；CachedDecoratorTransform 语义 U）。
    pub transform_slots: [u64; 4],
    /// Native +0x138: changed/new blueprints with nonzero byte skip both
    /// upserts, leaving their existing collider keys pending removal.
    pub u138: u8,
    /// 碰撞体列表（set_colliders 0x14017a22f0/0x1408ed6f0 消费面；原 +0x0 区域）。
    pub colliders: Vec<DecoratorCollision>,
    /// Canonical outline Transform values: rotation/translation/scale, as read
    /// by native Option<Transform>::eq@1408251f0. Clutter's actual writer is
    /// set_cached_transform. This is the fullvisual presence check's field.
    pub transform: Option<
        country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransform,
    >,
    /// Legacy tuple compatibility only. It is not the original optional outline
    /// Transform and cannot satisfy the actual fullvisual presence check.
    pub outline: Option<(f32, f32, f32)>,
    /// Legacy scalar compatibility; not the native boxed ChimneyAssemblyParams.
    pub chimney: Option<f32>,
    /// Legacy byte compatibility; not the native boxed ClothInfo.
    pub cloth: Option<u8>,
}

impl DecoratorBlueprint {
    /// Empty original output state, including Unspecified mesh slots. This is
    /// semantic Default, not an all-zero native memory layout claim.
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
        self.door_stairs_assembly = Some(Box::new(DoorStairsAssemblyParams { words: params }));
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
    mut storage: ResMut<DecoratorStorage>,
    mut blueprints: ResMut<DecoratorBlueprints>,
) {
    // The original marks Storage's changed tick before iter_mut checks its
    // immutable guard. Its body does not clear the authoritative decorator map.
    let storage = &mut *storage;
    assert!(!storage.immutable, "assertion failed: !self.immutable");
    // Native marks the blueprint owner only on an actual attempted ID removal,
    // even when that ID is absent from its map. Merely visiting empty buckets
    // must not mark a stable blueprint resource changed.
    let has_ids = storage
        .per_wall
        .values()
        .chain(storage.terrain.iter())
        .chain(storage.stairs.iter())
        .any(|bucket| !bucket.modified_ids.is_empty() || !bucket.removed_ids.is_empty())
        || storage.removed.values().any(|ids| !ids.is_empty());
    if has_ids {
        blueprints.clear_blueprints(storage);
    }
}

pub fn copy_blueprints_to_prev(
    blueprints: Res<DecoratorBlueprints>,
    mut previous: ResMut<PrevDecoratorBlueprints>,
) {
    blueprints.copy_blueprints_to_prev(&mut previous);
}

/// B registered collider-only system@140827510/1009B. Full visual
/// instantiation also invokes this same helper; it has separate dependencies.
pub fn instantiate_blueprint_colliders(
    mut world: ResMut<RaycastWorld>,
    mut colliders: ResMut<DecoratorPhysicsColliders>,
    current: Res<DecoratorBlueprints>,
    previous: Res<PrevDecoratorBlueprints>,
) {
    instantiate_decorator_colliders(&mut colliders, &current, &previous, &mut world);
}

/// B helper@140829850/6470B: reconcile the two actual live collider maps.
/// The original PHYSICS_DEBUG_LOG_ENABLE/log branches are not recovered here;
/// see COLLIDER_ONLY_PHASE_DESIGN_V1.md. This is the reviewed business sequence,
/// not a claim of complete native diagnostic side-effect equivalence.
pub fn instantiate_decorator_colliders(
    colliders: &mut DecoratorPhysicsColliders,
    current: &DecoratorBlueprints,
    previous: &PrevDecoratorBlueprints,
    world: &mut RaycastWorld,
) {
    let mut collision_removals: HashSet<DecoratorId> = colliders.first.keys().copied().collect();
    let mut interaction_removals: HashSet<DecoratorId> = colliders.second.keys().copied().collect();
    for (&raw_id, blueprint) in &current.map {
        let id = DecoratorId(raw_id);
        // Native pointer fast path precedes the skip byte and both slot checks.
        if previous
            .map
            .get(&raw_id)
            .is_some_and(|old| Arc::ptr_eq(old, blueprint))
        {
            collision_removals.remove(&id);
            interaction_removals.remove(&id);
            continue;
        }
        if blueprint.u138 != 0 {
            continue;
        }
        if let BlueprintMeshSlot::Mesh(positioned) = &blueprint.collision_mesh {
            let source = RaycastGeoSource::decorator(
                id,
                blueprint
                    .storage_addr
                    .expect("collider blueprint has no recovered storage source"),
            );
            let outcome = upsert_collider(
                world,
                &mut colliders.first,
                id,
                source,
                Arc::clone(&positioned.mesh),
                positioned.position,
                positioned.rotation,
            );
            if !matches!(outcome, ColliderUpsert::Inserted { .. }) {
                collision_removals.remove(&id);
            }
        }
        // First Disabled/Unspecified both skip. Check second only after the
        // first slot's real side effects; prevalidating would change panic order.
        match &blueprint.interaction_mesh {
            BlueprintMeshSlot::Mesh(positioned) => {
                let source = RaycastGeoSource::decorator_interaction(
                    id,
                    blueprint
                        .storage_addr
                        .expect("interaction blueprint has no recovered storage source"),
                    blueprint.interaction_source_flags,
                );
                let outcome = upsert_collider(
                    world,
                    &mut colliders.second,
                    id,
                    source,
                    Arc::clone(&positioned.mesh),
                    positioned.position,
                    positioned.rotation,
                );
                if !matches!(outcome, ColliderUpsert::Inserted { .. }) {
                    interaction_removals.remove(&id);
                }
            }
            BlueprintMeshSlot::Disabled => {}
            BlueprintMeshSlot::Unspecified => {
                panic!("decorator blueprint did not specify its interaction mesh")
            }
        }
    }
    // The sets originate from live owners, so even an empty current/prev pair
    // removes stale geometry. Preserve original first-map-before-second order.
    for id in collision_removals {
        remove_collider(world, &mut colliders.first, &id);
    }
    for id in interaction_removals {
        remove_collider(world, &mut colliders.second, &id);
    }
}

/// Original B fullvisual system@140827910/7991B. The three recurring output
/// accumulators are filled before either Arc identity or value equality can
/// retain persistent entities. The independent original preview collider phase
/// does not replace this body's real collider-helper call.
#[allow(clippy::too_many_arguments)]
pub fn instantiate_blueprints(
    visual_state: ResMut<DecoratorVisualState>,
    wall_holes: ResMut<WallHoles>,
    commands: Commands,
    world: ResMut<RaycastWorld>,
    physics_colliders: ResMut<DecoratorPhysicsColliders>,
    window_candidates: ResMut<WindowAutoClutterCandidates>,
    chimney_tracker: ResMut<WallAttachedDecoTracker<ChimneyAssemblyParams>>,
    door_stairs_tracker: ResMut<WallAttachedDecoTracker<DoorStairsAssemblyParams>>,
    blueprints: Res<DecoratorBlueprints>,
    previous: Res<PrevDecoratorBlueprints>,
    meshes: Res<AssetMeshLibrary>,
    shaders: Res<SharedAssetShaderLibrary>,
    _textures: Res<AssetTextureLibrary>,
    on_wall_deleted: EventReader<OnWallDeleted>,
) {
    instantiate_blueprint_visual_state(
        visual_state,
        wall_holes,
        commands,
        world,
        physics_colliders,
        window_candidates,
        chimney_tracker,
        door_stairs_tracker,
        blueprints,
        previous,
        meshes,
        shaders,
        on_wall_deleted,
    );
}

/// Same production body with the original tracked SystemParam wrappers. The
/// public system retains the unused texture library scheduling dependency;
/// keeping these wrappers avoids eagerly marking unvisited mutable owners.
#[allow(clippy::too_many_arguments)]
fn instantiate_blueprint_visual_state(
    mut visual_state: ResMut<DecoratorVisualState>,
    mut wall_holes: ResMut<WallHoles>,
    mut commands: Commands,
    mut world: ResMut<RaycastWorld>,
    mut physics_colliders: ResMut<DecoratorPhysicsColliders>,
    mut window_candidates: ResMut<WindowAutoClutterCandidates>,
    mut chimney_tracker: ResMut<WallAttachedDecoTracker<ChimneyAssemblyParams>>,
    mut door_stairs_tracker: ResMut<WallAttachedDecoTracker<DoorStairsAssemblyParams>>,
    blueprints: Res<DecoratorBlueprints>,
    previous: Res<PrevDecoratorBlueprints>,
    meshes: Res<AssetMeshLibrary>,
    shaders: Res<SharedAssetShaderLibrary>,
    mut on_wall_deleted: EventReader<OnWallDeleted>,
) {
    visual_state.cloth_infos.clear();
    instantiate_decorator_colliders(&mut physics_colliders, &blueprints, &previous, &mut world);
    let mut removed_ids: HashSet<u32> = previous.map.keys().copied().collect();

    for (&id, blueprint) in &blueprints.map {
        let old = previous.map.get(&id);
        let identity_changed = old.map_or(true, |old| !Arc::ptr_eq(blueprint, old));
        removed_ids.remove(&id);

        if !blueprint.wall_holes.is_empty() {
            let Some(wall) = blueprint
                .storage_addr
                .and_then(DecoratorStorageAddr::try_as_wall)
            else {
                log::error!(
                    "Blueprint asks for for window holes, but it was not created as a wall decorator."
                );
                panic!("explicit panic");
            };
            wall_holes
                .extend(wall, blueprint.wall_holes.iter().copied())
                .unwrap_or_else(|error| panic!("wall hole accumulation failed: {error:?}"));
        }
        window_candidates
            .0
            .extend(blueprint.window_auto_clutter_candidates.iter().cloned());
        if let Some(cloth) = &blueprint.cloth_info {
            visual_state.cloth_infos.push((**cloth).clone());
        }

        if !identity_changed || old.is_some_and(|old| blueprint_value_eq(blueprint, old)) {
            continue;
        }
        remove_persistent_visuals(&mut visual_state, &mut commands, id);
        for builder in blueprint.visual_entities.clone() {
            builder.build(
                &mut commands,
                id,
                &meshes,
                &shaders.0,
                &mut visual_state.persistent,
            );
        }
        visual_state.seal_persistent_entities(id);

        // Native checks the actual optional outline Transform only after all
        // persistent build/seal effects. Clutter's real writer is precisely
        // set_cached_transform; the unrelated legacy outline tuple is not it.
        if blueprint.transform.is_none() {
            log::error!(
                "Decorator blueprint did not specify an outline.\nMake sure to call `set_outline` on the `DecoratorBlueprint`."
            );
            panic!("explicit panic");
        }
        let id = DecoratorId(id);
        if let Some(old) = old {
            if old.chimney_assembly.is_some() {
                chimney_tracker.remove(
                    old.storage_addr
                        .expect("previous chimney source is unrecovered"),
                    id,
                );
            }
            if old.door_stairs_assembly.is_some() {
                door_stairs_tracker.remove(
                    old.storage_addr
                        .expect("previous door-stairs source is unrecovered"),
                    id,
                );
            }
        }
        if let Some(params) = &blueprint.chimney_assembly {
            chimney_tracker.add(
                blueprint
                    .storage_addr
                    .expect("chimney source is unrecovered"),
                id,
                params,
            );
        }
        if let Some(params) = &blueprint.door_stairs_assembly {
            door_stairs_tracker.add(
                blueprint
                    .storage_addr
                    .expect("door-stairs source is unrecovered"),
                id,
                params,
            );
        }
    }

    for id in removed_ids {
        remove_persistent_visuals(&mut visual_state, &mut commands, id);
        let old = previous
            .map
            .get(&id)
            .expect("removed previous blueprint is missing");
        let id = DecoratorId(id);
        if old.chimney_assembly.is_some() {
            chimney_tracker.remove(
                old.storage_addr
                    .expect("removed chimney source is unrecovered"),
                id,
            );
        }
        if old.door_stairs_assembly.is_some() {
            door_stairs_tracker.remove(
                old.storage_addr
                    .expect("removed door-stairs source is unrecovered"),
                id,
            );
        }
    }
    // Reading the actual event cursor remains after all earlier side effects.
    for deleted in on_wall_deleted.read() {
        chimney_tracker.on_wall_removed(deleted.0);
        door_stairs_tracker.on_wall_removed(deleted.0);
    }
}

fn remove_persistent_visuals(state: &mut DecoratorVisualState, commands: &mut Commands, id: u32) {
    if let Some(previous) = state.remove_persistent_entities(id) {
        for entity in previous.entities {
            commands.entity(entity).despawn();
        }
    }
}

/// Original whole-blueprint equality from the complete body, without comparing
/// its unrecovered host-only legacy compatibility fields. All actual native
/// outputs below have canonical types; a caller cannot replace it with builder
/// PartialEq or mesh hashes.
fn blueprint_value_eq(left: &DecoratorBlueprint, right: &DecoratorBlueprint) -> bool {
    left.storage_addr == right.storage_addr
        && left.visual_entities.len() == right.visual_entities.len()
        && left
            .visual_entities
            .iter()
            .zip(&right.visual_entities)
            .all(|(a, b)| visual_builder_value_eq(a, b))
        && left.wall_holes == right.wall_holes
        && left.dormer_holes.len() == right.dormer_holes.len()
        && left
            .dormer_holes
            .iter()
            .zip(&right.dormer_holes)
            .all(|(a, b)| dormer_hole_value_eq(a, b))
        && left.collision_mesh == right.collision_mesh
        && left.interaction_mesh == right.interaction_mesh
        && left.window_auto_clutter_candidates == right.window_auto_clutter_candidates
        && left.cloth_info == right.cloth_info
        && left.chimney_assembly == right.chimney_assembly
        && left.door_stairs_assembly == right.door_stairs_assembly
        && left.transform == right.transform
        && left.u138 == right.u138
        && left.interaction_source_flags == right.interaction_source_flags
}

/// Native full-visual equality for the canonical 48-byte GPU record. The
/// original compares words 3/7 as integers and every other word as an ordered
/// f32; deriving Eq on the GPU owner's twelve raw words is not this contract.
fn dormer_hole_value_eq(left: &DormerHoleSsbo, right: &DormerHoleSsbo) -> bool {
    left.0
        .iter()
        .zip(&right.0)
        .enumerate()
        .all(|(index, (&a, &b))| {
            if matches!(index, 3 | 7) {
                a == b
            } else {
                f32::from_bits(a) == f32::from_bits(b)
            }
        })
}

fn option_value_eq<T>(
    left: Option<&T>,
    right: Option<&T>,
    equal: impl FnOnce(&T, &T) -> bool,
) -> bool {
    match (left, right) {
        (None, None) => true,
        (Some(a), Some(b)) => equal(a, b),
        _ => false,
    }
}

fn packed_float_eq(left: &[u8], right: &[u8], offset: usize) -> bool {
    let a = u32::from_le_bytes(left[offset..offset + 4].try_into().unwrap());
    let b = u32::from_le_bytes(right[offset..offset + 4].try_into().unwrap());
    f32::from_bits(a) == f32::from_bits(b)
}

fn packed_float_pair_eq(left: u64, right: u64) -> bool {
    f32::from_bits(left as u32) == f32::from_bits(right as u32)
        && f32::from_bits((left >> 32) as u32) == f32::from_bits((right >> 32) as u32)
}

/// Exact projection of the represented native builder fields, from the complete
/// slice equality at 0x1412a92a0/2178B. This deliberately does not derive byte
/// equality for packed floats or compare host-only convenience fields. Original
/// native None markers (BrickColor/WallFlag/Smoke=2, solid selector=9) are
/// projected before inspecting payload, just as that helper does.
fn visual_builder_value_eq(
    left: &WindowVisualEntityBuilder,
    right: &WindowVisualEntityBuilder,
) -> bool {
    let brick_color_eq = || {
        option_value_eq(
            left.brick_color.as_ref().filter(|value| value[20] != 2),
            right.brick_color.as_ref().filter(|value| value[20] != 2),
            |a, b| {
                a[0..8] == b[0..8]
                    && [8, 12, 16]
                        .into_iter()
                        .all(|offset| packed_float_eq(a, b, offset))
                    && a[20] == b[20]
            },
        )
    };
    let wall_flag_eq = || {
        option_value_eq(
            left.wall_flag_cloth
                .as_ref()
                .filter(|value| value.2 as u8 != 2),
            right
                .wall_flag_cloth
                .as_ref()
                .filter(|value| value.2 as u8 != 2),
            |a, b| {
                a.0 as u32 == b.0 as u32
                    && f32::from_bits((a.0 >> 32) as u32) == f32::from_bits((b.0 >> 32) as u32)
                    && packed_float_pair_eq(a.1, b.1)
                    && a.2 as u8 == b.2 as u8
            },
        )
    };
    left.translation == right.translation
        && left.rotation == right.rotation
        && left.scale == right.scale
        && left.semi_transparent == right.semi_transparent
        && left.color_id == right.color_id
        && left.normal_offset == right.normal_offset
        && left.mesh_name == right.mesh_name
        && left.normal_map == right.normal_map
        && left.nani_name == right.nani_name
        && left.scale_override == right.scale_override
        && option_value_eq(
            left.gothic_window_brick_deformation.as_ref(),
            right.gothic_window_brick_deformation.as_ref(),
            |a, b| (0..60).step_by(4).all(|offset| packed_float_eq(a, b, offset)),
        )
        && brick_color_eq()
        && option_value_eq(
            left.clothline_cloth.as_ref(),
            right.clothline_cloth.as_ref(),
            |a, b| packed_float_pair_eq(a.0, b.0) && a.1 == b.1,
        )
        && left.window_light == right.window_light
        && left.balcony_door == right.balcony_door
        && left.smoke.as_ref().filter(|value| value.0 != 2)
            == right.smoke.as_ref().filter(|value| value.0 != 2)
        && left.roof_flag_cloth == right.roof_flag_cloth
        && wall_flag_eq()
        && option_value_eq(
            left.sittable_for_birds.as_ref(),
            right.sittable_for_birds.as_ref(),
            |a, b| {
                packed_float_pair_eq(a.0, b.0) && f32::from_bits(a.1) == f32::from_bits(b.1)
            },
        )
        // NameHash::eq@140c9cd70 is an exact u64 comparison.
        && left.albedo_map == right.albedo_map
        && left.solid_texture.filter(|&value| value != 9)
            == right.solid_texture.filter(|&value| value != 9)
        && left.lantern_light_params == right.lantern_light_params
}

// 原 validate_blueprints539B 与实际回调8161B；完整依赖必须来自同一个 App World。
#[allow(clippy::too_many_arguments)]
pub fn validate_blueprints(
    blueprints: Res<DecoratorBlueprints>,
    walls: Res<country_core::resources::walls::public_walls::PublicWalls>,
    mut storage: ResMut<DecoratorStorage>,
    deferred: Res<crate::deferred_decorator_history::DeferredDecoratorEditHistory>,
    raycast: Res<RaycastWorld>,
    roofs: bevy_ecs::system::Query<&country_core::resources::roofs::roof_shape::Roof>,
    derived: country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,
    mut backup: ResMut<crate::backup::DecoratorBackup>,
    dispatcher: Res<
        crate::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher,
    >,
    mut stream: ResMut<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,
    transforms: Res<
        country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms,
    >,
    mut outlines: bevy_ecs::event::EventWriter<
        country_core::systems::wall::structure_outline::OutlineWallCmd,
    >,
    cursor: Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,
    mut mute: bevy_ecs::event::EventWriter<
        crate::audio::op_preview_audio::MutePreviewDecoratorOpSounds,
    >,
) {
    // 原顺序无条件标记 Storage changed 并只清元数据，随后才检查待提交 records。
    storage.clear_modification_tracking();
    if deferred.records.is_empty() {
        return;
    }
    let mut specific_feedback = false;
    let derived = derived.as_view();
    let rejected = deferred
        .potentially_surviving_affected_decorators()
        .any(|locator| {
            !validate_affected_decorator(
                locator,
                &blueprints,
                &storage,
                &walls,
                &roofs,
                derived,
                &transforms,
                &raycast,
                &mut stream,
                &mut outlines,
                &mut specific_feedback,
            )
        });
    if rejected && backup.created {
        if !specific_feedback {
            if let Some(ty) = dispatcher.decorator_ty() {
                crate::ui_systems::ui_place_decorator::show_decorator_icon(
                    ty.0,
                    cursor.0,
                    &mut stream,
                );
            }
        }
        backup.state = 2;
        mute.write(crate::audio::op_preview_audio::MutePreviewDecoratorOpSounds);
    }
}

#[allow(clippy::too_many_arguments)]
fn validate_affected_decorator(
    locator: &country_core::resources::walls::decorator::DecoratorLocator,
    blueprints: &DecoratorBlueprints,
    storage: &DecoratorStorage,
    walls: &country_core::resources::walls::public_walls::PublicWalls,
    roofs: &bevy_ecs::system::Query<&country_core::resources::roofs::roof_shape::Roof>,
    derived: country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoView<'_>,
    transforms: &country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms,
    raycast: &RaycastWorld,
    stream: &mut country_core::systems::ui::signifiers::signifier_create::UiSignifierStream,
    outlines: &mut bevy_ecs::event::EventWriter<
        country_core::systems::wall::structure_outline::OutlineWallCmd,
    >,
    specific_feedback: &mut bool,
) -> bool {
    use crate::cull_oob_decorators::{DecoratorLocationCheckResult, is_decorator_location_valid};
    let id = DecoratorId(locator.decorator);
    // 原140a1c5f0按地址与id直接查桶，不先把同地址编辑折叠或通过另一d2s索引改地址。
    let Some(info) = storage.get(locator.addr).and_then(|bucket| bucket.get(id)) else {
        return true;
    };
    let Some(blueprint) = blueprints.get(id.0) else {
        return false;
    };
    let result = is_decorator_location_valid(
        id,
        info,
        locator.addr,
        walls,
        roofs,
        derived,
        transforms,
        raycast,
    );
    // 原回调在分派 Valid/Invalid 前仍必需读取 subtype 与当前缓存变换。
    let subtype = derived.subtype(id).0;
    let transform = transforms.transform(id.0);
    if let DecoratorLocationCheckResult::Invalid(reason) = result {
        show_intended_placement(subtype, info.rank(), &transform, derived, stream, false);
        *specific_feedback = true;
        show_invalid_location_feedback(id, reason, &transform, walls, stream, outlines);
        return false;
    }
    let DecoratorStorageAddr::Wall(wall) = locator.addr else {
        return true;
    };
    let BlueprintMeshSlot::Mesh(collision) = &blueprint.collision_mesh else {
        return true;
    };
    let shape = validation_overlap_shape(collision);
    let Some(source) = raycast.decorator_overlaps_with_others(&shape, id, wall, false) else {
        return true;
    };
    *specific_feedback = true;
    if source.tag() == 3 {
        let other_id = DecoratorId(u32::from_le_bytes(source.0[4..8].try_into().unwrap()));
        let other_addr = match u64::from_le_bytes(source.0[8..16].try_into().unwrap()) {
            0 => DecoratorStorageAddr::Wall(country_core::resources::walls::WallId(
                u64::from_le_bytes(source.0[16..24].try_into().unwrap()),
            )),
            1 => DecoratorStorageAddr::Terrain,
            2 => DecoratorStorageAddr::Stair,
            tag => panic!("invalid decorator source storage tag {tag}"),
        };
        let other = storage
            .get(other_addr)
            .and_then(|bucket| bucket.get(other_id))
            .expect("blocking decorator source has no owning record");
        let family = other.ty.wrapping_sub(0x38);
        // 原898c..89a1：非该建筑装饰族继续迭代，但保留前面已写的 specific 标志。
        if family >= 8 || family == 6 {
            return true;
        }
        let other_subtype = derived.subtype(other_id).0;
        // 原查找次序先 required collision_outline，后 required transform。
        let other_outline = derived.mesh_library().get_collision_outline(
            country_core::resources::walls::decorator_subtype::DecoratorLibKey::new(
                other_subtype,
                other.rank(),
            ),
        );
        let mut other_transform = transforms.transform(other_id.0);
        if other_subtype == 3 {
            if let Some(transform) = blueprints
                .get(other_id.0)
                .and_then(|blueprint| blueprint.transform)
            {
                other_transform = transform;
            }
        }
        let affine = validation_affine(&other_transform);
        other_outline.display(&affine, stream, true);
    }
    show_intended_placement(subtype, info.rank(), &transform, derived, stream, false);
    false
}

/// 原140b26a80 tuple专化：external * mesh.local；不同于另一 into_transformed_shape 专化。
fn validation_overlap_shape(
    mesh: &PositionedDecoratorMesh,
) -> country_core::systems::collision::world::OffsetColliderTransform {
    let external =
        utils::iso_transform::IsoTransform::from_translation_rotation(mesh.position, mesh.rotation);
    let pose = mesh.mesh.local_transform().nested_in(&external);
    country_core::systems::collision::world::OffsetColliderTransform {
        position: pose.translation,
        rotation: pose.rotation,
        shape: Arc::clone(
            &mesh
                .mesh
                .shape
                .as_ref()
                .expect("blueprint collision has no physical shape")
                .0,
        ),
    }
}

fn validation_affine(
    transform: &country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransform,
) -> glam::Affine3A {
    glam::Affine3A::from_scale_rotation_translation(
        transform.scale,
        transform.rotation,
        transform.translation,
    )
}

fn show_intended_placement(
    subtype: u8,
    rank: u8,
    transform: &country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransform,
    derived: country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoView<'_>,
    stream: &mut country_core::systems::ui::signifiers::signifier_create::UiSignifierStream,
    dashed: bool,
) {
    let outline = derived.mesh_library().get_collision_outline(
        country_core::resources::walls::decorator_subtype::DecoratorLibKey::new(subtype, rank),
    );
    let affine = validation_affine(transform);
    outline.display(&affine, stream, dashed);
}

/// 原434B：prepared XZ 与真实 arch heights zip，减 padding 后按原闭合门追加首点。
fn show_max_wall_y(
    wall: &country_core::resources::walls::public_wall_state::PublicWallState,
    padding: f32,
    stream: &mut country_core::systems::ui::signifiers::signifier_create::UiSignifierStream,
) {
    let path = &wall
        .prepared_path
        .as_ref()
        .expect("wall path has not been prepared")
        .positions;
    let heights = wall
        .arch_segment_heights()
        .expect("wall arch heights have not been calculated");
    let mut points: Vec<_> = path
        .iter()
        .zip(heights)
        .map(|(p, h)| Vec3::new(p[0], *h - padding, p[2]))
        .collect();
    if wall.shape().tag() != 2 {
        points.push(*points.first().expect("empty maximum-wall-y outline"));
    }
    validation_debug_line(points, stream);
}

fn validation_debug_line(
    points: Vec<Vec3>,
    stream: &mut country_core::systems::ui::signifiers::signifier_create::UiSignifierStream,
) {
    country_core::utils::debug_viz::DebugLine::new(points)
        .dashed()
        .line_width(3.0)
        .show(stream);
}

fn feedback_point_coord(
    points: &[[f32; 3]],
    index: usize,
) -> (Vec3, utils::curve::CurveInternalCoord) {
    use utils::curve::{
        CurveCoordSemanticThresholded3D, CurveInternalCoord, curve_slice::CurveSlice,
    };
    let coord = CurveInternalCoord::new_for_point_count(index, 0.0, points.len());
    let view = CurveSlice::<[f32; 3], _, CurveCoordSemanticThresholded3D, Vec3, _>::from_coord_range_with_remap(
        points, coord, coord, |_, point| Vec3::from_array(*point));
    (view.sample_at_coord(coord), coord)
}

#[allow(clippy::too_many_arguments)]
fn show_invalid_location_feedback(
    id: DecoratorId,
    reason: crate::cull_oob_decorators::DecoratorLocationInvalidReason,
    transform: &country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransform,
    walls: &country_core::resources::walls::public_walls::PublicWalls,
    stream: &mut country_core::systems::ui::signifiers::signifier_create::UiSignifierStream,
    outlines: &mut bevy_ecs::event::EventWriter<
        country_core::systems::wall::structure_outline::OutlineWallCmd,
    >,
) {
    use crate::cull_oob_decorators::DecoratorLocationInvalidReason as Reason;
    match reason {
        Reason::WallCutoutBoundary { wall, coord } => {
            let wall = walls
                .get(wall)
                .expect("invalid location wall does not exist");
            let path = &wall
                .prepared_path
                .as_ref()
                .expect("wall path has not been prepared")
                .positions;
            let mut points = Vec::with_capacity(9);
            for offset in -4..=4 {
                let index = (coord.segment as u32).wrapping_add_signed(offset) as i32;
                // 原此分支用独立第三高度表的 len 作范围门，不是 prepared point len。
                if index < 0 || index >= wall.segment_heights.len() as i32 {
                    continue;
                }
                let (mut point, coordinate) = feedback_point_coord(path, index as usize);
                let first = wall.segment_heights[coordinate.segment];
                let height =
                    (wall.segment_heights[coordinate.segment + 1] - first) * coordinate.t + first;
                point.y = height + point.y;
                points.push(point);
            }
            validation_debug_line(points, stream);
        }
        Reason::RoofBounds {
            position,
            direction,
            wall,
        } => {
            validation_debug_line(vec![position, position - direction * 2.0], stream);
            outlines.write(
                country_core::systems::wall::structure_outline::OutlineWallCmd {
                    wall,
                    strength: 2.0,
                    kind: 0,
                },
            );
        }
        Reason::WallEnds {
            left_oob,
            right_oob,
            wall,
        } => {
            let wall = walls
                .get(wall)
                .expect("invalid location wall does not exist");
            if left_oob {
                let bottom = Vec3::from_array(
                    *wall
                        .geometry
                        .outline
                        .first()
                        .expect("wall original outline is empty"),
                );
                let top = *wall
                    .arch_segment_heights()
                    .expect("wall arch heights missing")
                    .first()
                    .expect("wall arch heights empty");
                validation_debug_line(vec![bottom, Vec3::new(bottom.x, top, bottom.z)], stream);
            }
            if right_oob {
                let bottom = Vec3::from_array(
                    *wall
                        .geometry
                        .outline
                        .last()
                        .expect("wall original outline is empty"),
                );
                let top = *wall
                    .arch_segment_heights()
                    .expect("wall arch heights missing")
                    .last()
                    .expect("wall arch heights empty");
                validation_debug_line(vec![bottom, Vec3::new(bottom.x, top, bottom.z)], stream);
            }
        }
        Reason::SharpWallTurn { wall, coord } => {
            let wall = walls
                .get(wall)
                .expect("invalid location wall does not exist");
            let path = &wall
                .prepared_path
                .as_ref()
                .expect("wall path has not been prepared")
                .positions;
            let mut points = Vec::with_capacity(9);
            for offset in -4..=4 {
                let index = (coord.segment as u32).wrapping_add_signed(offset) as i32;
                if index < 0 || index >= path.len() as i32 {
                    continue;
                }
                let (point, _) = feedback_point_coord(path, index as usize);
                points.push(Vec3::new(point.x, transform.translation.y, point.z));
            }
            validation_debug_line(points, stream);
        }
        Reason::WallTop { padding, wall } => show_max_wall_y(
            walls
                .get(wall)
                .expect("invalid location wall does not exist"),
            padding,
            stream,
        ),
        Reason::BalconyUpperClearance { wall } => show_max_wall_y(
            walls
                .get(wall)
                .expect("invalid location wall does not exist"),
            0.0,
            stream,
        ),
        Reason::WallCorner {
            wall,
            rectangle,
            corner,
        } => {
            let wall = walls
                .get(wall)
                .expect("invalid location wall does not exist");
            let point = rectangle.get_point(corner);
            // 原595B最近坐标 kernel 复用唯一curve原语，不重新建一条不同长度缓存的curve。
            let coordinate =
                utils::curve::approx_curve_coord(wall.geometry.outline.as_slice(), point, |p| {
                    glam::Vec2::new(p[0], p[2])
                });
            let first = wall.geometry.outline[coordinate.segment][1];
            let next = wall.geometry.outline[coordinate.segment + 1][1];
            let bottom = next * coordinate.t + first * (1.0 - coordinate.t);
            validation_debug_line(
                vec![
                    Vec3::new(point.x, bottom, point.y),
                    Vec3::new(point.x, wall.max_y(), point.y),
                ],
                stream,
            );
            show_delete_scribble(id, transform, stream);
        }
        Reason::WallLowerBoundary { wall } => {
            let wall = walls
                .get(wall)
                .expect("invalid location wall does not exist");
            validation_debug_line(
                wall.geometry
                    .outline
                    .iter()
                    .copied()
                    .map(Vec3::from_array)
                    .collect(),
                stream,
            );
        }
        Reason::FlatRoofForbidden | Reason::GableBandForbidden | Reason::MissingRoof => {}
    }
}

/// 原473B闭包：真实命名网格/位置/旋转/scale.5/duration0，归共享builder与资源owner。
fn show_delete_scribble(
    id: DecoratorId,
    transform: &country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransform,
    stream: &mut country_core::systems::ui::signifiers::signifier_create::UiSignifierStream,
) {
    use country_core::systems::ui::signifiers::signifier_factory::{
        SignifierEntityBuilder, SignifierFactory,
    };
    let position = transform.translation;
    let rotation = transform.rotation;
    let name = format!("despawn_decorator_{id:?}_{position:?}");
    let factory = SignifierFactory::from_closure(
        &name,
        Arc::new(move |meshes, shaders, commands| {
            vec![
                SignifierEntityBuilder::from_mesh_name("decorator_delete_scribble")
                    .animation_duration(0.0)
                    .position(position)
                    .rotation(rotation)
                    .scale(0.5)
                    .build(meshes, shaders, commands),
            ]
        }),
    );
    stream.write_factory(&factory);
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

    fn real_clutter_blueprint(current: &mut DecoratorBlueprints, id: u32) {
        let center = Vec3::new(0.0, 2.0, 0.0);
        let blueprint = current.insert_for(id, DecoratorStorageAddr::Terrain, 0);
        blueprint.set_positioned_colliders(
            Arc::new(RaycastMeshData::new_cuboid_at(
                Vec3::new(1.5, 3.5, 5.5),
                center,
            )),
            Arc::new(RaycastMeshData::new_cuboid_at(
                Vec3::new(2.0, 4.0, 6.0),
                center,
            )),
            &Vec3::ZERO,
            &Quat::IDENTITY,
        );
    }

    #[test]
    fn real_phase_creates_two_distinct_receipts_and_queries_both_shapes() {
        let id = DecoratorId(71);
        let mut current = DecoratorBlueprints::default();
        real_clutter_blueprint(&mut current, id.0);
        let previous = PrevDecoratorBlueprints::default();
        let mut maps = DecoratorPhysicsColliders::default();
        let mut world = RaycastWorld::new();
        instantiate_decorator_colliders(&mut maps, &current, &previous, &mut world);
        let first = maps.first[&id];
        let second = maps.second[&id];
        assert_ne!(first.registry_index, second.registry_index);
        assert_ne!(first.handle().slot, second.handle().slot);
        assert_eq!(
            world
                .get_collider_mesh(first.registry_index, first.generation)
                .source,
            RaycastGeoSource::decorator(id, DecoratorStorageAddr::Terrain),
        );
        assert_eq!(
            world
                .get_collider_mesh(second.registry_index, second.generation)
                .source,
            RaycastGeoSource::decorator_interaction(id, DecoratorStorageAddr::Terrain, 0),
        );
        world.flush_colliders();
        let ray_origin = Vec3::new(0.0, 10.0, 0.0);
        assert_eq!(
            world
                .raycast(ray_origin, Vec3::NEG_Y, 1 << first.handle().slot)
                .toi,
            6.25
        );
        assert_eq!(
            world
                .raycast(ray_origin, Vec3::NEG_Y, 1 << second.handle().slot)
                .toi,
            6.0
        );
    }

    #[test]
    fn unchanged_arc_precedes_skip_flag_but_changed_skip_removes_both() {
        let id = DecoratorId(72);
        let mut current = DecoratorBlueprints::default();
        real_clutter_blueprint(&mut current, id.0);
        let mut previous = PrevDecoratorBlueprints::default();
        let mut maps = DecoratorPhysicsColliders::default();
        let mut world = RaycastWorld::new();
        instantiate_decorator_colliders(&mut maps, &current, &previous, &mut world);
        let first = maps.first[&id].handle();
        let second = maps.second[&id].handle();
        current.get_mut(id.0).unwrap().u138 = 1;
        current.copy_blueprints_to_prev(&mut previous);
        instantiate_decorator_colliders(&mut maps, &current, &previous, &mut world);
        assert_eq!(maps.first[&id].handle(), first);
        assert_eq!(maps.second[&id].handle(), second);
        current
            .insert_for(id.0, DecoratorStorageAddr::Terrain, 0)
            .u138 = 1;
        instantiate_decorator_colliders(&mut maps, &current, &previous, &mut world);
        assert!(maps.first.is_empty());
        assert!(maps.second.is_empty());
    }

    #[test]
    fn first_unspecified_skips_but_second_unspecified_panics_after_first_upsert() {
        let id = DecoratorId(73);
        let mut current = DecoratorBlueprints::default();
        let blueprint = current.insert_for(id.0, DecoratorStorageAddr::Terrain, 0);
        blueprint.interaction_mesh = BlueprintMeshSlot::Disabled;
        let previous = PrevDecoratorBlueprints::default();
        let mut maps = DecoratorPhysicsColliders::default();
        let mut world = RaycastWorld::new();
        instantiate_decorator_colliders(&mut maps, &current, &previous, &mut world);
        assert!(maps.first.is_empty() && maps.second.is_empty());
        let blueprint = current.insert_for(id.0, DecoratorStorageAddr::Terrain, 0);
        blueprint.collision_mesh = BlueprintMeshSlot::Mesh(PositionedDecoratorMesh {
            mesh: Arc::new(RaycastMeshData::new_cuboid_at(Vec3::splat(2.0), Vec3::Y)),
            position: Vec3::ZERO,
            rotation: Quat::IDENTITY,
        });
        let failure = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            instantiate_decorator_colliders(&mut maps, &current, &previous, &mut world);
        }));
        assert!(failure.is_err());
        assert_eq!(
            maps.first.len(),
            1,
            "the original first upsert precedes this failure"
        );
        assert!(maps.second.is_empty());
        let receipt = maps.first[&id];
        assert_eq!(
            world
                .get_collider_mesh(receipt.registry_index, receipt.generation)
                .source,
            RaycastGeoSource::decorator(id, DecoratorStorageAddr::Terrain),
        );
    }

    #[test]
    fn explicit_disable_and_empty_current_clean_actual_live_maps_without_prev_keys() {
        let id = DecoratorId(74);
        let mut current = DecoratorBlueprints::default();
        real_clutter_blueprint(&mut current, id.0);
        let mut previous = PrevDecoratorBlueprints::default();
        let mut maps = DecoratorPhysicsColliders::default();
        let mut world = RaycastWorld::new();
        instantiate_decorator_colliders(&mut maps, &current, &previous, &mut world);
        let second = maps.second[&id].handle();
        current.copy_blueprints_to_prev(&mut previous);
        current.get_mut(id.0).unwrap().collision_mesh = BlueprintMeshSlot::Disabled;
        instantiate_decorator_colliders(&mut maps, &current, &previous, &mut world);
        assert!(maps.first.is_empty());
        assert_eq!(maps.second[&id].handle(), second);
        current.map.clear();
        previous.map.clear();
        instantiate_decorator_colliders(&mut maps, &current, &previous, &mut world);
        assert!(maps.first.is_empty() && maps.second.is_empty());
    }

    #[test]
    fn native_dormer_equality_uses_float_words_and_keeps_integer_words_exact() {
        let a = DormerHoleSsbo([0; 12]);
        let mut b = a;
        b.0[4] = (-0.0f32).to_bits();
        assert!(dormer_hole_value_eq(&a, &b));
        b.0[3] = (-0.0f32).to_bits();
        assert!(!dormer_hole_value_eq(&a, &b));
        b.0[3] = 0;
        b.0[11] = f32::NAN.to_bits();
        assert!(!dormer_hole_value_eq(&b, &b));
    }

    #[test]
    fn native_builder_gothic_packed_floats_preserve_ordered_nan_and_zero_rules() {
        let mut a = WindowVisualEntityBuilder::default();
        a.gothic_window_brick_deformation = Some([0; 60]);
        let mut b = a.clone();
        b.gothic_window_brick_deformation.as_mut().unwrap()[56..60]
            .copy_from_slice(&(-0.0f32).to_le_bytes());
        assert!(visual_builder_value_eq(&a, &b));
        b.gothic_window_brick_deformation.as_mut().unwrap()[24..28]
            .copy_from_slice(&f32::NAN.to_le_bytes());
        assert!(!visual_builder_value_eq(&b, &b));
    }

    #[test]
    fn native_builder_brick_and_wall_flags_ignore_only_original_padding() {
        let mut a = WindowVisualEntityBuilder::default();
        a.brick_color = Some([0; 24]);
        a.wall_flag_cloth = Some((5, 0, 0));
        let mut b = a.clone();
        b.brick_color.as_mut().unwrap()[21..24].copy_from_slice(&[17, 39, 91]);
        b.brick_color.as_mut().unwrap()[8..12].copy_from_slice(&(-0.0f32).to_le_bytes());
        b.wall_flag_cloth = Some((5 | (u64::from((-0.0f32).to_bits()) << 32), 0, 0xabc0_0000));
        assert!(visual_builder_value_eq(&a, &b));
        b.wall_flag_cloth.as_mut().unwrap().0 += 1;
        assert!(!visual_builder_value_eq(&a, &b));
        b = a.clone();
        b.brick_color.as_mut().unwrap()[12..16].copy_from_slice(&f32::NAN.to_le_bytes());
        assert!(!visual_builder_value_eq(&b, &b));
    }

    #[test]
    fn native_builder_distinguishes_bird_float_and_clothline_integer_tail() {
        let mut a = WindowVisualEntityBuilder::default();
        a.sittable_for_birds = Some((0, 0));
        a.clothline_cloth = Some((0, 0));
        let mut b = a.clone();
        b.sittable_for_birds = Some((u64::from((-0.0f32).to_bits()) << 32, (-0.0f32).to_bits()));
        b.clothline_cloth = Some((u64::from((-0.0f32).to_bits()), 0));
        assert!(visual_builder_value_eq(&a, &b));
        b.clothline_cloth.as_mut().unwrap().1 = (-0.0f32).to_bits();
        assert!(!visual_builder_value_eq(&a, &b));
        b = a.clone();
        b.sittable_for_birds.as_mut().unwrap().1 = f32::NAN.to_bits();
        assert!(!visual_builder_value_eq(&b, &b));
    }

    #[test]
    fn native_builder_equality_uses_actual_names_and_not_host_outer_id() {
        let mut a = WindowVisualEntityBuilder::default();
        a.mesh_name = "plant_pot_v1";
        a.nani_name = "nani_clutter_SolidAlbedoMap";
        let mut b = a.clone();
        b.decorator_id = 99;
        b.preview = true;
        b.window_glass = true;
        b.shader = Some(0x1234);
        assert!(visual_builder_value_eq(&a, &b));
        b.mesh_name = "plant_pot_v2";
        assert!(!visual_builder_value_eq(&a, &b));
    }

    struct FullVisualFixture {
        ecs: bevy_ecs::world::World,
    }

    impl FullVisualFixture {
        fn new(id: u32) -> Self {
            use country_core::resources::walls::{
                WallId, cached_decorator_transforms::CachedDecoratorTransform,
            };
            use country_core::systems::collision::WindowAutoClutterKind;
            let mut current = DecoratorBlueprints::default();
            real_clutter_blueprint(&mut current, id);
            let blueprint = current.get_mut(id).unwrap();
            blueprint.storage_addr = Some(DecoratorStorageAddr::Wall(WallId(501)));
            blueprint.set_cached_transform(CachedDecoratorTransform {
                rotation: Quat::IDENTITY,
                translation: Vec3::ZERO,
                scale: Vec3::ONE,
            });
            let mut builder = WindowVisualEntityBuilder::new(Vec3::ZERO, Quat::IDENTITY, 999);
            builder.mesh_name = "clutter/plant_pot_v1";
            builder.nani_name = "nani_solid_vertex_color";
            blueprint.visual_entities.push(builder);
            blueprint.wall_holes.push(RawWallHole {
                center: [0.0, 0.0],
                dimensions: [1.0, 1.0],
                kind: 0,
            });
            blueprint
                .window_auto_clutter_candidates
                .push(WindowAutoClutterCandidate {
                    kind: WindowAutoClutterKind::WindowFlowerbed { mesh_index: 0 },
                    rotation: Quat::IDENTITY,
                    translation: Vec3::ZERO,
                    scale: Vec3::ONE,
                    key: 0xabc,
                    wall: WallId(501),
                });
            blueprint.cloth_info = Some(Box::new(ClothInfo {
                transform: country_core::utils::components::transform::Transform::identity(),
                wall: WallId(501),
                dimensions: [2.0, 3.0],
                decorator_id: DecoratorId(id),
            }));
            let mut ecs = bevy_ecs::world::World::new();
            ecs.insert_resource(current);
            ecs.insert_resource(PrevDecoratorBlueprints::default());
            ecs.insert_resource(DecoratorVisualState::default());
            ecs.insert_resource(WallHoles::default());
            ecs.insert_resource(RaycastWorld::new());
            ecs.insert_resource(DecoratorPhysicsColliders::default());
            ecs.insert_resource(WindowAutoClutterCandidates::default());
            ecs.insert_resource(WallAttachedDecoTracker::<ChimneyAssemblyParams>::default());
            ecs.insert_resource(WallAttachedDecoTracker::<DoorStairsAssemblyParams>::default());
            ecs.insert_resource(AssetMeshLibrary::new());
            ecs.insert_resource(SharedAssetShaderLibrary(Arc::new(AssetShaderLibrary::new())));
            ecs.insert_resource(bevy_ecs::event::Events::<OnWallDeleted>::default());
            Self { ecs }
        }

        fn run(&mut self) {
            use bevy_ecs::system::RunSystemOnce;
            self.ecs
                .run_system_once(instantiate_blueprint_visual_state)
                .unwrap();
        }

        fn snapshot(&mut self) {
            let mut params = bevy_ecs::system::SystemState::<(
                Res<DecoratorBlueprints>,
                ResMut<PrevDecoratorBlueprints>,
            )>::new(&mut self.ecs);
            let (current, mut previous) = params.get_mut(&mut self.ecs);
            current.copy_blueprints_to_prev(&mut previous);
        }

        fn reset_surrounding_frame_outputs(&mut self) {
            // These are caller-owned/drain fixture phases, not another consumer.
            *self.ecs.resource_mut::<WallHoles>() = WallHoles::default();
            self.ecs
                .resource_mut::<WindowAutoClutterCandidates>()
                .0
                .clear();
        }
    }

    #[test]
    fn fullvisual_arc_and_value_equal_frames_recollect_all_outputs_without_respawn() {
        use country_core::resources::walls::WallId;
        let mut fixture = FullVisualFixture::new(81);
        fixture.run();
        let entities = fixture
            .ecs
            .resource::<DecoratorVisualState>()
            .persistent_entities_of(81)
            .unwrap()
            .to_vec();
        assert_eq!(entities.len(), 1);
        assert!(
            fixture
                .ecs
                .resource::<DecoratorVisualState>()
                .persistent
                .get(999)
                .is_none(),
            "map ID is authoritative"
        );
        fixture.snapshot();
        for different_arc in [false, true] {
            fixture.reset_surrounding_frame_outputs();
            if different_arc {
                let old = fixture.ecs.resource::<DecoratorBlueprints>().map[&81]
                    .as_ref()
                    .clone();
                fixture
                    .ecs
                    .resource_mut::<DecoratorBlueprints>()
                    .map
                    .insert(81, Arc::new(old));
                assert!(!Arc::ptr_eq(
                    &fixture.ecs.resource::<DecoratorBlueprints>().map[&81],
                    &fixture.ecs.resource::<PrevDecoratorBlueprints>().map[&81]
                ));
            }
            fixture.run();
            let state = fixture.ecs.resource::<DecoratorVisualState>();
            assert_eq!(state.persistent_entities_of(81).unwrap(), entities);
            assert!(state.persistent.get(81).unwrap().is_sealed());
            assert!(fixture.ecs.entities().contains(entities[0]));
            assert_eq!(
                fixture
                    .ecs
                    .resource::<WindowAutoClutterCandidates>()
                    .0
                    .len(),
                1
            );
            assert_eq!(state.cloth_infos.len(), 1, "first Vec clears then refills");
            assert_eq!(
                fixture.ecs.resource::<WallHoles>().storage[&WallId(501)]
                    .raw_holes()
                    .len(),
                1
            );
        }
    }

    #[test]
    fn fullvisual_changed_builder_replaces_and_removed_id_despawns_actual_entities() {
        let mut fixture = FullVisualFixture::new(82);
        fixture.run();
        let old = fixture
            .ecs
            .resource::<DecoratorVisualState>()
            .persistent_entities_of(82)
            .unwrap()[0];
        fixture.snapshot();
        fixture
            .ecs
            .resource_mut::<DecoratorBlueprints>()
            .get_mut(82)
            .unwrap()
            .visual_entities[0]
            .translation
            .x = 1.0;
        fixture.reset_surrounding_frame_outputs();
        fixture.run();
        let new = fixture
            .ecs
            .resource::<DecoratorVisualState>()
            .persistent_entities_of(82)
            .unwrap()[0];
        assert_ne!(old, new);
        assert!(!fixture.ecs.entities().contains(old));
        assert!(fixture.ecs.entities().contains(new));
        fixture.snapshot();
        fixture
            .ecs
            .resource_mut::<DecoratorBlueprints>()
            .map
            .remove(&82);
        fixture.reset_surrounding_frame_outputs();
        fixture.run();
        assert!(
            fixture
                .ecs
                .resource::<DecoratorVisualState>()
                .persistent_entities_of(82)
                .is_none()
        );
        assert!(!fixture.ecs.entities().contains(new));
        let maps = fixture.ecs.resource::<DecoratorPhysicsColliders>();
        assert!(maps.first.is_empty() && maps.second.is_empty());
        assert!(
            fixture
                .ecs
                .resource::<DecoratorVisualState>()
                .cloth_infos
                .is_empty()
        );
        assert!(
            fixture
                .ecs
                .resource::<WindowAutoClutterCandidates>()
                .0
                .is_empty()
        );
    }

    #[test]
    fn fullvisual_missing_canonical_outline_fails_after_build_seal_and_colliders() {
        let mut fixture = FullVisualFixture::new(83);
        // A legacy tuple is deliberately present: it is not the original option.
        let mut current = fixture.ecs.resource_mut::<DecoratorBlueprints>();
        let blueprint = current.get_mut(83).unwrap();
        blueprint.transform = None;
        blueprint.outline = Some((1.0, 2.0, 3.0));
        drop(current);
        let failure = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| fixture.run()));
        assert!(failure.is_err());
        let state = fixture.ecs.resource::<DecoratorVisualState>();
        assert_eq!(state.persistent_entities_of(83).unwrap().len(), 1);
        assert!(state.persistent.get(83).unwrap().is_sealed());
        let maps = fixture.ecs.resource::<DecoratorPhysicsColliders>();
        assert_eq!(maps.first.len(), 1);
        assert_eq!(maps.second.len(), 1);
        // The failed system's Commands are not claimed to have been applied.
    }

    #[test]
    fn fullvisual_does_not_eagerly_mark_unvisited_mutable_owners_changed() {
        use bevy_ecs::change_detection::DetectChanges;
        let mut fixture = FullVisualFixture::new(84);
        fixture
            .ecs
            .resource_mut::<DecoratorBlueprints>()
            .map
            .clear();
        fixture.ecs.clear_trackers();
        fixture.run();
        assert!(!fixture.ecs.resource_ref::<WallHoles>().is_changed());
        assert!(
            !fixture
                .ecs
                .resource_ref::<WindowAutoClutterCandidates>()
                .is_changed()
        );
        assert!(
            !fixture
                .ecs
                .resource_ref::<WallAttachedDecoTracker<ChimneyAssemblyParams>>()
                .is_changed()
        );
        assert!(
            !fixture
                .ecs
                .resource_ref::<WallAttachedDecoTracker<DoorStairsAssemblyParams>>()
                .is_changed()
        );
    }

    #[test]
    fn clear_system_keeps_storage_tick_and_marks_blueprints_only_for_affected_ids() {
        use bevy_ecs::{change_detection::DetectChanges, system::RunSystemOnce};
        use country_core::resources::walls::decorator_storage::Decorators;
        let mut ecs = bevy_ecs::world::World::new();
        ecs.insert_resource(DecoratorStorage::default());
        ecs.insert_resource(DecoratorBlueprints::default());
        ecs.clear_trackers();
        ecs.run_system_once(clear_blueprints).unwrap();
        assert!(ecs.resource_ref::<DecoratorStorage>().is_changed());
        assert!(!ecs.resource_ref::<DecoratorBlueprints>().is_changed());

        let mut bucket = Decorators::default();
        bucket.removed_ids.insert(91);
        ecs.resource_mut::<DecoratorStorage>().terrain = Some(bucket);
        ecs.clear_trackers();
        ecs.run_system_once(clear_blueprints).unwrap();
        assert!(
            ecs.resource_ref::<DecoratorBlueprints>().is_changed(),
            "a real attempted remove marks changed even when that ID was absent"
        );

        ecs.resource_mut::<DecoratorStorage>().immutable = true;
        ecs.clear_trackers();
        let failure = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            ecs.run_system_once(clear_blueprints).unwrap();
        }));
        assert!(failure.is_err());
        assert!(ecs.resource_ref::<DecoratorStorage>().is_changed());
    }

    #[test]
    fn fullvisual_reads_actual_wall_delete_events_and_clears_both_tracker_owners() {
        use country_core::resources::walls::WallId;
        let mut fixture = FullVisualFixture::new(85);
        let wall = WallId(501);
        fixture
            .ecs
            .resource_mut::<WallAttachedDecoTracker<ChimneyAssemblyParams>>()
            .add(
                DecoratorStorageAddr::Wall(wall),
                DecoratorId(900),
                &ChimneyAssemblyParams { words: [0; 10] },
            );
        fixture
            .ecs
            .resource_mut::<WallAttachedDecoTracker<DoorStairsAssemblyParams>>()
            .add(
                DecoratorStorageAddr::Wall(wall),
                DecoratorId(900),
                &DoorStairsAssemblyParams { words: [0; 8] },
            );
        fixture
            .ecs
            .resource_mut::<bevy_ecs::event::Events<OnWallDeleted>>()
            .send(OnWallDeleted(wall));
        fixture.run();
        fixture
            .ecs
            .resource_mut::<WallAttachedDecoTracker<ChimneyAssemblyParams>>()
            .with_diffs(|_, _| panic!("removed chimney wall must not retain a dirty batch"));
        fixture
            .ecs
            .resource_mut::<WallAttachedDecoTracker<DoorStairsAssemblyParams>>()
            .with_diffs(|_, _| panic!("removed door-stairs wall must not retain a dirty batch"));
    }
}

#[cfg(test)]
mod native_validation_tests {
    use super::*;
    use crate::{
        audio::op_preview_audio::MutePreviewDecoratorOpSounds,
        backup::DecoratorBackup,
        deferred_decorator_history::{DeferredDecoratorEditHistory, DeferredDecoratorRecord},
        ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher,
    };
    use bevy_ecs::{event::Events, prelude::*, system::RunSystemOnce, world::CommandQueue};
    use country_core::{
        asset_libraries::shader_library::AssetShaderLibrary,
        resources::{
            mouse_terrain_raycast::CursorPositionSS,
            walls::{
                InnerWalls, WallId, WallState,
                cached_decorator_transforms::{
                    CachedDecoratorTransform, CachedDecoratorTransforms,
                },
                decorator::{DecoratorInfo, DecoratorLocator},
                decorator_subtype::{
                    DecoratorCollisionOutline, DecoratorLibKey, DecoratorMeshLibrary,
                    DecoratorSubtype, DerivedDecoratorSubtypes,
                },
                freeform_height_cache::FreeformWallHeightCache,
                public_walls::PublicWalls,
                publication_fields::WallShapeSnapshot,
                publication_source::TerrainHeightGrid512,
            },
        },
        systems::{
            ui::signifiers::signifier_create::{UiSignifierEntry, UiSignifierStream},
            wall::{
                structure_outline::OutlineWallCmd, wall_state::path_preparation::PreparedWallPath,
            },
        },
    };

    fn transform(position: Vec3) -> CachedDecoratorTransform {
        CachedDecoratorTransform {
            rotation: Quat::IDENTITY,
            translation: position,
            scale: Vec3::ONE,
        }
    }

    fn fixture() -> World {
        let mut world = World::new();
        world.init_resource::<DecoratorStorage>();
        world.init_resource::<DecoratorBlueprints>();
        world.init_resource::<DeferredDecoratorEditHistory>();
        world.init_resource::<DerivedDecoratorSubtypes>();
        world.init_resource::<DecoratorMeshLibrary>();
        world.init_resource::<CachedDecoratorTransforms>();
        world.init_resource::<PublicWalls>();
        world.init_resource::<DecoratorBackup>();
        world.insert_resource(DecoratorAffordanceDispatcher {
            state: 6,
            payload: 0,
            ty: 0x38,
            interaction: None,
        });
        world.insert_resource(CursorPositionSS(glam::Vec2::new(17.0, 23.0)));
        world.insert_resource(RaycastWorld::new());
        world.init_resource::<UiSignifierStream>();
        world.init_resource::<Events<OutlineWallCmd>>();
        world.init_resource::<Events<MutePreviewDecoratorOpSounds>>();
        world
    }

    fn stored(world: &mut World, id: u32, addr: DecoratorStorageAddr, info: DecoratorInfo) {
        world
            .resource_mut::<DecoratorStorage>()
            .get_or_insert_default(addr)
            .insert(DecoratorId(id), info);
        world
            .resource_mut::<DecoratorStorage>()
            .decorator_to_storage
            .insert(DecoratorId(id), addr);
    }
    fn affected(world: &mut World, id: u32, addr: DecoratorStorageAddr) {
        let mut deferred = world.resource_mut::<DeferredDecoratorEditHistory>();
        deferred.add(DeferredDecoratorRecord {
            kind: 3,
            f1: u64::from(id),
            ..Default::default()
        });
        deferred.add_potentially_surviving_affected_decorator(DecoratorLocator::new(addr, id));
    }
    fn terrain_info() -> DecoratorInfo {
        DecoratorInfo {
            u20: 3 << 16,
            ty: 0x3e,
            rank: 1,
            ..Default::default()
        }
    }
    fn add_derived(world: &mut World, id: u32, subtype: u8, position: Vec3) {
        world
            .resource_mut::<DerivedDecoratorSubtypes>()
            .insert(id, DecoratorSubtype(subtype));
        world
            .resource_mut::<CachedDecoratorTransforms>()
            .insert(id, transform(position));
    }
    fn run(world: &mut World) {
        world.run_system_once(validate_blueprints).unwrap();
    }

    #[test]
    fn no_records_still_clears_tracking_without_erasing_authority_or_requesting_restore() {
        let mut world = fixture();
        stored(&mut world, 7, DecoratorStorageAddr::Terrain, terrain_info());
        world
            .resource_mut::<DecoratorStorage>()
            .get_or_insert_default(DecoratorStorageAddr::Terrain)
            .removed_ids
            .insert(99);
        world.resource_mut::<DecoratorBackup>().created = true;
        world.resource_mut::<DecoratorBackup>().state = 1;
        world
            .resource_mut::<DeferredDecoratorEditHistory>()
            .add_potentially_surviving_affected_decorator(DecoratorLocator::new(
                DecoratorStorageAddr::Terrain,
                7,
            ));
        run(&mut world);
        let storage = world.resource::<DecoratorStorage>();
        let bucket = storage.get(DecoratorStorageAddr::Terrain).unwrap();
        assert!(bucket.get(DecoratorId(7)).is_some());
        assert!(bucket.modified_ids.is_empty() && bucket.removed_ids.is_empty());
        assert_eq!(
            storage.lookup_storage(DecoratorId(7)),
            Some(DecoratorStorageAddr::Terrain)
        );
        assert_eq!(world.resource::<DecoratorBackup>().state, 1);
        assert_eq!(
            world
                .resource::<Events<MutePreviewDecoratorOpSounds>>()
                .len(),
            0
        );
    }

    #[test]
    fn distinct_locators_on_one_wall_and_records_gate_preserve_actual_history() {
        let mut world = fixture();
        let addr = DecoratorStorageAddr::Wall(WallId(42));
        // 一个已删除、一个仍在而缺 blueprint；不能用同墙第一个记录代替实际id。
        affected(&mut world, 7, addr);
        affected(&mut world, 8, addr);
        stored(&mut world, 8, addr, terrain_info());
        world.resource_mut::<DecoratorBackup>().created = true;
        world.resource_mut::<DecoratorAffordanceDispatcher>().state = 0;
        run(&mut world);
        assert_eq!(world.resource::<DecoratorBackup>().state, 2);
        assert_eq!(
            world
                .resource::<Events<MutePreviewDecoratorOpSounds>>()
                .len(),
            1
        );
        assert_eq!(world.resource::<UiSignifierStream>().committed_len(), 1);
        let deferred = world.resource::<DeferredDecoratorEditHistory>();
        assert_eq!(deferred.records.len(), 2);
        assert_eq!(deferred.potentially_surviving_affected_decorators.len(), 2);
        assert!(
            !deferred.pending,
            "原门不依赖pending，也不在验证中改history"
        );
    }

    #[test]
    fn missing_blueprint_diagnostics_do_not_manufacture_backup() {
        let mut world = fixture();
        stored(&mut world, 7, DecoratorStorageAddr::Terrain, terrain_info());
        affected(&mut world, 7, DecoratorStorageAddr::Terrain);
        world.resource_mut::<DecoratorAffordanceDispatcher>().state = 0;
        run(&mut world);
        assert!(!world.resource::<DecoratorBackup>().created);
        assert_eq!(world.resource::<DecoratorBackup>().state, 0);
        assert_eq!(
            world
                .resource::<Events<MutePreviewDecoratorOpSounds>>()
                .len(),
            0
        );
        assert_eq!(world.resource::<UiSignifierStream>().committed_len(), 0);
    }

    #[test]
    fn terrain_valid_still_requires_actual_subtype_and_cache_before_destination_skip() {
        let mut world = fixture();
        stored(&mut world, 7, DecoratorStorageAddr::Terrain, terrain_info());
        affected(&mut world, 7, DecoratorStorageAddr::Terrain);
        world
            .resource_mut::<DecoratorBlueprints>()
            .insert_for(7, DecoratorStorageAddr::Terrain, 0);
        add_derived(&mut world, 7, 25, Vec3::new(3.0, 4.0, 5.0));
        world.resource_mut::<DecoratorBackup>().created = true;
        world.resource_mut::<DecoratorBackup>().state = 1;
        run(&mut world);
        assert_eq!(world.resource::<DecoratorBackup>().state, 1);
        assert_eq!(
            world
                .resource::<Events<MutePreviewDecoratorOpSounds>>()
                .len(),
            0
        );
        assert_eq!(world.resource::<UiSignifierStream>().committed_len(), 0);
        *world.resource_mut::<CachedDecoratorTransforms>() = CachedDecoratorTransforms::default();
        assert!(
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| run(&mut world))).is_err()
        );
    }

    #[test]
    fn locator_uses_its_actual_address_instead_of_a_different_d2s_address() {
        let mut world = fixture();
        let addr = DecoratorStorageAddr::Wall(WallId(42));
        stored(&mut world, 7, addr, terrain_info());
        world
            .resource_mut::<DecoratorStorage>()
            .decorator_to_storage
            .insert(DecoratorId(7), DecoratorStorageAddr::Terrain);
        affected(&mut world, 7, addr);
        world.resource_mut::<DecoratorBackup>().created = true;
        run(&mut world);
        assert_eq!(
            world.resource::<DecoratorBackup>().state,
            2,
            "原显式地址桶仍有对象，缺blueprint拒绝"
        );
    }

    fn wall_fixture(looped: bool) -> PublicWalls {
        use country_core::resources::roofs::geometry::surface::Curve2;
        let points = if looped {
            vec![[0.0, 0.0], [8.0, 0.0], [8.0, 8.0], [0.0, 8.0], [0.0, 0.0]]
        } else {
            vec![[0.0, 0.0], [8.0, 0.0]]
        };
        let curve = Curve2::try_new(points, looped).unwrap();
        let mut snapshot = WallState::new(curve, 10.0, 0, 0, false).snapshot().clone();
        let mut bytes = [0; 28];
        bytes[..4].copy_from_slice(&(if looped { 0_u32 } else { 2_u32 }).to_le_bytes());
        for (index, value) in [1.0_f32, 0.0, 0.0, 0.0, 8.0, 8.0].into_iter().enumerate() {
            bytes[4 + index * 4..8 + index * 4].copy_from_slice(&value.to_le_bytes());
        }
        snapshot.source.shape = WallShapeSnapshot::from_observed(bytes);
        snapshot.source.curve_kind_tag = if looped { 0 } else { 2 };
        let mut inner = InnerWalls::new();
        inner.new_wall_with_id(WallId(42), WallState::from_snapshot(snapshot, 0));
        let mut walls = PublicWalls::default();
        walls.consume(
            &mut inner,
            &TerrainHeightGrid512 {
                encoded: vec![0; 512 * 512].into(),
                minimum_y: 0.0,
                maximum_y: 0.0,
                sampling_fix: false,
            },
        );
        let wall = walls.get_mut(WallId(42)).unwrap();
        wall.prepared_path = Some(PreparedWallPath {
            positions: vec![
                [0.0, 1.0, 0.0],
                [1.0, 2.0, 0.0],
                [2.0, 3.0, 0.0],
                [3.0, 4.0, 0.0],
                [4.0, 5.0, 0.0],
                [5.0, 6.0, 0.0],
            ],
            points_u: vec![0.0, 0.2, 0.4, 0.6, 0.8, 1.0],
            length: 5.0,
            segment_values: vec![0.0; 6],
        });
        wall.segment_heights = vec![10.0, 20.0, 30.0];
        wall.freeform_heights = Some(FreeformWallHeightCache {
            maximum_y: 15.0,
            arch_segment_heights: vec![9.0, 10.0, 11.0, 12.0, 13.0, 14.0],
            segment_heights: vec![0.0; 6],
        });
        walls
    }

    fn emit(world: &mut World, reason: crate::cull_oob_decorators::DecoratorLocationInvalidReason) {
        let transform = CachedDecoratorTransform {
            rotation: Quat::from_rotation_y(0.25),
            translation: Vec3::new(11.0, 7.0, 13.0),
            scale: Vec3::ONE,
        };
        world
            .run_system_once(
                move |walls: Res<PublicWalls>,
                      mut stream: ResMut<UiSignifierStream>,
                      mut outline: EventWriter<OutlineWallCmd>| {
                    show_invalid_location_feedback(
                        DecoratorId(7),
                        reason,
                        &transform,
                        &walls,
                        &mut stream,
                        &mut outline,
                    );
                },
            )
            .unwrap();
    }

    fn draw_entries(entries: Vec<UiSignifierEntry>) -> (World, AssetMeshLibrary, Vec<Entity>) {
        use country_core::render::mesh::{ATTR_POSITION, Mesh};
        let meshes = AssetMeshLibrary::new();
        let mut scribble = Mesh::new("显式scribble测试三角形");
        scribble.set_attribute(ATTR_POSITION, vec![Vec3::ZERO, Vec3::X, Vec3::Y]);
        scribble.set_indices(vec![0, 1, 2]);
        meshes.add(country_core::asset_libraries::mesh_library::MeshAsset::new(
            scribble,
            Some("decorator_delete_scribble".into()),
        ));
        let shaders = AssetShaderLibrary::new();
        for name in ["vertex_color_shader", "dashed_line"] {
            shaders.add_raster(
                country_core::render::shader::RasterShaderProgram::new_draw(
                    name,
                    Default::default(),
                    &country_core::render::test_device::TestDevice(1),
                ),
                false,
            );
        }
        let mut world = World::new();
        let mut queue = CommandQueue::default();
        let mut entities = Vec::new();
        for entry in entries {
            let UiSignifierEntry::Factory(factory) = entry else {
                panic!("原反馈必须是真工厂")
            };
            entities.extend(factory.wake(
                &meshes,
                &shaders,
                &mut Commands::new(&mut queue, &world),
            ));
        }
        queue.apply(&mut world);
        (world, meshes, entities)
    }
    fn positions(world: &World, meshes: &AssetMeshLibrary, entity: Entity) -> Vec<Vec3> {
        meshes
            .get(
                *world
                    .get::<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>>(
                        entity,
                    )
                    .unwrap(),
            )
            .unwrap()
            .positions_without_indices()
            .to_vec()
    }

    #[test]
    fn cutout_and_turn_feedback_keep_required_sample_bounds_and_distinct_y_sources() {
        use crate::cull_oob_decorators::DecoratorLocationInvalidReason as R;
        let coord = utils::curve::CurveInternalCoord {
            segment: 2,
            t: 0.75,
        };
        // 原8e71..8e7d还检查两张表的相邻采样；高度表短于path不能补默认值。
        let mut mismatched = fixture();
        mismatched.insert_resource(wall_fixture(false));
        assert!(
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                emit(
                    &mut mismatched,
                    R::WallCutoutBoundary {
                        wall: WallId(42),
                        coord,
                    },
                );
            }))
            .is_err()
        );
        let mut world = fixture();
        let mut walls = wall_fixture(false);
        walls.get_mut(WallId(42)).unwrap().segment_heights =
            vec![10.0, 20.0, 30.0, 40.0, 50.0, 60.0];
        world.insert_resource(walls);
        emit(
            &mut world,
            R::WallCutoutBoundary {
                wall: WallId(42),
                coord,
            },
        );
        emit(
            &mut world,
            R::SharpWallTurn {
                wall: WallId(42),
                coord,
            },
        );
        let entries = world.resource_mut::<UiSignifierStream>().take_entries();
        let (draw, meshes, entities) = draw_entries(entries);
        assert_eq!(
            positions(&draw, &meshes, entities[0]),
            vec![
                Vec3::new(0.0, 11.0, 0.0),
                Vec3::new(1.0, 22.0, 0.0),
                Vec3::new(2.0, 33.0, 0.0),
                Vec3::new(3.0, 44.0, 0.0),
                Vec3::new(4.0, 55.0, 0.0),
                Vec3::new(5.0, 66.0, 0.0),
            ]
        );
        assert_eq!(
            positions(&draw, &meshes, entities[1]),
            (0..6)
                .map(|i| Vec3::new(i as f32, 7.0, 0.0))
                .collect::<Vec<_>>()
        );
        for entity in entities {
            assert_eq!(
                *draw
                    .get::<country_core::render::pass_builder::draw_list::DrawTopology>(entity)
                    .unwrap(),
                country_core::render::pass_builder::draw_list::DrawTopology::Alt(3.0)
            );
        }
    }

    #[test]
    fn roof_feedback_preserves_zero_direction_and_writes_real_roof_outline_payload() {
        use crate::cull_oob_decorators::DecoratorLocationInvalidReason as R;
        let mut world = fixture();
        let position = Vec3::new(3.0, 4.0, 5.0);
        emit(
            &mut world,
            R::RoofBounds {
                position,
                direction: Vec3::ZERO,
                wall: WallId(42),
            },
        );
        let outlines = world.resource::<Events<OutlineWallCmd>>();
        let mut cursor = outlines.get_cursor();
        let event = cursor.read(outlines).next().unwrap();
        assert_eq!(
            (event.wall, event.strength, event.kind),
            (WallId(42), 2.0, 0)
        );
        let entries = world.resource_mut::<UiSignifierStream>().take_entries();
        let (draw, meshes, entities) = draw_entries(entries);
        assert_eq!(
            positions(&draw, &meshes, entities[0]),
            vec![position, position]
        );
        assert_eq!(
            draw.get::<country_core::components::DashedLineLength>(entities[0])
                .unwrap()
                .0,
            0.0
        );
    }

    #[test]
    fn wall_end_and_lower_feedback_use_original_outline_instead_of_prepared_points() {
        use crate::cull_oob_decorators::DecoratorLocationInvalidReason as R;
        let mut world = fixture();
        world.insert_resource(wall_fixture(false));
        let original: Vec<_> = world
            .resource::<PublicWalls>()
            .get(WallId(42))
            .unwrap()
            .geometry
            .outline
            .iter()
            .copied()
            .map(Vec3::from_array)
            .collect();
        emit(
            &mut world,
            R::WallEnds {
                left_oob: true,
                right_oob: true,
                wall: WallId(42),
            },
        );
        emit(&mut world, R::WallLowerBoundary { wall: WallId(42) });
        let entries = world.resource_mut::<UiSignifierStream>().take_entries();
        let (draw, meshes, entities) = draw_entries(entries);
        assert_eq!(
            positions(&draw, &meshes, entities[0]),
            vec![original[0], Vec3::new(original[0].x, 9.0, original[0].z)]
        );
        let last = *original.last().unwrap();
        assert_eq!(
            positions(&draw, &meshes, entities[1]),
            vec![last, Vec3::new(last.x, 14.0, last.z)]
        );
        assert_eq!(positions(&draw, &meshes, entities[2]), original);
    }

    #[test]
    fn maximum_height_feedback_uses_zip_and_original_shape_closure_gate() {
        for looped in [false, true] {
            let mut world = fixture();
            let mut walls = wall_fixture(looped);
            walls
                .get_mut(WallId(42))
                .unwrap()
                .freeform_heights
                .as_mut()
                .unwrap()
                .arch_segment_heights
                .truncate(2);
            world.insert_resource(walls);
            world
                .run_system_once(
                    |walls: Res<PublicWalls>, mut stream: ResMut<UiSignifierStream>| {
                        show_max_wall_y(walls.get(WallId(42)).unwrap(), 0.5, &mut stream);
                    },
                )
                .unwrap();
            let entries = world.resource_mut::<UiSignifierStream>().take_entries();
            let (draw, meshes, entities) = draw_entries(entries);
            let mut expected = vec![Vec3::new(0.0, 8.5, 0.0), Vec3::new(1.0, 9.5, 0.0)];
            if looped {
                expected.push(expected[0]);
            }
            assert_eq!(positions(&draw, &meshes, entities[0]), expected);
        }
    }

    #[test]
    fn corner_feedback_commits_real_line_then_owned_scribble_at_cached_pose() {
        use crate::cull_oob_decorators::DecoratorLocationInvalidReason as R;
        let mut world = fixture();
        world.insert_resource(wall_fixture(true));
        let rectangle = utils::geometry::Rectangle2d::new_axis_aligned(
            glam::Vec2::new(4.0, 4.0),
            glam::Vec2::splat(8.0),
        );
        let point = rectangle.get_point(0);
        emit(
            &mut world,
            R::WallCorner {
                wall: WallId(42),
                rectangle,
                corner: 0,
            },
        );
        let entries = world.resource_mut::<UiSignifierStream>().take_entries();
        let UiSignifierEntry::Factory(scribble) = &entries[1] else {
            panic!("scribble factory expected")
        };
        assert_eq!(
            scribble.name(),
            Some("despawn_decorator_DecoratorId(7)_Vec3(11.0, 7.0, 13.0)")
        );
        let (draw, meshes, entities) = draw_entries(entries);
        let line = positions(&draw, &meshes, entities[0]);
        assert_eq!((line[0].x, line[0].z), (point.x, point.y));
        assert_eq!(
            line[1].y,
            world
                .resource::<PublicWalls>()
                .get(WallId(42))
                .unwrap()
                .max_y()
        );
        let transform = draw
            .get::<country_core::utils::components::transform::Transform>(entities[1])
            .unwrap();
        assert_eq!(transform.translation, Vec3::new(11.0, 7.0, 13.0));
        assert_eq!(transform.rotation, Quat::from_rotation_y(0.25));
        assert_eq!(transform.scale, Vec3::ZERO);
        let animation = draw
            .get::<country_core::systems::ui::signifiers::animation::SignifierAnimation>(
                entities[1],
            )
            .unwrap();
        assert_eq!((animation.value, animation.duration), (0.5, 0.0));
    }

    #[test]
    fn reason_tags_with_no_extra_payload_do_not_invent_an_additional_diagnostic() {
        use crate::cull_oob_decorators::DecoratorLocationInvalidReason as R;
        let mut world = fixture();
        for reason in [R::FlatRoofForbidden, R::GableBandForbidden, R::MissingRoof] {
            emit(&mut world, reason);
        }
        assert_eq!(world.resource::<UiSignifierStream>().committed_len(), 0);
        assert_eq!(world.resource::<Events<OutlineWallCmd>>().len(), 0);
    }

    #[test]
    fn tuple_overlap_adapter_keeps_external_times_local_pose_and_same_shape_arc() {
        let data = Arc::new(RaycastMeshData::new_cuboid_at(Vec3::ONE, Vec3::X));
        let mesh = PositionedDecoratorMesh {
            mesh: Arc::clone(&data),
            position: Vec3::new(5.0, 6.0, 7.0),
            rotation: Quat::from_rotation_y(std::f32::consts::FRAC_PI_2),
        };
        let got = validation_overlap_shape(&mesh);
        let expected = mesh.position + mesh.rotation * Vec3::X;
        assert_eq!(got.position, expected);
        assert_eq!(got.rotation, mesh.rotation);
        assert!(Arc::ptr_eq(&got.shape, &data.shape.as_ref().unwrap().0));
    }

    fn overlap_fixture(other_type: u8) -> World {
        let mut world = fixture();
        let addr = DecoratorStorageAddr::Wall(WallId(42));
        stored(&mut world, 7, addr, terrain_info());
        stored(
            &mut world,
            99,
            addr,
            DecoratorInfo {
                ty: other_type,
                rank: 1,
                ..terrain_info()
            },
        );
        add_derived(&mut world, 7, 25, Vec3::new(1.0, 2.0, 3.0));
        let mesh = Arc::new(RaycastMeshData::new_cuboid_at(Vec3::splat(2.0), Vec3::ZERO));
        world
            .resource_mut::<DecoratorBlueprints>()
            .insert_for(7, addr, 0)
            .collision_mesh = BlueprintMeshSlot::Mesh(PositionedDecoratorMesh {
            mesh: Arc::clone(&mesh),
            position: Vec3::ZERO,
            rotation: Quat::IDENTITY,
        });
        let mut map = std::collections::HashMap::new();
        upsert_collider(
            &mut world.resource_mut::<RaycastWorld>(),
            &mut map,
            DecoratorId(99),
            RaycastGeoSource::decorator(DecoratorId(99), addr),
            mesh,
            Vec3::ZERO,
            Quat::IDENTITY,
        );
        world.resource_mut::<RaycastWorld>().flush_colliders();
        world
    }

    fn check_overlap(world: &mut World) -> (bool, bool) {
        world.run_system_once(
            |blueprints: Res<DecoratorBlueprints>, storage: Res<DecoratorStorage>, walls: Res<PublicWalls>,
             roofs: Query<&country_core::resources::roofs::roof_shape::Roof>,
             derived: country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,
             cache: Res<CachedDecoratorTransforms>, raycast: Res<RaycastWorld>,
             mut stream: ResMut<UiSignifierStream>, mut outlines: EventWriter<OutlineWallCmd>| {
                let mut specific = false;
                let accepted = validate_affected_decorator(&DecoratorLocator::new(DecoratorStorageAddr::Wall(WallId(42)), 7),
                    &blueprints, &storage, &walls, &roofs, derived.as_view(), &cache, &raycast,
                    &mut stream, &mut outlines, &mut specific);
                (accepted, specific)
            }).unwrap()
    }

    #[test]
    fn real_overlap_type_gate_continues_with_specific_flag_before_optional_outline_lookups() {
        // 原898c..89a1把62及范围外命中送回8600；原已写specific不能回滚。
        // 故意不装阻挡物subtype/cache/outline，证明该门发生在它们之前。
        for ty in [0, 55, 62, 64, 255] {
            let mut world = overlap_fixture(ty);
            assert_eq!(check_overlap(&mut world), (true, true), "type={ty}");
            assert_eq!(world.resource::<UiSignifierStream>().committed_len(), 0);
        }
    }

    #[test]
    fn real_overlap_corner_override_displays_blocker_then_intended_and_rejects() {
        let mut world = overlap_fixture(56);
        add_derived(&mut world, 99, 3, Vec3::new(10.0, 20.0, 30.0));
        let addr = DecoratorStorageAddr::Wall(WallId(42));
        world
            .resource_mut::<DecoratorBlueprints>()
            .insert_for(99, addr, 0)
            .transform = Some(CachedDecoratorTransform {
            translation: Vec3::new(50.0, 60.0, 70.0),
            rotation: Quat::IDENTITY,
            scale: Vec3::splat(2.0),
        });
        let mut library = world.resource_mut::<DecoratorMeshLibrary>();
        library.collision_outline.insert(
            DecoratorLibKey::new(3, 1),
            DecoratorCollisionOutline::Lines(vec![Vec3::ZERO, Vec3::X]),
        );
        library.collision_outline.insert(
            DecoratorLibKey::new(25, 1),
            DecoratorCollisionOutline::Lines(vec![Vec3::ZERO, Vec3::Y]),
        );
        drop(library);
        assert_eq!(check_overlap(&mut world), (false, true));
        let entries = world.resource_mut::<UiSignifierStream>().take_entries();
        assert_eq!(entries.len(), 2);
        let (draw, meshes, entities) = draw_entries(entries);
        assert_eq!(
            positions(&draw, &meshes, entities[0]),
            vec![Vec3::new(50.0, 60.0, 70.0), Vec3::new(52.0, 60.0, 70.0)]
        );
        assert_eq!(
            positions(&draw, &meshes, entities[1]),
            vec![Vec3::new(1.0, 2.0, 3.0), Vec3::new(1.0, 3.0, 3.0)]
        );
    }
}
