//! UNVERIFIED integration draft: actual ConstructGateCmd -> CPU gate geometry,
//! source metadata and Bevy-owned entities. GPU asset/pool transport is pending.
//! Original container implementation is reused through std/Bevy, never copied.
use crate::{
    BuildingStage, CreateRoof, DeleteRoof, ObjectTransform, PublicWalls, RemoveWall,
    RoofGeometryChanged, WallCollectionState, WallSpatialDirty,
    nani::{
        NaniInstanceDesc, NaniInstanceMeshDesc, NaniSolidVertexColorBundle,
        SolidVertexColorInstanceDataMarker,
    },
    wall_arches::{ConstructGateCmd, WallArchesBuildState},
    wall_paths::WallPreparedPaths,
};
use bevy_ecs::{prelude::*, schedule::ApplyDeferred, system::SystemParam};
use country_core::recovered_building::{
    building_gates::{
        GateConstructionInput, GateCpuOutput, GateMeshProjection, GateNaniProjection,
        construct_gate_cpu,
    },
    building_wall_constructor::WallBrickInstance,
    building_wall_state::WallId,
};
use std::collections::{HashMap, HashSet};

/// Local CPU asset identity only. This never represents a GPU/renderer handle.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GateCpuMeshHandle(pub u64);
#[derive(Resource, Default)]
pub struct GateCpuMeshAssets {
    next_identity: u64,
    pub meshes: HashMap<GateCpuMeshHandle, GateMeshProjection>,
    owners: HashMap<Entity, GateCpuMeshHandle>,
}
impl GateCpuMeshAssets {
    fn insert(&mut self, mesh: GateMeshProjection) -> GateCpuMeshHandle {
        self.next_identity = self
            .next_identity
            .checked_add(1)
            .expect("CPU mesh identity exhausted");
        let handle = GateCpuMeshHandle(self.next_identity);
        self.meshes.insert(handle, mesh);
        handle
    }
    fn withdraw_entity(&mut self, entity: Entity) {
        if let Some(handle) = self.owners.remove(&entity) {
            self.meshes.remove(&handle);
        }
    }
}
#[derive(Component, Clone, Debug)]
pub struct WoodenGate {
    pub wall: WallId,
    pub ordinal: u64,
    pub top_positions: Vec<[f32; 3]>,
    pub endpoint_directions: [[f32; 2]; 2],
    pub seed: u32,
    pub observed_source_metadata_69: u8,
}
#[derive(Component, Clone, Copy, Debug)]
pub struct GateCpuDrawable {
    pub mesh: GateCpuMeshHandle,
    /// Actual original shader choice. No GPU shader handle is synthesized.
    pub shader_key: &'static str,
}
#[derive(Component, Clone, Copy, Debug)]
pub struct GateTransientMesh {
    pub cpu_mesh: GateCpuMeshHandle,
}
#[derive(Component, Clone, Debug)]
pub struct GateCpuSupportBricks {
    pub wall: WallId,
    pub ordinal: u64,
    pub instances: Vec<WallBrickInstance>,
}
#[derive(Component, Clone, Debug)]
pub struct GateNaniSource {
    pub wall: WallId,
    pub ordinal: u64,
    pub projection: GateNaniProjection,
}
/// Native Gates contains door/post entities, not the separate SSBO brick slots.
#[derive(Resource, Default)]
pub struct Gates(pub HashMap<WallId, Vec<Entity>>);
/// CPU support entity ownership stands in front of the explicit SSBO boundary.
#[derive(Resource, Default)]
pub struct GateSupportEntities(pub HashMap<WallId, Vec<Entity>>);
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PendingGateReason {
    MissingRawAuthority,
    MissingPublicWall,
    MissingPreparedPath,
    MissingTerrain,
    MissingSourceMetadata,
    OriginalCpuBranch,
}
#[derive(Resource, Default)]
pub struct GateBuildState {
    fingerprints: HashMap<WallId, Vec<u8>>,
    pub pending: HashMap<WallId, PendingGateReason>,
    pub rebuilt: Vec<WallId>,
    /// CPU transport reaches these records; renderer handles/uploads are absent.
    pub cpu_outputs: HashMap<WallId, Vec<GateCpuOutput>>,
}
#[derive(SystemParam)]
pub struct GateInvalidationReaders<'w, 's> {
    changed: EventReader<'w, 's, WallSpatialDirty>,
    created: EventReader<'w, 's, CreateRoof>,
    geometry: EventReader<'w, 's, RoofGeometryChanged>,
    removed: EventReader<'w, 's, RemoveWall>,
    deleted_roofs: EventReader<'w, 's, DeleteRoof>,
}

