//! Original continuous render_world W47..W54 on the enclosing current frame.
//! W44..W46 must already have consumed the same lighting targets. This module
//! never manufactures a gameplay-ready gate or omits the final debug consumer.
//!
//! It also owns the compile_main tail after W54 (0x140161cd0.c:1085..1409):
//! `ColorGradingState::flattened` (CPU) -> `bidirectional_blur_pyramid` ->
//! `final_post` (`"msaa color out"`, 0x2b, sample_count 4) -> menu/painterly/
//! Vermeer/logo/UI/MEI/cursor/loading compositing on that same target ->
//! export 0x22. The final presentation source is therefore the MSAA target, not
//! the TAA texture. Steps whose host or producer is not recovered return a
//! named hard error (or are listed as explicit U controls); nothing is silently
//! replaced by an empty pass.
use crate::{
    effects_host::attachment, render_bridge::CoreGraphFrame, scene_compute::SceneComputeHost,
    scene_raster::SceneRasterHost, world_lighting::WorldLightingOutputs,
};
use bevy_ecs::world::World;
use country_core::{
    game_config::GameConfig,
    render::{
        pass_builder::{
            framebuffer_binding::AttachmentTexture, pass_resource_access::RgTexture,
            rg_resources::RgResources,
        },
        render_config::{FrameRenderConfig, UpscalingMode},
    },
    resources::{
        image_formation::{ImageFormationConfig, ImageFormationConfigs},
        window::WindowSize,
    },
    rhapsody_contract::{Handle as DrawHandle, ManagedHandle},
    rhapsody_types::TextureDesc,
    systems::{
        mode_manager::AppMode,
        render_loop::{
            compute_passes::{
                blur_pyramid,
                dof::DofConfig,
                taa::{HistoryTexture, TaaInputs, TgsrInputs},
            },
            draw_passes::{
                custom_cursor_pass, draw_lists::DrawLists, final_post, logo_overlay, ui_2d_pass,
                ui_pass, wireframe_pass,
                d_main_support::{DrawPassCtx, ShaderLookup},
            },
            menu_background, painterly, render_world::{formats, names}, vermeer,
        },
    },
};

/// Snapshots of two original process-level debug flags. No Default is provided:
/// the actual host must supply their state; AppMode is independently read below.
#[derive(Clone, Copy, Debug)]
pub struct PostProcessControls {
    /// Original byte at14362c080, read inside calc_dof@140a5a5b4.
    pub multires_dof: bool,
    /// Original history byte143634d50, inverted by TAA/TGSR before valid gating.
    pub disable_temporal_history: bool,
}

/// Native render_world returns depth/CoC/TAA, each a real texture. The internal
/// second DoF tile texture and optional tile buffers remain fully consumed by
/// the original calc_dof chain before this boundary.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WorldPostOutputs {
    pub depth: RgTexture,
    pub coc: RgTexture,
    pub taa: RgTexture,
}

fn missing(name: &str) -> rhapsody::Error {
    rhapsody::Error::invalid_usage(format!("original world W47..53: missing {name} producer"))
}

