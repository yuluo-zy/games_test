//! Desktop shell: rays/UI -> pure tool transactions -> command inbox.
use crate::{
    camera::{CameraAction, OrbitCamera},
    picking::hit_distance,
    pointer::{DesktopInputSet, PointerFrame, PointerRouterPlugin},
    renderer::DisplayedBuilding,
};
use bevy::prelude::*;
use garden_application::{
    EditCommand,
    tools::{GroundPoint, Tool, ToolController},
};
use garden_bevy::{
    CommandInbox, EditFeedback, EditorState, GardenSet, GenerationFailures, Operation,
};
use garden_domain::{Building, BuildingEdit, Roof};

pub struct DesktopEditorPlugin;
impl Plugin for DesktopEditorPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<PointerRouterPlugin>() {
            app.add_plugins(PointerRouterPlugin);
        }
        app.init_resource::<DesktopEditor>()
            .configure_sets(Update, DesktopInputSet::Tools.before(GardenSet::Commit))
            .add_systems(
                Update,
                (ui_actions, world_tools)
                    .chain()
                    .in_set(DesktopInputSet::Tools),
            )
            .add_systems(
                Update,
                (feedback, status_text, button_color)
                    .chain()
                    .after(GardenSet::Commit),
            )
            .add_systems(Update, draw_overlay.after(GardenSet::Collect));
    }
}
#[derive(Component, Clone, Copy)]
pub enum EditorAction {
    Select,
    Build,
    Move,
    Cancel,
    Retry,
    Delete,
    Roof,
    HeightUp,
    HeightDown,
    Undo,
    Redo,
}
#[derive(Component)]
pub struct EditorStatus;

