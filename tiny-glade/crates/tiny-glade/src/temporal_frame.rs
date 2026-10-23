//! Candidate camera history, committed once after image-writing submission.
use bevy_ecs::{
    component::Tick,
    world::{World, WorldId},
};
use country_core::{
    render::{
        RenderTemporalState, frame_constants::FrameConstants, render_config::FrameRenderConfig,
    },
    resources::shader_refactoring_state::ShaderRefactoringState,
    systems::{
        backend::prepare_frame_constants::{USE_TAA_JITTER, prepare_camera_views},
        camera_rig::MainCamera,
    },
};

#[derive(Clone, Copy, Default)]
pub(crate) struct LateInputMark {
    sequence: u64,
    barrier: u64,
    motion: glam::DVec2,
}

/// OS input remains owned by the ordinary input pipeline. This bounded mailbox
/// only exposes a non-consuming preview of motion after its consumed watermark.
#[derive(Clone, Default)]
pub(crate) struct LateCameraMailbox(std::sync::Arc<parking_lot::Mutex<LateInputMark>>);

impl LateCameraMailbox {
    pub(crate) fn mark(&self) -> LateInputMark { *self.0.lock() }

    pub(crate) fn invalidate(&self) {
        let mut state = self.0.lock();
        state.sequence = state.sequence.wrapping_add(1);
        state.barrier = state.barrier.wrapping_add(1);
        state.motion = glam::DVec2::ZERO;
    }

    pub(crate) fn observe(&self, event: &crate::event_loop::OsWindowEvent) {
        if let crate::event_loop::OsWindowEvent::CursorRelative(delta) = event {
            let delta = glam::DVec2::new(delta[0] as f64, delta[1] as f64);
            if delta.is_finite() {
                let mut state = self.0.lock();
                state.sequence = state.sequence.wrapping_add(1);
                state.motion += delta;
                return;
            }
        }
        // A button/binding/capture/extent change can change the meaning of
        // subsequent motion. Never preview across such an unconsumed boundary.
        self.invalidate();
    }
}

#[derive(bevy_ecs::resource::Resource, Clone)]
pub(crate) struct LateCameraFrame {
    source: WorldId,
    camera_ticks: (Tick, Tick),
    mailbox: LateCameraMailbox,
    consumed: LateInputMark,
    rotation: country_core::systems::camera_rig::main_camera::CameraRotationPreview,
    sensitivity: f32,
    invert_x: bool,
    invert_y: bool,
}

impl LateCameraFrame {
    pub(crate) fn capture(game: &World, mailbox: &LateCameraMailbox, consumed: LateInputMark) -> Option<Self> {
        use country_core::{
            game_config::GameConfig,
            input::{InputRecordReplayMode, action_set::{MainActionSet, DigitalActionPhase}},
            resources::mouse_capture::CursorCapturedPosition,
            systems::{backend::input::input_record_replay::InputRecordReplayState,
                main_camera::{CameraSnapshot, CameraSnapshotInterpolation},
                mode_manager::{AppMode, EditSubject}, render_loop::vermeer::VermeerState},
        };
        let actions = game.get_resource::<MainActionSet>()?;
        if actions.digital[2].phase != DigitalActionPhase::Held
            || actions.digital.iter().enumerate().any(|(slot, action)| slot != 2 && action.phase != DigitalActionPhase::Idle)
            || actions.analog[2..].iter().any(|value| *value != glam::Vec2::ZERO)
            || game.get_resource::<CursorCapturedPosition>()?.0.is_none()
            || !matches!(game.get_resource::<InputRecordReplayState>()?.mode,
                InputRecordReplayMode::None | InputRecordReplayMode::AutoRecordPanicRepro)
            || game.get_resource::<CameraSnapshot>()?.interpolation != CameraSnapshotInterpolation::Disabled
            || game.get_resource::<VermeerState>()?.should_freeze_camera()
            || !matches!(game.get_resource::<AppMode>()?, AppMode::Editing(EditSubject::None) | AppMode::Photomode)
        { return None; }
        let config = &game.get_resource::<GameConfig>()?.game;
        let ticks = game.get_resource_change_ticks::<MainCamera>()?;
        Some(Self {
            source: game.id(), camera_ticks: (ticks.added, ticks.changed),
            mailbox: mailbox.clone(), consumed,
            rotation: game.get_resource::<MainCamera>()?.rotation_preview()?,
            sensitivity: config.camera_rotation_sensitivity,
            invert_x: config.camera_invert_x, invert_y: config.camera_invert_y,
        })
    }