/// Parent registers the Gates stage after WallArches, before generic Nani staging.
pub fn register(app: &mut bevy_app::App) {
    app.init_resource::<Gates>()
        .init_resource::<GateSupportEntities>()
        .init_resource::<GateCpuMeshAssets>()
        .init_resource::<GateBuildState>()
        .add_systems(
            bevy_app::Update,
            (consume, ApplyDeferred, clean_removed_meshes)
                .chain()
                .in_set(BuildingStage::Gates),
        );
}
fn withdraw(
    wall: WallId,
    remove_entry: bool,
    commands: &mut Commands,
    gates: &mut Gates,
    support: &mut GateSupportEntities,
    assets: &mut GateCpuMeshAssets,
    state: &mut GateBuildState,
) {
    let entities = if remove_entry {
        gates.0.remove(&wall).unwrap_or_default()
    } else {
        gates.0.entry(wall).or_default().drain(..).collect()
    };
    for entity in entities {
        assets.withdraw_entity(entity);
        commands.entity(entity).despawn();
    }
    for entity in support.0.remove(&wall).unwrap_or_default() {
        commands.entity(entity).despawn();
    }
    state.fingerprints.remove(&wall);
    state.cpu_outputs.remove(&wall);
    if remove_entry {
        state.pending.remove(&wall);
    }
}
fn fingerprint(host: &crate::PublishedWall, prepared_generation: u64) -> Vec<u8> {
    let mut bytes = host.observed_projection().to_vec();
    bytes.extend(host.publication_revision.to_le_bytes());
    bytes.extend(prepared_generation.to_le_bytes());
    bytes.push(u8::from(host.observed_source_metadata_complete));
    bytes
}
fn consume(
    mut commands: Commands,
    mut events: EventReader<ConstructGateCmd>,
    mut invalidations: GateInvalidationReaders,
    walls: Res<PublicWalls>,
    source: Res<WallCollectionState>,
    paths: Res<WallPreparedPaths>,
    arches: Res<WallArchesBuildState>,
    mut gates: ResMut<Gates>,
    mut support: ResMut<GateSupportEntities>,
    mut assets: ResMut<GateCpuMeshAssets>,
    mut state: ResMut<GateBuildState>,
    mut report: ResMut<crate::BuildingReport>,
) {
    state.rebuilt.clear();
    let mut rebuild: HashSet<_> = invalidations.changed.read().map(|e| e.0).collect();
    rebuild.extend(invalidations.created.read().map(|e| e.0.wall_id()));
    rebuild.extend(invalidations.geometry.read().map(|e| e.wall));
    rebuild.extend(arches.rebuilt.iter().copied());
    rebuild.extend(arches.pending.iter().copied());
    let mut remove: HashSet<_> = invalidations.removed.read().map(|e| e.0).collect();
    // DeleteRoof removes the Gates entry even while the wall itself remains.
    remove.extend(invalidations.deleted_roofs.read().map(|e| e.0));
    remove.extend(
        gates
            .0
            .keys()
            .filter(|id| !source.raw_sources.contains_key(id) || !walls.walls.contains_key(id))
            .copied(),
    );
    let mut refresh: HashSet<_> = HashSet::new();
    for (&wall, previous) in &state.fingerprints {
        let current = walls.walls.get(&wall).and_then(|host| {
            paths
                .current(wall, host.publication_revision)
                .map(|path| fingerprint(host, path.generation))
        });
        if current.as_ref() != Some(previous) {
            refresh.insert(wall);
        }
    }
    rebuild.extend(refresh);
    let requests: Vec<_> = events.read().cloned().collect();
    // A new command batch rebuilds one wall once, then appends every ordinal.
    rebuild.extend(requests.iter().map(|e| e.0.wall_id));
    let mut ids: Vec<_> = remove.iter().copied().collect();
    ids.sort_unstable_by_key(|w| w.0);
    for wall in ids {
        withdraw(
            wall,
            true,
            &mut commands,
            &mut gates,
            &mut support,
            &mut assets,
            &mut state,
        );
    }
    let mut ids: Vec<_> = rebuild
        .into_iter()
        .filter(|wall| !remove.contains(wall))
        .collect();
    ids.sort_unstable_by_key(|w| w.0);
    for wall in ids {
        withdraw(
            wall,
            false,
            &mut commands,
            &mut gates,
            &mut support,
            &mut assets,
            &mut state,
        );
    }
    let mut failed_walls = HashSet::new();
    for request in requests {
        if failed_walls.contains(&request.0.wall_id) {
            continue;
        }
        let wall = request.0.wall_id;
        // A deleted host's queued commands must never resurrect its ownership.
        if !source.raw_sources.contains_key(&wall) {
            state
                .pending
                .insert(wall, PendingGateReason::MissingRawAuthority);
            continue;
        }
        let Some(host) = walls.walls.get(&wall) else {
            state
                .pending
                .insert(wall, PendingGateReason::MissingPublicWall);
            continue;
        };
        if !host.observed_source_metadata_complete {
            state
                .pending
                .insert(wall, PendingGateReason::MissingSourceMetadata);
            continue;
        }
        let Some(path) = paths.current(wall, host.publication_revision) else {
            state
                .pending
                .insert(wall, PendingGateReason::MissingPreparedPath);
            continue;
        };
        let Some(terrain) = source.terrain.as_ref() else {
            state
                .pending
                .insert(wall, PendingGateReason::MissingTerrain);
            continue;
        };
        let projection = host.observed_projection();
        let input = GateConstructionInput {
            segment: &request.0.segment,
            prepared_positions: &path.path.positions,
            prepared_u: &path.path.points_u,
            observed_public_288: f32::from_le_bytes(projection[0x288..0x28c].try_into().unwrap()),
            observed_public_29a: projection[0x29a],
            wall_id: wall,
            ordinal: request.0.ordinal,
        };
        let output = match construct_gate_cpu(input, terrain) {
            Ok(value) => value,
            Err(error) => {
                withdraw(
                    wall,
                    false,
                    &mut commands,
                    &mut gates,
                    &mut support,
                    &mut assets,
                    &mut state,
                );
                state
                    .pending
                    .insert(wall, PendingGateReason::OriginalCpuBranch);
                failed_walls.insert(wall);
                state.rebuilt.retain(|id| *id != wall);
                report
                    .faults
                    .push(format!("Gate CPU {}: {error:?}", wall.0));
                continue;
            }
        };
        if !output.bricks.is_empty() {
            let entity = commands
                .spawn(GateCpuSupportBricks {
                    wall,
                    ordinal: request.0.ordinal,
                    instances: output.bricks.clone(),
                })
                .id();
            support.0.entry(wall).or_default().push(entity);
        }
        for leaf in &output.leaves {
            let handle = assets.insert(leaf.mesh.clone());
            let entity = commands
                .spawn((
                    WoodenGate {
                        wall,
                        ordinal: request.0.ordinal,
                        top_positions: leaf.top_positions.clone(),
                        endpoint_directions: leaf.endpoint_directions,
                        seed: leaf.seed,
                        observed_source_metadata_69: leaf.observed_public_29a,
                    },
                    GateCpuDrawable {
                        mesh: handle,
                        shader_key: "solid_vertex_color_wooden_gate",
                    },
                    GateTransientMesh { cpu_mesh: handle },
                    ObjectTransform::default(),
                ))
                .id();
            assets.owners.insert(entity, handle);
            gates.0.entry(wall).or_default().push(entity);
        }
        for projection in &output.nanis {
            let entity = commands
                .spawn((
                    NaniSolidVertexColorBundle {
                        descriptor: NaniInstanceDesc {
                            mesh: NaniInstanceMeshDesc::Named(projection.definition_id),
                            subset_kind: projection.subset_kind,
                        },
                        transform: nani_transform(projection),
                        marker: SolidVertexColorInstanceDataMarker,
                    },
                    GateNaniSource {
                        wall,
                        ordinal: request.0.ordinal,
                        projection: projection.clone(),
                    },
                ))
                .id();
            gates.0.entry(wall).or_default().push(entity);
        }
        state.cpu_outputs.entry(wall).or_default().push(output);
        state
            .fingerprints
            .insert(wall, fingerprint(host, path.generation));
        state.pending.remove(&wall);
        if !state.rebuilt.contains(&wall) {
            state.rebuilt.push(wall);
        }
    }
}
fn nani_transform(p: &GateNaniProjection) -> ObjectTransform {
    let mut bytes = [0; 48];
    // The original ObjectTransform48 has 40 meaningful bytes and padding.
    for (i, f) in p
        .rotation
        .into_iter()
        .chain(p.position)
        .chain(p.scale)
        .enumerate()
    {
        bytes[i * 4..i * 4 + 4].copy_from_slice(&f.to_le_bytes());
    }
    ObjectTransform(bytes)
}
fn clean_removed_meshes(
    mut removed: RemovedComponents<GateTransientMesh>,
    mut assets: ResMut<GateCpuMeshAssets>,
) {
    for entity in removed.read() {
        assets.withdraw_entity(entity);
    }
}