#[derive(Resource)]
pub struct DesktopEditor {
    pub tools: ToolController,
    pub status: String,
    pending: Option<EditCommand>,
}
impl Default for DesktopEditor {
    fn default() -> Self {
        Self {
            tools: ToolController::default(),
            status: "Select a house, or choose Build and draw on the grass.".into(),
            pending: None,
        }
    }
}
impl DesktopEditor {
    pub fn submit(&mut self, command: EditCommand, inbox: &mut CommandInbox) {
        match inbox.submit(Operation::Edit(command.clone())) {
            Ok(()) => {
                self.pending = None;
                self.status = "Edit queued; waiting for acceptance.".into();
            }
            Err(_) => {
                self.pending = Some(command);
                self.status = "Queue full. Retry or Cancel; your edit is retained.".into();
            }
        }
    }
}
type ActionButton<'a> = (
    &'a Interaction,
    Option<&'a EditorAction>,
    Option<&'a CameraAction>,
);
fn ui_actions(
    buttons: Query<ActionButton<'_>, Changed<Interaction>>,
    editor: Res<EditorState>,
    mut state: ResMut<DesktopEditor>,
    mut inbox: ResMut<CommandInbox>,
) {
    for (interaction, action, camera) in &buttons {
        if *interaction != Interaction::Pressed {
            continue;
        }
        if camera.is_some() {
            state.tools.cancel();
        }
        let Some(action) = action else {
            continue;
        };
        state.tools.cancel();
        if matches!(action, EditorAction::Cancel) {
            state.pending = None;
            state.status = "Cancelled; no history entry added.".into();
            continue;
        }
        if matches!(action, EditorAction::Retry) {
            if let Some(command) = state.pending.take() {
                state.submit(command, &mut inbox);
            }
            continue;
        }
        if state.pending.is_some() {
            state.status = "Retry or Cancel the pending edit first.".into();
            continue;
        }
        let tool = match action {
            EditorAction::Select => Some(Tool::Select),
            EditorAction::Build => Some(Tool::Build),
            EditorAction::Move => Some(Tool::Move),
            _ => None,
        };
        if let Some(tool) = tool {
            state.tools.set_tool(tool);
            state.status = match tool {
                Tool::Select => "Click a visible house to select.",
                Tool::Build => "Left-drag on grass. Minimum size 1.2m; release to build.",
                Tool::Move => "Left-drag a visible house to move; right-click cancels.",
            }
            .into();
            continue;
        }
        let command = match action {
            EditorAction::Undo => EditCommand::Undo,
            EditorAction::Redo => EditCommand::Redo,
            _ => {
                let Some(id) = state.tools.selected() else {
                    state.status = "Select a house first.".into();
                    continue;
                };
                let Some(building) = editor.editor().get(id) else {
                    state.status = "Selected house no longer exists.".into();
                    continue;
                };
                let block = &building.blocks()[0];
                match action {
                    EditorAction::Delete => EditCommand::Delete(id),
                    EditorAction::Roof => EditCommand::Edit {
                        building: id,
                        edit: BuildingEdit::SetRoof {
                            block: block.id,
                            roof: match block.roof_intent {
                                Roof::Gabled => Roof::Hipped,
                                Roof::Hipped => Roof::Flat,
                                Roof::Flat => Roof::Gabled,
                            },
                        },
                    },
                    EditorAction::HeightUp | EditorAction::HeightDown => EditCommand::Edit {
                        building: id,
                        edit: BuildingEdit::Resize {
                            block: block.id,
                            footprint: block.footprint,
                            height: block.height
                                + if matches!(action, EditorAction::HeightUp) {
                                    3.0
                                } else {
                                    -3.0
                                },
                        },
                    },
                    _ => continue,
                }
            }
        };
        state.submit(command, &mut inbox);
    }
}
fn ground(ray: Ray3d) -> Option<GroundPoint> {
    if ray.direction.y.abs() < 1e-6 {
        return None;
    }
    let distance = -ray.origin.y / ray.direction.y;
    if distance < 0.0 || !distance.is_finite() {
        return None;
    }
    let point = ray.get_point(distance);
    Some(GroundPoint {
        x: f64::from(point.x),
        z: f64::from(point.z),
    })
}
fn world_tools(
    frame: Res<PointerFrame>,
    editor: Res<EditorState>,
    cameras: Query<(&Camera, &Transform), With<OrbitCamera>>,
    displayed: Query<(&DisplayedBuilding, &Transform)>,
    mut state: ResMut<DesktopEditor>,
    mut inbox: ResMut<CommandInbox>,
) {
    if frame.cancel_world {
        state.tools.cancel();
        state.status = "Drag cancelled; no history change.".into();
        return;
    }
    if state.pending.is_some() {
        return;
    }
    if !(frame.world_begin || frame.world_held || frame.world_finish) {
        return;
    }
    let ray = frame.cursor.and_then(|cursor| {
        cameras.iter().next().and_then(|(camera, transform)| {
            camera
                .viewport_to_world(&GlobalTransform::from(*transform), cursor)
                .ok()
        })
    });
    let Some(ray) = ray else {
        if frame.world_finish {
            state.tools.cancel();
            state.status = "No valid view ray; edit cancelled.".into();
        }
        return;
    };
    if frame.world_begin && state.tools.tool() != Tool::Build {
        let picked = displayed
            .iter()
            .filter(|(b, _)| editor.editor().get(b.target.building).is_some())
            .filter_map(|(b, t)| {
                hit_distance(ray, b, t).map(|distance| (distance, b.target.building, b.ticket))
            })
            .min_by(|a, b| a.0.total_cmp(&b.0).then(a.1.cmp(&b.1)));
        state.tools.select(picked.map(|(_, id, _)| id));
        if state.tools.tool() == Tool::Select {
            state.status = if picked.is_some() {
                "House selected."
            } else {
                "Selection cleared."
            }
            .into();
            return;
        }
        if !picked.is_some_and(|(_, _, ticket)| editor.editor().accepts(ticket)) {
            state.status = "Choose a house whose visible geometry is up to date.".into();
            return;
        }
    }
    let Some(point) = ground(ray) else {
        state.tools.cancel();
        state.status = "Pointer has no ground intersection; cancelled.".into();
        return;
    };
    if frame.world_begin
        && let Err(error) = state.tools.begin(editor.editor(), point)
    {
        state.status = format!("Cannot start: {error:?}");
        return;
    }
    if state.tools.dragging() {
        match state.tools.update(point) {
            Ok(()) => {
                state.status = "Preview only. Release to commit; right-click to cancel.".into()
            }
            Err(error) => {
                state.status = format!("Invalid preview: {error:?}. Release will not commit.")
            }
        }
        if frame.world_finish {
            let candidate_id = state.tools.candidate().map(Building::id);
            match state.tools.finish() {
                Ok(Some(command)) => {
                    if let Some(id) = candidate_id {
                        state.tools.select(Some(id));
                    }
                    state.submit(command, &mut inbox);
                }
                Ok(None) => (),
                Err(error) => state.status = format!("Edit cancelled: {error:?}"),
            }
        }
    }
}
fn feedback(
    mut results: ResMut<EditFeedback>,
    mut failures: ResMut<GenerationFailures>,
    editor: Res<EditorState>,
    mut state: ResMut<DesktopEditor>,
) {
    for outcome in results.drain() {
        state.status = match outcome.result {
            Ok(Some(change)) => format!(
                "Accepted house #{} revision {}; geometry may still be generating.",
                change.building.get(),
                change.revision
            ),
            Ok(None) => "No scene change.".into(),
            Err(error) => format!("Edit rejected: {error}"),
        };
    }
    for (ticket, error) in failures.drain() {
        state.status = format!("Geometry failed for #{}: {error}", ticket.building.get());
    }
    if state.pending.is_none()
        && state
            .tools
            .selected()
            .is_some_and(|id| editor.editor().get(id).is_none())
    {
        state.tools.select(None);
    }
}
fn status_text(state: Res<DesktopEditor>, mut labels: Query<&mut Text, With<EditorStatus>>) {
    if !state.is_changed() {
        return;
    }
    let selected = state
        .tools
        .selected()
        .map_or("none".into(), |id| format!("#{}", id.get()));
    for mut text in &mut labels {
        *text = Text::new(format!(
            "Tool: {:?} | Selected: {}\n{}",
            state.tools.tool(),
            selected,
            state.status
        ));
    }
}
fn button_color(
    state: Res<DesktopEditor>,
    mut buttons: Query<(&EditorAction, &Interaction, &mut BackgroundColor)>,
) {
    for (action, interaction, mut color) in &mut buttons {
        let selected = matches!(
            (action, state.tools.tool()),
            (EditorAction::Select, Tool::Select)
                | (EditorAction::Build, Tool::Build)
                | (EditorAction::Move, Tool::Move)
        );
        color.0 = if *interaction == Interaction::Pressed {
            Color::srgb_u8(73, 99, 70)
        } else if *interaction == Interaction::Hovered || selected {
            Color::srgb_u8(141, 161, 126)
        } else {
            Color::srgb_u8(108, 132, 99)
        };
    }
}
fn wire_box(gizmos: &mut Gizmos, min: Vec3, max: Vec3, transform: Transform, color: Color) {
    let matrix = transform.compute_affine();
    let corners: Vec<_> = (0..8)
        .map(|i| {
            matrix.transform_point3(Vec3::new(
                if i & 1 == 0 { min.x } else { max.x },
                if i & 2 == 0 { min.y } else { max.y },
                if i & 4 == 0 { min.z } else { max.z },
            ))
        })
        .collect();
    for i in 0..8 {
        for bit in [1, 2, 4] {
            if i & bit == 0 {
                gizmos.line(corners[i], corners[i | bit], color);
            }
        }
    }
}
fn wire_building(gizmos: &mut Gizmos, b: &Building, color: Color) {
    let p = b.placement();
    let transform = Transform::from_xyz(p.x as f32, p.elevation as f32, p.z as f32)
        .with_rotation(Quat::from_rotation_y(p.yaw as f32));
    for block in b.blocks() {
        let mut base = 0.0;
        let mut parent = block.parent;
        while let Some(id) = parent {
            let support = b.block(id).unwrap();
            base += support.height;
            parent = support.parent;
        }
        let r = block.footprint;
        wire_box(
            gizmos,
            Vec3::new(r.x as f32, base as f32 + 0.05, r.z as f32),
            Vec3::new(
                (r.x + r.width) as f32,
                (base + block.height) as f32,
                (r.z + r.depth) as f32,
            ),
            transform,
            color,
        );
    }
}
fn draw_overlay(
    state: Res<DesktopEditor>,
    displayed: Query<(&DisplayedBuilding, &Transform)>,
    mut gizmos: Gizmos,
) {
    if let Some(id) = state.tools.selected() {
        for (shown, transform) in &displayed {
            if shown.target.building == id {
                wire_box(
                    &mut gizmos,
                    shown.min - Vec3::splat(0.04),
                    shown.max + Vec3::splat(0.04),
                    *transform,
                    Color::srgb_u8(246, 201, 86),
                );
            }
        }
    }
    let color = if state.tools.valid() {
        Color::srgb_u8(58, 213, 219)
    } else {
        Color::srgb_u8(240, 91, 73)
    };
    if let Some(building) = state.tools.candidate() {
        wire_building(&mut gizmos, building, color);
    } else if let Some((a, b)) = state.tools.build_outline() {
        wire_box(
            &mut gizmos,
            Vec3::new(a.x.min(b.x) as f32, 0.05, a.z.min(b.z) as f32),
            Vec3::new(a.x.max(b.x) as f32, 3.0, a.z.max(b.z) as f32),
            Transform::default(),
            color,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        catalog::ArtCatalog,
        renderer::{PresentationPlugin, RenderStats},
    };
    use garden_bevy::{GardenPlugin, Settings};
    use garden_domain::{BuildingId, sample_building};
    use std::time::{Duration, Instant};

    fn app(settings: Settings) -> App {
        let mut app = App::new();
        app.init_resource::<Assets<Mesh>>()
            .init_resource::<Assets<StandardMaterial>>()
            .init_resource::<DesktopEditor>()
            .init_resource::<PointerFrame>()
            .add_plugins((
                GardenPlugin::new(settings).unwrap(),
                PresentationPlugin::new(ArtCatalog::warm_stone()),
            ))
            .add_systems(
                Update,
                (ui_actions, world_tools).chain().before(GardenSet::Commit),
            )
            .add_systems(Update, feedback.after(GardenSet::Commit));
        let orbit = OrbitCamera::from_view(Vec3::new(10.0, 15.0, 20.0), Vec3::ZERO).unwrap();
        let mut camera = Camera::default();
        camera.computed.clip_from_view =
            Mat4::perspective_infinite_reverse_rh(std::f32::consts::PI / 3.0, 1.5, 0.1);
        camera.computed.target_info = Some(bevy::camera::RenderTargetInfo {
            physical_size: UVec2::new(1440, 960),
            scale_factor: 1.0,
        });
        app.world_mut().spawn((camera, orbit.transform(), orbit));
        app
    }
    fn cursor(app: &mut App, point: Vec3) -> Vec2 {
        let mut q = app.world_mut().query::<(&Camera, &Transform)>();
        let (camera, transform) = q.iter(app.world()).next().unwrap();
        camera
            .world_to_viewport(&GlobalTransform::from(*transform), point)
            .unwrap()
    }
    fn frame(app: &mut App, point: Vec3, begin: bool, held: bool, finish: bool) {
        let cursor = cursor(app, point);
        *app.world_mut().resource_mut::<PointerFrame>() = PointerFrame {
            cursor: Some(cursor),
            world_begin: begin,
            world_held: held,
            world_finish: finish,
            ..default()
        };
        app.update();
        *app.world_mut().resource_mut::<PointerFrame>() = PointerFrame::default();
    }
    fn await_upload(app: &mut App, uploads: u64) {
        let deadline = Instant::now() + Duration::from_secs(5);
        loop {
            app.update();
            if app.world().resource::<RenderStats>().uploads >= uploads {
                break;
            }
            assert!(Instant::now() < deadline);
            std::thread::sleep(Duration::from_millis(1));
        }
    }
    #[test]
    fn camera_rays_build_pick_move_and_undo_through_real_pipeline() {
        let mut app = app(Settings::default());
        app.world_mut()
            .resource_mut::<DesktopEditor>()
            .tools
            .set_tool(Tool::Build);
        frame(&mut app, Vec3::ZERO, true, true, false);
        frame(&mut app, Vec3::new(4.0, 0.0, 5.0), false, true, false);
        assert_eq!(
            app.world()
                .resource::<EditorState>()
                .editor()
                .objects()
                .count(),
            0
        );
        frame(&mut app, Vec3::new(4.0, 0.0, 5.0), false, false, true);
        assert_eq!(
            app.world().resource::<EditorState>().editor().undo_count(),
            1
        );
        await_upload(&mut app, 1);
        app.world_mut()
            .resource_mut::<DesktopEditor>()
            .tools
            .set_tool(Tool::Select);
        frame(&mut app, Vec3::new(2.0, 1.0, 2.0), true, false, true);
        let id = app
            .world()
            .resource::<DesktopEditor>()
            .tools
            .selected()
            .unwrap();
        app.world_mut()
            .resource_mut::<DesktopEditor>()
            .tools
            .set_tool(Tool::Move);
        frame(&mut app, Vec3::new(2.0, 1.0, 2.0), true, true, false);
        frame(&mut app, Vec3::new(5.0, 0.0, 6.0), false, true, false);
        assert!(
            app.world()
                .resource::<EditorState>()
                .editor()
                .get(id)
                .unwrap()
                .placement()
                .x
                .abs()
                < 0.001
        );
        frame(&mut app, Vec3::new(5.0, 0.0, 6.0), false, false, true);
        assert_eq!(
            app.world().resource::<EditorState>().editor().undo_count(),
            2
        );
        assert!(
            app.world()
                .resource::<EditorState>()
                .editor()
                .get(id)
                .unwrap()
                .placement()
                .x
                > 1.0
        );
        app.world_mut()
            .spawn((Interaction::Pressed, EditorAction::Undo));
        app.update();
        assert!(
            app.world()
                .resource::<EditorState>()
                .editor()
                .get(id)
                .unwrap()
                .placement()
                .x
                .abs()
                < 0.001
        );
    }
    #[test]
    fn cancelled_pointer_preview_never_creates_or_changes_history() {
        let mut app = app(Settings::default());
        app.world_mut()
            .resource_mut::<DesktopEditor>()
            .tools
            .set_tool(Tool::Build);
        frame(&mut app, Vec3::ZERO, true, true, false);
        frame(&mut app, Vec3::new(4.0, 0.0, 5.0), false, true, false);
        *app.world_mut().resource_mut::<PointerFrame>() = PointerFrame {
            cancel_world: true,
            ..default()
        };
        app.update();
        assert_eq!(
            app.world().resource::<EditorState>().editor().undo_count(),
            0
        );
        assert!(!app.world().resource::<DesktopEditor>().tools.dragging());
    }
    #[test]
    fn full_inbox_retains_command_until_visible_retry_accepts_it() {
        let mut app = app(Settings {
            command_capacity: 1,
            ..default()
        });
        let id = BuildingId::new(1).unwrap();
        app.world_mut()
            .resource_mut::<CommandInbox>()
            .submit(Operation::Invalidate(id))
            .unwrap();
        app.world_mut()
            .resource_scope(|world, mut state: Mut<DesktopEditor>| {
                state.submit(
                    EditCommand::Create(sample_building(id, 3.0, 3.0)),
                    &mut world.resource_mut::<CommandInbox>(),
                );
                assert!(state.pending.is_some());
            });
        app.update();
        assert!(
            app.world()
                .resource::<EditorState>()
                .editor()
                .get(id)
                .is_none()
        );
        app.world_mut()
            .spawn((Interaction::Pressed, EditorAction::Retry));
        app.update();
        assert!(
            app.world()
                .resource::<EditorState>()
                .editor()
                .get(id)
                .is_some()
        );
        assert!(app.world().resource::<DesktopEditor>().pending.is_none());
    }
}
