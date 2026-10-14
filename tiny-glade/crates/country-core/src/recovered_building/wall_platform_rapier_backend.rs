//! Actual dependency-backed platform collider storage and query adapter.
//! Original private selection/lifecycle policy is in wall_platform_collision.
//! DirtyTiles rasterization is a required caller-supplied owner, never defaulted.
use crate::recovered_building::building_wall_platform_collision::*;
use crate::recovered_building::building_wall_state::WallId;
use rapier3d::prelude::*;
use slotmap::{Key, SlotMap, new_key_type};
use std::sync::Arc;
new_key_type! { pub struct PlatformMeshKey; }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PlatformRapierHandle {
    pub mesh: PlatformMeshKey,
    pub collider: ColliderHandle,
    pub subworld: u8,
}
#[derive(Clone, Debug)]
pub enum PlatformColliderDiff {
    Upserted(PlatformRapierHandle),
    Removed(PlatformRapierHandle),
}
impl PlatformColliderDiff {
    pub fn original_discriminant(&self) -> u32 {
        match self {
            Self::Upserted(_) => 0,
            Self::Removed(_) => 1,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlatformRapierError {
    MissingCollider,
    MissingMesh,
    UnsupportedPose,
}
pub struct PlatformRapierMesh {
    pub key: PlatformStairColliderKey,
    pub data: Arc<PlatformStairIndexedMesh>,
}
pub struct PlatformRapierBackend<R: FnMut([f32; 4])> {
    pub colliders: ColliderSet,
    pub bodies: RigidBodySet,
    pub islands: IslandManager,
    pub queries: QueryPipeline,
    pub meshes: SlotMap<PlatformMeshKey, PlatformRapierMesh>,
    /// The original subworld dirty marker gates the subsequent query update.
    pub dirty: bool,
    pub diffs: Vec<PlatformColliderDiff>,
    rasterize_xz: R,
}
#[derive(Clone, Copy, Debug)]
pub struct PlatformRayHit {
    pub wall: WallId,
    pub handle: PlatformRapierHandle,
    pub time_of_impact: f32,
    pub normal: [f32; 3],
}
impl<R: FnMut([f32; 4])> PlatformRapierBackend<R> {
    pub fn new(rasterize_xz: R) -> Self {
        Self {
            colliders: ColliderSet::new(),
            bodies: RigidBodySet::new(),
            islands: IslandManager::new(),
            queries: QueryPipeline::new(),
            meshes: SlotMap::with_key(),
            dirty: false,
            diffs: Vec::new(),
            rasterize_xz,
        }
    }
    pub fn flush_queries(&mut self) {
        if self.dirty {
            self.queries.update(&self.colliders);
            self.dirty = false;
        }
    }
    /// Explicit dependency query over this platform subworld only. It does not
    /// claim the original all-subworld RaycastWorld query filters are restored.
    pub fn cast_ray(
        &self,
        origin: [f32; 3],
        direction: [f32; 3],
        max_toi: f32,
        solid: bool,
    ) -> Option<PlatformRayHit> {
        let ray = Ray::new(
            Point::new(origin[0], origin[1], origin[2]),
            Vector::new(direction[0], direction[1], direction[2]),
        );
        let (collider, intersection) = self.queries.cast_ray_and_get_normal(
            &self.bodies,
            &self.colliders,
            &ray,
            max_toi,
            solid,
            QueryFilter::default(),
        )?;
        let body = self.colliders.get(collider)?;
        let mesh = PlatformMeshKey::from(slotmap::KeyData::from_ffi(body.user_data as u64));
        let data = self.meshes.get(mesh)?;
        Some(PlatformRayHit {
            wall: data.key.wall,
            handle: PlatformRapierHandle {
                mesh,
                collider,
                subworld: 7,
            },
            time_of_impact: intersection.time_of_impact,
            normal: [
                intersection.normal.x,
                intersection.normal.y,
                intersection.normal.z,
            ],
        })
    }
    fn dirty_aabb(&mut self, aabb: rapier3d::parry::bounding_volume::Aabb) {
        self.dirty = true;
        (self.rasterize_xz)([aabb.mins.x, aabb.mins.z, aabb.maxs.x, aabb.maxs.z]);
    }
}
impl<R: FnMut([f32; 4])> PlatformStairCollisionBackend for PlatformRapierBackend<R> {
    type Handle = PlatformRapierHandle;
    type Error = PlatformRapierError;
    fn pose(&self, handle: &Self::Handle) -> Result<ColliderPose, Self::Error> {
        self.meshes
            .get(handle.mesh)
            .ok_or(PlatformRapierError::MissingMesh)?;
        let position = self
            .colliders
            .get(handle.collider)
            .ok_or(PlatformRapierError::MissingCollider)?
            .position();
        let t = position.translation.vector;
        let q = position.rotation.coords;
        Ok(ColliderPose {
            translation: [t.x, t.y, t.z],
            rotation: [q.x, q.y, q.z, q.w],
        })
    }
    fn insert(
        &mut self,
        key: PlatformStairColliderKey,
        spec: PlatformStairColliderSpec,
    ) -> Result<Self::Handle, Self::Error> {
        // This actual platform creator supplies identity. Other source owners
        // need their own proven IsoTransform composition rather than inference.
        if !spec.pose.ordered_eq(ColliderPose::IDENTITY) {
            return Err(PlatformRapierError::UnsupportedPose);
        }
        let points = spec
            .mesh
            .positions
            .iter()
            .map(|p| Point::new(p[0], p[1], p[2]))
            .collect();
        let mesh = self.meshes.insert(PlatformRapierMesh {
            key,
            data: spec.mesh.clone(),
        });
        let body = ColliderBuilder::trimesh(points, spec.mesh.triangles.clone())
            .sensor(spec.is_sensor())
            .position(Isometry::identity())
            .user_data(mesh.data().as_ffi() as u128)
            .build();
        let aabb = body.compute_aabb();
        let collider = self.colliders.insert(body);
        let handle = PlatformRapierHandle {
            mesh,
            collider,
            subworld: spec.subworld(),
        };
        self.dirty_aabb(aabb);
        self.diffs.push(PlatformColliderDiff::Upserted(handle));
        Ok(handle)
    }
    fn remove(&mut self, handle: &Self::Handle) -> Result<(), Self::Error> {
        if self.meshes.get(handle.mesh).is_none() {
            return Err(PlatformRapierError::MissingMesh);
        }
        // Original dispose passes wake_up=false (1408d95ed).
        let body = self
            .colliders
            .remove(handle.collider, &mut self.islands, &mut self.bodies, false)
            .ok_or(PlatformRapierError::MissingCollider)?;
        self.meshes
            .remove(handle.mesh)
            .ok_or(PlatformRapierError::MissingMesh)?;
        self.dirty_aabb(body.compute_aabb());
        self.diffs.push(PlatformColliderDiff::Removed(*handle));
        Ok(())
    }
}
