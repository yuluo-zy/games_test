//! Original B prepare_frame_constants @140af40b0: fourteen canonical input
//! resources, actual camera/TAA/atmosphere, and canonical outline reader.
use crate::systems::wall::structure_outline::OutlineStairsCmd;
use crate::{
    game_config::GameConfig,
    render::{
        atmosphere::DerivedAtmosphere,
        directional_light::DirectionalLight,
        frame_constants::FrameConstants,
        render_config::FrameRenderConfig,
        render_temporal_state::{FrameCameraMatrices, RenderTemporalState},
        view_constants::ViewportConstantBuilder,
    },
    resources::{
        editing_highlight::EditingHighlight,
        image_formation::ImageFormationConfigs,
        session::ActiveSession,
        shader_refactoring_state::ShaderRefactoringState,
        stairs::stairs_state::{StairGraphEdge, TransientStairEdgeIdAllocator},
        time::GameTime,
        time_of_day::TimeOfDay,
    },
    systems::{camera_rig::main_camera::MainCamera, mode_manager::AppMode},
};
use bevy_ecs::prelude::*;
use glam::{Mat4, UVec2, Vec2, Vec3, Vec4};
use std::sync::atomic::{AtomicBool, Ordering};
use utils::camera::TaaJitterSet;

/// Original native tuple is fourteen Res values, each 0x20 bytes; the comments
/// are tuple offsets, not layouts of these named Rust references.
pub struct FrameConstantInputs<'a> {
    pub light: &'a DirectionalLight,               // +000
    pub formation: &'a ImageFormationConfigs,      // +020
    pub atmosphere: &'a DerivedAtmosphere,         // +040
    pub camera: &'a MainCamera,                    // +060
    pub time: &'a GameTime,                        // +080
    pub render_config: &'a FrameRenderConfig,      // +0a0
    pub taa: &'a TaaJitterSet,                     // +0c0
    pub refactoring: &'a ShaderRefactoringState,   // +0e0
    pub highlight: &'a EditingHighlight,           // +100
    pub config: &'a GameConfig,                    // +120
    pub mode: &'a AppMode,                         // +140
    pub stairs: &'a TransientStairEdgeIdAllocator, // +160
    pub session: &'a ActiveSession,                // +180
    pub time_of_day: &'a TimeOfDay,                // +1a0
}

fn camera_matrices(camera: &MainCamera) -> FrameCameraMatrices {
    // Native memcpy first 0x100 bytes. Existing camera_matrices() only returns
    // two matrices and therefore cannot implement the original producer.
    FrameCameraMatrices {
        projection: camera.read_matrix(0),
        inverse_projection: camera.read_matrix(0x40),
        view: camera.read_matrix(0x80),
        inverse_view: camera.read_matrix(0xc0),
    }
}

fn viewport(
    current: FrameCameraMatrices,
    previous: FrameCameraMatrices,
    extent: UVec2,
) -> ViewportConstantBuilder {
    ViewportConstantBuilder {
        clip_from_view: current.projection,
        view_from_clip: current.inverse_projection,
        view_from_world: current.view,
        world_from_view: current.inverse_view,
        prev_clip_from_view: previous.projection,
        prev_view_from_clip: previous.inverse_projection,
        prev_view_from_world: previous.view,
        prev_world_from_view: previous.inverse_view,
        viewport: extent,
        pixel_offset: Vec2::ZERO,
        prev_pixel_offset: Vec2::ZERO,
    }
}

fn shadow_projection(scale_bits: u32) -> Mat4 {
    let scale = f32::from_bits(scale_bits);
    Mat4::from_cols(
        Vec4::new(scale, 0.0, 0.0, 0.0),
        Vec4::new(0.0, scale, 0.0, 0.0),
        Vec4::new(0.0, 0.0, f32::from_bits(0xba800000), 0.0),
        Vec4::new(-0.0, -0.0, 0.5, 1.0),
    )
}

