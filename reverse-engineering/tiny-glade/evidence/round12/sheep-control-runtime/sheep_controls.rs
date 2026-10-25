//! B SheepControl component and complete control runtime@1416f11a0.
//! Freeze and manually captured snapshots have separate owned Vec payloads.
use crate::{
    game_config::developer_hotkeys_enabled,
    resources::{
        critters::{CritterGoal, sheep::{Sheep, SheepMoveState, control::{self, SheepControlSnapshot}}},
        cursor_terrain_raycast::CursorTerrainRaycast,
        gui::{DebugGui, EguiContextResource},
    },
    systems::{
        history::finish_replay::CmdUnfreezeSheep,
        mode_manager::AppMode,
        ui::resources::{ui_state::UiState, widget_params::{WidgetParams, WidgetVolumeShape}},
    },
    utils::components::transform::Transform,
};
use bevy_ecs::prelude::*;
use bevy_input::{ButtonInput, keyboard::KeyCode};
use glam::Vec3;

/// Original byte+48. Both nonzero variants consume a clicked ground point.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(u8)]
pub enum PickingMode {
    #[default]
    None = 0,
    Goal = 1,
    Position = 2,
}

/// Spawned once by startup_sheep, never registered as a Resource.
#[derive(Component, Clone, Debug)]
pub struct SheepControl {
    /// Original first Option<Vec<SheepSnapshot>>: continuously applied freeze.
    pub snapshot: Option<SheepControlSnapshot>,
    /// Original second Option<Vec<SheepSnapshot>>: Take/Restore/Save/Load data.
    pub manual_snapshot: Option<SheepControlSnapshot>,
    pub wireframe: Entity,
    pub solid: Entity,
    /// Native Sheep+20 identifier (0/1/2), currently named Sheep.pet_target.
    pub selected_sheep: u64,
    pub picking_mode: PickingMode,
}

impl SheepControl {
    pub fn new(wireframe: Entity, solid: Entity) -> Self {
        Self { snapshot: None, manual_snapshot: None, wireframe, solid,
            selected_sheep: 0, picking_mode: PickingMode::None }
    }
}