    pub(crate) fn attach(frame: Option<&Self>, game: &World, render: &mut World) {
        render.remove_resource::<LatchedCameraInput>();
        render.remove_resource::<LateCameraFrame>();
        if let Some(frame) = frame.filter(|frame| frame.source == game.id()
            && game.get_resource_change_ticks::<MainCamera>()
                .is_some_and(|ticks| (ticks.added, ticks.changed) == frame.camera_ticks)) {
            render.insert_resource(frame.clone());
        }
    }
}

/// Diagnostic identity of the one camera used by all declaration consumers.
/// Pending preparation and resolution retries retain this exact version.
#[derive(bevy_ecs::resource::Resource, Clone, Copy, Debug)]
pub(crate) struct LatchedCameraInput {
    pub source: WorldId,
    pub consumed_sequence: u64,
    pub sampled_sequence: u64,
    pub previewed: bool,
}

/// Last input sample before draw lists, culling, LOD and frame constants. This
/// changes the render world's isolated camera only, never the game rig, input
/// reader or editing projection. Calling it again while Pending is a no-op.
pub(crate) fn latch_camera_preview(render: &mut World) {
    if render.contains_resource::<LatchedCameraInput>() { return; }
    let Some(frame) = render.get_resource::<LateCameraFrame>().cloned() else { return; };
    let latest = frame.mailbox.mark();
    let motion = (latest.motion - frame.consumed.motion).as_vec2();
    let mut previewed = false;
    if latest.barrier == frame.consumed.barrier && motion != glam::Vec2::ZERO {
        let mut yaw = ((motion.x * -1.2) * 0.25) * frame.sensitivity;
        let mut pitch = (motion.y * -0.25) * frame.sensitivity;
        if frame.invert_x { yaw = -yaw; }
        if frame.invert_y { pitch = -pitch; }
        if let Some(mut camera) = render.get_resource_mut::<MainCamera>() {
            previewed = frame.rotation.apply(&mut camera, yaw, pitch);
        }
    }
    render.insert_resource(LatchedCameraInput { source: frame.source,
        consumed_sequence: frame.consumed.sequence, sampled_sequence: latest.sequence, previewed });
}

pub(crate) struct TemporalCommit {
    world: WorldId,
    ticks: (Tick, Tick),
    camera_added: Option<Tick>,
    candidate: RenderTemporalState,
}

#[derive(bevy_ecs::resource::Resource)]
pub(crate) struct RenderCameraCandidate(pub RenderTemporalState);

fn camera_candidate(world: &mut World) -> Option<RenderTemporalState> {
    let mut candidate = *world.get_resource::<RenderTemporalState>()?;
    let config = *world.resource::<FrameRenderConfig>();
    let views = prepare_camera_views(
        world.resource::<MainCamera>(), &mut candidate, config.extent_10,
        world.resource::<utils::camera::TaaJitterSet>(),
        world.resource::<ShaderRefactoringState>(),
        USE_TAA_JITTER.load(std::sync::atomic::Ordering::Relaxed),
    );
    let mut constants = world.resource_mut::<FrameConstants>();
    constants.views[..2].copy_from_slice(&views);
    constants.frame_counter = candidate.frame_index;
    constants.inner_rendering_resolution = config.extent_10;
    constants.global_mip_bias = (config.extent_10[0] as f32 / (config.extent_8[0] as f32).max(1.0)).log2();
    Some(candidate)
}

/// The worker may reduce resolution during compilation. Publish the camera
/// history corresponding to its actual image, instead of committing the older
/// game-side preview after that image is submitted.
pub(crate) fn prepare_render_candidate(world: &mut World) {
    if let Some(candidate) = camera_candidate(world) {
        world.insert_resource(RenderCameraCandidate(candidate));
    }
}

