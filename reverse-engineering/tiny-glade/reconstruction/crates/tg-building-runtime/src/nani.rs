//! Game-owned Nani creation, atlas selection, and CPU update boundary.
//!
//! B's StoneFloor bundle explicitly contains descriptor, Transform and a typed
//! zero-size marker. None has required components. Bevy performs registration,
//! insertion, change/removal tracking and allocation; GPU handles remain owned
//! by the deferred backend and are never synthesized here.
use crate::{Animation, ObjectTransform, Roof};
use bevy_ecs::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NaniInstanceMeshDesc {
    Named(u64),
    AtlasPart(u32),
}

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub struct NaniInstanceDesc {
    pub mesh: NaniInstanceMeshDesc,
    pub subset_kind: u8,
}
impl Default for NaniInstanceDesc {
    fn default() -> Self {
        Self {
            mesh: NaniInstanceMeshDesc::AtlasPart(0),
            subset_kind: 9,
        }
    }
}

/// Original InstanceDataMarker<StoneFloorInstanceData>. No payload or handle.
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct StoneFloorInstanceDataMarker;

/// The gate's original bundle uses SolidVertexColor, separately from floors.
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct SolidVertexColorInstanceDataMarker;

/// Observed gate bundle word. Its GPU upload interpretation remains pending.
#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ObservedSolidVertexColorWord30(pub u32);

/// Actual CPU writer's two boolean outputs. Query policy is not inferred from
/// the unrelated gate bundle word, nor defaulted when its owner is missing.
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub struct ObservedSolidVertexColorFlags {
    pub flag_30: bool,
    pub flag_34: bool,
}

#[derive(Bundle)]
pub struct NaniSolidVertexColorBundle {
    pub descriptor: NaniInstanceDesc,
    pub transform: ObjectTransform,
    pub marker: SolidVertexColorInstanceDataMarker,
    pub observed_word: ObservedSolidVertexColorWord30,
}

#[derive(Bundle, Default)]
pub struct NaniStoneFloorBundle {
    pub descriptor: NaniInstanceDesc,
    pub transform: ObjectTransform,
    pub marker: StoneFloorInstanceDataMarker,
}