/// `outline_events`: original EventReader yields in order. `Some(edge)` is a
/// decoded original payload tag3; `None` is any other tag. The adapter must
/// preserve the reader position when the first allocated edge terminates it.
/// Native EventInstance is 24B (8B id + 16B payload), kind at record+0x14,
/// endpoints at record+8/+0xc. The ECS entry decodes the canonical struct.
/// `use_taa_jitter` is the actual global @14362c0a0 (release byte=1).
pub fn prepare_frame_constants_from_inputs(
    input: &FrameConstantInputs<'_>,
    temporal: &mut RenderTemporalState,
    outline_events: &mut impl Iterator<Item = Option<StairGraphEdge>>,
    use_taa_jitter: bool,
) -> FrameConstants {
    let formation = input.formation.current();
    let ev_shift = if formation.flag_28 == 0 {
        formation.params_18[1]
    } else {
        0.0
    };
    if input.refactoring.disc >= 2 {
        temporal.frame_index = input.refactoring.payload & 3;
    }
    let jitter = if use_taa_jitter {
        let samples = &input.taa.0;
        // Native remainder-by-zero panic is retained for an empty set.
        samples[temporal.frame_index as usize % samples.len()]
    } else {
        Vec2::ZERO
    };
    let current = camera_matrices(input.camera);
    let previous = if temporal.frame_index == 0 {
        current
    } else {
        temporal.camera
    };
    let extent = UVec2::from_array(input.render_config.extent_10);
    let main_unjittered = viewport(current, previous, extent).build();
    let previous_jitter = temporal.previous_jitter;
    temporal.camera = current;
    temporal.previous_jitter = jitter;
    let mut main_jittered = main_unjittered;
    main_jittered.set_pixel_offset(jitter, previous_jitter, extent);

    let mut editing_color = input.highlight.color;
    let mut stair_id = 0;
    while let Some(event) = outline_events.next() {
        if let Some(edge) = event {
            editing_color = Vec3::new(f32::from_bits(0x3dcccccd), f32::from_bits(0x3cf5c290), 0.0);
            if let Some(id) = input.stairs.get(edge) {
                stair_id = id.0;
                break;
            }
        }
    }
    let shadows = [0x3c7c0fc1, 0x3bfc0fc1, 0x3b100901].map(|bits| {
        let projection = shadow_projection(bits);
        let matrices = FrameCameraMatrices {
            projection,
            inverse_projection: projection.inverse(),
            view: input.light.view,
            inverse_view: input.light.view.inverse(),
        };
        viewport(matrices, matrices, UVec2::splat(1200)).build()
    });
    let atmosphere = input.atmosphere;
    let lighting_only = formation.flag_3c != 0;
    let light_scale = (input.config.video.brightness_shift
        + (ev_shift + if lighting_only { -1.0 } else { 0.0 }))
    .exp2();
    let tint = input.light.tint * light_scale;
    let mut render_overrides = u32::from(formation.flag_3c);
    if formation.flag_3d != 0 {
        render_overrides += 2;
    }
    if input.camera.mode_byte() == 3 {
        render_overrides = u32::from(formation.flag_3c);
    }
    if input.mode.native_tag() != 0x13 {
        render_overrides = u32::from(formation.flag_3c) + 2;
    }
    // Native maxsd/minsd propagate NaN; cvttsd2si stores low32=0.
    // This explicit clamp has the same observable uint result for NaN.
    let nonnegative = if input.time.total_secs >= 0.0 {
        input.time.total_secs
    } else {
        0.0
    };
    let seconds = if nonnegative <= 4294967295.0 {
        nonnegative
    } else {
        4294967295.0
    };
    FrameConstants {
        sun_color: (tint * atmosphere.sun_tint * 30.0).extend(1.0).to_array(),
        sky_low_color: (atmosphere.sky_low_color * tint * 8.16)
            .extend(1.0)
            .to_array(),
        sky_high_color: (atmosphere.sky_high_color * tint * 2.72)
            .extend(1.0)
            .to_array(),
        fog_tint_thickness: atmosphere
            .fog_tint
            .extend(atmosphere.fog_thickness)
            .to_array(),
        fake_light_color: (atmosphere.fake_light_color * 30.0 * light_scale)
            .extend(1.0)
            .to_array(),
        fake_light_dir: atmosphere.fake_light_dir.extend(0.0).to_array(),
        fake_light2_color: (atmosphere.fake_light2_color * 30.0 * light_scale)
            .extend(1.0)
            .to_array(),
        fake_light2_dir: atmosphere.fake_light2_dir.extend(0.0).to_array(),
        frame_counter: temporal.frame_index,
        delta_time_seconds: input.time.delta,
        ev_mult: ev_shift.exp2(),
        glade_theme_id: [0, 3, 1, 2, 4][input.session.meta.glade.native_tag() as usize],
        inner_rendering_resolution: input.render_config.extent_10,
        game_time_sec: seconds as u32,
        game_time_frac: (input.time.total_secs % 1.0) as f32,
        editing_highlight_color: editing_color.to_array(),
        time_of_day: input.time_of_day.time_of_day,
        dir_to_sun: atmosphere.dir_to_sun.to_array(),
        camera_idx: input.camera.mode_byte() as u32,
        dir_to_moon: atmosphere.dir_to_moon.to_array(),
        render_overrides,
        global_mip_bias: (input.render_config.extent_10[0] as f32
            / (input.render_config.extent_8[0] as f32).max(1.0))
        .log2(),
        highlighted_transient_stair_edge_id: stair_id,
        pad1: 1,
        pad2: 2,
        debug_tweakers: [0.0; 4],
        // Original final memcpy @140af4a56..4ab7, after set_pixel_offset.
        views: [
            main_jittered,
            main_unjittered,
            shadows[0],
            shadows[1],
            shadows[2],
        ],
    }
}