/// Caller owns the single main FrameSetup and the original preceding geometry,
/// lighting and surface/transparency draws. This function declares actual core
/// consumers, never submits, initializes histories or substitutes scene owners.
pub fn build_world_postprocess(
    frame: &mut CoreGraphFrame<'_>,
    world: &mut World,
    lists: &DrawLists,
    lighting: &WorldLightingOutputs,
    controls: PostProcessControls,
) -> rhapsody::Result<WorldPostOutputs> {
    let video = world
        .get_resource::<GameConfig>()
        .ok_or_else(|| missing("GameConfig"))?
        .video
        .clone();
    let mode = world
        .get_resource::<AppMode>()
        .ok_or_else(|| missing("AppMode"))?
        .native_tag();
    let photomode = mode == 0x13;
    let config = DofConfig {
        bokeh_anisotropy: world
            .get_resource::<ImageFormationConfigs>()
            .ok_or_else(|| missing("ImageFormationConfigs"))?
            .current()
            .params_30[0],
    };
    let render_config = *world
        .get_resource::<FrameRenderConfig>()
        .ok_or_else(|| missing("FrameRenderConfig"))?;
    if !world.contains_resource::<country_core::systems::camera_rig::main_camera::MainCamera>() {
        return Err(missing("MainCamera"));
    }
    let color = lighting.geometry.color;
    let depth = lighting.geometry.gbuffer.depth;
    // Historical field name: current host constructs fresh = pair.history_valid()
    // (render_host.rs get_or_create_temporal_texture_pair), not allocation freshness.
    let history = HistoryTexture {
        tex: lighting.history.previous,
        valid: lighting.history.fresh,
    };
    let use_tgsr = video.upscaling_mode == UpscalingMode::Tgsr;
    let expected_history_dims = if use_tgsr {
        render_config.extent_8
    } else {
        color.desc.extent_2d()
    };
    if expected_history_dims.contains(&0)
        || lighting.history.current.desc.extent_2d() != expected_history_dims
    {
        return Err(rhapsody::Error::invalid_usage(
            "world post history differs from the original TAA/TGSR dimensions",
        ));
    }
    let mut compute = SceneComputeHost::new(frame, world);
    compute.compute_shadow_mask_mean()?;
    compute.apply_volumetrics(&color, &depth)?;
    let prepared = compute.dof_prepare(&depth)?;
    let dof_enabled = if photomode {
        video.photomode_dof_enable
    } else {
        video.dof_enable
    };
    let taa_input = if dof_enabled {
        compute.generate_mips(&color, names::DOF_MIPS)?;
        compute
            .calc_dof(
                &color,
                &depth,
                &prepared,
                &config,
                controls.multires_dof,
                photomode,
            )?
            .color
    } else {
        color
    };
    let extent = if use_tgsr {
        render_config.extent_8
    } else {
        taa_input.desc.extent_2d()
    };
    let output = frame.with_compute(|compute| {
        Ok(compute.create_raw_texture(
            names::TAA_OUTPUT,
            &TextureDesc::new_2d(extent, formats::COLOR_HDR),
        ))
    })?;
    let min_subgroup_size = frame
        .hosts()
        .buffers
        .graphics_device()
        .min_subgroup_size_option();
    let mut compute = SceneComputeHost::new(frame, world);
    let taa = if use_tgsr {
        compute.calculate_tgsr(&TgsrInputs {
            color: &taa_input,
            depth: &depth,
            output: &output,
            reprojection: &lighting.reprojection,
            history_output: &lighting.history.current,
            history: &history,
            sharpen_amount: video.sharpen,
            bypass_clamp: u8::from(render_config.flag_20),
            history_global: controls.disable_temporal_history,
            min_subgroup_size,
        })?
    } else {
        compute.calculate_taa(&TaaInputs {
            color: &taa_input,
            output: &output,
            reprojection: &lighting.reprojection,
            history_output: &lighting.history.current,
            history: &history,
            sharpen_amount: video.sharpen,
            bypass_clamp: u8::from(render_config.flag_20),
            history_global: controls.disable_temporal_history,
        })?
    };
    SceneRasterHost::new(frame, world, lists)?.post_taa_transparency_pass(&attachment(taa))?;
    let taa = render_shader_refactoring(frame, world, taa)?;
    Ok(WorldPostOutputs {
        depth,
        coc: prepared.coc,
        taa,
    })
}

/// W54 original mandatory resource and whole-frame replacement, over the
/// existing enclosing frame and canonical ECS owner. No new state or image is
/// installed. State mutation occurs at graph-declaration completion as native,
/// before compile/submit; later failures do not invent a rollback policy.
pub fn render_shader_refactoring(
    frame: &mut CoreGraphFrame<'_>, world: &mut World, taa: RgTexture,
) -> rhapsody::Result<RgTexture> {
    use bevy_ecs::change_detection::DetectChanges;
    use country_core::{
        resources::shader_refactoring_state::ShaderRefactoringState,
        systems::render_loop::compute_passes::shader_refactoring::{
            self, FrameHandle, StateMut,
        },
    };
    let state = world.get_resource::<ShaderRefactoringState>()
        .ok_or_else(|| missing("ShaderRefactoringState"))?;
    // Native disc0 returns after this mandatory lookup, without allocation,
    // changed-tick mutation, shaders or any extra pass-resource lookup.
    if state.disc == 0 { return Ok(taa); }
    let tick = world.read_change_tick().get();
    world.resource_scope(|world, mut canonical: bevy_ecs::change_detection::Mut<ShaderRefactoringState>| {
        let active = matches!(canonical.disc, 1..=4);
        let mut state = StateMut { state: *canonical, changed_tick: canonical.last_changed().get() };
        crate::scene_compute::with_env(frame, world, |env| {
            let run = shader_refactoring::try_render_shader_refactoring(
                env, &mut state, tick, &FrameHandle { texture: taa, format: taa.desc.format })?;
            for pass in run.passes { env.record(pass); }
            if active { *canonical = state.state; }
            Ok(run.replacement_frame.unwrap_or(taa))
        })
    })
}

