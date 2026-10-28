//! B `systems/wall/adjust_wall_height/ui_adjust_wall_height` (VA `0x140b2fc00`, 3680 B).
//!
//! 11-param `FnSystem` (PDB authority):
//! 1 `Res<AppMode>` · 2 `Res<PublicWalls>` · 3 `Query<&Roof>` · 4 `Res<UiState>` ·
//! 5 `EventWriter<FinishAdjWallHeightMode>` · 6 `EventWriter<AdjustWallHeightUi>` ·
//! 7 `EventWriter<InitAdjWallHeightMode>` · 8 `Query<&RoofAnimation>` · 9 `Res<UiSignifierStream>` ·
//! 10 `EventReader<MainCameraMoved>` · 11 `Res<GameTime>`.
//!
//! Evidence: spec `work/m01-scratch/shape-edit-spec.md` §1.1 + independent machine-code
//! decode `work/m01-scratch/impl_uawh.asm` (real jump table `0x142aeba90` =
//! `[0→skip, 1→adjust+init, 2→adjust, 3→finish]`; index `clamp(stage-2,0,3)`).
//!
//! This system is the real producer of the three `*AdjWallHeightMode*` events that
//! `ui_adjust_wall_height_mode` and `history_record_adjust_wall_height` consume.

use bevy_ecs::prelude::*;
use glam::Vec3;

use crate::resources::events::{FinishAdjWallHeightMode, InitAdjWallHeightMode};
use crate::resources::roofs::roof_animation::RoofAnimation;
use crate::resources::roofs::Roof;
use crate::resources::walls::public_walls::PublicWalls;
use crate::resources::walls::WallId;
use crate::resources::time::GameTime;
use crate::systems::camera_rig::main_camera::MainCameraMoved;
use crate::systems::mode_manager::{AppMode, EditSubject};
use crate::systems::ui::resources::ui_state::UiState;
use crate::systems::ui::resources::widget_params::{WidgetGeometry, WidgetParams};
use crate::systems::ui::signifiers::signifier_create::UiSignifierStream;
use crate::systems::ui::signifiers::signifier_factory::{
    SignifierEntityBuilder, SignifierFactory, SignifierMesh,
};
use crate::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::AdjustWallHeightUi;
use crate::systems::wall::adjust_wall_height::UiWallHeight;
use std::sync::Arc;

/// `0x142925984` = 0.1 (roof flat threshold).
const ROOF_FLAT_THRESHOLD: f32 = 0.1;
/// `0x142ab3c08` = `[0.7, 0.5]` indexed by `(shape_tag < 2)`. This is the native widget's
/// colour/env scalar (NOT a picking radius); it occupies the volume scalar lane.
const WIDGET_ENV_TABLE: [f32; 2] = [0.7, 0.5];

/// B `UiWallHeight::widget` (`0x1409566d0`): register the height-edit widget for `wall`
/// from the wall's curve points lifted by its relative heights.
///
/// `lift`: `mode == 0 → +0.6`; `mode != 0 && !flat_w_no_rim → +0`; `mode != 0 &&
/// flat_w_no_rim → -0.29`. The normal editing path is `flag8 = 0`, i.e. `mode = 1`.
fn register_widget<'a>(
    ui_state: &'a UiState,
    state: &crate::resources::walls::public_wall_state::PublicWallState,
    wall: WallId,
    flat_w_no_rim: bool,
) -> (
    crate::systems::ui::resources::ui_state::UiWidgetContext<'a>,
    Vec<Vec3>,
) {
    let uh = UiWallHeight {
        wall,
        flag: 0,
        mode: 1,
        points: state
            .geometry
            .outline
            .iter()
            .map(|point| Vec3::from_array(*point))
            .collect(),
        heights: state.geometry.relative_heights.clone(),
    };
    // Widget hit geometry comes from the lifted curve points; the `[0.7, 0.5]` scalar is
    // the native colour/env value (indexed by `shape_tag < 2`), not a geometric radius.
    // SphereCloud is the ruled native geometry (`0x202` is `extra_78`; `FUN_140b34ec0` name-only).
    let centers = uh.lifted_points(flat_w_no_rim);
    let env = WIDGET_ENV_TABLE[(state.shape().tag() < 2) as usize];
    let context = ui_state.add(
        &uh.widget_name(),
        WidgetParams::default().geometry(WidgetGeometry::SphereCloud {
            centers: centers.clone(),
            radius: env,
        }),
    );
    (context, centers)
}

