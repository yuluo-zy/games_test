//! Original wall/roof color picker, B 0x141c9ab70 and cursor factory 0x141c9b510.
use std::sync::Arc;
use bevy_ecs::prelude::*;
use country_core::{
    asset_libraries::{mesh_library::AssetMeshLibrary, shader_library::AssetShaderLibrary},
    components::{Ui2dExtent, Ui2dImage, Ui2DFollowMouse, UiInactive},
    resources::{mouse_terrain_raycast::{CameraCursorRay, CursorPositionSS}, walls::WallId},
    systems::{collision::world::RaycastWorld, mode_manager::AppMode,
        ui::{resources::{ui_state::UiState, widget_params::{UiSounds, WidgetGeometry, WidgetParams, WidgetVolumeShape}},
            signifiers::{SignifierEntityBuilder, SignifierFactory, signifier_factory::SignifierMesh}},
        wall::structure_outline::OutlineWallCmd},
};
use crate::{radial_menu::ColorRadialMenu, HideColorMenuCmd, ShowColorMenuCmd};

/// The two original factories capture the cursor position and call this helper
/// with active=false for the canvas and active=true for the wall widget.
fn color_cursor_icon(position: glam::Vec2, active: bool, meshes: &AssetMeshLibrary,
    shaders: &AssetShaderLibrary, commands: &mut Commands) -> Entity {
    let entity = SignifierEntityBuilder::new(SignifierMesh::Empty)
        .animation_duration(0.0).build(meshes, shaders, commands);
    let mut target = commands.entity(entity);
    if !active { target.insert(UiInactive); }
    target.insert((Ui2dExtent { pos: position, size: glam::Vec2::splat(60.0) },
        Ui2dImage::new("color_icon/color_cursor"), Ui2DFollowMouse));
    entity
}

fn cursor_factory(position: glam::Vec2, active: bool) -> SignifierFactory {
    cursor_factory_named("my_signifier", position, active)
}

pub(crate) fn cursor_factory_named(name: &str, position: glam::Vec2, active: bool) -> SignifierFactory {
    SignifierFactory::from_closure(name, Arc::new(move |meshes, shaders, commands| {
        vec![color_cursor_icon(position, active, meshes, shaders, commands)]
    }))
}

/// The registry result's first 24 bytes are the native collider payload. Its
/// tag is byte 0, the roof subtype is byte 1, and the wall id is the qword at 8.
#[allow(clippy::too_many_arguments)]
pub fn ui_color_wall_and_roof(
    mode: Res<AppMode>, ui: Res<UiState>, cursor: Res<CursorPositionSS>,
    mut shows: EventWriter<ShowColorMenuCmd>, mut hides: EventWriter<HideColorMenuCmd>,
    menus: Query<&ColorRadialMenu>, mut outlines: EventWriter<OutlineWallCmd>,
    world: Res<RaycastWorld>, ray: Res<CameraCursorRay>,
) {
    if mode.native_tag() != 12 { return; }
    let canvas = WidgetParams::new(WidgetVolumeShape::clearing_canvas(3), [0; 2])
        .sounds(UiSounds::empty()).occlude_by_world(false)
        .with_signifiers(vec![cursor_factory(cursor.0, false)]);
    if ui.add("color_canvas", canvas).clicked() {
        hides.write(HideColorMenuCmd { cancel: true });
    }
    let hit = world.raycast(ray.origin, ray.dir, 0xcb);
    if !hit.is_hit() { return; }
    let category = match hit.handle[0] as u8 {
        0 | 6 | 7 => 3,
        2 => ((hit.handle[0] >> 8) as u8) & 1,
        8 => 2,
        _ => return,
    };
    let wall = WallId(hit.handle[1]);
    let menu_open = !menus.is_empty();
    let mut sounds = UiSounds::empty();
    sounds.slots[0][..4].copy_from_slice(&u32::from(!menu_open).to_le_bytes());
    sounds.slots[3][..4].copy_from_slice(&8u32.to_le_bytes());
    let distance = (ray.origin - hit.position).length();
    let mut params = WidgetParams::default().geometry(WidgetGeometry::TerrainCanvas { distance, layer: 0 })
        .sounds(sounds).with_signifiers(vec![cursor_factory(cursor.0, true)]);
    params.flags.0 |= 2;
    let widget = ui.add(&format!("color_wall_{wall}"), params);
    if !menu_open && widget.is_active() {
        outlines.write(OutlineWallCmd { wall, strength: 1.0, kind: u8::from(category > 1) });
    }
    if widget.clicked() {
        hides.write(HideColorMenuCmd { cancel: false });
        let kind = match category { 2 => 8, 3 => 0, other => (other & 1) + 1 };
        shows.write(ShowColorMenuCmd { kind, subtype: 0, decorator_id: 0, payload: wall.0, position: cursor.0 });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use country_core::{input::action_set::{ActionSetStates, DigitalActionPhase}, resources::{
        cursor_terrain_raycast::CursorTerrainRaycast, events::{EscapeMenuJustOpened, ShutdownGameCmd},
        mouse_capture::OsCursorVisibilityCmd, read_mouse_depth_value::MouseDepthValue,
        window::{WindowFocusEvent, WindowSize}}, systems::{brush_preview::OverwriteBrushPreview,
        camera_rig::MainCamera, ui::{hover_intersection::hover_intersection, ui_update::ui_update,
            resources::widget_params::PlayUiSound}}};
    use glam::{Mat4, Vec2};

    #[test]
    fn real_canvas_picker_click_cancels_color_menu() {
        bevy_tasks::ComputeTaskPool::get_or_init(|| bevy_tasks::TaskPoolBuilder::new().num_threads(2).build());
        let mut world = World::new();
        world.insert_resource(AppMode::Color);
        world.insert_resource(UiState::at_startup());
        world.insert_resource(CursorPositionSS(Vec2::splat(50.0)));
        world.insert_resource(CameraCursorRay::default());
        world.insert_resource(RaycastWorld::default());
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
        world.init_resource::<Events<OutlineWallCmd>>();
        let mut schedule = Schedule::default();
        schedule.add_systems((ui_color_wall_and_roof, hover_intersection, ui_update).chain());
        for phase in [DigitalActionPhase::Idle, DigitalActionPhase::JustPressed, DigitalActionPhase::JustReleased] {
            world.resource_mut::<ActionSetStates>().digital[0].phase = phase;
            schedule.run(&mut world);
        }
        // The producer consumes the click state on the next update, as in the
        // real set ordering, after ui_update publishes the released widget.
        world.resource_mut::<ActionSetStates>().digital[0].phase = DigitalActionPhase::Idle;
        schedule.run(&mut world);
        let events = world.resource_mut::<Events<HideColorMenuCmd>>().drain().collect::<Vec<_>>();
        assert_eq!(events.len(), 1);
        assert!(events[0].cancel);
        assert!(world.resource::<Events<ShowColorMenuCmd>>().is_empty());
    }
}