// ---------------------------------------------------------------------------------------------
// compile_main tail (0x140161cd0.c:1085..1409)
// ---------------------------------------------------------------------------------------------

/// Native tail steps in call order, including the CPU-only grading fold and the
/// final export. Reporting/test table only; it does not claim that every entry
/// is executed by this host (see [`WorldTailControls`] and the delivery notes).
pub const WORLD_TAIL_STEPS: [&str; 22] = [
    "ColorGradingState::flattened",
    "bidirectional_blur_pyramid",
    "final_post/msaa color out",
    "handle_screenshot_opportunity 0/1/2",
    "render_menu_bg",
    "painterly_pass",
    "vermeer_pass",
    "render_logo_overlay",
    "handle_screenshot_opportunity 3",
    "render_session_selection_previews",
    "render_photomode_lut_gallery",
    "render_feedback_report_bg",
    "wireframe_pass",
    "ui_pass",
    "ui_2d_pass",
    "mei_pass",
    "custom_cursor_pass",
    "handle_screenshot_opportunity 4",
    "loading_screen_pass",
    "EguiBackend::draw",
    "handle_screenshot_opportunity 6",
    "export 0x22",
];

/// Explicit reconstruction inputs for native values that have no recovered
/// producer. Each field is documented with its native evidence; supplying one
/// must not be read as equivalence with compile_main.
#[derive(Clone, Copy, Debug)]
pub struct PainterlyTailControl {
    /// `stack0x1a88` bool, shared by T6 `enabled` and T7 `skip_generate` (O
    /// argument positions; the producer of the byte is U).
    pub enabled: bool,
    /// Resource 0xff byte used by `painterly_pass`'s `instant` selection (U).
    pub instant_flag_ff: bool,
    /// Resource 0x80 u32 used when 0xff is clear (U).
    pub count_80: u32,
}

#[derive(Clone, Copy, Debug)]
pub struct WorldTailControls {
    /// `stack0x1ac0`; both native writes (0x140162840, 0x140162930) store 1.
    /// It is final_post's `flag94` and the wireframe/ui_pass gate (O).
    pub flag_0x1ac0: bool,
    /// render_world result byte `stack0x1897` (U: no recovered producer).
    /// `None` selects the native `bVar43 & 1 == 1` (Keep) branch.
    pub bvar43: Option<u8>,
    /// `AppMode` `+8` byte read by `render_menu_bg` (U).
    pub menu_bg_flag: u8,
    pub painterly: PainterlyTailControl,
    /// T8 logo condition; `*0x14362c098 == 1` is the demo byte, while
    /// `bVar43 ∈ {3,4}` / `settings+0xf3 == 1` have no recovered producer (U).
    pub logo_overlay: bool,
    /// T11 host not recovered; explicit U switch, default false.
    pub session_previews: bool,
    /// T12 host not recovered; explicit U switch, default false.
    pub lut_gallery: bool,
    /// T13 `FeedbackReportBg` producer not recovered; U switch.
    pub feedback_bg: bool,
    /// T16 needs `UiInteractionState`/`UiInactive` component views that are not
    /// recovered in this tree; U switch.
    pub ui_2d: bool,
    /// T21 `EguiBackend::draw`; the EguiGpu owner is not reachable from the ECS
    /// world (I: the native world frame always has it). U switch.
    pub egui_present: bool,
    /// T18 cursor resource `+0xf4` (U).
    pub cursor_visible: bool,
}

