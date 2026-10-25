//! Gate command batches produce CPU geometry and Bevy-owned entities.
//! CPU mesh ownership is tracked separately from renderer asset/upload handles.
use crate::{
    BuildingStage, DeleteRoof, ObjectTransform, PublicWalls, RemoveWall, WallCollectionState,
    nani::{
        NaniInstanceDesc, NaniInstanceMeshDesc, NaniSolidVertexColorBundle,
        SolidVertexColorInstanceDataMarker,
    },
    wall_arches::{ConstructGateCmd, WallArchesBuildState},
    wall_paths::WallPreparedPaths,
};
use bevy_ecs::{prelude::*, schedule::ApplyDeferred, system::SystemParam};
use country_core::systems::wall::construction::gates::GateConstructionInput;
use country_core::systems::wall::construction::gates::GateCpuOutput;
use country_core::systems::wall::construction::gates::GateMeshProjection;
use country_core::systems::wall::construction::gates::GateNaniProjection;
use country_core::systems::wall::construction::gates::construct_gate_cpu;
use country_core::systems::wall::construction::bricks::WallBrickInstance;
use country_core::resources::walls::publication_fields::WallId;
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

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
    /// Shader identifier consumed by the renderer.
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
/// Door and post entities owned by each wall.
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
    command_batches: HashMap<WallId, Vec<ConstructGateCmd>>,
    // Keep allocation identities alive while terrain is part of a fingerprint.
    terrain_snapshot: Option<Arc<[u16]>>,
    pub pending: HashMap<WallId, PendingGateReason>,
    pub rebuilt: Vec<WallId>,
    /// CPU transport reaches these records; renderer handles/uploads are absent.
    pub cpu_outputs: HashMap<WallId, Vec<GateCpuOutput>>,
}
#[derive(SystemParam)]
pub struct GateInvalidationReaders<'w, 's> {
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
        std::mem::take(gates.0.entry(wall).or_default())
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
    state.pending.remove(&wall);
}
fn fingerprint(
    host: &crate::PublishedWall,
    prepared_generation: u64,
    terrain: Option<
        &country_core::resources::walls::publication_source::TerrainHeightGrid512,
    >,
) -> Vec<u8> {
    let mut bytes = host.observed_projection().to_vec();
    bytes.extend(host.publication_revision.to_le_bytes());
    bytes.extend(prepared_generation.to_le_bytes());
    bytes.push(u8::from(host.observed_source_metadata_complete));
    bytes.push(u8::from(terrain.is_some()));
    if let Some(terrain) = terrain {
        bytes.extend((terrain.encoded.as_ptr() as usize).to_le_bytes());
        bytes.extend(terrain.minimum_y.to_le_bytes());
        bytes.extend(terrain.maximum_y.to_le_bytes());
        bytes.push(u8::from(terrain.sampling_fix));
    }
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
    // The upstream producer owns geometry invalidation. A no-op roof/spatial
    // event may leave its cached command batch unchanged and emit no commands;
    // withdrawing on that notification alone would permanently erase gates.
    let mut rebuild: HashSet<_> = arches.rebuilt.iter().copied().collect();
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
                .map(|path| fingerprint(host, path.generation, source.terrain.as_ref()))
        });
        if current.as_ref() != Some(previous) {
            refresh.insert(wall);
        }
    }
    rebuild.extend(refresh);
    // A producer rebuild replaces the whole batch, including an empty result.
    // Retain successful batches as well: terrain changes can invalidate CPU
    // geometry without changing the upstream arch segmentation.
    for wall in arches.rebuilt.iter().chain(&arches.pending) {
        state.command_batches.remove(wall);
    }
    let mut incoming = HashMap::<WallId, Vec<ConstructGateCmd>>::new();
    for request in events.read() {
        incoming
            .entry(request.0.wall_id)
            .or_default()
            .push(request.clone());
    }
    rebuild.extend(incoming.keys().copied());
    state.command_batches.extend(incoming);
    rebuild.extend(state.pending.iter().filter_map(|(&wall, reason)| {
        (*reason != PendingGateReason::OriginalCpuBranch).then_some(wall)
    }));
    let mut ids: Vec<_> = remove.iter().copied().collect();
    ids.sort_unstable_by_key(|w| w.0);
    for wall in ids {
        state.command_batches.remove(&wall);
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
    for &wall in &ids {
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
    // Detach the batch map while updating build state. Each request borrows
    // retained geometry, including retries while dependencies are unavailable.
    let command_batches = std::mem::take(&mut state.command_batches);
    let mut failed_walls = HashSet::new();
    for request in ids
        .iter()
        .filter_map(|wall| command_batches.get(wall))
        .flatten()
    {
        if failed_walls.contains(&request.0.wall_id) {
            continue;
        }
        let wall = request.0.wall_id;
        if remove.contains(&wall) {
            continue;
        }
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
        let input = GateConstructionInput {
            segment: &request.0.segment,
            prepared_positions: &path.path.positions,
            prepared_u: &path.path.points_u,
            observed_public_288: host.state.float_0x288(),
            observed_public_29a: host.state.observed_29a(),
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
                state.fingerprints.insert(
                    wall,
                    fingerprint(host, path.generation, source.terrain.as_ref()),
                );
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
        state.fingerprints.insert(
            wall,
            fingerprint(host, path.generation, source.terrain.as_ref()),
        );
        state.pending.remove(&wall);
        if !state.rebuilt.contains(&wall) {
            state.rebuilt.push(wall);
        }
    }
    state.command_batches = command_batches;
    state.terrain_snapshot = source
        .terrain
        .as_ref()
        .map(|terrain| terrain.encoded.clone());
}
fn nani_transform(projection: &GateNaniProjection) -> ObjectTransform {
    let mut bytes = [0; 48];
    // Transform records contain 10 floats followed by 8 padding bytes.
    for (index, value) in projection
        .rotation
        .into_iter()
        .chain(projection.position)
        .chain(projection.scale)
        .enumerate()
    {
        bytes[index * 4..index * 4 + 4].copy_from_slice(&value.to_le_bytes());
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