type SheepQuery<'w, 's> = Query<'w, 's, (Entity, &'static mut Transform, &'static mut Sheep, &'static mut CritterGoal)>;

/// Original helper@1416f24f0: query-order zip to the shorter sequence.
pub fn apply_sheep_snapshot(sheep: &mut SheepQuery<'_, '_>, snapshot: &SheepControlSnapshot) {
    for ((_, mut transform, mut sheep, mut goal), saved) in sheep.iter_mut().zip(&snapshot.sheep) {
        control::apply_row(saved, &mut transform, &mut sheep, &mut goal);
    }
}

fn capture(sheep: &SheepQuery<'_, '_>) -> SheepControlSnapshot {
    control::capture_sheep_snapshot(sheep.iter().map(|(_, transform, sheep, goal)| (transform, sheep, goal)))
}

fn restore_manual(controls: &mut SheepControl, sheep: &mut SheepQuery<'_, '_>) {
    let Some(saved) = controls.manual_snapshot.as_ref() else { return; };
    apply_sheep_snapshot(sheep, saved);
    // Original clone occurs only when the first Option was already Some.
    if controls.snapshot.is_some() { controls.snapshot = Some(saved.clone()); }
}

const SNAPSHOT_DIRECTORY: &str = "tmp/sheep_snapshots/";

fn save_manual_snapshot(saved: &SheepControlSnapshot) {
    let directory = std::path::Path::new(SNAPSHOT_DIRECTORY);
    let _ = std::fs::create_dir_all(directory);
    // Original canonicalize failure unwraps, even if directory creation failed.
    let directory = directory.canonicalize().expect("canonical sheep snapshot directory");
    let Some(path) = rfd::FileDialog::new().add_filter("json", &["json"]).set_directory(directory).save_file() else { return; };
    let bytes = serde_json::to_vec_pretty(saved).expect("serialize sheep snapshot");
    if let Err(error) = std::fs::write(&path, bytes) { log::error!("{error}"); }
}

fn load_manual_snapshot(controls: &mut SheepControl, sheep: &mut SheepQuery<'_, '_>) {
    // Original Load canonicalizes without creating the directory first.
    let directory = std::path::Path::new(SNAPSHOT_DIRECTORY).canonicalize().expect("canonical sheep snapshot directory");
    let Some(path) = rfd::FileDialog::new().add_filter("json", &["json"]).set_directory(directory).pick_file() else { return; };
    let Ok(text) = std::fs::read_to_string(path) else { return; };
    let Ok(saved) = serde_json::from_str::<SheepControlSnapshot>(&text) else { return; };
    apply_sheep_snapshot(sheep, &saved);
    // Unlike Restore, original file Load changes only the manual snapshot.
    controls.manual_snapshot = Some(saved);
}

fn draw_controls(ui: &mut egui::Ui, frozen: &mut bool, controls: &mut SheepControl,
                 sheep: &mut SheepQuery<'_, '_>, commands: &mut Commands<'_, '_>) {
    ui.checkbox(frozen, "Freeze");
    ui.horizontal(|ui| {
        if ui.button("Take snapshot").clicked() { controls.manual_snapshot = Some(capture(sheep)); }
        if controls.manual_snapshot.is_some() && ui.button("Restore snapshot").clicked() { restore_manual(controls, sheep); }
        ui.horizontal(|ui| {
            if let Some(saved) = &controls.manual_snapshot {
                if ui.button("Save").clicked() { save_manual_snapshot(saved); }
            }
            if ui.button("Load").clicked() { load_manual_snapshot(controls, sheep); }
        });
    });
    ui.horizontal(|ui| {
        if ui.button("Next sheep").clicked() {
            controls.selected_sheep = controls.selected_sheep.wrapping_add(1) % 3;
        }
        ui.label(format!("Selected: {}", controls.selected_sheep));
    });
    if ui.button("Set goal").clicked() { controls.picking_mode = PickingMode::Goal; }
    if ui.button("Set position").clicked() { controls.picking_mode = PickingMode::Position; }
    for (_, transform, live, goal) in sheep.iter() {
        if live.pet_target != controls.selected_sheep { continue; }
        ui.label(format!("{:?}", live.move_state));
        commands.entity(controls.wireframe).insert(*transform);
        if goal.goal_tag == 1 {
            commands.entity(controls.solid).insert(Transform::from_translation_scale(goal.position + Vec3::Y, Vec3::splat(0.5)));
        } else {
            commands.entity(controls.solid).remove::<Transform>();
        }
    }
}

/// Returns whether a row was dragged; native refreshes the freeze after dragging.
fn handle_mouse_picking(controls: &mut SheepControl, sheep: &mut SheepQuery<'_, '_>,
                        cursor: &CursorTerrainRaycast, ui: &UiState) -> bool {
    let mut moved = false;
    let position = cursor.position();
    // Original SSE evaluates (z*z + x*x) + y*y once before iterating.
    let delta = cursor.delta;
    let distance = ((delta.z*delta.z + delta.x*delta.x) + delta.y*delta.y).sqrt();
    // Original writes X/Z only, preserving each sheep's current height.
    for (_, mut transform, live, _) in sheep.iter_mut() {
        let widget = ui.add(&format!("sheep_selection_{}", live.pet_target),
            WidgetParams::new(WidgetVolumeShape::sphere(transform.translation + Vec3::new(0.0,0.8,0.0),1.25),[0xe01,0]));
        if widget.dragged() == 4 && distance > f32::from_bits(0x3c23_d70a) {
            transform.translation.x = position.x;
            transform.translation.z = position.z;
            moved = true;
        }
        if widget.clicked() { controls.selected_sheep = live.pet_target; }
    }
    if controls.picking_mode == PickingMode::None { return moved; }
    let widget = ui.add("sheep_pick_point", WidgetParams::new(
        WidgetVolumeShape::sphere(position,0.25),[0x101,0]).occlude_by_world(false));
    if !widget.clicked() { return moved; }
    let mode = controls.picking_mode;
    if let Some((_, mut transform, mut live, mut goal)) = sheep.iter_mut().find(|(_,_,live,_)| live.pet_target == controls.selected_sheep) {
        match mode {
            PickingMode::Position => {
                transform.translation.x = position.x;
                transform.translation.z = position.z;
            }
            PickingMode::Goal => {
                goal.goal_tag = 1;
                goal.position = Vec3::new(position.x,0.0,position.z);
                live.move_state = SheepMoveState::Walking { timeout_s: 0.0,
                    goal_change_timeout: f32::MAX, force_reach_goal: true };
            }
            PickingMode::None => unreachable!(),
        }
        controls.picking_mode = PickingMode::None;
        // Original successful pick refreshes a frozen snapshot below.
        moved = true;
    } else {
        controls.picking_mode = PickingMode::None;
    }
    moved
}

/// Original ten SystemParams in order. Mode is present but unused in B body.
pub fn sheep_controls(
    _mode: Res<AppMode>,
    mut sheep: SheepQuery,
    mut controllers: Query<&mut SheepControl>,
    cursor: Res<CursorTerrainRaycast>,
    ui: Res<UiState>,
    context: Option<Res<EguiContextResource>>,
    mut debug: ResMut<DebugGui>,
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    mut unfreeze: EventReader<CmdUnfreezeSheep>,
) {
    let mut controls = controllers.single_mut().expect("exactly one SheepControl component");
    if let Some(snapshot) = &controls.snapshot { apply_sheep_snapshot(&mut sheep,snapshot); }
    let mut frozen = controls.snapshot.is_some();
    if developer_hotkeys_enabled() && keys.just_pressed(KeyCode::KeyU) { frozen = !frozen; }
    let mut moved = false;
    if !debug.enabled || !debug.show_sheep_controls {
        commands.entity(controls.wireframe).remove::<Transform>();
        commands.entity(controls.solid).remove::<Transform>();
    } else if let Some(context) = context {
        egui::Window::new("Sheep control").open(&mut debug.show_sheep_controls).show(&context,|panel| {
            draw_controls(panel,&mut frozen,&mut controls,&mut sheep,&mut commands);
        });
        moved = handle_mouse_picking(&mut controls,&mut sheep,&cursor,&ui);
    }
    if moved && controls.snapshot.is_some() { controls.snapshot = Some(capture(&sheep)); }
    if frozen {
        if controls.snapshot.is_none() { controls.snapshot = Some(capture(&sheep)); }
    } else { controls.snapshot = None; }
    // The current frame restores/edits/captures before consuming unit events.
    if unfreeze.read().count() != 0 { controls.snapshot = None; }
}