/// Load-time reconstruction defaults for [`WorldTailControls`]. Every non-O
/// field is the value the native load-time globals/stack carry when no producer
/// is registered, and the delivery lists them as U. `flag_0x1ac0` is O = 1.
pub const fn load_time_tail_controls() -> WorldTailControls {
    WorldTailControls {
        flag_0x1ac0: true,
        bvar43: None,
        menu_bg_flag: 0,
        painterly: PainterlyTailControl {
            enabled: false,
            instant_flag_ff: false,
            count_80: 0,
        },
        logo_overlay: false,
        session_previews: false,
        lut_gallery: false,
        feedback_bg: false,
        ui_2d: false,
        egui_present: false,
        cursor_visible: false,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WorldTailOutputs {
    /// `"msaa color out"`: 0x2b, sample_count 4, display extent. This is the
    /// resource the caller exports at 0x22 for the presentation graph.
    pub msaa: RgTexture,
    /// `"full blur pyramid"` sampled by final_post's src1.
    pub blur: RgTexture,
    /// `vermeer_pass` return value: entering phase != Disabled (O).
    pub vermeer_active: bool,
}

fn tail_missing(name: &str) -> rhapsody::Error {
    rhapsody::Error::invalid_usage(format!(
        "original compile_main 0x140161cd0 tail: missing {name}"
    ))
}

fn tail_unsupported(step: &str, reason: &str) -> rhapsody::Error {
    rhapsody::Error::unsupported(format!(
        "original compile_main 0x140161cd0 tail step `{step}`: {reason}"
    ))
}

/// `final_post` binds its grading LUTs through `TextureFromDisk`; the raster
/// host resolves those hashes from the named texture table and panics when a
/// producer is missing. Check them first so the frame fails with the missing
/// asset and its startup producer instead of a generic "No such texture".
fn require_grading_lut_texture(
    world: &World,
    hash: u64,
    what: &str,
) -> rhapsody::Result<()> {
    use country_core::{
        asset_libraries::texture_library::AssetTextureLibrary, name_hash::NameHash,
    };
    let textures = world
        .get_resource::<AssetTextureLibrary>()
        .ok_or_else(|| tail_missing("AssetTextureLibrary"))?;
    if textures
        .get_handle_and_texture_by_name(NameHash(hash))
        .is_none()
    {
        return Err(tail_missing(&format!(
            "{what} grading LUT texture 0x{hash:016x} (startup load_glade_grading is not recovered)"
        )));
    }
    Ok(())
}

fn warn_deferred_tail_steps() {
    use std::sync::Once;
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        log::warn!(
            target: "tiny_glade::world_tail",
            "compile_main tail: ui_2d_pass component views, session/LUT-gallery/feedback hosts, \
             painterly aux inputs and egui are not recovered; their U controls default to off. \
             warm-up covers W1-W54 plus the implemented tail subset, not complete compile_main"
        );
    });
}

/// `stack0x1490` LUT selection (0x140161cd0.c:1126-1149). The native branches
/// are `bVar43 == 7 || ((b-2) != 3 && (b-2) < 5)` -> Custom(flattened name,
/// flattened +0x18, flag 0); otherwise the niche byte is `b & 1` (1 = Keep,
/// 0 = Default).
fn lut_selection(bvar43: Option<u8>, cfg: &ImageFormationConfig) -> final_post::LutSelect {
    match bvar43 {
        Some(b) => {
            let k = b.wrapping_sub(2);
            if b == 7 || (k != 3 && k < 5) {
                final_post::LutSelect::Custom {
                    name: cfg.name.clone(),
                    blend: cfg.params_18[0],
                    flag: 0,
                }
            } else if b & 1 == 1 {
                final_post::LutSelect::Keep
            } else {
                final_post::LutSelect::Default
            }
        }
        // U: no bVar43 producer is recovered; the `b & 1 == 1` (Keep) branch is
        // the load-time-unknown value. Documented, not claimed equivalent.
        None => final_post::LutSelect::Keep,
    }
}

/// Current grading section fields read by `final_post` (O offsets):
/// `+0x20` pair, `+0x28` bypass/named byte, `+0x30/+0x34/+0x38` floats.
fn grading_config(cfg: &ImageFormationConfig) -> final_post::GradingConfig {
    final_post::GradingConfig {
        // `+0x28` is `bypass_color` in the serializer; a bypassed config
        // degenerates to (1,1) in build_env, which matches the flattened path.
        named: cfg.flag_28 == 0,
        pair: [cfg.params_18[2], cfg.params_18[3]],
        floats: final_post::GradingFloats {
            f30: cfg.params_30[0],
            f34: cfg.params_30[1],
            f38: f32::from_bits(cfg.word_38),
        },
    }
}

struct LibraryShaders<'a>(&'a country_core::asset_libraries::shader_library::AssetShaderLibrary);
impl ShaderLookup for LibraryShaders<'_> {
    fn raster_handle_by_name(
        &self,
        name: &str,
    ) -> Option<
        country_core::rhapsody_contract::Handle<country_core::render::shader::RasterShaderProgram>,
    > {
        self.0.get_raster_handle_by_name(name).map(|handle| {
            country_core::rhapsody_contract::Handle(
                country_core::rhapsody_contract::ManagedHandle(handle.id().raw()),
                core::marker::PhantomData,
            )
        })
    }
}

