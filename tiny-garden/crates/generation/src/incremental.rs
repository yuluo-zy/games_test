//! Engine-independent identities, dependency inputs and immutable CPU chunks.
//! A generation cache is exactly one accepted snapshot per building, not an
//! unbounded history of every dimension the user has ever dragged through.
use crate::mesh::{BuildingMesh, GeometryProfile, MaterialKey, MeshData, MeshError};
use crate::{BlockLayout, BuildingLayout, Face};
use garden_domain::{BlockId, Facade, Roof};
use std::{
    collections::BTreeMap,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PartKind {
    Wall(Face),
    Roof,
    Windows(Face, u8, u16),
    Door,
    Chimney,
    Whole,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PartId {
    pub block: BlockId,
    pub kind: PartKind,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct BatchId {
    pub part: PartId,
    pub material: MaterialKey,
}

/// Exact dependency key, separate from identity and placement. Template/rules
/// revision must change when compiler semantics change.
#[derive(Debug, Clone, PartialEq)]
pub struct GeometryKey {
    pub block: BlockLayout,
    pub profile: GeometryProfile,
    pub revision: u64,
}
#[derive(Debug, Clone)]
pub struct PartInput {
    pub id: PartId,
    pub key: GeometryKey,
    pub offset: [f32; 3],
}
#[derive(Debug, Clone)]
pub struct PreparedBatch {
    pub id: BatchId,
    pub material: MaterialKey,
    pub data: Arc<MeshData>,
    pub tangents: Arc<Vec<[f32; 4]>>,
    pub offset: [f32; 3],
    pub min: [f32; 3],
    pub max: [f32; 3],
}
#[derive(Debug, Clone, Default)]
pub struct PreparedBuilding {
    pub batches: Vec<PreparedBatch>,
    pub keys: BTreeMap<PartId, GeometryKey>,
    pub rebuilt_parts: usize,
    pub reused_parts: usize,
    pub generated_vertices: usize,
}
impl PreparedBuilding {
    pub fn triangles(&self) -> usize {
        self.batches.iter().map(|b| b.data.triangles()).sum()
    }
    pub fn vertices(&self) -> usize {
        self.batches.iter().map(|b| b.data.positions.len()).sum()
    }
    pub fn whole(mesh: BuildingMesh) -> Self {
        let id = PartId {
            block: BlockId::new(1).unwrap(),
            kind: PartKind::Whole,
        };
        let mut result = Self {
            rebuilt_parts: 1,
            ..Self::default()
        };
        result.push(id, [0.; 3], mesh);
        result
    }
    fn push(&mut self, id: PartId, offset: [f32; 3], mesh: BuildingMesh) {
        for batch in mesh.batches {
            let mut min = [f32::INFINITY; 3];
            let mut max = [f32::NEG_INFINITY; 3];
            for p in &batch.data.positions {
                for i in 0..3 {
                    min[i] = min[i].min(p[i]);
                    max[i] = max[i].max(p[i]);
                }
            }
            self.generated_vertices += batch.data.positions.len();
            self.batches.push(PreparedBatch {
                id: BatchId {
                    part: id,
                    material: batch.material,
                },
                material: batch.material,
                data: Arc::new(batch.data),
                tangents: Arc::default(),
                offset,
                min,
                max,
            });
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Cancellation(Arc<AtomicBool>);
impl Cancellation {
    pub fn cancel(&self) {
        self.0.store(true, Ordering::Relaxed);
    }
    pub fn is_cancelled(&self) -> bool {
        self.0.load(Ordering::Relaxed)
    }
    pub fn check(&self) -> Result<(), MeshError> {
        if self.is_cancelled() {
            Err(MeshError("generation cancelled"))
        } else {
            Ok(())
        }
    }
}

/// Layout already includes the entrance override shared by wall holes and art.
/// Wall/roof/window inputs contain only their own dependencies. Translating an
/// upper tier changes offsets, not any of its local meshes or local UVs.
pub fn plan(
    layout: &BuildingLayout,
    profile: GeometryProfile,
) -> Result<Vec<PartInput>, MeshError> {
    profile.validate()?;
    let p = layout.placement;
    if [p.x, p.z, p.elevation, p.yaw]
        .into_iter()
        .any(|v| !v.is_finite() || v.abs() > 10_000.)
    {
        return Err(MeshError("placement outside presentation range"));
    }
    if layout.blocks.is_empty() || layout.blocks.len() > 3 {
        return Err(MeshError("invalid block count"));
    }
    let mut result = Vec::new();
    for source in &layout.blocks {
        let r = source.footprint;
        if !r.valid()
            || !(1.2..=100.).contains(&r.width)
            || !(1.2..=100.).contains(&r.depth)
            || !(2.2..=16.).contains(&source.height)
            || !(1..=4).contains(&source.stories)
            || source.windows.len() > 1000
            || [r.x, r.z, source.base_elevation]
                .into_iter()
                .any(|v| !v.is_finite() || v.abs() > 1000.)
        {
            return Err(MeshError("invalid block layout"));
        }
        for w in &source.windows {
            if w.key.building != layout.building
                || w.key.block != source.block
                || [w.along_wall, w.elevation, w.width, w.height]
                    .into_iter()
                    .any(|v| !v.is_finite())
                || w.width <= 0.
                || w.height <= 0.
            {
                return Err(MeshError("invalid window layout"));
            }
        }
        let origin = [r.x as f32, source.base_elevation as f32, r.z as f32];
        let mut local = source.clone();
        local.footprint.x = 0.;
        local.footprint.z = 0.;
        local.base_elevation = 0.;
        local.terraces.clear();
        for w in &mut local.windows {
            w.elevation -= source.base_elevation;
        }
        let mut emit = |kind, block: BlockLayout, offset| {
            result.push(PartInput {
                id: PartId {
                    block: source.block,
                    kind,
                },
                key: GeometryKey {
                    block,
                    profile,
                    revision: crate::RULES_REVISION,
                },
                offset,
            });
        };
        for face in [Face::Front, Face::Back, Face::Left, Face::Right] {
            let mut wall = local.clone();
            wall.effective_roof = Roof::Flat;
            wall.windows.retain(|w| w.key.face == face);
            emit(PartKind::Wall(face), wall, origin);
            for story in 0..source.stories {
                let mut windows = local.clone();
                windows.windows.retain(|w| {
                    w.key.face == face
                        && w.key.story == story
                        && !(w.height == 2. && w.key.slot < 0x8000)
                });
                if windows.windows.is_empty() {
                    continue;
                }
                // Fixed-size upload chunks: eight windows, not one draw call per
                // window and not a 100m facade's entire art mesh in one upload.
                windows.windows.sort_by_key(|w| w.key.slot);
                for (group, members) in windows.windows.chunks(8).enumerate() {
                    let center = members[0].elevation;
                    let mut chunk = windows.clone();
                    chunk.windows = members.to_vec();
                    for w in &mut chunk.windows {
                        w.elevation -= center;
                    }
                    chunk.height = 0.;
                    chunk.stories = 0;
                    chunk.effective_roof = Roof::Flat;
                    chunk.facade = Facade::Stone;
                    emit(
                        PartKind::Windows(face, story, group as u16),
                        chunk,
                        [origin[0], origin[1] + center as f32, origin[2]],
                    );
                }
            }
        }
        let mut door = local.clone();
        door.windows
            .retain(|w| w.height == 2. && w.key.slot < 0x8000);
        if !door.windows.is_empty() {
            let center = door.windows[0].elevation;
            for w in &mut door.windows {
                w.elevation -= center;
            }
            door.height = 0.;
            door.stories = 0;
            door.effective_roof = Roof::Flat;
            door.facade = Facade::Stone;
            emit(
                PartKind::Door,
                door,
                [origin[0], origin[1] + center as f32, origin[2]],
            );
        }
        let top = [origin[0], origin[1] + source.height as f32, origin[2]];
        local.windows.clear();
        local.height = 0.;
        local.stories = 0;
        emit(PartKind::Roof, local.clone(), top);
        if local.effective_roof != Roof::Flat && r.width.min(r.depth) >= 2. {
            local.facade = Facade::Stone;
            emit(PartKind::Chimney, local, top);
        }
    }
    Ok(result)
}

/// Reuse by identity AND exact geometry inputs. Output owns a complete target
/// snapshot so it is safe even if the renderer hasn't published the baseline.
/// Only changed chunks are compiled; all other buffers are shared, not copied.
pub fn reconcile(
    inputs: Vec<PartInput>,
    baseline: Option<&PreparedBuilding>,
    cancel: &Cancellation,
    mut compile: impl FnMut(&PartInput) -> Result<BuildingMesh, MeshError>,
) -> Result<PreparedBuilding, MeshError> {
    let mut next = PreparedBuilding::default();
    for input in inputs {
        cancel.check()?;
        if let Some(old) = baseline.filter(|old| old.keys.get(&input.id) == Some(&input.key)) {
            for batch in old.batches.iter().filter(|b| b.id.part == input.id) {
                let mut batch = batch.clone();
                batch.offset = input.offset;
                next.batches.push(batch);
            }
            next.reused_parts += 1;
        } else {
            next.push(input.id, input.offset, compile(&input)?);
            next.rebuilt_parts += 1;
        }
        next.keys.insert(input.id, input.key);
    }
    cancel.check()?;
    Ok(next)
}

/// Structural-only engine adapter also benefits from face/roof reuse. Real art
/// kits use the richer window-group plan through their own compilation port.
pub fn compile_structure(
    layout: &BuildingLayout,
    profile: GeometryProfile,
    baseline: Option<&PreparedBuilding>,
    cancel: &Cancellation,
) -> Result<PreparedBuilding, MeshError> {
    let inputs = plan(layout, profile)?
        .into_iter()
        .filter(|p| matches!(p.id.kind, PartKind::Wall(_) | PartKind::Roof))
        .collect();
    reconcile(inputs, baseline, cancel, |input| {
        crate::mesh::compile_structure_part(
            &input.key.block,
            profile,
            if let PartKind::Wall(face) = input.id.kind {
                Some(face)
            } else {
                None
            },
            true,
        )
    })
}