/// gate1: native `entry[-9]` = value start `+0x297` = `PublicWallState.being_bulldozed: bool`
/// (`public_wall_state.rs` writes `bytes[0x297] = being_bulldozed as u8`). Writer:
/// `publish_public_wall_state` (`0x140a8b530`, projected write `0x140a8d825..0x140a8d914`)
/// with `being_bulldozed = bulldozing.contains(id)`. `!= 0 → skip`.
fn wall_is_editable(state: &crate::resources::walls::public_wall_state::PublicWallState) -> bool {
    !state.is_being_bulldozed()
}

/// B `UiWallHeight::signifier` (`0x140956cd0`, normal `state+9 == 1` branch): emit the
/// `wall_height_edit_{wall}` signifier through the real factory path
/// (`SignifierFactory::from_closure` + `UiSignifierStream::write_factory`); never the
/// numeric 5-qword Raw adapter (`SIGNIFIER-CONTRACT`).
///
/// Guard: `ctx+0x18 == 0 || ctx+0x38 != wall || ctx+0x28 == 2 → return`.
fn emit_signifier(
    ui_state: &UiState,
    stream: &mut UiSignifierStream,
    wall: WallId,
    widget: crate::systems::ui::resources::ui_state::WidgetId,
) {
    // `ctx+0x38` is the *registered* widget id (the `edit_height_{wall}` hash), not the raw
    // WallId. Comparing against `WidgetId(wall.0)` would never match.
    if ui_state.hover_byte() == 0
        || ui_state.active_widget() != widget
        || ui_state.intersection_tag() == 2
    {
        return;
    }
    let name = format!("wall_height_edit_{wall}");
    let mut factory = SignifierFactory::from_closure(
        &name,
        Arc::new(move |meshes, shaders, commands| {
            // CONSTANT HANDLE per worker `0x1409576d0`: identity orientation `{0,0,0,1.0}`,
            // scale `1.5`, tag `0x101`. It is a FIXED handle — NOT the lifted curve points
            // (those are only the widget SphereCloud hit volume), no wall colour, no angle.
            // The worker also emits the material record `{FUN_1412dc8f0, f32 pair, 3.0/1, 0,
            // FUN_141564d30}` + `FUN_1412ddf90(&0x142ab48b0)`; `SignifierEntityBuilder`
            // cannot express that material/descriptor record directly, so the mesh source is
            // a KNOWN LIMITATION (constant `Empty` branch), never a point set.
            let entity = SignifierEntityBuilder::new(SignifierMesh::Empty)
                .rotation(glam::Quat::IDENTITY)
                .scale(1.5)
                .disable_default_animation()
                .animation_duration(0.0)
                .build(meshes, shaders, commands);
            vec![entity]
        }),
    );
    // Native factory marker `0x142ab4830`.
    factory.marker = 0x142ab4830;
    stream.write_factory(&factory);
}