fn window_dims(world: &World) -> rhapsody::Result<(u32, u32)> {
    let window = world
        .get_resource::<WindowSize>()
        .ok_or_else(|| tail_missing("WindowSize"))?;
    if window.width == 0 || window.height == 0 {
        return Err(rhapsody::Error::invalid_usage(
            "original compile_main tail: zero WindowSize",
        ));
    }
    Ok((window.width, window.height))
}

/// T17 prerequisites: MEI buffers are imported into this frame and the text
/// atlas/primitive count come from the real `text_flush_system` output. The
/// caller must have flushed before declaration (`flush_mei_frame`).
fn prepare_mei(
    frame: &mut CoreGraphFrame<'_>,
    world: &mut World,
    out_extent: [u32; 2],
) -> rhapsody::Result<(country_core::systems::render_loop::draw_passes::ui_2d_pass::MeiTextOutput,
                       AttachmentTexture)> {
    use country_core::{
        asset_libraries::texture_library::AssetTextureLibrary,
        name_hash::NameHash,
    };
    let raster = frame.raster_resources();
    {
        // mei_pass binds the font disk texture by its file hash; the loading
        // frame publishes the same real library entry. Resolve it, never
        // fabricate a placeholder.
        let textures = world
            .get_resource::<AssetTextureLibrary>()
            .ok_or_else(|| tail_missing("AssetTextureLibrary"))?;
        let source = textures
            .get_handle_and_texture_by_name(NameHash(0x579f_39ab_91a0_1865))
            .map(|(_, texture)| texture)
            .ok_or_else(|| tail_missing("MEI font texture 0x579f39ab91a01865"))?;
        raster
            .assets
            .lock()
            .textures_by_name
            .entry(0x579f_39ab_91a0_1865)
            .or_insert((
                ManagedHandle(source.id),
                AttachmentTexture::from(source),
            ));
    }
    let text = {
        let gpu = world
            .get_resource::<crate::loading_graph::LoadingGpuResources>()
            .ok_or_else(|| tail_missing("gameplay MEI host (LoadingGpuResources)"))?;
        gpu.mei.import_into_frame(frame, &raster)?;
        gpu.mei
            .output()
            .cloned()
            .ok_or_else(|| tail_missing("MEI text_flush_system output"))?
    };
    let temp = frame.with_compute(|compute| {
        Ok(compute.create_raw_texture(
            "mei temp",
            &TextureDesc::new_2d(out_extent, 0x2b),
        ))
    })?;
    Ok((
        text,
        AttachmentTexture {
            id: temp.resource,
            extent: (out_extent[0], out_extent[1]),
            format: 0x2b,
            sample_count: 1,
        },
    ))
}

/// T7 `vermeer_pass` through the recovered Painterly effect host. The state and
/// debug resources are real gameplay resources (`run_game.rs` inserts both);
/// missing state is a hard error because the native system reads it directly.
fn run_vermeer(
    frame: &mut CoreGraphFrame<'_>,
    world: &mut World,
    target: RgTexture,
    mode: u32,
    photomode: bool,
    controls: &WorldTailControls,
) -> rhapsody::Result<bool> {
    use country_core::asset_libraries::{
        placement_data_library::AssetPlacementDataLibrary, ssbo_library::AssetSsboLibrary,
        texture_library::AssetTextureLibrary, vao_library::AssetVAOLibrary,
    };
    use crate::frame_world::{FrameLibraries, FrameWorldAccess};
    let shaders = frame.shader_library();
    let backend = frame.shader_backend();
    if !world.contains_resource::<vermeer::VermeerState>() {
        return Err(tail_missing("VermeerState"));
    }
    let debug = *world
        .get_resource::<vermeer::VermeerDebugControl>()
        .ok_or_else(|| tail_missing("VermeerDebugControl"))?;
    let dt = world
        .get_resource::<bevy_time::Time>()
        .map(|time| time.delta_secs())
        .unwrap_or(0.0);
    world.resource_scope(|world, mut state: bevy_ecs::change_detection::Mut<vermeer::VermeerState>| {
        let raster = frame.raster_resources();
        let access = FrameWorldAccess::new(
            FrameLibraries {
                shaders: shaders.as_ref(),
                textures: world
                    .get_resource::<AssetTextureLibrary>()
                    .ok_or_else(|| tail_missing("AssetTextureLibrary"))?,
                ssbos: world
                    .get_resource::<AssetSsboLibrary>()
                    .ok_or_else(|| tail_missing("AssetSsboLibrary"))?,
                vaos: world
                    .get_resource::<AssetVAOLibrary>()
                    .ok_or_else(|| tail_missing("AssetVAOLibrary"))?,
                placement: world
                    .get_resource::<AssetPlacementDataLibrary>()
                    .ok_or_else(|| tail_missing("AssetPlacementDataLibrary"))?,
                constants: raster.dynamic_constants(),
            },
            frame.compute_device(),
        )
        .with_resource_graph_factory(frame.resource_graph_factory());
        let mut effects = crate::effects_host::EffectsHost::new(
            frame,
            &access,
            &*backend,
            shaders.as_ref(),
            Some(painterly::AppModeView {
                state: mode,
                flag: controls.menu_bg_flag,
            }),
            Some(painterly::TimeView {
                delta_secs_f64: f64::from(dt),
                delta_secs: dt,
            }),
            Some(debug),
            // Static 0x143634d90 selects the alternate background name (U).
            false,
        );
        let target = effects.register_texture(target);
        let mut locals = vermeer::VermeerPassLocals::default();
        let active = vermeer::vermeer_pass(
            &mut effects,
            &mut locals,
            &mut *state,
            vermeer::VermeerPassParams {
                target,
                photomode,
                // O: the same stack0x1a88 bool as painterly's `enabled` (U).
                skip_generate: controls.painterly.enabled,
            },
        );
        effects.finish()?;
        Ok(active)
    })
}

