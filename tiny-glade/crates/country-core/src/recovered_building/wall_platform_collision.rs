//! Platform-stair collision mesh identity policy from the B native owner.
//! std owns SipHash; rapier/parry/slotmap own physics and storage algorithms.
//! This module does not claim the original collider backend or query is restored.
use crate::recovered_building::building_wall_state::WallId;
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PlatformStairColliderKey {
    pub wall: WallId,
}
impl PlatformStairColliderKey {
    /// The original RaycastGeoSource discriminant at this creator is seven.
    pub fn source_discriminant(self) -> u8 {
        7
    }
}

/// Original new_indexed hashes triangle [u32;3] indices followed by raw Vec3 bits as UVec3,
/// then folds each present auxiliary index slice into the previous hash.
/// None and Some(empty) have distinct original fingerprints.
pub fn indexed_mesh_hash(
    positions: &[[f32; 3]],
    triangles: &[[u32; 3]],
    auxiliary_float3: Option<&[[f32; 3]]>,
    auxiliary_float2: Option<&[[f32; 2]]>,
) -> u64 {
    let point_bits = positions
        .iter()
        .map(|p| glam::UVec3::from_array(p.map(f32::to_bits)))
        .collect::<Vec<_>>();
    let mut hash = DefaultHasher::new();
    triangles.hash(&mut hash);
    point_bits.hash(&mut hash);
    let mut fingerprint = hash.finish();
    if let Some(values) = auxiliary_float3 {
        let values = values
            .iter()
            .map(|p| glam::UVec3::from_array(p.map(f32::to_bits)))
            .collect::<Vec<_>>();
        let mut hash = DefaultHasher::new();
        fingerprint.hash(&mut hash);
        values.hash(&mut hash);
        fingerprint = hash.finish();
    }
    if let Some(values) = auxiliary_float2 {
        let values = values
            .iter()
            .map(|p| glam::UVec2::from_array(p.map(f32::to_bits)))
            .collect::<Vec<_>>();
        let mut hash = DefaultHasher::new();
        fingerprint.hash(&mut hash);
        values.hash(&mut hash);
        fingerprint = hash.finish();
    }
    fingerprint
}

