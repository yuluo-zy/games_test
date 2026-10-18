/// Terrain-tool endpoint of B `ui_edit_focus` @0x1409771f0. The existing
/// GeoSource converters above retain their separate contract. Other selection
/// branches remain with their current owners; this restores the real terrain
/// right-click producer, rather than deriving completion from mode changes.
fn terrain_focus_input_enabled(
    mode: AppMode,
    navigation: &crate::systems::ui::resources::widget_params::UiNavState,
    actions: &crate::input::action_set::MainActionSet,
    hovered: &crate::systems::terrain::terrain_hover_idx::HoveredTerrainStrokeId,
) -> bool {
    mode == AppMode::Terrain
        && !(navigation.focus.is_none() && actions.digital[18].just_pressed())
        && actions.digital[1].just_pressed()
        && hovered.tag & 1 != 0
}

/// B @1409779bf and @140977a40 sum y²+x² before z². Keep the scalar order:
/// Vec3::distance may choose a different dot-product reduction.
fn focus_distance(point: glam::Vec3, origin: glam::Vec3) -> f32 {
    let y = point.y - origin.y;
    let x = point.x - origin.x;
    let z = point.z - origin.z;
    ((y * y + x * x) + z * z).sqrt()
}

fn terrain_focus_target(
    camera: &crate::resources::mouse_terrain_raycast::CameraCursorRay,
    cursor: &crate::resources::cursor_terrain_raycast::CursorTerrainRaycast,
    hovered: &crate::systems::terrain::terrain_hover_idx::HoveredTerrainStrokeId,
    geometry: &crate::systems::collision::world::RaycastSimpleResult,
) -> Option<u32> {
    let geometry_distance = if geometry.is_miss() {
        f32::MAX
    } else {
        focus_distance(geometry.position, camera.origin)
    };
    // Native position() returns ZERO for an absent cursor; it does not reject
    // that branch. Ordered strict comparison rejects equal distances and NaNs.
    let terrain_distance = focus_distance(cursor.position(), camera.origin);
    (geometry_distance > terrain_distance && hovered.tag == 1).then_some(hovered.id as u32)
}

/// Uses the real input, UI, collider query and terrain hover resources. Optional
/// host resources defer the system until their actual owners install them;
/// there is no fabricated cursor/raycast/action fallback.
pub fn ui_edit_focus_terrain(
    mode: bevy_ecs::prelude::Res<AppMode>,
    actions: Option<bevy_ecs::prelude::Res<crate::input::action_set::MainActionSet>>,
    ui: Option<bevy_ecs::prelude::Res<crate::systems::ui::resources::ui_state::UiState>>,
    navigation: Option<bevy_ecs::prelude::Res<crate::systems::ui::resources::widget_params::UiNavState>>,
    camera: Option<bevy_ecs::prelude::Res<crate::resources::mouse_terrain_raycast::CameraCursorRay>>,
    collision: Option<bevy_ecs::prelude::Res<crate::systems::collision::world::RaycastWorld>>,
    hovered: bevy_ecs::prelude::Res<crate::systems::terrain::terrain_hover_idx::HoveredTerrainStrokeId>,
    cursor: Option<bevy_ecs::prelude::Res<crate::resources::cursor_terrain_raycast::CursorTerrainRaycast>>,
    mut requests: bevy_ecs::event::EventWriter<crate::systems::mode_manager::RequestAppModeChange>,
    mut hints: bevy_ecs::event::EventWriter<crate::resources::onboarding::NotifyHintSystemTerrain>,
) {
    use crate::systems::ui::resources::widget_params::{UiSounds, WidgetParams, WidgetVolumeShape};
    let (Some(actions), Some(ui), Some(navigation), Some(camera), Some(collision), Some(cursor)) =
        (actions, ui, navigation, camera, collision, cursor)
    else {
        return;
    };
    if !terrain_focus_input_enabled(*mode, &navigation, &actions, &hovered) {
        return;
    }
    // B @140977467..504: actual edit_focus_canvas with volume tag5, max
    // distance in size.x, empty sounds and occlude_by_world byte0.
    ui.add(
        "edit_focus_canvas",
        WidgetParams::new_with_sounds_and_volume(
            UiSounds::empty(),
            WidgetVolumeShape { tag: 5, layer: 0, size: glam::Vec2::new(f32::MAX, 0.0) },
        )
        .occlude_by_world(false),
    );
    let geometry = collision.raycast_simple(camera.origin, camera.dir, 0x3f43);
    if let Some(stroke_id) = terrain_focus_target(&camera, &cursor, &hovered, &geometry) {
        // B @140977ce6..d75: request first, then terrain notification byte2.
        requests.write(crate::systems::mode_manager::RequestAppModeChange(
            AppMode::Editing(EditSubject::Terrain(stroke_id)),
        ));
        hints.write(crate::resources::onboarding::NotifyHintSystemTerrain { tag: 2 });
    }
}