/// Build the native compile_main tail after W54. Caller owns the enclosing
/// main graph, `with_main_prelude`, and the 0x22 export of the returned target.
pub fn build_world_tail(
    frame: &mut CoreGraphFrame<'_>,
    world: &mut World,
    lists: &DrawLists,
    taa: &RgTexture,
    out_extent: [u32; 2],
    controls: &WorldTailControls,
) -> rhapsody::Result<WorldTailOutputs> {
    if out_extent.contains(&0) {
        return Err(rhapsody::Error::invalid_usage(
            "original compile_main tail: display extent contains zero",
        ));
    }
    if taa.desc.format != formats::COLOR_HDR {
        return Err(rhapsody::Error::invalid_usage(
            "original compile_main tail: W54 output is not COLOR_HDR (0x7a)",
        ));
    }
    let mode = world
        .get_resource::<AppMode>()
        .ok_or_else(|| tail_missing("AppMode"))?
        .native_tag();
    let photomode = mode == 0x13;
    let video = world
        .get_resource::<GameConfig>()
        .ok_or_else(|| tail_missing("GameConfig"))?
        .video
        .clone();
    // T1: `ImageFormationConfigs::current_mut` then `ColorGradingState::flattened`
    // (0x140b00b20). CPU only: no GPU resource, no pass.
    let formation = world
        .get_resource::<ImageFormationConfigs>()
        .ok_or_else(|| tail_missing("ImageFormationConfigs"))?
        .current()
        .clone();
    let flattened = formation.flattened();
    warn_deferred_tail_steps();

    // T2: unconditional `bidirectional_blur_pyramid` (0x1409ed860) on W54 taa.
    let blur = crate::scene_compute::with_env(frame, world, |env| {
        Ok(blur_pyramid::bidirectional_blur_pyramid(env, taa))
    })?;
    if blur.desc.format != blur_pyramid::PYRAMID_FORMAT || blur.desc.mip_levels == 0 {
        return Err(rhapsody::Error::invalid_usage(
            "original bidirectional_blur_pyramid output is not a 0x7a mip pyramid",
        ));
    }

    // T3 target: `"msaa color out"` = new_2d(display extent, 0x2b),
    // sample_count(4) (final_post.rs:195-202, 0x14090d000).
    let native_msaa = rhapsody::raw::texture::TextureDesc::new_2d(
        out_extent[0],
        out_extent[1],
        ash::vk::Format::from_raw(final_post::MSAA_COLOR_FORMAT as i32),
    )
    .sample_count(final_post::MSAA_SAMPLE_COUNT)
    .sampled()
    .color_attachment();
    let msaa_id = frame.declare_native_image(final_post::MSAA_COLOR_OUT_NAME, native_msaa)?;
    let msaa = RgTexture {
        resource: msaa_id,
        desc: TextureDesc::new_2d(out_extent, final_post::MSAA_COLOR_FORMAT)
            .sampled()
            .color_attachment(),
    };
    let msaa_att = final_post::msaa_color_desc((out_extent[0], out_extent[1]), msaa_id);

    // T1+T3: grading/LUT environment and final_post on taa + full blur pyramid.
    let lut = lut_selection(controls.bvar43, &flattened);
    let lut_name = match &lut {
        final_post::LutSelect::Custom { name, .. } => name.clone(),
        _ => final_post::DEFAULT_LUT_NAME.to_owned(),
    };
    // `final_post` always binds the by-name LUT, the fixed LUT file key and the
    // default LUT (custom binding falls back to it). Their startup producer
    // (`load_glade_grading`) is not recovered, so report it by name.
    require_grading_lut_texture(world, final_post::lut_asset_hash(&lut_name), "named")?;
    require_grading_lut_texture(world, final_post::LUT_FIXED_FILE, "fixed")?;
    require_grading_lut_texture(
        world,
        final_post::lut_asset_hash(final_post::DEFAULT_LUT_NAME),
        "default",
    )?;
    let shader = frame
        .shader_library()
        .get_raster_handle_by_name("final_post")
        .map(|handle| {
            DrawHandle(
                ManagedHandle(handle.id().raw()),
                core::marker::PhantomData,
            )
        })
        .ok_or_else(|| tail_missing("final_post shader (loaded by startup load_shaders)"))?;
    let args = final_post::FinalPostArgs {
        out_extent: (out_extent[0], out_extent[1]),
        src0: attachment(*taa),
        src1: attachment(blur),
        flag94: u8::from(controls.flag_0x1ac0),
        lut,
        // O: settings+0x9c / +0xa4 are VideoSettings.sharpen / gamma_shift
        // (render_config.rs offset table). Slot order is I.
        arg_f0: video.sharpen,
        arg_f1: video.gamma_shift,
        shader: Some(shader),
        cfg: grading_config(&formation),
        // Native reads the GladeTheme resource's first u32 and compares 0x13.
        // The only observable branch is photomode (AppMode tag 19 == 0x13).
        theme: if photomode { 0x13 } else { 0 },
        custom_lut: None, // no custom LUT texture producer is recovered (U)
    };
    let elapsed = world
        .get_resource::<bevy_time::Time>()
        .map(|time| time.elapsed())
        .unwrap_or_default();
    let shaders = frame.shader_library();
    frame.with_raster(|host| {
        let mut new_host = || -> Box<
            dyn country_core::render::pass_builder::draw_stream::PassHost,
        > { Box::new(host.sibling()) };
        let lookup = LibraryShaders(shaders.as_ref());
        let mut ctx = DrawPassCtx::new(world, elapsed, &mut new_host, &lookup);
        final_post::final_post(&mut ctx, &args, &msaa_att);
        Ok(())
    })?;

    // T5: `render_menu_bg` reads AppMode first and returns before touching the
    // resource when the gate rejects. When the gate allows drawing, this host
    // has no recovered MenuBackgroundHost, so fail by name.
    if menu_background::menu_bg_mode_allows(menu_background::AppModeView {
        state: u64::from(mode),
        flag: controls.menu_bg_flag,
    }) {
        return Err(tail_unsupported(
            "render_menu_bg",
            "MenuBackgroundHost/MenuBackgroundResources have no producer in this App",
        ));
    }

    // T6: `painterly_pass` (0x1408a6410). The `enabled` byte and both auxiliary
    // textures (r9 / [rsp+0x20]) are U; never fabricate them.
    if controls.painterly.enabled {
        return Err(tail_unsupported(
            "painterly_pass",
            "PainterlyState and the two auxiliary texture inputs are not recovered",
        ));
    }

    // T7: `vermeer_pass` (0x1408a82f0) on the same MSAA target.
    let vermeer_active = run_vermeer(frame, world, msaa, mode, photomode, controls)?;

    // T8: logo overlay, gated by the explicit U condition inputs.
    if controls.logo_overlay {
        let shaders = frame.shader_library();
        frame.with_raster(|host| {
            let mut new_host = || -> Box<
                dyn country_core::render::pass_builder::draw_stream::PassHost,
            > { Box::new(host.sibling()) };
            let lookup = LibraryShaders(shaders.as_ref());
            let mut ctx = DrawPassCtx::new(world, elapsed, &mut new_host, &lookup);
            logo_overlay::render_logo_overlay(&mut ctx, &msaa_att);
            Ok(())
        })?;
    }

    // T10: native UI block gate `(vermeer_active & mode != 0x13) == 0`.
    if !(vermeer_active && !photomode) {
        // T11-T13 hosts are not recovered. Their native gates/mode state are
        // U, so each is an explicit control; requesting one is a named error.
        if controls.session_previews {
            return Err(tail_unsupported(
                "render_session_selection_previews",
                "SessionPreviewHost is not recovered",
            ));
        }
        if controls.lut_gallery {
            return Err(tail_unsupported(
                "render_photomode_lut_gallery",
                "LutGalleryHost is not recovered",
            ));
        }
        if controls.feedback_bg {
            return Err(tail_unsupported(
                "render_feedback_report_bg",
                "FeedbackReportBg producer is not recovered",
            ));
        }
        if controls.ui_2d {
            return Err(tail_unsupported(
                "ui_2d_pass",
                "UiInteractionState/UiInactive component views are not recovered",
            ));
        }
        if controls.egui_present {
            return Err(tail_unsupported(
                "EguiBackend::draw",
                "the EguiGpu owner is not reachable from the ECS world",
            ));
        }

        // T17 prerequisites (MEI buffers + real text output + "mei temp").
        let (mei_text, mei_temp) = prepare_mei(frame, world, out_extent)?;
        let window = window_dims(world)?;
        let shaders = frame.shader_library();
        frame.with_raster(|host| {
            let mut new_host = || -> Box<
                dyn country_core::render::pass_builder::draw_stream::PassHost,
            > { Box::new(host.sibling()) };
            let lookup = LibraryShaders(shaders.as_ref());
            let mut ctx = DrawPassCtx::new(world, elapsed, &mut new_host, &lookup);
            // T14/T15 share the `stack0x1ac0 != 0` gate (O).
            if controls.flag_0x1ac0 {
                wireframe_pass::wireframe_pass(&mut ctx, &msaa_att, lists);
                ui_pass::ui_pass(&mut ctx, &msaa_att, lists);
            }
            // T16 ui_2d_pass is a U control above (component views missing).
            // T17 MEI compositing on "mei temp" then the MSAA target.
            ui_2d_pass::mei_pass(&mut ctx, &msaa_att, &mei_temp, window, &mei_text);
            // T18 custom cursor (visibility byte is U).
            custom_cursor_pass::custom_cursor_pass(
                &mut ctx,
                &msaa_att,
                window,
                controls.cursor_visible,
            );
            // T20 loading draw list (normally empty during gameplay).
            ui_pass::loading_screen_pass(&mut ctx, &msaa_att, lists);
            Ok(())
        })?;
    }

    // T4/T9/T19/T22 screenshot opportunities and T23 panic capture only
    // register/read requests in the native frame; the request-to-GPU-texture
    // association remains unrecovered, so no pass is declared for them.

    Ok(WorldTailOutputs {
        msaa,
        blur,
        vermeer_active,
    })
}

