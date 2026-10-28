//! B circle-edit arc signifier (0x140a64a40).
use bevy_ecs::prelude::*;
use std::sync::Arc;
use ::utils::geometry::Circle2d;
use crate::asset_libraries::mesh_library::AssetMeshLibrary;
use crate::render::mesh::Mesh;
use crate::asset_libraries::Handle;
use crate::systems::ui::signifiers::animation::SignifierAnimation;
use crate::systems::ui::signifiers::signifier_create::UiSignifierStream;
use crate::systems::ui::signifiers::signifier_factory::{SignifierEntityBuilder, SignifierFactory, SignifierMesh};
use crate::systems::wall::shapes::circle::ui_edit_circle::AnimatedCircleSignifierResource;

/// Native line component: circle at +0, retained data at +0x10, total 0x28.
#[derive(Component, Clone, Copy, Debug)]
pub struct SignifierEditCircleShapeLine {
    pub circle: Circle2d,
    pub retained: [u8; 24],
}

pub fn register_required_components() -> super::edit_arrow::ComponentRegistration {
    super::edit_arrow::ComponentRegistration {
        size: 0x28,
        type_id: (0x7c65f1f941b3ec27, 0x92748f8a36511053),
        name: "country_core::systems::wall::shapes::circle::signifiers::edit_line::SignifierEditCircleShapeLine",
    }
}

fn current_circle(resource: &AnimatedCircleSignifierResource) -> Circle2d {
    let mut circle = Circle2d::new(glam::Vec2::ZERO, resource.angle_base + 0.23);
    circle.center = -circle.get_pos_at_u(0.75);
    circle
}

