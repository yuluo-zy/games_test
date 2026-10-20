//! Original platform collider consumer1412453c0, attached to the canonical world.
use super::*;
use crate::resources::walls::WallId;
use crate::systems::wall::construction::platform_stairs::WallPlatformStairGeometry;
use crate::systems::wall::construction::platform_collision::PlatformStairIndexedMesh;

/// Native wall-keyed collider instances. Rapier storage, mesh Arcs, query
/// pipelines, registry identity, dirty tiles and diffs belong to RaycastWorld.
#[derive(Debug, Default)]
pub struct PlatformStairColliderMap {
    #[cfg(feature="native-physics")]
    entries: std::collections::HashMap<WallId, ColliderMeshInstance>,
}
impl PlatformStairColliderMap {
    pub fn len(&self) -> usize {
        #[cfg(feature="native-physics")] { self.entries.len() }
        #[cfg(not(feature="native-physics"))] { 0 }
    }
    pub fn is_empty(&self) -> bool { self.len()==0 }
    #[cfg(feature="native-physics")]
    pub fn instance(&self,wall:WallId) -> Option<ColliderMeshInstance> {self.entries.get(&wall).copied()}
}
impl RaycastWorld {
    #[cfg(feature="native-physics")]
    pub fn remove_platform_stair_collider(&mut self, map:&mut PlatformStairColliderMap,wall:WallId)
        -> Option<SubworldColliderHandle> {
        remove_collider(self,&mut map.entries,&wall)
    }
    #[cfg(not(feature="native-physics"))]
    pub fn remove_platform_stair_collider(&mut self,_map:&mut PlatformStairColliderMap,_wall:WallId)
        -> Option<SubworldColliderHandle> { None }

    #[cfg(feature="native-physics")]
    pub fn update_platform_stair_collider(&mut self,map:&mut PlatformStairColliderMap,
        wall:WallId,geometry:&WallPlatformStairGeometry) -> Option<ColliderUpsert> {
        if geometry.cells.is_empty() {
            self.remove_platform_stair_collider(map,wall);
            return None;
        }
        let indexed=PlatformStairIndexedMesh::from_geometry(geometry);
        let positions:Vec<_>=indexed.positions.iter().copied().map(glam::Vec3::from_array).collect();
        let data=Arc::new(RaycastMeshData::new_indexed(&indexed.triangles,&positions,None,None));
        assert_eq!(data.hash,indexed.fingerprint,"platform indexed-mesh hash boundary");
        Some(upsert_collider_with_source_payload(self,&mut map.entries,wall,
            RaycastGeoSource::PlatformStair,Some(wall.0),data,glam::Vec3::ZERO,glam::Quat::IDENTITY))
    }
    #[cfg(not(feature="native-physics"))]
    pub fn update_platform_stair_collider(&mut self,_map:&mut PlatformStairColliderMap,
        _wall:WallId,geometry:&WallPlatformStairGeometry) {
        assert!(geometry.cells.is_empty(),"platform collider construction requires native-physics");
    }
}