impl TemporalCommit {
    /// Refresh only camera constants after the preceding frame's feedback.
    /// Simulation may have run while that frame was still rendering.
    pub(crate) fn prepare(world: &mut World) -> Option<Self> {
        let ticks = world.get_resource_change_ticks::<RenderTemporalState>()?;
        let ticks = (ticks.added, ticks.changed);
        let candidate = camera_candidate(world)?;
        Some(Self {
            world: world.id(),
            ticks,
            camera_added: world.get_resource_change_ticks::<MainCamera>().map(|ticks| ticks.added),
            candidate,
        })
    }

    pub(crate) fn finish(mut self, game: &mut World, history_submitted: bool, actual: Option<RenderTemporalState>) {
        if !history_submitted || game.id() != self.world {
            return;
        }
        // Ordinary camera movement during the following Update is expected.
        // Removal/reinsertion installs a different camera owner, however: an
        // image from the retired owner must not seed that camera's history.
        if game.get_resource_change_ticks::<MainCamera>().map(|ticks| ticks.added)
            != self.camera_added
        {
            return;
        }
        let Some(ticks) = game.get_resource_change_ticks::<RenderTemporalState>() else {
            return;
        };
        // A resize/reset/remove-reinsert during Update invalidates this older
        // candidate, even if its GPU work subsequently finishes successfully.
        if (ticks.added, ticks.changed) != self.ticks {
            return;
        }
        if let Some(actual) = actual { self.candidate = actual; }
        self.candidate.frame_render_returned();
        *game.resource_mut::<RenderTemporalState>() = self.candidate;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use country_core::render::FrameCameraMatrices;

    fn late_fixture() -> World {
        use country_core::{
            game_config::GameConfig,
            input::action_set::{MainActionSet, DigitalActionPhase},
            resources::mouse_capture::CursorCapturedPosition,
            systems::{backend::input::input_record_replay::InputRecordReplayState,
                camera_rig::main_camera::CameraMode,
                main_camera::CameraSnapshot, mode_manager::AppMode,
                render_loop::vermeer::{VermeerState, VermeerPhase}},
        };
        let mut world = World::new();
        let mut camera = MainCamera::new(1.0);
        camera.set_mode_byte(CameraMode::FREE);
        world.insert_resource(camera);
        let mut actions = MainActionSet::default();
        actions.digital[2].phase = DigitalActionPhase::Held;
        world.insert_resource(actions);
        world.insert_resource(CursorCapturedPosition(Some(glam::Vec2::splat(50.0))));
        world.insert_resource(InputRecordReplayState::default());
        world.insert_resource(CameraSnapshot::default());
        world.insert_resource(VermeerState { phase: VermeerPhase::Disabled, ..Default::default() });
        world.insert_resource(AppMode::default());
        world.insert_resource(GameConfig::default());
        world
    }

    #[test]
    fn late_motion_after_extract_is_latched_once_and_does_not_consume_game_input() {
        use country_core::systems::backend::input::translate_input_to_cursor_moved::InputCursorMoved;
        let mut game = late_fixture();
        game.init_resource::<bevy_ecs::event::Events<InputCursorMoved>>();
        game.send_event(InputCursorMoved::Relative(glam::Vec2::new(20.0, 4.0)));
        let mailbox = LateCameraMailbox::default();
        let frame = LateCameraFrame::capture(&game, &mailbox, mailbox.mark()).unwrap();
        let mut render = World::new();
        render.insert_resource(game.resource::<MainCamera>().render_snapshot());
        LateCameraFrame::attach(Some(&frame), &game, &mut render);
        let original = game.resource::<MainCamera>().read_matrix(0xc0);
        mailbox.observe(&crate::event_loop::OsWindowEvent::CursorRelative([20.0, 4.0]));
        latch_camera_preview(&mut render);
        let latched = render.resource::<MainCamera>().read_matrix(0xc0);
        assert_ne!(latched, original, "must sample input arriving after extraction");
        let version = *render.resource::<LatchedCameraInput>();
        assert!(version.previewed);
        assert_eq!(version.source, game.id());
        assert!(version.sampled_sequence > version.consumed_sequence);
        mailbox.observe(&crate::event_loop::OsWindowEvent::CursorRelative([30.0, 5.0]));
        latch_camera_preview(&mut render);
        assert_eq!(render.resource::<MainCamera>().read_matrix(0xc0), latched,
            "cold Pending/retry must not change an already declared camera");
        assert_eq!(game.resource::<MainCamera>().read_matrix(0xc0), original);
        assert_eq!(game.resource::<bevy_ecs::event::Events<InputCursorMoved>>().len(), 1);
    }

    #[test]
    fn late_camera_freezes_editing_replay_other_actions_and_new_os_boundaries() {
        use country_core::{input::{InputRecordReplayMode, action_set::{MainActionSet, DigitalActionPhase}},
            systems::{backend::input::input_record_replay::InputRecordReplayState, mode_manager::AppMode}};
        let mailbox = LateCameraMailbox::default();
        for blocked in 0..5 {
            let mut game = late_fixture();
            match blocked {
                0 => *game.resource_mut::<AppMode>() = AppMode::Terrain,
                1 => game.resource_mut::<InputRecordReplayState>().mode = InputRecordReplayMode::Replay,
                2 => game.resource_mut::<MainActionSet>().digital[0].phase = DigitalActionPhase::JustPressed,
                3 => game.resource_mut::<MainActionSet>().analog[4].x = 1.0,
                _ => game.resource_mut::<MainActionSet>().digital[2].phase = DigitalActionPhase::JustPressed,
            }
            assert!(LateCameraFrame::capture(&game, &mailbox, mailbox.mark()).is_none());
        }
        let mut game = late_fixture();
        let frame = LateCameraFrame::capture(&game, &mailbox, mailbox.mark()).unwrap();
        let mut render = World::new();
        render.insert_resource(game.resource::<MainCamera>().render_snapshot());
        LateCameraFrame::attach(Some(&frame), &game, &mut render);
        mailbox.observe(&crate::event_loop::OsWindowEvent::CursorRelative([20.0, 4.0]));
        mailbox.observe(&crate::event_loop::OsWindowEvent::Focused(false));
        latch_camera_preview(&mut render);
        assert!(!render.resource::<LatchedCameraInput>().previewed);
        assert_eq!(render.resource::<MainCamera>().read_matrix(0xc0), game.resource::<MainCamera>().read_matrix(0xc0));
        game.increment_change_tick();
        game.resource_mut::<MainCamera>().write_matrix(0xc0,
            &glam::Mat4::from_translation(glam::Vec3::X));
        LateCameraFrame::attach(Some(&frame), &game, &mut render);
        assert!(!render.contains_resource::<LateCameraFrame>(), "newer simulation camera wins over stale preview");
        let replacement = World::new();
        LateCameraFrame::attach(Some(&frame), &replacement, &mut render);
        assert!(!render.contains_resource::<LateCameraFrame>());
        assert!(!render.contains_resource::<LatchedCameraInput>());
    }

    #[test]
    fn late_camera_resolution_retry_keeps_pose_and_actual_temporal_candidate() {
        use country_core::{game_config::GameConfig, resources::window::WindowSize};
        let game = late_fixture();
        let mailbox = LateCameraMailbox::default();
        let frame = LateCameraFrame::capture(&game, &mailbox, mailbox.mark()).unwrap();
        let mut render = World::new();
        render.insert_resource(game.resource::<MainCamera>().render_snapshot());
        render.insert_resource(FrameRenderConfig::at_startup(WindowSize::new(1280, 720), &GameConfig::default().video));
        render.insert_resource(FrameConstants::default());
        render.insert_resource(utils::camera::TaaJitterSet::default());
        render.insert_resource(ShaderRefactoringState::default());
        render.insert_resource(RenderTemporalState::new(FrameCameraMatrices {
            projection: glam::Mat4::IDENTITY, inverse_projection: glam::Mat4::IDENTITY,
            view: glam::Mat4::IDENTITY, inverse_view: glam::Mat4::IDENTITY,
        }));
        LateCameraFrame::attach(Some(&frame), &game, &mut render);
        mailbox.observe(&crate::event_loop::OsWindowEvent::CursorRelative([20.0, 4.0]));
        latch_camera_preview(&mut render);
        prepare_render_candidate(&mut render);
        let actual = render.resource::<RenderCameraCandidate>().0;
        assert_ne!(actual.camera.inverse_view, glam::Mat4::IDENTITY);
        render.resource_mut::<FrameRenderConfig>().extent_10 = [640, 360];
        prepare_render_candidate(&mut render);
        let retry = render.resource::<RenderCameraCandidate>().0;
        assert_eq!(retry.camera, actual.camera);
        assert_eq!(retry.frame_index, actual.frame_index);
        assert_eq!(render.resource::<FrameConstants>().inner_rendering_resolution, [640, 360]);
        assert_eq!(render.resource::<RenderTemporalState>().camera.inverse_view, glam::Mat4::IDENTITY,
            "preparation and retry must not commit an unsubmitted preview");
    }
    fn fixture() -> (World, TemporalCommit) {
        let camera = FrameCameraMatrices {
            projection: glam::Mat4::IDENTITY,
            inverse_projection: glam::Mat4::IDENTITY,
            view: glam::Mat4::IDENTITY,
            inverse_view: glam::Mat4::IDENTITY,
        };
        let mut world = World::new();
        world.insert_resource(RenderTemporalState::new(camera));
        let ticks = world
            .get_resource_change_ticks::<RenderTemporalState>()
            .unwrap();
        let mut candidate = *world.resource::<RenderTemporalState>();
        candidate.previous_jitter = glam::Vec2::new(0.25, 0.5);
        let commit = TemporalCommit {
            world: world.id(),
            ticks: (ticks.added, ticks.changed),
            camera_added: None,
            candidate,
        };
        world.increment_change_tick();
        (world, commit)
    }
    #[test]
    fn submitted_retry_commits_the_actual_render_camera_history() {
        let (mut world, commit) = fixture();
        let mut actual = commit.candidate;
        actual.previous_jitter = glam::Vec2::new(0.75, 0.125);
        commit.finish(&mut world, true, Some(actual));
        assert_eq!(world.resource::<RenderTemporalState>().previous_jitter, actual.previous_jitter);
        assert_eq!(world.resource::<RenderTemporalState>().frame_index, 1);
    }

    #[test]
    fn only_image_submission_advances_history_and_notifies_ecs() {
        let (mut world, commit) = fixture();
        let before = world
            .get_resource_change_ticks::<RenderTemporalState>()
            .unwrap()
            .changed;
        commit.finish(&mut world, true, None);
        assert_eq!(world.resource::<RenderTemporalState>().frame_index, 1);
        assert_eq!(
            world.resource::<RenderTemporalState>().previous_jitter,
            glam::Vec2::new(0.25, 0.5)
        );
        assert_ne!(
            world
                .get_resource_change_ticks::<RenderTemporalState>()
                .unwrap()
                .changed,
            before
        );
        let (mut world, commit) = fixture();
        commit.finish(&mut world, false, None);
        assert_eq!(world.resource::<RenderTemporalState>().frame_index, 0);
        assert_eq!(
            world.resource::<RenderTemporalState>().previous_jitter,
            glam::Vec2::ZERO
        );
    }
    #[test]
    fn reset_removal_and_application_switch_reject_old_history() {
        let (mut world, commit) = fixture();
        world
            .resource_mut::<RenderTemporalState>()
            .render_resources_rebuilt();
        commit.finish(&mut world, true, None);
        assert_eq!(world.resource::<RenderTemporalState>().frame_index, 0);
        let (mut world, commit) = fixture();
        world.remove_resource::<RenderTemporalState>();
        commit.finish(&mut world, true, None);
        assert!(!world.contains_resource::<RenderTemporalState>());
        let (_, commit) = fixture();
        let (mut other, _) = fixture();
        commit.finish(&mut other, true, None);
        assert_eq!(other.resource::<RenderTemporalState>().frame_index, 0);
    }

    #[test]
    fn replacing_camera_owner_rejects_old_history_but_motion_does_not() {
        for replace in [false, true] {
            let (mut world, mut commit) = fixture();
            world.insert_resource(MainCamera::default());
            commit.camera_added = world.get_resource_change_ticks::<MainCamera>().map(|t| t.added);
            world.increment_change_tick();
            if replace {
                world.remove_resource::<MainCamera>();
                world.insert_resource(MainCamera::default());
            } else {
                world.resource_mut::<MainCamera>().write_matrix(0xc0,
                    &glam::Mat4::from_translation(glam::Vec3::X));
            }
            commit.finish(&mut world, true, None);
            assert_eq!(world.resource::<RenderTemporalState>().frame_index,
                if replace { 0 } else { 1 });
        }
    }
}
