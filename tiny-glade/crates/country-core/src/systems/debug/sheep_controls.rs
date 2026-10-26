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

type SheepQuery<'w, 's> = Query<'w, 's, (Entity, ComponentWrite<Transform>, ComponentWrite<Sheep>, &'static mut CritterGoal)>;

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

#[cfg(test)]
mod original_control_tests {
    use super::*;
    use bevy_ecs::system::SystemState;
    use crate::systems::ui::resources::ui_state::{UiInteraction, WidgetId};

    fn bytes(text: &str) -> Vec<u8> {
        (0..text.len()).step_by(2).map(|i| u8::from_str_radix(&text[i..i+2],16).unwrap()).collect()
    }
    fn word(data: &[u8], at: usize) -> u32 { u32::from_le_bytes(data[at..at+4].try_into().unwrap()) }
    fn float(data: &[u8], at: usize) -> f32 { f32::from_bits(word(data,at)) }
    fn vec3(data: &[u8], at: usize) -> Vec3 { Vec3::new(float(data,at),float(data,at+4),float(data,at+8)) }
    fn movement(data: &[u8]) -> SheepMoveState {
        if data[0] == 0 { SheepMoveState::Idle { timeout_s: float(data,4) } }
        else { SheepMoveState::Walking { timeout_s:float(data,4),goal_change_timeout:float(data,8),force_reach_goal:data[1]!=0 } }
    }
    fn snapshot(value: &serde_json::Value) -> Option<SheepControlSnapshot> {
        value.as_array().map(|rows| SheepControlSnapshot { sheep: rows.iter().map(|row| {
            let row=bytes(row.as_str().unwrap());
            control::SheepSnapshot { position:vec3(&row,28),goal:(word(&row,0)==1).then(||vec3(&row,4)),move_state:movement(&row[16..28]) }
        }).collect() })
    }
    fn populate() -> (World, [Entity;3], Entity, [Entity;2]) {
        let mut world=World::new();
        let markers=[world.spawn(Transform::identity()).id(),world.spawn(Transform::identity()).id()];
        let controls=world.spawn(SheepControl::new(markers[0],markers[1])).id();
        let sheep=std::array::from_fn(|i|world.spawn((
            Transform::from_translation(Vec3::new(10.+i as f32,20.+i as f32,30.+i as f32)),
            Sheep { behavior_state:0,goal_timer:0.,pet_phase:0,pet_progress:0.,pet_target:i as u64,
                control_flag:0,move_state:SheepMoveState::Idle {timeout_s:f32::from_bits(0x40000000+i as u32)},amplitude:5. },
            CritterGoal {goal_tag:1,position:Vec3::new(40.+i as f32,50.+i as f32,60.+i as f32)},
        )).id());
        (world,sheep,controls,markers)
    }
    fn compare(world: &World, sheep: [Entity;3], controls: Entity, expected: &serde_json::Value) {
        for (i,id) in sheep.into_iter().enumerate() {
            let t=world.get::<Transform>(id).unwrap();
            let p=&expected["transforms"][i];
            assert_eq!(t.translation,Vec3::new(p[0].as_f64().unwrap() as f32,p[1].as_f64().unwrap() as f32,p[2].as_f64().unwrap() as f32));
            let g=bytes(expected["goals"][i].as_str().unwrap());
            let actual=world.get::<CritterGoal>(id).unwrap();
            assert_eq!(actual.goal_tag,word(&g,0));
            if actual.goal_tag==1 { assert_eq!(actual.position,vec3(&g,4)); }
            assert_eq!(world.get::<Sheep>(id).unwrap().move_state,movement(&bytes(expected["movement"][i].as_str().unwrap())));
            assert_eq!(world.get::<Sheep>(id).unwrap().amplitude,5.);
        }
        let actual=world.get::<SheepControl>(controls).unwrap();
        assert_eq!(actual.snapshot,snapshot(&expected["snapshot"]));
        assert_eq!(actual.selected_sheep,expected["selected"].as_u64().unwrap());
        assert_eq!(actual.picking_mode as u8,expected["picking"].as_u64().unwrap() as u8);
    }

    #[test]
    fn actual_component_runtime_restores_before_unfreeze_on_original_nonempty_fixtures() {
        let fixtures:serde_json::Value=serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"),"/tests/fixtures/original-sheep-controls-runtime-b.json"))).unwrap();
        let mut checked=0;
        for case in fixtures.as_array().unwrap() {
            // The no-key cases exercise the real system without modifying a
            // process-global developer-hotkey owner during parallel tests.
            if case["press"].as_bool().unwrap() || case["developer"].as_bool().unwrap() { continue; }
            let (mut world,sheep,controls,markers)=populate();
            world.get_mut::<SheepControl>(controls).unwrap().snapshot=snapshot(&case["snapshot"]);
            world.insert_resource(AppMode::default());
            world.insert_resource(CursorTerrainRaycast::default());
            world.insert_resource(UiState::at_startup());
            world.insert_resource(DebugGui::default());
            world.insert_resource(ButtonInput::<KeyCode>::default());
            world.init_resource::<Events<CmdUnfreezeSheep>>();
            if case["unfreeze"].as_bool().unwrap() {world.resource_mut::<Events<CmdUnfreezeSheep>>().send(CmdUnfreezeSheep);}
            let mut schedule=Schedule::default();schedule.add_systems(sheep_controls);schedule.run(&mut world);
            compare(&world,sheep,controls,&case["result"]);
            for marker in markers {assert!(world.get::<Transform>(marker).is_none());}
            checked+=1;
        }
        assert_eq!(checked,8);
    }

    #[test]
    fn actual_picking_consumer_matches_48_original_runtime_fixtures() {
        let fixtures:serde_json::Value=serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"),"/tests/fixtures/original-sheep-controls-picking-b.json"))).unwrap();
        for case in fixtures.as_array().unwrap() {
            let input=&case["inputs"];
            let (mut world,sheep,controls,_)=populate();
            {
                let mut control=world.get_mut::<SheepControl>(controls).unwrap();
                control.snapshot=snapshot(&input["snapshot"]);
                control.selected_sheep=input["selected"].as_u64().unwrap_or(0);
                control.picking_mode=match input["picking"].as_u64().unwrap_or(0) {0=>PickingMode::None,1=>PickingMode::Goal,2=>PickingMode::Position,_=>unreachable!()};
            }
            let hit=&input["hit"];
            let name=match hit[2].as_u64().unwrap() {100=>"sheep_selection_0",101=>"sheep_selection_1",102=>"sheep_selection_2",999=>"sheep_pick_point",_=>"unused"};
            let ui=UiState::new(UiInteraction {native_tag:hit[0].as_u64().unwrap() as u8,native_phase:hit[1].as_u64().unwrap() as u8,widget:WidgetId::from_name(name)});
            let cursor=CursorTerrainRaycast {tag:u32::from(input["cursor_hit"].as_bool().unwrap_or(true)),pos:Vec3::new(7.,99.,8.),
                delta:input["delta_bits"].as_array().map(|v|Vec3::new(f32::from_bits(v[0].as_u64().unwrap() as u32),f32::from_bits(v[1].as_u64().unwrap() as u32),f32::from_bits(v[2].as_u64().unwrap() as u32))).unwrap_or(Vec3::X),speed:0.};
            let mut state=SystemState::<(SheepQuery,Query<&mut SheepControl>)>::new(&mut world);
            {
                let (mut sheep,mut controllers)=state.get_mut(&mut world);
                let mut controls=controllers.single_mut().unwrap();
                if let Some(snapshot)=&controls.snapshot { apply_sheep_snapshot(&mut sheep,snapshot); }
                let moved=handle_mouse_picking(&mut controls,&mut sheep,&cursor,&ui);
                if moved && controls.snapshot.is_some() { controls.snapshot=Some(capture(&sheep)); }
            }
            state.apply(&mut world);
            compare(&world,sheep,controls,&case["result"]);
            assert!(ui.widget(WidgetId::from_name("sheep_selection_2")).is_some());
        }
        assert_eq!(fixtures.as_array().unwrap().len(),48);
    }

    #[test]
    fn manual_restore_preserves_distinct_freeze_ownership_and_only_syncs_an_existing_freeze() {
        for frozen in [false,true] {
            let (mut world,entities,controls,_)=populate();
            let saved=SheepControlSnapshot {sheep:vec![control::SheepSnapshot {position:Vec3::X,goal:None,move_state:SheepMoveState::Idle {timeout_s:17.}}]};
            {
                let mut controller=world.get_mut::<SheepControl>(controls).unwrap();
                controller.manual_snapshot=Some(saved.clone());
                controller.snapshot=frozen.then(||SheepControlSnapshot {sheep:vec![]});
            }
            let mut state=SystemState::<(SheepQuery,Query<&mut SheepControl>)>::new(&mut world);
            {
                let (mut sheep,mut controls)=state.get_mut(&mut world);
                restore_manual(&mut controls.single_mut().unwrap(),&mut sheep);
            }
            assert_eq!(world.get::<Transform>(entities[0]).unwrap().translation,Vec3::X);
            assert_eq!(world.get::<Transform>(entities[1]).unwrap().translation,Vec3::new(11.,21.,31.));
            let controller=world.get::<SheepControl>(controls).unwrap();
            assert_eq!(controller.manual_snapshot.as_ref(),Some(&saved));
            assert_eq!(controller.snapshot.as_ref(),frozen.then_some(&saved));
        }
    }

    #[test]
    fn real_egui_window_populates_widgets_and_debug_transforms_for_selected_live_sheep() {
        let (mut world,_,controls,markers)=populate();
        world.insert_resource(AppMode::default());world.insert_resource(CursorTerrainRaycast::default());
        world.insert_resource(UiState::at_startup());world.insert_resource(ButtonInput::<KeyCode>::default());
        world.init_resource::<Events<CmdUnfreezeSheep>>();
        world.insert_resource(DebugGui {enabled:true,show_sheep_controls:true,..Default::default()});
        let context=egui::Context::default();world.insert_resource(EguiContextResource(context.clone()));
        context.begin_frame(egui::RawInput::default());
        let mut schedule=Schedule::default();schedule.add_systems(sheep_controls);schedule.run(&mut world);
        let _=context.end_frame();
        assert_eq!(world.get::<Transform>(markers[0]).unwrap().translation,Vec3::new(10.,20.,30.));
        assert_eq!(world.get::<Transform>(markers[1]).unwrap().translation,Vec3::new(40.,51.,60.));
        assert_eq!(world.get::<Transform>(markers[1]).unwrap().scale,Vec3::splat(0.5));
        assert!(world.get::<SheepControl>(controls).unwrap().snapshot.is_none());
        assert!(world.resource::<UiState>().widget(WidgetId::from_name("sheep_selection_0")).is_some());
    }
}

use crate::render::extract::ComponentWrite;
