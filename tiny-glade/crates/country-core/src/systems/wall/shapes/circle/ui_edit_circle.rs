//! B `systems/wall/shapes/circle/ui_edit_circle.rs` — `ui_edit_circle` (VA `0x14165d460`, 30 params).
//!
//! Spec §6. This rebuilds the file from the invented 5-param event shell to the **30-param
//! PDB-ordered FnSystem structure**. The evidence-unrecovered pieces — `CircleGrabParams` /
//! `CircleSignifierParams` Local fields (defaults UNKNOWN) and the exact wall-selection via
//! `UiState` + `WallTriggerVolume` + SS→ray→ground radius math — are `TODO(evidence)`.
//! Nothing here is claimed complete.

use bevy_ecs::prelude::*;

use glam::Vec2;

use crate::resources::critters::sheep::CmdCheckForSheepInsideBuilding;
use crate::resources::glade_border::{DisplayOutOfBorder, FeedbackInputOutOfBorder, GladeBorder};
use crate::resources::history::History;
use crate::resources::mouse_terrain_raycast::CursorPositionSS;
use crate::resources::onboarding::NotifyHintSystemShapeEditWasPerformed;
use crate::resources::time::GameTime;
use crate::resources::tree_storage::HideTreesinAreaCmd;
use crate::resources::walls::decorator_archivist::{
    DecoratorArchivalResources, LiveDecoratorArchivist,
};
use crate::resources::walls::decorator_storage::DecoratorStorage;
use crate::resources::walls::public_walls::{PublicWalls, WallEntities};
use crate::resources::walls::wall_trigger_volume::WallTriggerVolume;
use crate::resources::walls::{InnerWalls, WallId, WallState};
use crate::resources::window::WindowSize;
use crate::systems::audio::{PlayBuildingAudio, RequestWallMaterialAudio};
use crate::systems::camera_rig::main_camera::{MainCamera, MainCameraMoved};
use crate::systems::mode_manager::{AppMode, EditSubject};
use crate::systems::ui::resources::ui_state::UiState;
use crate::systems::ui::signifiers::signifier_create::{UiAppendSignifier, UiSignifierStream};
use crate::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter;
use crate::systems::wall::wall_highlight::HighlightWallCmd;

/// Radius-change history threshold (0x3c23d70a @0x142960674) — S5 release decision.
pub const CHANGE_THRESHOLD: f32 = 0.01;
/// Circle sample step (0x3e4ccccd @0x1429cd068) — S4 `Circle2d::as_curve2`.
const CIRCLE_SAMPLE_STEP: f32 = 0.2;
/// S2 radius bounds (native 0.9 / 8.0).
pub const CIRCLE_MIN_RADIUS: f32 = 0.9;
pub const CIRCLE_MAX_RADIUS: f32 = 8.0;

/// S2 radius clamp: `clamp(0.9, 8.0)`.
pub fn clamp_circle_radius(radius: f32) -> f32 {
    radius.clamp(CIRCLE_MIN_RADIUS, CIRCLE_MAX_RADIUS)
}

/// S5 release decision: `|Δr| > 0.01` → commit (else restore). NaN → changed (native
/// `UCOMISS` unordered is treated as "not equal").
pub fn circle_radius_changed(previous: f32, current: f32) -> bool {
    let delta = (current - previous).abs();
    !(delta <= CHANGE_THRESHOLD)
}

/// S4 apply: `Circle2d::new(center, r).as_curve2(0.2)` → `update_wall_state_curve` →
/// `update_wall_state_enclosure` (circle snapshot). Returns false if the curve update fails.
fn apply_circle_radius(
    inner: &mut InnerWalls,
    wall: WallId,
    center: glam::Vec2,
    radius: f32,
) -> bool {
    let circle = ::utils::geometry::Circle2d::new(center, radius);
    let curve = circle.as_curve2(CIRCLE_SAMPLE_STEP);
    inner.update_wall_state_curve(wall, crate::systems::wall::shapes::to_authority_curve(&curve));
    let snapshot = crate::resources::walls::publication_fields::WallShapeSnapshot::from_observed(
        crate::systems::wall::valid_enclosure::ShapeParameters::from_shape(
            crate::systems::wall::valid_enclosure::Shape::Circle(circle),
        )
        .to_record(),
    );
    inner.update_wall_state_enclosure(wall, snapshot)
}

