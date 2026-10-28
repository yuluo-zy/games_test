//! B rectangle edge signifier (0x1408dda10).
use bevy_ecs::prelude::*;
use crate::asset_libraries::{Handle, mesh_library::AssetMeshLibrary};
use crate::render::mesh::Mesh;
use crate::systems::ui::signifiers::{animation::SignifierAnimation, signifier_create::UiSignifierStream};
use crate::utils::components::transform::Transform;
use super::ui_edit_rectangle_dims::UiEditRectangleActiveWidgetResource;
use crate::systems::ui::signifiers::signifier_factory::{SignifierEntityBuilder, SignifierFactory, SignifierMesh};
use std::sync::Arc;

/// B EditRectangleLinePreview owns a complete Curve2 (56 bytes).
#[derive(Component, Clone, Debug)]
pub struct EditRectangleLinePreview(pub ::utils::curve::Curve2);

fn animated_interval(eased: f32) -> (f32, f32) {
    let s = eased * 0.5 + (1.0 - eased) * 0.01;
    let start = 0.5 - s;
    let end = 0.5 + s;
    assert!((0.0..=1.0).contains(&start) && (0.0..=1.0).contains(&end), "invalid CurveU");
    (start, end)
}

pub fn signifier_edit_dims(
    active: Res<UiEditRectangleActiveWidgetResource>,
    mut stream: ResMut<UiSignifierStream>,
    mut query: Query<(&EditRectangleLinePreview, &Handle<Mesh>, &SignifierAnimation, &mut Transform)>,
    mut meshes: ResMut<AssetMeshLibrary>,
) {
    if let Some(edge) = &active.0 {
        let curve = edge.curve.clone();
        let position = edge.position;
        let name = format!("rectangle_edit_dims_line_WidgetId {{ hash: {} }}", edge.widget.0);
        let mut factory = SignifierFactory::from_closure(&name, Arc::new(move |meshes, shaders, commands| {
            let entity = SignifierEntityBuilder::new(SignifierMesh::Empty)
                .position(position).animation_duration(0.3).custom_shader("vertex_color_shader")
                .build(meshes, shaders, commands);
            commands.entity(entity).insert((EditRectangleLinePreview(curve.clone()), crate::render::pass_builder::draw_list::DrawTopology::Alt(3.0)));
            vec![entity]
        }));
        factory.marker = 0x142aa4cb0;
        stream.write_factory(&factory);
    }
    for (line, handle, animation, mut transform) in &mut query {
        if let Some(active) = &active.0 { transform.translation = active.position; }
        let (start, end) = animated_interval(animation.eased);
        let curve = line.0.try_slice(start, end).expect("rectangle signifier slice");
        crate::utils::geo::update_curve_mesh(meshes.get_mut(*handle), curve.points.into_iter().map(crate::utils::geo::x0y));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::asset_libraries::mesh_library::MeshAsset;
    use glam::Vec2;

    #[test]
    fn animated_interval_checks_endpoints_and_unordered_values() {
        assert_eq!(animated_interval(1.0), (0.0, 1.0));
        assert_eq!(animated_interval(0.0), (0.49, 0.51));
        for eased in [f32::NAN, f32::INFINITY, 2.0] {
            assert!(std::panic::catch_unwind(|| animated_interval(eased)).is_err());
        }
    }

    #[test]
    fn real_schedule_updates_rectangular_line_mesh() {
        let mut world = World::new();
        let meshes = AssetMeshLibrary::default();
        let handle = meshes.add(MeshAsset::new(Mesh::new("rectangle line"), None));
        world.insert_resource(meshes); world.init_resource::<UiSignifierStream>();
        world.init_resource::<UiEditRectangleActiveWidgetResource>();
        let curve = ::utils::curve::Curve2::try_new(vec![Vec2::new(-2.0, 0.0), Vec2::new(2.0, 0.0)], false).unwrap();
        let entity = world.spawn((EditRectangleLinePreview(curve), handle, SignifierAnimation { eased: 0.5, ..Default::default() }, Transform::default())).id();
        let mut schedule = Schedule::default(); schedule.add_systems(signifier_edit_dims); schedule.run(&mut world);
        let points = world.resource::<AssetMeshLibrary>().get(handle).unwrap().positions_without_indices();
        assert_eq!(points.len(), 2);
        assert!((points[0] - glam::Vec3::new(-1.02, 0.0, 0.0)).length() < 1.0e-6);
        assert!((points[1] - glam::Vec3::new(1.02, 0.0, 0.0)).length() < 1.0e-6);
        world.get_mut::<SignifierAnimation>(entity).unwrap().eased = 1.0;
        world.resource_mut::<UiEditRectangleActiveWidgetResource>().0 = Some(super::super::ui_edit_rectangle_dims::RectangleActiveEdge {
            curve: ::utils::curve::Curve2::try_new(vec![Vec2::ZERO, Vec2::X], false).unwrap(),
            widget: crate::systems::ui::resources::ui_state::WidgetId(7),
            position: glam::Vec3::new(4.0, 5.0, 6.0),
        });
        schedule.run(&mut world);
        assert_eq!(world.get::<Transform>(entity).unwrap().translation, glam::Vec3::new(4.0, 5.0, 6.0));
        assert_eq!(world.resource::<AssetMeshLibrary>().get(handle).unwrap().positions_without_indices(), vec![glam::Vec3::new(-2.0, 0.0, 0.0), glam::Vec3::new(2.0, 0.0, 0.0)]);
        use crate::systems::ui::signifiers::UiSignifierEntry;
        use crate::asset_libraries::shader_library::AssetShaderLibrary;
        use crate::render::{shader::RasterShaderProgram, test_device::TestDevice};
        let entries = world.resource_mut::<UiSignifierStream>().take_entries();
        let UiSignifierEntry::Factory(factory) = &entries[0] else { panic!("rectangle line factory") };
        assert_eq!(factory.name(), Some("rectangle_edit_dims_line_WidgetId { hash: 7 }"));
        assert_eq!(factory.marker, 0x142aa4cb0);
        let shaders = AssetShaderLibrary::new();
        shaders.add_raster(RasterShaderProgram::new_draw("vertex_color_shader", Default::default(), &TestDevice(1)), false);
        let meshes = AssetMeshLibrary::default(); let mut spawned = World::new();
        let mut queue = bevy_ecs::world::CommandQueue::default(); let mut commands = Commands::new(&mut queue, &spawned);
        let child = factory.wake(&meshes, &shaders, &mut commands)[0];
        drop(commands); queue.apply(&mut spawned);
        assert_eq!(spawned.get::<EditRectangleLinePreview>(child).unwrap().0.points, vec![Vec2::ZERO, Vec2::X]);
        assert_eq!(spawned.get::<Transform>(child).unwrap().translation, glam::Vec3::new(4.0, 5.0, 6.0));
        assert_eq!(spawned.get::<SignifierAnimation>(child).unwrap().duration, 0.3);
        assert_eq!(*spawned.get::<crate::render::pass_builder::draw_list::DrawTopology>(child).unwrap(), crate::render::pass_builder::draw_list::DrawTopology::Alt(3.0));
        // A zero-length reconstructed slice must fail rather than quietly emit a mesh.
        world.get_mut::<EditRectangleLinePreview>(entity).unwrap().0.points = vec![Vec2::ZERO, Vec2::ZERO];
        assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| schedule.run(&mut world))).is_err());
    }
}