/// Original release global byte@14362c0a0=1. Atomic storage is a source-host
/// threading adaptation; it does not change the selected resource identity.
pub static USE_TAA_JITTER: AtomicBool = AtomicBool::new(true);

type OriginalFrameResources<'w> = (
    Res<'w, DirectionalLight>,
    Res<'w, ImageFormationConfigs>,
    Res<'w, DerivedAtmosphere>,
    Res<'w, MainCamera>,
    Res<'w, GameTime>,
    Res<'w, FrameRenderConfig>,
    Res<'w, TaaJitterSet>,
    Res<'w, ShaderRefactoringState>,
    Res<'w, EditingHighlight>,
    Res<'w, GameConfig>,
    Res<'w, AppMode>,
    Res<'w, TransientStairEdgeIdAllocator>,
    Res<'w, ActiveSession>,
    Res<'w, TimeOfDay>,
);

/// Exact original fourteen-resource tuple and final three ECS parameters.
/// The EventReader stops at the first matching allocated edge, preserving
/// later unread events. No command is manufactured to force a highlight.
pub fn prepare_frame_constants(
    resources: OriginalFrameResources<'_>,
    mut temporal: ResMut<RenderTemporalState>,
    mut constants: ResMut<FrameConstants>,
    mut outlines: EventReader<OutlineStairsCmd>,
) {
    let input = FrameConstantInputs {
        light: &resources.0,
        formation: &resources.1,
        atmosphere: &resources.2,
        camera: &resources.3,
        time: &resources.4,
        render_config: &resources.5,
        taa: &resources.6,
        refactoring: &resources.7,
        highlight: &resources.8,
        config: &resources.9,
        mode: &resources.10,
        stairs: &resources.11,
        session: &resources.12,
        time_of_day: &resources.13,
    };
    // Actual canonical16B command is a struct: kind3 is a style/flag,
    // not an invented enum variant or a guessed Option resource.
    let mut events = outlines.read().map(|command| {
        if command.kind == 3 {
            Some(command.segment.into())
        } else {
            None
        }
    });
    *constants = prepare_frame_constants_from_inputs(
        &input,
        &mut temporal,
        &mut events,
        USE_TAA_JITTER.load(Ordering::Relaxed),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_ecs_prepare_preserves_view_order_jitter_and_unread_outline_events() {
        use crate::{
            resources::{
                game_clock::{ClockGate, DeltaSource},
                session::{SessionId, SessionMeta},
                stairs::stairs_state::StairSegment,
                time_of_day::TodLightingSnapshot,
                walls::decorator::DecoratorId,
                window::WindowSize,
            },
            systems::time_of_day::{suncalc_pos_to_vec, write_derived_atmosphere},
        };

        // This fixture is the original assets/starting-builds/full/09-garden/
        // meta.json envelope. Exercise the actual file loader and session owner.
        let directory = tempfile::tempdir().unwrap();
        let metadata = directory.path().join("meta.json");
        std::fs::write(&metadata, br#"{"state":{"glade":"Autumn"},"version":1}"#).unwrap();
        let session = ActiveSession::new(
            SessionId::new(),
            SessionMeta::try_load_from_path(metadata).unwrap(),
        );

        // The snapshot is copied from the original assets/tod/default.json
        // entry at time_of_day=0.27. Angles, viewport and replay duration below
        // are explicit test inputs, not asserted original startup parameters.
        let snapshot: TodLightingSnapshot = serde_json::from_str(
            r#"{
                "sun_sky": {
                    "sun_tint_color": [0.48191673, 0.39131403, 0.252196],
                    "moon_tint_color": [0.0, 0.0, 0.0],
                    "sky_low_color": [0.08690908, 0.27562594, 0.43454543],
                    "sky_high_color": [0.033527013, 0.13521335, 0.2707951]
                },
                "base_fog_thickness": 0.005,
                "fog_tint": [0.97385174, 1.0065603, 1.0120116],
                "speed_bias": -0.5
            }"#,
        )
        .unwrap();
        let mut tod = TimeOfDay::default();
        tod.time_of_day = 0.27;
        let (sun, moon) = suncalc_pos_to_vec(0.3, 1.0, 0.7, tod.sun_xz_rotation);
        let mut light = DirectionalLight::look_at(Vec3::new(3.0, 5.0, 2.0), Vec3::ZERO);
        let mut atmosphere = DerivedAtmosphere::default();
        write_derived_atmosphere(
            &mut atmosphere,
            &mut light,
            &snapshot,
            sun,
            moon,
            tod.day_night_continuous,
            tod.day_night_continuous_prev,
            0.125,
        );
        assert!(atmosphere.sun_tint.min_element() > 0.0);
        let light_view = light.view;

        let window = WindowSize::new(1280, 720);
        let config = GameConfig::default();
        let render_config = FrameRenderConfig::at_startup(window, &config.video);
        // MainCamera::new owns real Dolly rigs. Its four matrices are initially
        // identity in the original constructor; no fake native allocation or
        // hand-written camera matrix replaces that constructor here.
        let camera = MainCamera::new(window.aspect_ratio());
        let actual_camera = camera_matrices(&camera);
        let temporal = RenderTemporalState::new(actual_camera);
        let taa = TaaJitterSet::default();
        assert!(USE_TAA_JITTER.load(Ordering::Relaxed));
        let first_jitter = taa.0[0];
        let second_jitter = taa.0[1];
        assert_ne!(first_jitter, Vec2::ZERO);
        assert_ne!(first_jitter, second_jitter);
        let mut time = GameTime::default();
        time.update(
            ClockGate {
                state_is_zero: true,
                mode_tag: 2,
                mode_flag: 0,
                paused: false,
            },
            DeltaSource::Replay { frame_delta: 12.25 },
        )
        .unwrap();

        let first_segment = StairSegment::new(DecoratorId(11), DecoratorId(7));
        let second_segment = StairSegment::new(DecoratorId(19), DecoratorId(23));
        let mut stairs = TransientStairEdgeIdAllocator::default();
        let first_id = stairs.get_or_allocate(first_segment.into());
        let second_id = stairs.get_or_allocate(second_segment.into());
        assert_ne!(first_id, second_id);
        assert_ne!(first_id.0, 0);
        let highlight = Vec3::new(0.2, 0.4, 0.8);

        let mut world = World::new();
        world.insert_resource(light);
        world.insert_resource(ImageFormationConfigs::default());
        world.insert_resource(atmosphere);
        world.insert_resource(camera);
        world.insert_resource(time);
        world.insert_resource(render_config);
        world.insert_resource(taa);
        world.insert_resource(ShaderRefactoringState::default());
        world.insert_resource(EditingHighlight {
            color: highlight,
            enabled: true,
        });
        world.insert_resource(config);
        world.insert_resource(AppMode::Photomode);
        world.insert_resource(stairs);
        world.insert_resource(session);
        world.insert_resource(tod);
        world.insert_resource(temporal);
        world.insert_resource(FrameConstants::default());
        world.init_resource::<Events<OutlineStairsCmd>>();

        // Tag3 is the observed prepare-consumer discriminator, supplied as an
        // explicit command fixture. The existing color UI emits tag0 instead.
        let outline = |segment, kind| OutlineStairsCmd {
            segment,
            strength: 2.0,
            kind,
            include_connected: true,
        };
        world
            .resource_mut::<Events<OutlineStairsCmd>>()
            .send_batch([
                outline(second_segment, 0),
                outline(StairSegment::new(DecoratorId(29), DecoratorId(31)), 3),
                outline(first_segment, 3),
                outline(second_segment, 3),
            ]);
        let mut schedule = Schedule::default();
        schedule.add_systems(prepare_frame_constants);
        schedule.run(&mut world);

        let first = world.resource::<FrameConstants>();
        assert_eq!(first.highlighted_transient_stair_edge_id, first_id.0);
        assert_eq!(first.glade_theme_id, 2); // original Autumn theme lookup
        assert_eq!(first.game_time_sec, 12);
        assert_eq!(first.game_time_frac, 0.25);
        assert_eq!(first.delta_time_seconds, 12.25);
        assert_eq!(first.inner_rendering_resolution, [1280, 720]);
        assert!(first.sun_color[..3].iter().all(|value| *value > 0.0));
        assert_eq!(first.views[0].pixel_offset, first_jitter);
        assert_eq!(first.views[0].prev_pixel_offset, Vec2::ZERO);
        assert_eq!(first.views[1].pixel_offset, Vec2::ZERO);
        assert_ne!(
            first.views[0].clip_from_view_jittered,
            first.views[1].clip_from_view_jittered
        );
        for view in &first.views[..2] {
            assert_eq!(view.clip_from_view, actual_camera.projection);
            assert_eq!(view.world_from_view, actual_camera.inverse_view);
        }
        for (index, scale_bits) in [0x3c7c0fc1, 0x3bfc0fc1, 0x3b100901].into_iter().enumerate() {
            let view = &first.views[index + 2];
            assert_eq!(view.clip_from_view.x_axis.x.to_bits(), scale_bits);
            assert_eq!(view.clip_from_view.z_axis.z.to_bits(), 0xba800000);
            assert_eq!(view.view_from_world, light_view);
            assert_eq!(view.prev_view_from_world, light_view);
            assert_eq!(view.pixel_offset, Vec2::ZERO);
        }
        assert_eq!(world.resource::<RenderTemporalState>().frame_index, 0);
        assert_eq!(
            world.resource::<RenderTemporalState>().previous_jitter,
            first_jitter
        );

        // Keep the same Schedule/SystemState and Events resource. Only the real
        // render-return lifecycle advances temporal state; prepare never does.
        world
            .resource_mut::<RenderTemporalState>()
            .frame_render_returned();
        schedule.run(&mut world);
        let second = world.resource::<FrameConstants>();
        assert_eq!(second.highlighted_transient_stair_edge_id, second_id.0);
        assert_eq!(second.frame_counter, 1);
        assert_eq!(second.views[0].pixel_offset, second_jitter);
        assert_eq!(second.views[0].prev_pixel_offset, first_jitter);
        assert_eq!(second.views[1].pixel_offset, Vec2::ZERO);

        schedule.run(&mut world);
        let exhausted = world.resource::<FrameConstants>();
        assert_eq!(exhausted.highlighted_transient_stair_edge_id, 0);
        assert_eq!(exhausted.editing_highlight_color, highlight.to_array());
    }

    #[test]
    fn native_shadow_projection_uses_exact_three_original_scales_and_depth_range() {
        for bits in [0x3c7c0fc1, 0x3bfc0fc1, 0x3b100901] {
            let matrix = shadow_projection(bits);
            assert_eq!(matrix.x_axis.x.to_bits(), bits);
            assert_eq!(matrix.y_axis.y.to_bits(), bits);
            assert_eq!(matrix.z_axis.z.to_bits(), 0xba800000);
            assert_eq!(matrix.w_axis, Vec4::new(-0.0, -0.0, 0.5, 1.0));
            assert_eq!(matrix.w_axis.x.to_bits(), 0x80000000);
            assert_eq!(matrix.w_axis.y.to_bits(), 0x80000000);
        }
    }
}
