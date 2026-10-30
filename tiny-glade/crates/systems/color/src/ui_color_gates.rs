//! B1422afe70: aggregate each wall's live wooden gate meshes into one picker.
use bevy_ecs::prelude::*;
use std::sync::Arc;
use country_core::{asset_libraries::{mesh_library::AssetMeshLibrary, Handle},
    components::WoodenGate, render::mesh::Mesh,
    resources::{mouse_terrain_raycast::CursorPositionSS, walls::wooden_gates::Gates},
    systems::{mode_manager::AppMode, ui::{resources::{ui_state::UiState,
        widget_params::{WidgetParams, WidgetVolumeShape}}, signifiers::{UiSignifierStream,
        SignifierFactory, SignifierEntityBuilder}}}};
use crate::{HideColorMenuCmd, ShowColorMenuCmd};

pub fn ui_color_gates(mode: Res<AppMode>, ui: Res<UiState>, mut signifiers: ResMut<UiSignifierStream>,
    gates: Res<Gates>, mesh_handles: Query<&Handle<Mesh>, With<WoodenGate>>,
    gate_components: Query<&WoodenGate>, meshes: Res<AssetMeshLibrary>,
    mut shows: EventWriter<ShowColorMenuCmd>, mut hides: EventWriter<HideColorMenuCmd>,
    cursor: Res<CursorPositionSS>) {
    if mode.native_tag() != 12 { return; }
    for (wall, entities) in &gates.per_wall {
        let mut triangles = Vec::new();
        for bits in entities {
            let entity = Entity::from_bits(*bits);
            let Ok(handle) = mesh_handles.get(entity) else { continue; };
            let mesh = meshes.get(*handle).expect("original gate mesh is missing");
            let (indices, positions) = mesh.positions();
            for indices in indices.chunks_exact(3) {
                triangles.push(utils::geometry::intersect::Triangle::new(
                    positions[indices[0] as usize], positions[indices[1] as usize], positions[indices[2] as usize]));
            }
        }
        let widget = ui.add(&format!("color_gates_{wall}"),
            WidgetParams::new(WidgetVolumeShape::default(), [0; 2]).mesh_volume(triangles));
        if widget.is_active() {
            signifiers.write_factory(&crate::ui_color_wall_and_roof::cursor_factory_named(
                "color_icon_color_gate", cursor.0, true));
            for (index, bits) in entities.iter().enumerate() {
                let Ok(gate) = gate_components.get(Entity::from_bits(*bits)) else { continue; };
                let points: Vec<_> = gate.top_positions.iter().copied().map(glam::Vec3::from_array).collect();
                let name = format!("highlight_gate{wall}{index}");
                let factory = SignifierFactory::from_closure(&name, Arc::new(move |meshes, shaders, commands| {
                    let entity = SignifierEntityBuilder::from_points(points.clone())
                        .animation_duration(0.0).build(meshes, shaders, commands);
                    commands.entity(entity).insert(country_core::render::pass_builder::draw_list::DrawTopology::Alt(3.0));
                    vec![entity]
                }));
                signifiers.write_factory(&factory);
            }
        }
        if widget.clicked() {
            hides.write(HideColorMenuCmd { cancel: false });
            shows.write(ShowColorMenuCmd { kind: 5, subtype: 0, decorator_id: 0, payload: wall.0, position: cursor.0 });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use country_core::{asset_libraries::mesh_library::MeshAsset, render::mesh::ATTR_POSITION,
        input::action_set::{ActionSetStates, DigitalActionPhase}, resources::{
        cursor_terrain_raycast::CursorTerrainRaycast, events::{EscapeMenuJustOpened, ShutdownGameCmd},
        mouse_capture::OsCursorVisibilityCmd, read_mouse_depth_value::MouseDepthValue,
        window::{WindowFocusEvent, WindowSize}}, systems::{
        brush_preview::OverwriteBrushPreview, camera_rig::MainCamera,
        ui::{hover_intersection::hover_intersection, ui_update::ui_update,
        resources::widget_params::PlayUiSound}}};
    use glam::{Mat4, Vec2, Vec3};

    #[test]
    fn real_mesh_picker_click_opens_gate_color_menu() {
        bevy_tasks::ComputeTaskPool::get_or_init(|| bevy_tasks::TaskPoolBuilder::new().num_threads(2).build());
        let mut world = World::new();
        let library = AssetMeshLibrary::new();
        let mut mesh = Mesh::new("gate picker test");
        mesh.set_indices(vec![0, 1, 2]);
        mesh.set_attribute(ATTR_POSITION, vec![Vec3::new(-1.0, -1.0, 0.5), Vec3::new(1.0, -1.0, 0.5), Vec3::new(0.0, 1.0, 0.5)]);
        let handle = library.add(MeshAsset::new(mesh, None));
        let wall = country_core::resources::walls::WallId(7);
        let entity = world.spawn((handle, WoodenGate { top_positions: vec![[0.0, 1.0, 0.5], [1.0, 1.0, 0.5]], wall, endpoint_directions: [[0.0; 2]; 2], seed: 0, clutter_version: 0 })).id();
        let mut gates = Gates::default();
        gates.for_wall_mut(wall).push(entity.to_bits());
        world.insert_resource(gates);
        world.insert_resource(library);
        world.insert_resource(AppMode::Color);
        world.insert_resource(UiState::at_startup());
        world.init_resource::<UiSignifierStream>();
        world.insert_resource(CursorPositionSS(Vec2::splat(50.0)));
        let mut camera = MainCamera::new(1.0);
        for offset in [0x40, 0x80, 0xc0] { camera.write_matrix(offset, &Mat4::IDENTITY); }
        world.insert_resource(camera);
        world.insert_resource(CursorTerrainRaycast::default());
        world.insert_resource(MouseDepthValue::default());
        world.insert_resource(WindowSize::new(100, 100));
        world.insert_resource(ActionSetStates::default());
        world.init_resource::<Events<WindowFocusEvent>>();
        world.init_resource::<Events<EscapeMenuJustOpened>>();
        world.init_resource::<Events<ShutdownGameCmd>>();
        world.init_resource::<Events<OverwriteBrushPreview>>();
        world.init_resource::<Events<OsCursorVisibilityCmd>>();
        world.init_resource::<Events<PlayUiSound>>();
        world.init_resource::<Events<ShowColorMenuCmd>>();
        world.init_resource::<Events<HideColorMenuCmd>>();
        let mut schedule = Schedule::default();
        schedule.add_systems((ui_color_gates, hover_intersection, ui_update).chain());
        for phase in [DigitalActionPhase::Idle, DigitalActionPhase::JustPressed, DigitalActionPhase::JustReleased, DigitalActionPhase::Idle] {
            world.resource_mut::<ActionSetStates>().digital[0].phase = phase;
            schedule.run(&mut world);
        }
        let shows = world.resource_mut::<Events<ShowColorMenuCmd>>().drain().collect::<Vec<_>>();
        assert_eq!(shows.len(), 1);
        assert_eq!(shows[0].kind, 5);
        assert_eq!(shows[0].subtype, 0);
        assert_eq!(shows[0].decorator_id, 0);
        assert_eq!(shows[0].payload, wall.0);
        assert_eq!(shows[0].position, Vec2::splat(50.0));
        let hides = world.resource_mut::<Events<HideColorMenuCmd>>().drain().collect::<Vec<_>>();
        assert_eq!(hides.len(), 1);
        assert!(!hides[0].cancel);
    }
}