use crate::recovered_building::building_wall_platform_stairs::WallPlatformStairGeometry;
use std::{collections::HashMap, sync::Arc};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColliderPose {
    pub translation: [f32; 3],
    /// Original quaternion component order x,y,z,w.
    pub rotation: [f32; 4],
}
impl ColliderPose {
    pub const IDENTITY: Self = Self {
        translation: [0.; 3],
        rotation: [0., 0., 0., 1.],
    };
    /// Original ordered float comparisons accept signed zero, reject NaN.
    pub fn ordered_eq(self, other: Self) -> bool {
        self.translation
            .into_iter()
            .zip(other.translation)
            .all(|(a, b)| a == b)
            && self
                .rotation
                .into_iter()
                .zip(other.rotation)
                .all(|(a, b)| a == b)
    }
}
#[derive(Clone, Debug)]
pub struct PlatformStairIndexedMesh {
    pub positions: Vec<[f32; 3]>,
    pub triangles: Vec<[u32; 3]>,
    pub fingerprint: u64,
}
impl PlatformStairIndexedMesh {
    pub fn from_geometry(geometry: &WallPlatformStairGeometry) -> Self {
        let positions = geometry
            .cells
            .iter()
            .flat_map(|c| c.vertices)
            .collect::<Vec<_>>();
        let triangles = geometry
            .cells
            .iter()
            .flat_map(|c| {
                c.indices
                    .chunks_exact(3)
                    .map(|p| [p[0], p[1], p[2]])
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let fingerprint = indexed_mesh_hash(&positions, &triangles, None, None);
        Self {
            positions,
            triangles,
            fingerprint,
        }
    }
}
#[derive(Clone, Debug)]
pub struct PlatformStairColliderSpec {
    pub mesh: Arc<PlatformStairIndexedMesh>,
    pub pose: ColliderPose,
}
impl PlatformStairColliderSpec {
    pub fn source_discriminant(&self) -> u8 {
        7
    }
    pub fn subworld(&self) -> u8 {
        7
    }
    /// Original to_offset_collider_builder explicitly sets sensor byte true.
    pub fn is_sensor(&self) -> bool {
        true
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColliderUpsertAction {
    Insert,
    Replace,
    Keep,
}
/// Original tag7 mesh equality compares the mesh fingerprint and WallId at
/// source+8. The original RNE jump table overrides the inaccurate patched CFG.
pub fn platform_collider_action(
    key: PlatformStairColliderKey,
    existing: Option<(Option<ColliderPose>, PlatformStairColliderKey, u64)>,
    requested: ColliderPose,
    fingerprint: u64,
) -> ColliderUpsertAction {
    match existing {
        None => ColliderUpsertAction::Insert,
        Some((Some(pose), old_key, old))
            if old_key == key && pose.ordered_eq(requested) && old == fingerprint =>
        {
            ColliderUpsertAction::Keep
        }
        Some(_) => ColliderUpsertAction::Replace,
    }
}
/// Adapter boundary for the actual locked Rapier/Parry/slotmap backend. Insert
/// must construct an indexed SharedShape, insert the sensor collider into
/// subworld7, rasterize its XZ AABB, mark that subworld dirty and publish Upsert.
/// Remove must dispose the collider and mesh instance, rasterize the old AABB,
/// mark the old subworld dirty and publish Removed. No physics algorithm is
/// reconstructed here; callers must supply this executable backend.
pub trait PlatformStairCollisionBackend {
    type Handle: Clone;
    type Error;
    fn pose(&self, handle: &Self::Handle) -> Result<ColliderPose, Self::Error>;
    fn insert(
        &mut self,
        key: PlatformStairColliderKey,
        spec: PlatformStairColliderSpec,
    ) -> Result<Self::Handle, Self::Error>;
    fn remove(&mut self, handle: &Self::Handle) -> Result<(), Self::Error>;
}
#[derive(Clone, Debug)]
pub struct PlatformStairOwnedCollider<H> {
    pub key: PlatformStairColliderKey,
    pub handle: H,
    pub mesh: Arc<PlatformStairIndexedMesh>,
}
#[derive(Clone, Debug)]
pub enum PlatformColliderUpdate<H> {
    Absent,
    Removed { previous: H },
    Inserted { current: H },
    Replaced { previous: H, current: H },
    Kept { current: H },
}
/// Game ownership and update policy; std supplies the map, backend supplies real
/// dependency operations. No original hashbrown/slotmap/Bevy layout is copied.
pub struct PlatformStairColliderWorld<B: PlatformStairCollisionBackend> {
    pub backend: B,
    pub colliders: HashMap<
        crate::recovered_building::building_wall_state::WallId,
        PlatformStairOwnedCollider<B::Handle>,
    >,
}
impl<B: PlatformStairCollisionBackend> PlatformStairColliderWorld<B> {
    pub fn new(backend: B) -> Self {
        Self {
            backend,
            colliders: HashMap::new(),
        }
    }
    pub fn remove(&mut self, wall: WallId) -> Result<PlatformColliderUpdate<B::Handle>, B::Error> {
        let Some(existing) = self.colliders.get(&wall) else {
            return Ok(PlatformColliderUpdate::Absent);
        };
        let previous = existing.handle.clone();
        self.backend.remove(&previous)?;
        self.colliders.remove(&wall);
        Ok(PlatformColliderUpdate::Removed { previous })
    }
    pub fn update(
        &mut self,
        wall: WallId,
        geometry: &WallPlatformStairGeometry,
    ) -> Result<PlatformColliderUpdate<B::Handle>, B::Error> {
        // The actual creator gates physics insertion by accumulated vertex count.
        if geometry.cells.is_empty() {
            return self.remove(wall);
        }
        let mesh = Arc::new(PlatformStairIndexedMesh::from_geometry(geometry));
        let old = match self.colliders.get(&wall) {
            Some(c) => Some((
                Some(self.backend.pose(&c.handle)?),
                c.key,
                c.mesh.fingerprint,
            )),
            None => None,
        };
        match platform_collider_action(
            PlatformStairColliderKey { wall },
            old,
            ColliderPose::IDENTITY,
            mesh.fingerprint,
        ) {
            ColliderUpsertAction::Keep => Ok(PlatformColliderUpdate::Kept {
                current: self.colliders[&wall].handle.clone(),
            }),
            ColliderUpsertAction::Insert => {
                let handle = self.backend.insert(
                    PlatformStairColliderKey { wall },
                    PlatformStairColliderSpec {
                        mesh: mesh.clone(),
                        pose: ColliderPose::IDENTITY,
                    },
                )?;
                self.colliders.insert(
                    wall,
                    PlatformStairOwnedCollider {
                        key: PlatformStairColliderKey { wall },
                        handle: handle.clone(),
                        mesh,
                    },
                );
                Ok(PlatformColliderUpdate::Inserted { current: handle })
            }
            ColliderUpsertAction::Replace => {
                let previous = self.colliders[&wall].handle.clone();
                self.backend.remove(&previous)?;
                self.colliders.remove(&wall);
                let handle = self.backend.insert(
                    PlatformStairColliderKey { wall },
                    PlatformStairColliderSpec {
                        mesh: mesh.clone(),
                        pose: ColliderPose::IDENTITY,
                    },
                )?;
                self.colliders.insert(
                    wall,
                    PlatformStairOwnedCollider {
                        key: PlatformStairColliderKey { wall },
                        handle: handle.clone(),
                        mesh,
                    },
                );
                Ok(PlatformColliderUpdate::Replaced {
                    previous,
                    current: handle,
                })
            }
        }
    }
}
