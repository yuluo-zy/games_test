
#[cfg(test)]
mod complete_builder_tests {
    use super::*;
    use crate::{asset_libraries::{mesh_library::MeshAsset, transient_mesh::TransientMesh},
        render::{mesh::Mesh, shader::RasterShaderProgram, test_device::TestDevice},
        systems::render_loop::draw_passes::draw_lists::{UiPass,WireframePass,DontCastShadow},
        utils::components::transform::Transform};
    use bevy_ecs::world::CommandQueue;
    use glam::{Vec3,Quat};
    fn libraries()->(AssetMeshLibrary,AssetShaderLibrary) {
        let meshes=AssetMeshLibrary::new();
        meshes.add(MeshAsset::new(Mesh::new("named"),Some("brush".to_owned())));
        let shaders=AssetShaderLibrary::new();
        shaders.add_raster(RasterShaderProgram::new_draw("vertex_color_shader",Default::default(),&TestDevice(1)),false);
        (meshes,shaders)
    }
    fn build(b:SignifierEntityBuilder,m:&AssetMeshLibrary,s:&AssetShaderLibrary,w:&mut World)->Entity {
        let mut queue=CommandQueue::default();
        let entity=b.build(m,s,&mut Commands::new(&mut queue,w));
        queue.apply(w);entity
    }
    #[test]
    fn named_mesh_resolves_real_handles_and_duration_zero_becomes_visible() {
        let (meshes,shaders)=libraries();let mut world=World::new();
        let rotation=Quat::from_rotation_y(0.7);let position=Vec3::new(1.,2.,3.);
        let entity=build(SignifierEntityBuilder::from_mesh_name("brush").animation_duration(0.0).rotation(rotation).position(position).scale(1.8),&meshes,&shaders,&mut world);
        let transform=world.get::<Transform>(entity).unwrap();
        assert_eq!((transform.rotation,transform.translation,transform.scale),(rotation,position,Vec3::ZERO));
        assert_eq!(*world.get::<crate::asset_libraries::Handle<Mesh>>(entity).unwrap(),meshes.get_handle_by_name("brush"));
        assert!(world.get::<UiPass>(entity).is_some());assert!(world.get::<DontCastShadow>(entity).is_some());
        assert!(world.get::<TransientMesh>(entity).is_none());
        assert!(world.get::<super::super::signifier_create::SignifierId>(entity).is_none());
        let mut anim=world.get_mut::<SignifierAnimation>(entity).unwrap();anim.advance(0.,false);
        assert_eq!((anim.t,anim.eased,anim.value),(1.,1.,1.8));
    }
    #[test]
    fn every_owned_mesh_branch_keeps_its_actual_transient_receipt() {
        let (meshes,shaders)=libraries();let mut world=World::new();
        let points=vec![Vec3::ZERO,Vec3::X];
        let sources=[SignifierMesh::Empty,SignifierMesh::Points(points.clone()),
            SignifierMesh::PointsWithColors{points:points.clone(),colors:vec![Vec3::Y,Vec3::Z]},
            SignifierMesh::Generated(Box::new(move ||points.clone()))];
        for (source,label) in sources.into_iter().zip(["signifier empty","signifier custom","signifier custom w colors","signifier custom fn"]) {
            let entity=build(SignifierEntityBuilder::new(source).disable_default_animation().scale(2.).set_ui_pass(0),&meshes,&shaders,&mut world);
            let transient=world.get::<TransientMesh>(entity).unwrap();let handle=transient.mesh_handle();
            assert_eq!(meshes.get(handle).unwrap().label,label);
            assert_eq!(world.get::<Transform>(entity).unwrap().scale,Vec3::splat(2.));
            assert!(world.get::<UiPass>(entity).is_none());
            world.despawn(entity);
            assert_eq!(meshes.transient_meshes().lock().unwrap().last(),Some(&handle));
        }
    }
    #[test]
    fn wireframe_precedes_ui_and_lifespan_parent_components_are_real() {
        let (meshes,shaders)=libraries();let mut world=World::new();
        let entity=build(SignifierEntityBuilder::from_mesh_name("brush").mark_as_debug().minimum_lifespan(0.7).parent_to_widget(),&meshes,&shaders,&mut world);
        assert!(world.get::<WireframePass>(entity).is_some());assert!(world.get::<UiPass>(entity).is_none());
        assert!(world.get::<super::super::animation::SignifierParentToVolumeOrigin>(entity).is_some());
        let life=world.get::<super::super::signifier_mark_for_delete::SignifierMinimumLifespan>(entity).unwrap();
        assert_eq!((life.t,life.min_lifespan),(0.,0.7));
    }
}