/// B 0x140a64a40: create only for circle; update all existing line entities.
pub fn signifier_edit_circle_line(
    mut query: Query<(&mut SignifierEditCircleShapeLine, &Handle<Mesh>, &SignifierAnimation)>,
    resource: Res<AnimatedCircleSignifierResource>,
    mut meshes: ResMut<AssetMeshLibrary>,
    mut stream: ResMut<UiSignifierStream>,
) {
    if resource.tag != 2 && resource.tag & 1 == 0 {
        let circle = current_circle(&resource);
        let mut retained = [0u8; 24];
        retained[..8].copy_from_slice(&resource.unknown_00[8..16]);
        let id = u64::from_le_bytes(resource.unknown_00[..8].try_into().unwrap());
        let name = format!("edit_circle_{:?}_line", crate::systems::ui::resources::ui_state::WidgetId(id));
        let mut factory = SignifierFactory::from_closure(&name, Arc::new(move |meshes, shaders, commands| {
            let entity = SignifierEntityBuilder::new(SignifierMesh::Empty)
                .animation_duration(0.3).custom_shader("vertex_color_shader")
                .build(meshes, shaders, commands);
            commands.entity(entity).insert((SignifierEditCircleShapeLine { circle, retained }, crate::render::pass_builder::draw_list::DrawTopology::Alt(3.0)));
            vec![entity]
        }));
        factory.marker = 0x142ad4518;
        stream.write_factory(&factory);
    }
    for (mut line, handle, animation) in &mut query {
        let s = animation.eased * 0.1 + (1.0 - animation.eased) * 0.01;
        let start = 0.75 - s;
        let end = 0.75 + s;
        assert!((0.0..=1.0).contains(&start) && (0.0..=1.0).contains(&end), "invalid CurveU");
        if resource.tag != 2 { line.circle = current_circle(&resource); }
        let curve = line.circle.as_curve2_range(0.2, start, end);
        let height = f32::from_le_bytes(line.retained[12..16].try_into().unwrap());
        crate::utils::geo::update_curve_mesh(meshes.get_mut(*handle), curve.points.into_iter().map(|p| crate::utils::geo::x_h_y(p, height)));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::asset_libraries::mesh_library::MeshAsset;
    use glam::Vec2;

    #[test]
    fn real_schedule_updates_mesh_and_preserves_fallback_circle() {
        let mut world = World::new();
        let meshes = AssetMeshLibrary::default();
        let handle = meshes.add(MeshAsset::new(Mesh::new("circle line test"), None));
        world.insert_resource(meshes);
        world.init_resource::<UiSignifierStream>();
        let mut resource = AnimatedCircleSignifierResource::default();
        resource.tag = 0;
        resource.angle_base = 2.0;
        world.insert_resource(resource);
        let mut retained = [42; 24];
        retained[12..16].copy_from_slice(&2.5f32.to_le_bytes());
        let line = SignifierEditCircleShapeLine { circle: Circle2d::new(Vec2::ZERO, 1.0), retained };
        let entity = world.spawn((line, handle, SignifierAnimation { eased: 0.5, ..Default::default() })).id();
        let mut schedule = Schedule::default();
        schedule.add_systems(signifier_edit_circle_line);
        schedule.run(&mut world);
        let expected = current_circle(&resource);
        let line = world.get::<SignifierEditCircleShapeLine>(entity).unwrap();
        assert_eq!(line.circle, expected);
        assert_eq!(line.retained, retained);
        let points = expected.as_curve2_range(0.2, 0.695, 0.805).points;
        let mesh = world.resource::<AssetMeshLibrary>().get(handle).unwrap();
        assert_eq!(mesh.positions_without_indices(), points.into_iter().map(|p| crate::utils::geo::x_h_y(p, 2.5)).collect::<Vec<_>>());
        assert_eq!(world.resource::<UiSignifierStream>().committed_len(), 1);
        use crate::systems::ui::signifiers::UiSignifierEntry;
        use crate::asset_libraries::shader_library::AssetShaderLibrary;
        use crate::render::{shader::RasterShaderProgram, test_device::TestDevice};
        let entries = world.resource_mut::<UiSignifierStream>().take_entries();
        let UiSignifierEntry::Factory(factory) = &entries[0] else { panic!("line factory") };
        assert_eq!(factory.name(), Some("edit_circle_WidgetId(0)_line"));
        assert_eq!(factory.marker, 0x142ad4518);
        assert_eq!(std::mem::size_of::<SignifierEditCircleShapeLine>(), 0x28);
        let shaders = AssetShaderLibrary::new();
        shaders.add_raster(RasterShaderProgram::new_draw("vertex_color_shader", Default::default(), &TestDevice(1)), false);
        let factory_meshes = AssetMeshLibrary::default();
        let mut spawned = World::new();
        let mut queue = bevy_ecs::world::CommandQueue::default();
        let mut commands = Commands::new(&mut queue, &spawned);
        let spawned_entity = factory.wake(&factory_meshes, &shaders, &mut commands)[0];
        drop(commands); queue.apply(&mut spawned);
        assert_eq!(spawned.get::<SignifierEditCircleShapeLine>(spawned_entity).unwrap().circle, expected);
        assert_eq!(spawned.get::<SignifierAnimation>(spawned_entity).unwrap().duration, 0.3);
        assert_eq!(*spawned.get::<crate::render::pass_builder::draw_list::DrawTopology>(spawned_entity).unwrap(), crate::render::pass_builder::draw_list::DrawTopology::Alt(3.0));
        world.resource_mut::<AnimatedCircleSignifierResource>().tag = 2;
        world.resource_mut::<AnimatedCircleSignifierResource>().angle_base = 9.0;
        schedule.run(&mut world);
        assert_eq!(world.get::<SignifierEditCircleShapeLine>(entity).unwrap().circle, expected);
        assert_eq!(world.resource::<UiSignifierStream>().committed_len(), 0);
        world.resource_mut::<AnimatedCircleSignifierResource>().tag = 1;
        schedule.run(&mut world);
        assert_eq!(world.resource::<UiSignifierStream>().committed_len(), 0);
        assert_eq!(world.get::<SignifierEditCircleShapeLine>(entity).unwrap().circle.radius, 9.23);
    }
}