/// B `AnimatedCircleSignifierResource` (size 64): circle / rectangle / freehand signifier
/// state shared by `signifier_edit_circle_arrow` (0x140b35120) and `..._line` (0x140a64a40).
/// Registration default only writes `+0x38 = 2` (Freehand); the other fields are UNKNOWN
/// and zeroed here (not fabricated business defaults).
#[repr(C)]
#[derive(bevy_ecs::resource::Resource, Clone, Copy, Debug)]
pub struct AnimatedCircleSignifierResource {
    /// +0x00..0x18: registration only writes `+0x38`; these bytes are UNKNOWN (zeroed here).
    pub unknown_00: [u8; 0x18],
    /// +0x18 `f32` base angle.
    pub angle_base: f32,
    /// +0x1c..0x20 padding.
    pub pad_1c: [u8; 0x4],
    /// +0x20 centre/origin `Vec3` (x@0x20, y@0x24, z@0x28).
    pub position: glam::Vec3,
    /// +0x2c axis normal `Vec3` (x@0x2c, y@0x30, z@0x34).
    pub normal: glam::Vec3,
    /// +0x38 shape tag: 0 = Circle, 1 = Rectangle, 2 = Freehand.
    pub tag: u8,
    /// +0x39..0x40 padding.
    pub pad_39: [u8; 0x7],
}

impl Default for AnimatedCircleSignifierResource {
    fn default() -> Self {
        Self {
            unknown_00: [0; 0x18],
            angle_base: 0.0,
            pad_1c: [0; 0x4],
            position: glam::Vec3::ZERO,
            normal: glam::Vec3::ZERO,
            tag: 2,
            pad_39: [0; 0x7],
        }
    }
}

/// B `ui_edit_circle::CircleGrabParams` (Local). Fields/default are UNKNOWN — the
/// `impl::default` body was not recovered (interleaved with resources in the decompile).
/// Empty structure only; do NOT fabricate behavior.
#[derive(Default, Debug)]
pub struct CircleGrabParams {
    // UNKNOWN fields (default body not recovered).
}

/// B `ui_edit_circle::CircleSignifierParams` (Local). Fields/default UNKNOWN; empty structure only.
#[derive(Default, Debug)]
pub struct CircleSignifierParams {
    // UNKNOWN fields (default body not recovered).
}

/// Legacy self-invented edit state (still registered by `shapes/mod.rs`; removed at step 7).
#[derive(Resource, Clone, Debug, Default)]
pub struct CircleEditState {
    pub previous: Option<WallState>,
}

/// B 0x141660880: full-circle preview signifier. NOTE: the native 0x28 record carries
/// `{1,1,center.x,center.y,circle_rec[0],circle_rec[+0xc],radius_a,radius_b}`; this legacy
/// 5-qword `UiAppendSignifier` form is a placeholder to be replaced with the real factory
/// record once the circle_signifier consumer contract is confirmed. (Not §6-body closure.)
pub fn show_full_circle_signifier(
    stream: &mut UiSignifierStream,
    center: Vec2,
    radius: f32,
) {
    stream.write(UiAppendSignifier {
        tag: 1,
        kind: 1,
        position: glam::Vec3::new(center.x, 0.0, center.y),
        scale: radius,
        name: 0,
    });
}

