//! Candidate original B prepare_frame_constants @140af40b0, not production.
//! Pure projection of all fourteen original resources. Native ECS dispatch and
//! the presently wrong OutlineStairsCmd payload require separate integration.
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
/// Native EventInstance is 24B (8B id + 16B payload), tag at record+0x14,
/// edge at record+8/+0xc. Current production `_gap:u64` is insufficient.
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
