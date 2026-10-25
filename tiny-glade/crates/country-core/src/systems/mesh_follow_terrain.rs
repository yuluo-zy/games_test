//! B140a68430/1556B and indexed map/collect1412fd550/496B.
//! This rewrites mesh vertices through the owning library, not entity transforms.
use crate::{
    components::MeshFollowTerrain,
    asset_libraries::{Handle, mesh_library::AssetMeshLibrary},
    render::mesh::{ATTR_POSITION, Mesh},
    resources::terrain_height::TerrainHeightsData,
};
use bevy_ecs::prelude::*;
pub fn mesh_follow_terrain(
    query: Query<&Handle<Mesh>, With<MeshFollowTerrain>>,
    mut meshes: ResMut<AssetMeshLibrary>,
    terrain: Res<TerrainHeightsData>,
) {
    for handle in &query {
        let mesh = meshes
            .get_mut(*handle)
            .expect("MeshFollowTerrain got an invalid mesh handle");
        let (indices, positions) = mesh.positions();
        let positions: Vec<glam::Vec3> = indices
            .iter()
            .map(|&index| {
                let p = positions[index as usize];
                glam::Vec3::new(p.x, terrain.sample_at_world_xz(p.x, p.z), p.z)
            })
            .collect();
        mesh.set_attribute(ATTR_POSITION, positions);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        asset_libraries::mesh_library::MeshAsset,
        render::mesh::{ATTR_COLOR, VertexAttributeValues},
        resources::terrain_height::TerrainHeightSsbo,
    };
    #[test]
    fn indexed_order_real_terrain_sampling_and_original_dirty_receipts() {
        let mut mesh = Mesh::new("follow");
        mesh.indices = vec![2, 0, 2, 1];
        let old = vec![
            glam::Vec3::new(-3., 8., 2.),
            glam::Vec3::new(4., 9., -5.),
            glam::Vec3::new(7., 10., 6.),
        ];
        mesh.set_attribute(ATTR_POSITION, old.clone());
        mesh.set_attribute(ATTR_COLOR, vec![glam::Vec3::ONE; 3]);
        let library = AssetMeshLibrary::new();
        let handle = library.add(MeshAsset::new(mesh, Some("follow".to_owned())));
        let mut terrain = TerrainHeightsData::<TerrainHeightSsbo>::new();
        terrain.set_height_range(0., 65535.);
        for (i, h) in terrain.heights_mut().iter_mut().enumerate() {
            *h = (i % 512) as u16;
        }
        let expected: Vec<_> = [2, 0, 2, 1]
            .map(|i| {
                let p = old[i];
                glam::Vec3::new(p.x, terrain.sample_at_world_xz(p.x, p.z), p.z)
            })
            .into_iter()
            .collect();
        let mut world = World::new();
        world.insert_resource(library);
        world.insert_resource(terrain);
        world.spawn((handle, MeshFollowTerrain));
        let mut schedule = Schedule::default();
        schedule.add_systems(mesh_follow_terrain);
        schedule.run(&mut world);
        let library = world.resource::<AssetMeshLibrary>();
        assert_eq!(library.modified_meshes, vec![handle]);
        let mesh = library.get(handle).unwrap();
        assert_eq!(mesh.indices, [2, 0, 2, 1]);
        assert_eq!(mesh.positions_without_indices(), expected);
        assert!(
            matches!(mesh.attributes.get(ATTR_COLOR),Some(VertexAttributeValues::Float3(v)) if v.len()==3)
        );
    }
    #[test]
    #[should_panic(expected = "MeshFollowTerrain got an invalid mesh handle")]
    fn invalid_handle_panics_in_actual_query() {
        let mut world = World::new();
        world.insert_resource(AssetMeshLibrary::new());
        world.insert_resource(TerrainHeightsData::<TerrainHeightSsbo>::new());
        world.spawn((Handle::<Mesh>::new(), MeshFollowTerrain));
        let mut schedule = Schedule::default();
        schedule.add_systems(mesh_follow_terrain);
        schedule.run(&mut world);
    }
}