/// B `ui_edit_circle` (`0x14165d460`, 30 params, PDB order).
///
/// STRUCTURE ONLY: AppMode gate (`mode<=5`) wired; the circle-radius edit body is
/// `TODO(evidence)` (wall selection via `UiState`+`WallTriggerVolume`, SS→ray→ground radius,
/// border events, `show_full_circle_signifier(3.0/5.0)`, and the 0.01-threshold history/
/// restore decision). `CircleGrabParams`/`CircleSignifierParams` defaults are UNKNOWN.
#[allow(clippy::too_many_arguments, unused_variables)]
pub fn ui_edit_circle<S: Send + Sync + 'static>(
    app_mode: Res<AppMode>,                                              // 1
    mut inner: ResMut<InnerWalls>,                                       // 2
    public: Res<PublicWalls>,                                            // 3
    entities: Res<WallEntities>,                                         // 4
    trigger_volumes: Query<&WallTriggerVolume>,                          // 5
    archival: DecoratorArchivalResources,                                // 6
    border: Res<GladeBorder>,                                            // 7
    ui_state: Res<UiState>,                                              // 8
    mut signifiers: ResMut<UiSignifierStream>,                           // 9
    mut grab: Local<CircleGrabParams>,                                   // 10
    mut signifier_params: Local<CircleSignifierParams>,                  // 11
    mut animated: ResMut<AnimatedCircleSignifierResource>,               // 12
    cursor_ss: Res<CursorPositionSS>,                                    // 13
    window: Res<WindowSize>,                                             // 14
    camera: Res<MainCamera>,                                             // 15
    mut archivist: ResMut<LiveDecoratorArchivist>,                       // 16
    mut storage: ResMut<DecoratorStorage>,                               // 17
    game_time: Res<GameTime>,                                            // 18
    time: Res<bevy_time::Time>,                                          // 19
    mut camera_moved: EventReader<MainCameraMoved>,                      // 20
    mut out_of_border: EventWriter<DisplayOutOfBorder>,                  // 21
    mut feedback_out_of_border: EventWriter<FeedbackInputOutOfBorder>,   // 22
    mut highlight: EventWriter<HighlightWallCmd>,                        // 23
    mut hint: EventWriter<NotifyHintSystemShapeEditWasPerformed>,        // 24
    mut sheep_check: EventWriter<CmdCheckForSheepInsideBuilding>,        // 25
    mut building_audio: EventWriter<PlayBuildingAudio>,                  // 26
    mut material_audio: EventWriter<RequestWallMaterialAudio>,           // 27
    mut hide_trees: EventWriter<HideTreesinAreaCmd>,                     // 28
    mut show_pins: EventWriter<ShowPinnedDecoClutter>,                   // 29
    mut history: ResMut<History<S>>,                                     // 30
) {
    // S0 gate: `mode <= 5`; only Editing(Wall) / Editing(Roof) (native tag 1/2) proceed.
    let tag = app_mode.native_tag();
    if tag > 5 {
        return;
    }
    if !matches!(
        *app_mode,
        AppMode::Editing(EditSubject::Wall(_)) | AppMode::Editing(EditSubject::Roof(_))
    ) {
        return;
    }
    // Consume the MainCameraMoved reader for this frame.
    let _ = camera_moved.read().last();
    // TODO(evidence): wall = UiState::dragged()/intersecting() + WallTriggerVolume Query;
    // assert simplified_curve_2d valid; update_circle_radius (SS→ray→ground; radius
    // clamp(0.9, 8.0), threshold 0.01); out-of-border → DisplayOutOfBorder + FeedbackInputOutOfBorder;
    // Circle2d::as_curve2(0.2) → InnerWalls::update_wall_state_curve/enclosure;
    // show_full_circle_signifier(3.0/5.0); release: |Δr|<=0.01 → extract_and_restore_to_storage,
    // else CompoundEditBuilder + History::add + NotifyHint + CmdCheckForSheep.
    // CircleGrabParams / CircleSignifierParams defaults = UNKNOWN (do not fabricate).
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn locals_and_resource_defaults() {
        let grab = CircleGrabParams::default();
        let _ = &grab;
        let signifier = CircleSignifierParams::default();
        let _ = &signifier;
        // AnimatedCircleSignifierResource default tag = 2 (Freehand) per registration.
        assert_eq!(AnimatedCircleSignifierResource::default().tag, 2);
        assert_eq!(
            std::mem::size_of::<AnimatedCircleSignifierResource>(),
            64
        );
    }

    #[test]
    fn s0_numeric_helpers_clamp_and_threshold() {
        // S2 clamp bounds 0.9 / 8.0.
        assert_eq!(clamp_circle_radius(0.1), 0.9);
        assert_eq!(clamp_circle_radius(100.0), 8.0);
        assert_eq!(clamp_circle_radius(4.0), 4.0);
        // S5 threshold 0.01 (NaN → changed, matching native UCOMISS unordered).
        assert!(!circle_radius_changed(2.0, 2.005));
        assert!(circle_radius_changed(2.0, 2.02));
        assert!(circle_radius_changed(f32::NAN, 2.0));
    }

    #[test]
    fn s4_apply_circle_updates_curve_and_enclosure_radius() {
        use crate::resources::roofs::geometry::surface::Curve2;
        let mut inner = InnerWalls::new();
        let curve = Curve2::try_new(vec![[0.0, 0.0], [4.0, 0.0]], false).unwrap();
        let wall = inner.new_wall_from_curve(WallState::new(curve, 2.0, 0, 0, false));
        let applied = apply_circle_radius(&mut inner, wall, glam::Vec2::new(1.0, 1.0), 3.0);
        assert!(applied, "apply_circle_radius should update the wall curve");
        let params = inner
            .get(wall)
            .unwrap()
            .get_enclosure_params()
            .expect("circle enclosure params");
        let radius =
            crate::systems::wall::valid_enclosure::ShapeParameters::from_snapshot(params)
                .as_circle()
                .radius;
        assert!((radius - 3.0).abs() < 1e-3, "enclosure radius {radius}");
    }
}