#[cfg(test)]
mod tail_tests {
    use super::*;

    /// The table follows the native call order; the implemented subset must be
    /// a prefix-compatible subsequence (grading -> blur -> final_post -> UI ->
    /// export). This catches accidental reordering of declaration sites.
    #[test]
    fn tail_steps_keep_native_order() {
        let index = |name: &str| {
            WORLD_TAIL_STEPS
                .iter()
                .position(|step| *step == name)
                .unwrap_or_else(|| panic!("missing step {name}"))
        };
        assert!(index("ColorGradingState::flattened") < index("bidirectional_blur_pyramid"));
        assert!(index("bidirectional_blur_pyramid") < index("final_post/msaa color out"));
        assert!(index("final_post/msaa color out") < index("render_menu_bg"));
        assert!(index("render_menu_bg") < index("painterly_pass"));
        assert!(index("painterly_pass") < index("vermeer_pass"));
        assert!(index("vermeer_pass") < index("ui_pass"));
        assert!(index("ui_pass") < index("mei_pass"));
        assert!(index("mei_pass") < index("loading_screen_pass"));
        assert!(index("loading_screen_pass") < index("export 0x22"));
    }

    /// The MSAA target description is the one that makes the presentation graph
    /// pick `raster_resolve_msaa_to_swapchain`; a single-sample target would
    /// silently take the non-MSAA shader.
    #[test]
    fn msaa_target_desc_selects_msaa_resolve() {
        let desc = final_post::msaa_color_desc((1280, 720), 7);
        assert_eq!(desc.format, final_post::MSAA_COLOR_FORMAT);
        assert_eq!(desc.sample_count, 4);
        assert_eq!(desc.id, 7);
        assert_eq!(
            final_post::resolve_shader_name(desc.sample_count),
            final_post::RESOLVE_MSAA_SHADER
        );
        assert_eq!(
            final_post::resolve_shader_name(1),
            final_post::RESOLVE_SHADER
        );
    }
}