pub fn register_nani_components(world: &mut World) {
    world.register_component::<NaniInstanceDesc>();
    world.register_component::<ObjectTransform>();
    world.register_component::<StoneFloorInstanceDataMarker>();
    world.register_component::<SolidVertexColorInstanceDataMarker>();
    world.register_component::<ObservedSolidVertexColorWord30>();
    world.register_component::<ObservedSolidVertexColorFlags>();
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NaniMeshAtlasEntry {
    pub atlas_part: u32,
    pub subset_kind: u8,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NaniAtlasError {
    MissingMatchingSubset,
    ChangedNamedMeshMustHaveOnePart,
    ChangedNamedMeshSubsetMismatch,
}

/// Initial named meshes scan for the first entry of the descriptor's subset.
/// Changed named meshes require one entry and the same subset. The atlas lookup
/// itself is supplied by its owner; hashing and resource decoding are excluded.
pub fn resolve_nani_mesh(
    descriptor: NaniInstanceDesc,
    entries: &[NaniMeshAtlasEntry],
    changed_active: bool,
) -> Result<u32, NaniAtlasError> {
    match descriptor.mesh {
        NaniInstanceMeshDesc::AtlasPart(part) => Ok(part),
        NaniInstanceMeshDesc::Named(_) if changed_active => {
            if entries.len() != 1 {
                return Err(NaniAtlasError::ChangedNamedMeshMustHaveOnePart);
            }
            if entries[0].subset_kind != descriptor.subset_kind {
                return Err(NaniAtlasError::ChangedNamedMeshSubsetMismatch);
            }
            Ok(entries[0].atlas_part)
        }
        NaniInstanceMeshDesc::Named(_) => entries
            .iter()
            .find(|entry| entry.subset_kind == descriptor.subset_kind)
            .map(|entry| entry.atlas_part)
            .ok_or(NaniAtlasError::MissingMatchingSubset),
    }
}

/// The observed 64-byte StoneFloorInstanceData upload record, not Rust's ABI.
#[derive(Clone, Debug, PartialEq)]
pub struct StoneFloorCpuProjection {
    pub record: [u8; 64],
    pub bounds_center: [f32; 3],
    pub bounds_radius: f32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StoneFloorProjectionError {
    UnsupportedRoofShape,
}

fn read_f32(bytes: &[u8], offset: usize) -> f32 {
    f32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap())
}
// The original calls sinf and cosf separately. Keep that dependency boundary;
// combining them into host sincosf can change an output by one ULP.
#[inline(never)]
fn circle_sin(value: f32) -> f32 {
    value.sin()
}
#[inline(never)]
fn circle_cos(value: f32) -> f32 {
    value.cos()
}

/// Roof branch of B nani_update_stone_floor, ending at the GPU writer call.
/// Transform is the fallback for ordinary Floor entities; a Roof supplies the
/// authoritative pivot/height, so its branch does not use Transform translation.
pub fn project_roof_stone_floor(
    roof: &[u8; 88],
    animation_first: f32,
) -> Result<StoneFloorCpuProjection, StoneFloorProjectionError> {
    let shape = u32::from_le_bytes(roof[8..12].try_into().unwrap());
    if shape > 1 {
        return Err(StoneFloorProjectionError::UnsupportedRoofShape);
    }
    let height = read_f32(roof, 0x30);
    let pitched = !(height <= 0.1);
    let mut margin = if pitched {
        read_f32(roof, 0x2c) * 0.5 + 0.63
    } else {
        0.0
    };
    let (orientation, mut dimensions) = if shape == 1 {
        let axes = if pitched && read_f32(roof, 0x34) == 1.0 {
            if roof[0x3c] & 1 != 0 {
                [1.0, 0.0]
            } else {
                [0.0, 1.0]
            }
        } else {
            [1.0, 1.0]
        };
        (
            [read_f32(roof, 0x0c), read_f32(roof, 0x10)],
            [
                read_f32(roof, 0x1c) + margin * axes[0],
                read_f32(roof, 0x20) + margin * axes[1],
            ],
        )
    } else {
        // Circle2d::get_tangent_at_u(0). The Windows CRT calls are dependency
        // boundaries; Rust's matching f32 math APIs are used, not reimplemented.
        let tau = std::f32::consts::TAU;
        let angle = read_f32(roof, 0x18) % tau;
        let positive = if angle < 0.0 { angle + tau } else { angle };
        let phase = positive + (1.0_f32 - 0.0) * tau;
        margin += read_f32(roof, 0x14) * 2.0;
        ([-circle_sin(phase), circle_cos(phase)], [margin, margin])
    };
    let tag_offset = shape as usize * 8;
    let y = if pitched {
        read_f32(roof, 0x4c)
    } else {
        read_f32(roof, 0x4c) + f32::from_bits(0xbf30a3d7)
    };
    let center = [
        read_f32(roof, 0x0c + tag_offset),
        y,
        read_f32(roof, 0x10 + tag_offset),
    ];
    if pitched {
        dimensions = [0.0; 2];
    }
    let style_tag = roof[0x40];
    let style_flags = match style_tag {
        1 => 0,
        2 => u32::from(roof[0x41]) | 2,
        _ => u32::from(roof[0x54]),
    };
    let style_parameter_0 = if style_tag == 2 {
        u32::from_le_bytes(roof[0x44..0x48].try_into().unwrap())
    } else {
        0
    };
    let style_parameter_1 = if style_tag >= 2 {
        u32::from_le_bytes(roof[0x48..0x4c].try_into().unwrap())
    } else {
        0
    };
    let words = [
        center[0].to_bits(),
        center[1].to_bits(),
        center[2].to_bits(),
        u32::from_le_bytes(roof[0..4].try_into().unwrap()),
        orientation[0].to_bits(),
        orientation[1].to_bits(),
        dimensions[0].to_bits(),
        dimensions[1].to_bits(),
        animation_first.to_bits(),
        shape,
        0,
        u32::from(pitched),
        style_flags,
        style_parameter_0,
        style_parameter_1,
        0,
    ];
    let mut record = [0; 64];
    for (word, target) in words.iter().zip(record.chunks_exact_mut(4)) {
        target.copy_from_slice(&word.to_le_bytes());
    }
    let a = dimensions[0].abs();
    let b = dimensions[1].abs();
    let radius = if a.is_nan() {
        b
    } else if b.is_nan() {
        a
    } else {
        a.max(b)
    };
    Ok(StoneFloorCpuProjection {
        record,
        bounds_center: center,
        bounds_radius: radius,
    })
}

#[derive(Clone, Debug, PartialEq)]
pub struct NaniCpuSnapshot {
    pub descriptor: NaniInstanceDesc,
    pub resolved_atlas_part: u32,
    pub transform: [u8; 48],
    pub stone_floor: StoneFloorCpuProjection,
}
#[derive(Clone, Debug)]
pub enum DeferredNaniRequest {
    Create {
        entity: Entity,
        snapshot: NaniCpuSnapshot,
    },
    Update {
        entity: Entity,
        snapshot: NaniCpuSnapshot,
        mesh_changed: bool,
    },
    Remove {
        entity: Entity,
    },
}
/// CPU-side upload intentions. These are not NaniActiveInstance confirmations.
#[derive(Resource, Default, Debug)]
pub struct DeferredNaniRequests(pub Vec<DeferredNaniRequest>);
#[derive(Resource, Default)]
pub struct NaniCpuStagingState {
    snapshots: HashMap<Entity, NaniCpuSnapshot>,
}
impl NaniCpuStagingState {
    pub fn contains(&self, entity: Entity) -> bool {
        self.snapshots.contains_key(&entity)
    }
    pub fn len(&self) -> usize {
        self.snapshots.len()
    }
    pub fn is_empty(&self) -> bool {
        self.snapshots.is_empty()
    }
}

/// Recovered CPU upload parameters, staged after roof state/animation changes.
/// This explicit backend adapter runs even when no graphics device is supplied.
pub fn stage_roof_nani_updates(
    roofs: Query<
        (
            Entity,
            &Roof,
            &Animation,
            &ObjectTransform,
            &NaniInstanceDesc,
        ),
        With<StoneFloorInstanceDataMarker>,
    >,
    atlas: Res<NaniMeshAtlas>,
    mut confirmations: ResMut<NaniBackendConfirmations>,
    mut diagnostics: ResMut<NaniCpuDiagnostics>,
    mut staging: ResMut<NaniCpuStagingState>,
    mut deferred: ResMut<DeferredNaniRequests>,
) {
    let mut current = HashMap::new();
    diagnostics.0.clear();
    for (entity, roof, animation, transform, descriptor) in &roofs {
        let first = read_f32(animation.0.bytes(), 0);
        let Ok(stone_floor) = project_roof_stone_floor(&roof.0, first) else {
            continue;
        };
        let named_entries = match descriptor.mesh {
            NaniInstanceMeshDesc::Named(name) => {
                atlas.0.get(&name).map(Vec::as_slice).unwrap_or(&[])
            }
            _ => &[],
        };
        let changed_active = confirmations.0.contains(&entity)
            && staging
                .snapshots
                .get(&entity)
                .is_some_and(|old| old.descriptor != *descriptor);
        let resolved_atlas_part =
            match resolve_nani_mesh(*descriptor, named_entries, changed_active) {
                Ok(part) => part,
                Err(error) => {
                    diagnostics.0.push((entity, error));
                    if let Some(old) = staging.snapshots.get(&entity) {
                        current.insert(entity, old.clone());
                    }
                    continue;
                }
            };
        let snapshot = NaniCpuSnapshot {
            descriptor: *descriptor,
            resolved_atlas_part,
            transform: transform.0,
            stone_floor,
        };
        match staging.snapshots.get(&entity) {
            None => deferred.0.push(DeferredNaniRequest::Create {
                entity,
                snapshot: snapshot.clone(),
            }),
            Some(previous) if previous != &snapshot => {
                deferred.0.push(DeferredNaniRequest::Update {
                    entity,
                    mesh_changed: previous.descriptor != snapshot.descriptor,
                    snapshot: snapshot.clone(),
                })
            }
            Some(_) => {}
        }
        current.insert(entity, snapshot);
    }
    for entity in staging.snapshots.keys() {
        if !current.contains_key(entity) {
            deferred
                .0
                .push(DeferredNaniRequest::Remove { entity: *entity });
        }
    }
    // This consumer only owns floor entities. Other typed Nani consumers must
    // not lose their real backend confirmations when the floor query runs.
    for entity in staging.snapshots.keys() {
        if !current.contains_key(entity) {
            confirmations.0.remove(entity);
        }
    }
    staging.snapshots = current;
}

/// CPU component snapshot, not a recovered GPU upload record. Implementation
/// is pending joint review in the current parallel batch.
#[derive(Clone, Debug, PartialEq)]
pub struct SolidNaniCpuSnapshot {
    pub descriptor: NaniInstanceDesc,
    pub resolved_atlas_part: u32,
    pub transform: [u8; 48],
    pub observed_word_30: u32,
    pub payload: Option<tiny_glade_reconstruction::building_nani_solid_vertex_color::SolidVertexColorCpuProjection>,
}

#[derive(Clone, Debug)]
pub enum SolidNaniCpuChange {
    Create { entity: Entity, snapshot: SolidNaniCpuSnapshot },
    Update { entity: Entity, snapshot: SolidNaniCpuSnapshot, mesh_changed: bool },
    Remove { entity: Entity },
}

#[derive(Resource, Default, Debug)]
pub struct SolidNaniCpuChanges(pub Vec<SolidNaniCpuChange>);

#[derive(Resource, Default)]
pub struct SolidNaniCpuStaging {
    pub snapshots: HashMap<Entity, SolidNaniCpuSnapshot>,
    pub pending_flag_owner: std::collections::HashSet<Entity>,
}

#[derive(Resource, Default, Debug)]
pub struct SolidNaniCpuDiagnostics(pub Vec<(Entity, NaniAtlasError)>);

/// Genuine Bevy typed component/atlas lifecycle. The original SolidVertexColor
/// GPU writer remains a separate unresolved consumer; no activation is made.
pub fn stage_solid_nani_updates(
    entities: Query<
        (Entity, &ObjectTransform, &NaniInstanceDesc, &ObservedSolidVertexColorWord30,
            Option<&ObservedSolidVertexColorFlags>),
        With<SolidVertexColorInstanceDataMarker>,
    >,
    atlas: Res<NaniMeshAtlas>,
    mut confirmations: ResMut<NaniBackendConfirmations>,
    mut diagnostics: ResMut<SolidNaniCpuDiagnostics>,
    mut staging: ResMut<SolidNaniCpuStaging>,
    mut changes: ResMut<SolidNaniCpuChanges>,
) {
    let mut current = HashMap::new();
    diagnostics.0.clear();
    staging.pending_flag_owner.clear();
    for (entity, transform, descriptor, observed_word, flags) in &entities {
        if flags.is_none() {
            staging.pending_flag_owner.insert(entity);
        }
        let entries = match descriptor.mesh {
            NaniInstanceMeshDesc::Named(name) => atlas.0.get(&name).map(Vec::as_slice).unwrap_or(&[]),
            NaniInstanceMeshDesc::AtlasPart(_) => &[],
        };
        let changed_active = confirmations.0.contains(&entity)
            && staging.snapshots.get(&entity).is_some_and(|old| old.descriptor != *descriptor);
        let resolved_atlas_part = match resolve_nani_mesh(*descriptor, entries, changed_active) {
            Ok(part) => part,
            Err(error) => {
                diagnostics.0.push((entity, error));
                if let Some(previous) = staging.snapshots.get(&entity) {
                    current.insert(entity, previous.clone());
                }
                continue;
            }
        };
        let snapshot = SolidNaniCpuSnapshot {
            descriptor: *descriptor,
            resolved_atlas_part,
            transform: transform.0,
            observed_word_30: observed_word.0,
            payload: flags.map(|flags| tiny_glade_reconstruction::building_nani_solid_vertex_color::project_solid_vertex_color(
                &transform.0, flags.flag_30, flags.flag_34,
            )),
        };
        match staging.snapshots.get(&entity) {
            None => changes.0.push(SolidNaniCpuChange::Create { entity, snapshot: snapshot.clone() }),
            Some(previous) if previous != &snapshot => changes.0.push(SolidNaniCpuChange::Update {
                entity,
                mesh_changed: previous.descriptor != snapshot.descriptor,
                snapshot: snapshot.clone(),
            }),
            Some(_) => {},
        }
        current.insert(entity, snapshot);
    }
    for entity in staging.snapshots.keys() {
        if !current.contains_key(entity) {
            changes.0.push(SolidNaniCpuChange::Remove { entity: *entity });
            confirmations.0.remove(entity);
        }
    }
    staging.snapshots = current;
}

/// Atlas metadata is supplied by its asset owner; Vec/HashMap decoding and
/// lookup remain library boundaries. Default AtlasPart(0) needs no name scan.
#[derive(Resource, Default)]
pub struct NaniMeshAtlas(pub HashMap<u64, Vec<NaniMeshAtlasEntry>>);
/// Only a real renderer backend may confirm activation. This adapter never
/// adds entries or manufactures NaniActiveInstance/InstanceHandle values.
#[derive(Resource, Default)]
pub struct NaniBackendConfirmations(pub std::collections::HashSet<Entity>);
#[derive(Resource, Default, Debug)]
pub struct NaniCpuDiagnostics(pub Vec<(Entity, NaniAtlasError)>);