/// B `ui_adjust_wall_height` (`0x140b2fc00`).
#[allow(clippy::too_many_arguments)]
pub fn ui_adjust_wall_height(
    app_mode: Res<AppMode>,
    public: Res<PublicWalls>,
    roofs: Query<&Roof>,
    ui_state: Res<UiState>,
    mut finish: EventWriter<FinishAdjWallHeightMode>,
    mut adjust_ui: EventWriter<AdjustWallHeightUi>,
    mut init: EventWriter<InitAdjWallHeightMode>,
    roof_animations: Query<&RoofAnimation>,
    mut stream: ResMut<UiSignifierStream>,
    mut camera_moved: EventReader<MainCameraMoved>,
    _time: Res<GameTime>,
) {
    let mode_tag = app_mode.native_tag();
    if mode_tag >= 6 {
        return;
    }
    // `wall = *((u64*)*param_1 + 2)` = the editing subject payload.
    let wall = match *app_mode {
        AppMode::Editing(EditSubject::Wall(id)) | AppMode::Editing(EditSubject::Roof(id)) => {
            WallId(id)
        }
        // Mode 0 has no wall payload; modes 3/4/5 return early natively.
        _ => return,
    };
    // MainCameraMoved reader: `has_new` = whether the camera produced events this tick.
    let has_new = camera_moved.read().count() > 0;

    // PublicWalls probe; `get` also performs the `simplified_curve_2d.is_valid()` assert
    // (native `-0x290`/`-0x270` guard).
    let Some(state) = public.get(wall) else {
        return;
    };
    // gate2: shape/curve point count (`-0x48 >= 2`). The spec records this as a shape
    // count (the old draft's "shape tag" reading was wrong); any valid published wall
    // satisfies it, so it does not restrict premade vs freehand. `get` already asserts
    // `is_valid()`; we keep the explicit count check for faithfulness.
    if state.simplified_curve_2d().points.len() < 2 {
        return;
    }
    // Roof-animation gate: the native roof float must be ordered-equal to 1.0, i.e. the
    // roof animation is settled (`RoofAnimation::value(0) == 1.0`).
    let anim_done = match public
        .roof_entity(wall)
        .and_then(|entity| roof_animations.get(entity).ok())
    {
        Some(animation) => !animation.inner().is_being_animated(),
        // No animation component: nothing is animating.
        None => true,
    };
    if !anim_done {
        return;
    }
    // gate1: `entry[-9]` = `PublicWallState.being_bulldozed` (see `wall_is_editable`).
    if !wall_is_editable(state) {
        return;
    }
    let roof = public.roof_component(wall, |entity| roofs.get(entity).ok());
    let flat_w_no_rim = roof.is_some_and(|roof| {
        roof.height_01() <= ROOF_FLAT_THRESHOLD && roof.observed()[0x3d] != 0
    });

    let (context, _centers) = register_widget(&ui_state, state, wall, flat_w_no_rim);

    // Widget guard (`0x140b30561`): process when the camera moved this tick, or when the
    // widget is the active hovered/clicked target (hover >= 2, id match, stage != 2).
    let widget_active = ui_state.hover_byte() >= 2
        && ui_state.active_widget() == context.id
        && ui_state.click_byte() != 2;
    if !(has_new || widget_active) {
        return;
    }

    emit_signifier(&ui_state, &mut stream, wall, context.id);

    // stage is `dragged()`: `hover >= 2 && id == self ? click_byte : 2`.
    // Native `0x140b3047b` = `movzx eax,[ctx+0x19]; add al,0xfe; cmp al,3; cmovae eax,3`
    // — an UNSIGNED u8 wrap clamp: stage {0,1,>=5} → 3 (finish), 2 → 0, 3 → 1, 4 → 2.
    // `saturating_sub` would drop release (phase 0/1) → finish; use wrapping_sub.
    let stage = context.dragged();
    let dispatch = stage.wrapping_sub(2).min(3);
    match dispatch {
        1 => {
            adjust_ui.write(AdjustWallHeightUi(wall));
            init.write(InitAdjWallHeightMode);
        }
        2 => {
            adjust_ui.write(AdjustWallHeightUi(wall));
        }
        3 => {
            finish.write(FinishAdjWallHeightMode(wall));
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resources::walls::publication_fields::WallShapeSnapshot;
    use crate::resources::walls::publication_source::TerrainHeightGrid512;
    use crate::resources::walls::{InnerWalls, WallState};
    use crate::resources::roofs::geometry::surface::Curve2;
    use crate::systems::ui::resources::ui_state::{UiInteraction, WidgetId};
    use bevy_ecs::system::RunSystemOnce;

    fn flat_terrain() -> TerrainHeightGrid512 {
        TerrainHeightGrid512 {
            encoded: vec![0u16; 512 * 512].into(),
            minimum_y: 0.0,
            maximum_y: 0.0,
            sampling_fix: true,
        }
    }

    /// Freehand wall (tag >= 2) with a `relative_heights` sample per outline point.
    fn freehand_wall_state() -> WallState {
        let curve = Curve2::try_new(vec![[0.0, 0.0], [4.0, 0.0]], false).unwrap();
        let mut state = WallState::new(curve, 2.0, 0, 0, false);
        // tag 2 = freehand, satisfying gate2 (`-0x48 >= 2`).
        state.snapshot.source.shape = WallShapeSnapshot::from_observed([2; 28]);
        state
    }

    fn world_with_wall() -> (World, WallId) {
        let mut world = World::new();
        let mut inner = InnerWalls::new();
        let wall = inner.new_wall_from_curve(freehand_wall_state());
        let mut public = PublicWalls::default();
        public.consume_with_paths(
            &mut inner,
            &flat_terrain(),
            None,
            &crate::resources::walls::public_walls::WallPublicationDirty::default(),
            &crate::resources::walls::public_walls::WallSpatialHash::default(),
        );
        world.insert_resource(inner);
        world.insert_resource(public);
        world.insert_resource(AppMode::Editing(EditSubject::Wall(wall.0)));
        world.insert_resource(UiState::new(UiInteraction {
            native_tag: 0,
            native_phase: 0,
            widget: WidgetId(0),
        }));
        world.init_resource::<Events<MainCameraMoved>>();
        world.init_resource::<Events<FinishAdjWallHeightMode>>();
        world.init_resource::<Events<AdjustWallHeightUi>>();
        world.init_resource::<Events<InitAdjWallHeightMode>>();
        world.init_resource::<UiSignifierStream>();
        world.insert_resource(crate::resources::time::GameTime::default());
        (world, wall)
    }

    fn widget_id(world: &World, wall: WallId) -> WidgetId {
        let state = world.resource::<PublicWalls>().get(wall).unwrap();
        let uh = UiWallHeight {
            wall,
            flag: 0,
            mode: 1,
            points: state
                .geometry
                .outline
                .iter()
                .map(|p| Vec3::from_array(*p))
                .collect(),
            heights: state.geometry.relative_heights.clone(),
        };
        WidgetId::from_name(&uh.widget_name())
    }

    fn set_interaction(world: &mut World, wall: WallId, hover: u8, stage: u8) {
        let widget = widget_id(world, wall);
        *world.resource_mut::<UiState>() = UiState::new(UiInteraction {
            native_tag: hover,
            native_phase: stage,
            widget,
        });
    }

    fn counts(world: &World) -> (usize, usize, usize) {
        let f = world.resource::<Events<FinishAdjWallHeightMode>>();
        let a = world.resource::<Events<AdjustWallHeightUi>>();
        let i = world.resource::<Events<InitAdjWallHeightMode>>();
        (
            f.get_cursor().read(f).count(),
            a.get_cursor().read(a).count(),
            i.get_cursor().read(i).count(),
        )
    }

    #[test]
    fn stage_three_emits_adjust_and_init() {
        let (mut world, wall) = world_with_wall();
        set_interaction(&mut world, wall, 2, 3);
        world.run_system_once(ui_adjust_wall_height).unwrap();
        assert_eq!(counts(&world), (0, 1, 1));
    }

    #[test]
    fn stage_four_emits_only_adjust() {
        let (mut world, wall) = world_with_wall();
        set_interaction(&mut world, wall, 2, 4);
        world.run_system_once(ui_adjust_wall_height).unwrap();
        assert_eq!(counts(&world), (0, 1, 0));
    }

    #[test]
    fn high_stages_five_six_and_wrap_emit_finish_with_wall_payload() {
        // Native unsigned clamp: stage 5, 6, and wrapping values (e.g. 254) all map to
        // case 3 (finish); the payload carries the edited WallId.
        for stage in [5u8, 6, 254] {
            let (mut world, wall) = world_with_wall();
            set_interaction(&mut world, wall, 2, stage);
            world.run_system_once(ui_adjust_wall_height).unwrap();
            assert_eq!(counts(&world), (1, 0, 0), "stage {stage} should emit Finish");
            let events = world.resource::<Events<FinishAdjWallHeightMode>>();
            let payloads: Vec<WallId> = events.get_cursor().read(events).map(|e| e.0).collect();
            assert_eq!(payloads, vec![wall], "stage {stage} finish payload WallId");
        }
    }

    #[test]
    fn release_phases_zero_and_one_emit_finish() {
        // Native unsigned wrap clamp: stage {0,1,>=5} → case 3 (finish). Release leaves
        // `ui_update` writing phase 0 (miss) or 1 (hit); both must emit
        // FinishAdjWallHeightMode carrying the edited WallId.
        for phase in [0u8, 1u8] {
            let (mut world, wall) = world_with_wall();
            set_interaction(&mut world, wall, 2, phase);
            world.run_system_once(ui_adjust_wall_height).unwrap();
            assert_eq!(
                counts(&world),
                (1, 0, 0),
                "release phase {phase} should emit FinishAdjWallHeightMode"
            );
            let events = world.resource::<Events<FinishAdjWallHeightMode>>();
            let payloads: Vec<WallId> = events
                .get_cursor()
                .read(events)
                .map(|event| event.0)
                .collect();
            assert_eq!(payloads, vec![wall], "finish payload must be the edited WallId");
        }
    }

    #[test]
    fn stage_two_emits_nothing() {
        let (mut world, wall) = world_with_wall();
        set_interaction(&mut world, wall, 2, 2);
        world.run_system_once(ui_adjust_wall_height).unwrap();
        assert_eq!(counts(&world), (0, 0, 0));
    }

    #[test]
    fn signifier_is_emitted_for_the_registered_widget_id() {
        let (mut world, wall) = world_with_wall();
        set_interaction(&mut world, wall, 2, 3);
        world.run_system_once(ui_adjust_wall_height).unwrap();
        assert!(
            world.resource::<UiSignifierStream>().committed_len() > 0,
            "signifier must be emitted for the registered edit_height_ widget id"
        );
    }

    #[test]
    fn camera_move_without_active_widget_emits_nothing() {
        let (mut world, _wall) = world_with_wall();
        world
            .resource_mut::<Events<MainCameraMoved>>()
            .send(MainCameraMoved);
        world.run_system_once(ui_adjust_wall_height).unwrap();
        assert_eq!(counts(&world), (0, 0, 0));
    }

    #[test]
    fn premade_wall_passes_the_count_gate() {
        // gate2 is the curve point count (`-0x48 >= 2`), not the premade tag, so a
        // premade wall (tag < 2) is NOT skipped and still emits.
        let mut world = World::new();
        let mut inner = InnerWalls::new();
        let curve = Curve2::try_new(vec![[0.0, 0.0], [4.0, 0.0]], false).unwrap();
        let wall = inner.new_wall_from_curve(WallState::new(curve, 2.0, 0, 0, false));
        let mut public = PublicWalls::default();
        public.consume_with_paths(
            &mut inner,
            &flat_terrain(),
            None,
            &crate::resources::walls::public_walls::WallPublicationDirty::default(),
            &crate::resources::walls::public_walls::WallSpatialHash::default(),
        );
        world.insert_resource(inner);
        world.insert_resource(public);
        world.insert_resource(AppMode::Editing(EditSubject::Wall(wall.0)));
        world.insert_resource(UiState::new(UiInteraction {
            native_tag: 0,
            native_phase: 0,
            widget: WidgetId(0),
        }));
        world.init_resource::<Events<MainCameraMoved>>();
        world.init_resource::<Events<FinishAdjWallHeightMode>>();
        world.init_resource::<Events<AdjustWallHeightUi>>();
        world.init_resource::<Events<InitAdjWallHeightMode>>();
        world.init_resource::<UiSignifierStream>();
        world.insert_resource(crate::resources::time::GameTime::default());
        set_interaction(&mut world, wall, 2, 5);
        world.run_system_once(ui_adjust_wall_height).unwrap();
        assert_eq!(counts(&world), (1, 0, 0));
    }

    #[test]
    fn gate1_skips_being_bulldozed_walls() {
        // gate1 (`entry[-9]`) = "being bulldozed → not editable"; candidate field is
        // `PublicWallState.being_bulldozed`. A bulldozing wall must be skipped; the same
        // wall emits once the flag clears.
        let (mut world, wall) = world_with_wall();
        world
            .resource_mut::<PublicWalls>()
            .get_mut(wall)
            .unwrap()
            .set_being_bulldozed(true);
        set_interaction(&mut world, wall, 2, 5);
        world.run_system_once(ui_adjust_wall_height).unwrap();
        assert_eq!(counts(&world), (0, 0, 0));
        world
            .resource_mut::<PublicWalls>()
            .get_mut(wall)
            .unwrap()
            .set_being_bulldozed(false);
        set_interaction(&mut world, wall, 2, 5);
        world.run_system_once(ui_adjust_wall_height).unwrap();
        assert_eq!(counts(&world), (1, 0, 0));
    }
}

/// Real physical-input chain: canonical `MouseButtonInput` + `InputCursorMoved` →
/// existing input CPU installer → `hover_intersection` picker → `ui_update` `UiState`
/// interaction → `ui_adjust_wall_height` → `AdjustWallHeightUi` / `InitAdjWallHeightMode`.
///
/// No action, interaction, click or command is manufactured: the widget is registered by
/// the system itself and picked from the real camera ray. (Downstream
/// mode → `InnerWalls` → `History` is a further leg that additionally needs
/// `AudioCursorMoved`; not exercised here.)
#[cfg(test)]
mod physical_chain {
    use super::*;
    use bevy_input::mouse::MouseButton;
    use glam::{Mat4, Vec2};

    use crate::resources::cursor_terrain_raycast::CursorTerrainRaycast;
    use crate::resources::events::{EscapeMenuJustOpened, ShutdownGameCmd};
    use crate::resources::mouse_capture::OsCursorVisibilityCmd;
    use crate::resources::mouse_terrain_raycast::CursorPositionSS;
    use crate::resources::roofs::geometry::surface::Curve2;
    use crate::resources::walls::public_walls::{WallPublicationDirty, WallSpatialHash};
    use crate::resources::walls::publication_source::TerrainHeightGrid512;
    use crate::resources::walls::{InnerWalls, WallState};
    use crate::resources::window::{WindowFocusEvent, WindowSize};
    use crate::systems::backend::input::button_input::MouseButtonInput;
    use crate::systems::backend::input::translate_input_to_cursor_moved::InputCursorMoved;
    use crate::systems::backend::input::{self, InputSystemSet};
    use crate::systems::brush_preview::OverwriteBrushPreview;
    use crate::systems::camera_rig::main_camera::MainCamera;
    use crate::systems::ui::hover_intersection::hover_intersection;
    use crate::systems::ui::resources::widget_params::PlayUiSound;
    use crate::systems::ui::ui_update::ui_update;

    fn flat_terrain() -> TerrainHeightGrid512 {
        TerrainHeightGrid512 {
            encoded: vec![0u16; 512 * 512].into(),
            minimum_y: 0.0,
            maximum_y: 0.0,
            sampling_fix: true,
        }
    }

    fn setup() -> (World, Schedule) {
        bevy_tasks::ComputeTaskPool::get_or_init(|| {
            bevy_tasks::TaskPoolBuilder::new().num_threads(2).build()
        });
        let mut world = World::new();
        // A flat wall at the world origin with zero height so the lifted widget points sit
        // at y = 0 on the camera ray through the screen centre.
        let mut inner = InnerWalls::new();
        let curve = Curve2::try_new(vec![[0.0, 0.0], [4.0, 0.0]], false).unwrap();
        let wall = inner.new_wall_from_curve(WallState::new(curve, 0.0, 0, 0, false));
        let mut public = PublicWalls::default();
        public.consume_with_paths(
            &mut inner,
            &flat_terrain(),
            None,
            &WallPublicationDirty::default(),
            &WallSpatialHash::default(),
        );
        world.insert_resource(inner);
        world.insert_resource(public);
        world.insert_resource(AppMode::Editing(EditSubject::Wall(wall.0)));
        let mut camera = MainCamera::new(1.0);
        for offset in [0x40usize, 0x80, 0xc0] {
            camera.write_matrix(offset, &Mat4::IDENTITY);
        }
        world.insert_resource(camera);
        world.insert_resource(CursorTerrainRaycast::default());
        world.insert_resource(CursorPositionSS(Vec2::splat(50.0)));
        world.insert_resource(WindowSize::new(100, 100));
        world.insert_resource(UiState::at_startup());
        world.insert_resource(UiSignifierStream::default());
        input::register_input_events(&mut world);
        // The input installer's control_input_replay needs the country-core Toolbar resource.
        world.insert_resource(crate::resources::toolbar::Toolbar::default());
        world.init_resource::<crate::resources::mouse_capture::CursorCapturedPosition>();
        world.init_resource::<crate::resources::shader_refactoring_state::ShaderRefactoringState>();
        world.init_resource::<crate::resources::gui::EscMenuState>();
        world.init_resource::<crate::game_profile::GameProfile>();
        world.init_resource::<crate::game_config::GameConfig>();
        world.init_resource::<Events<MainCameraMoved>>();
        world.init_resource::<Events<FinishAdjWallHeightMode>>();
        world.init_resource::<Events<AdjustWallHeightUi>>();
        world.init_resource::<Events<InitAdjWallHeightMode>>();
        world.init_resource::<Events<WindowFocusEvent>>();
        world.init_resource::<Events<EscapeMenuJustOpened>>();
        world.init_resource::<Events<ShutdownGameCmd>>();
        world.init_resource::<Events<OverwriteBrushPreview>>();
        world.init_resource::<Events<OsCursorVisibilityCmd>>();
        world.init_resource::<Events<PlayUiSound>>();

        let mut schedule = Schedule::default();
        input::add_input_cpu_consumers(&mut schedule);
        schedule.add_systems(
            (hover_intersection, ui_update, ui_adjust_wall_height)
                .chain()
                .in_set(InputSystemSet::Consume),
        );
        (world, schedule)
    }

    fn frame(world: &mut World, schedule: &mut Schedule, pressed: bool, position: Vec2) {
        {
            let mut mouse = world.resource_mut::<MouseButtonInput>();
            mouse.clear();
            if pressed {
                mouse.press_button(MouseButton::Left);
            } else {
                mouse.release_button(MouseButton::Left);
            }
        }
        world.send_event(InputCursorMoved::Absolute(position));
        world
            .resource_mut::<bevy_time::Time>()
            .advance_by(std::time::Duration::from_millis(16));
        schedule.run(world);
        world.flush();
    }

    fn adjust_n(world: &World) -> usize {
        let e = world.resource::<Events<AdjustWallHeightUi>>();
        e.get_cursor().read(e).count()
    }
    fn init_n(world: &World) -> usize {
        let e = world.resource::<Events<InitAdjWallHeightMode>>();
        e.get_cursor().read(e).count()
    }
    fn finish_n(world: &World) -> usize {
        let e = world.resource::<Events<FinishAdjWallHeightMode>>();
        e.get_cursor().read(e).count()
    }

    #[test]
    fn real_mouse_press_reaches_adjust_and_init_events_through_the_picker() {
        let (mut world, mut schedule) = setup();
        let start = Vec2::splat(50.0);
        frame(&mut world, &mut schedule, false, start); // register widget + ray
        frame(&mut world, &mut schedule, false, start); // hover the widget
        frame(&mut world, &mut schedule, true, start); // press → stage 3
        let ui = world.resource::<UiState>();
        assert!(
            ui.hover_byte() >= 2,
            "press should activate the height widget; hover={} widget={:?}",
            ui.hover_byte(),
            ui.active_widget()
        );
        assert!(adjust_n(&world) >= 1, "expected AdjustWallHeightUi on press");
        assert!(init_n(&world) >= 1, "expected InitAdjWallHeightMode on press");
        // Drag: move the cursor while pressed, then release.
        frame(&mut world, &mut schedule, true, Vec2::new(60.0, 50.0)); // drag continue
        frame(&mut world, &mut schedule, true, Vec2::new(70.0, 50.0)); // drag continue
        frame(&mut world, &mut schedule, false, Vec2::new(70.0, 50.0)); // release
        // Release wraps stage 0/1 → case 3 (native unsigned clamp), so Finish IS emitted.
        assert!(
            adjust_n(&world) >= 2,
            "drag continues should emit AdjustWallHeightUi (got {})",
            adjust_n(&world)
        );
        assert!(
            finish_n(&world) >= 1,
            "release should emit FinishAdjWallHeightMode via the wrapping clamp"
        );
    }

    fn setup_full() -> (World, Schedule, WallId) {
        use crate::input::action_set::MainActionSet;
        use crate::resources::mouse_terrain_raycast::{CameraCursorRay, camera_mouse_ray};
        use crate::resources::onboarding::NotifyHintSystemShapeEditWasPerformed;
        use crate::resources::walls::decorator_archivist::LiveDecoratorArchivist;
        use crate::resources::walls::decorator_storage::DecoratorStorage;
        use crate::resources::window::AudioCursorMoved;
        use crate::systems::audio::{PlayBuildingAudio, RequestWallMaterialAudio};
        use crate::systems::collision::world::RaycastWorld;
        use crate::systems::wall::adjust_wall_height::AdjustWallHeightCmd;
        use crate::systems::wall::adjust_wall_height::adjust_wall_height_impl::adjust_wall_height_impl;
        use crate::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::{
            AdjWallHeightModeCache, ui_adjust_wall_height_mode,
        };
        use crate::systems::wall::wall_state::publish_public_wall_state::WallPublicationTerrain;

        let (mut world, mut schedule) = setup();
        let wall = world
            .resource::<PublicWalls>()
            .wall_ids()
            .next()
            .expect("setup built a wall");
        world.init_resource::<MainActionSet>();
        world.init_resource::<CameraCursorRay>();
        world.init_resource::<RaycastWorld>();
        #[cfg(feature = "native-physics")]
        world.init_resource::<crate::resources::walls::decorator_collision::DecoratorPhysicsColliders>();
        world.init_resource::<LiveDecoratorArchivist>();
        world.init_resource::<DecoratorStorage>();
        world.init_resource::<AdjWallHeightModeCache>();
        world.insert_resource(WallPublicationTerrain(flat_terrain()));
        world.init_resource::<Events<AdjustWallHeightCmd>>();
        world.init_resource::<Events<RequestWallMaterialAudio>>();
        world.init_resource::<Events<PlayBuildingAudio>>();
        world.init_resource::<Events<AudioCursorMoved>>();
        world.init_resource::<Events<NotifyHintSystemShapeEditWasPerformed>>();
        // History recording target for the finish leg.
        let mut history = crate::resources::history::History::<()>::after_observed_future_clear();
        history.start_input_recording(0).unwrap();
        world.insert_resource(history);
        // Undo/redo application pipeline: history_process → replay_wall → pop.
        world.init_resource::<crate::systems::history::PendingHistoryCommands>();
        world.insert_resource(crate::resources::history::ReplayBuffer::new(1.0));
        world.init_resource::<Events<crate::systems::mode_manager::HistoryCommand>>();
        world.init_resource::<Events<crate::systems::history::history_process::DebugHistoryMsg>>();
        world.init_resource::<Events<crate::systems::history::history_process::NewReplayCommandEvent>>();
        world.init_resource::<Events<crate::systems::audio::PlayReplayAudio>>();
        world.init_resource::<Events<crate::systems::timelapse_replay::InterruptTimelapse>>();
        world.init_resource::<Events<crate::systems::history::HistoryReplayFailed>>();
        world.init_resource::<Events<crate::systems::history::replay_wall::WallHistoryReplayAudio>>();
        world.init_resource::<Events<crate::systems::history::replay_wall::WallHistoryRestoreColor>>();
        world.init_resource::<Events<crate::systems::history::ReplayBucketFinished>>();
        world.init_resource::<crate::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>();

        schedule.add_systems(
            (
                camera_mouse_ray,
                ui_adjust_wall_height_mode,
                adjust_wall_height_impl,
                crate::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::history_record_adjust_wall_height::<()>,
                crate::systems::history::history_process_system::<()>,
                crate::systems::history::replay_wall::replay_wall,
                crate::systems::history::pop_empty_replay_bucket,
            )
                .chain()
                .in_set(InputSystemSet::Consume)
                .after(ui_adjust_wall_height),
        );
        (world, schedule, wall)
    }

    /// Real input → AdjustWallHeightUi/Finish → mode → AdjustWallHeightCmd → impl → `InnerWalls`
    /// height; Finish → history_record → `History`; then real undo/redo
    /// (`history_process` → `replay_wall`) restores and re-applies the height.
    #[test]
    fn real_input_reaches_history_and_undo_redo_applies_to_inner_walls() {
        use crate::resources::history::EditType;
        use crate::resources::history::wall_edit::WallHistoryEdit;
        let (mut world, mut schedule, wall) = setup_full();
        let start = Vec2::splat(50.0);
        frame(&mut world, &mut schedule, false, start); // register
        frame(&mut world, &mut schedule, false, start); // hover
        frame(&mut world, &mut schedule, true, start); // press
        frame(&mut world, &mut schedule, true, Vec2::new(50.0, 40.0)); // drag init
        frame(&mut world, &mut schedule, true, Vec2::new(50.0, 30.0)); // drag update
        frame(&mut world, &mut schedule, false, Vec2::new(50.0, 30.0)); // release → finish
        let height = world.resource::<InnerWalls>().get(wall).map(|s| s.height());
        assert!(
            height.is_some_and(|h| h != 0.0),
            "InnerWalls height should change from 0 via mode→impl; got {height:?}"
        );
        let (from, to) = {
            let history = world.resource::<crate::resources::history::History<()>>();
            let entry = history.pending().expect("pending entry");
            let EditType::Wall(compound) = &entry.edits[0] else {
                panic!("expected wall edit")
            };
            let WallHistoryEdit::AdjustHeight { from, to, .. } = &compound.groups[0][0] else {
                panic!("expected AdjustHeight edit")
            };
            (*from, *to)
        };
        world
            .resource_mut::<crate::resources::history::History<()>>()
            .finish_input_recording()
            .expect("history finish succeeds")
            .expect("committed entry");
        // Real undo via the canonical HistoryCommand::UNDO ([0,0,0,0]). No real producer sets
        // byte2 (the consumer's instant flag), so drive the animated replay over many frames
        // until the height settles at the pre-edit value.
        world.send_event(crate::systems::mode_manager::HistoryCommand::UNDO);
        for _ in 0..400 {
            tick(&mut world, &mut schedule);
        }
        let after_undo = world
            .resource::<InnerWalls>()
            .get(wall)
            .map(|s| s.height())
            .unwrap();
        assert!(
            (after_undo - from).abs() < 1e-3,
            "undo should restore pre-edit height {from}; got {after_undo}"
        );
        // Real redo via the canonical HistoryCommand::REDO ([0,1,0,1]), animated to completion.
        world.send_event(crate::systems::mode_manager::HistoryCommand::REDO);
        for _ in 0..400 {
            tick(&mut world, &mut schedule);
        }
        let after_redo = world
            .resource::<InnerWalls>()
            .get(wall)
            .map(|s| s.height())
            .unwrap();
        assert!(
            (after_redo - to).abs() < 1e-3,
            "redo should re-apply edited height {to}; got {after_redo}"
        );
    }

    fn tick(world: &mut World, schedule: &mut Schedule) {
        world
            .resource_mut::<bevy_time::Time>()
            .advance_by(std::time::Duration::from_millis(16));
        schedule.run(world);
        world.flush();
    }
}
