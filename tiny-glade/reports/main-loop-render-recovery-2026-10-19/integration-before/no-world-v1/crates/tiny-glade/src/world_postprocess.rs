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
#[path = "session_preview_host.rs"]
mod session_preview_host;
pub use session_preview_host::install as install_session_previews;

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
                custom_cursor_pass,
                d_main_support::{DrawPassCtx, ShaderLookup},
                draw_lists::DrawLists,
                final_post, logo_overlay, ui_2d_pass, ui_pass, wireframe_pass,
            },
            menu_background, painterly,
            render_world::{formats, names},
            vermeer,
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
    frame: &mut CoreGraphFrame<'_>,
    world: &mut World,
    taa: RgTexture,
) -> rhapsody::Result<RgTexture> {
    use bevy_ecs::change_detection::DetectChanges;
    use country_core::{
        resources::shader_refactoring_state::ShaderRefactoringState,
        systems::render_loop::compute_passes::shader_refactoring::{self, FrameHandle, StateMut},
    };
    let state = world
        .get_resource::<ShaderRefactoringState>()
        .ok_or_else(|| missing("ShaderRefactoringState"))?;
    // Native disc0 returns after this mandatory lookup, without allocation,
    // changed-tick mutation, shaders or any extra pass-resource lookup.
    if state.disc == 0 {
        return Ok(taa);
    }
    let tick = world.read_change_tick().get();
    world.resource_scope(
        |world, mut canonical: bevy_ecs::change_detection::Mut<ShaderRefactoringState>| {
            let active = matches!(canonical.disc, 1..=4);
            let mut state = StateMut {
                state: *canonical,
                changed_tick: canonical.last_changed().get(),
            };
            crate::scene_compute::with_env(frame, world, |env| {
                let run = shader_refactoring::try_render_shader_refactoring(
                    env,
                    &mut state,
                    tick,
                    &FrameHandle {
                        texture: taa,
                        format: taa.desc.format,
                    },
                )?;
                for pass in run.passes {
                    env.record(pass);
                }
                if active {
                    *canonical = state.state;
                }
                Ok(run.replacement_frame.unwrap_or(taa))
            })
        },
    )
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

/// Native menu policy result carried through graph declaration.
#[derive(Clone, Copy, Debug)]
pub struct PainterlyTailControl {
    /// `stack0x1a88`: Game loading state && menu.should_show_world && menu.is_open.
    pub enabled: bool,
}

#[derive(Clone, Copy, Debug)]
pub struct WorldTailControls {
    /// `stack0x1ac0`: selected actual-world/PrimaryRT branch, including zero
    /// for static menu reuse. Also final_post flag94 and wireframe/UI gate.
    pub flag_0x1ac0: bool,
    /// First Awaiting screenshot purpose. No Awaiting capture is native 7.
    pub bvar43: u8,
    pub painterly: PainterlyTailControl,
    /// Demo-purpose branch or ordinary purpose3 with GameConfig.ui.game_logo.
    pub logo_overlay: bool,
    /// T18 GameConfig.ui.custom_cursor (+0xf4).
    pub cursor_visible: bool,
}

/// Snapshot native tail inputs from their canonical owners. The main-world
/// branch determines flag_0x1ac0; it cannot be inferred from display state.
pub fn resolve_world_tail_controls(world: &World, decision: crate::world_graph::MainWorldDecision) -> rhapsody::Result<WorldTailControls> {
    let config = world.get_resource::<GameConfig>().ok_or_else(|| tail_missing("GameConfig"))?;
    let purpose = crate::screenshot::active_capture_purpose(world);
    Ok(WorldTailControls {
        flag_0x1ac0: decision.render_world,
        bvar43: purpose,
        painterly: PainterlyTailControl {
            enabled: decision.painterly_enabled,
        },
        logo_overlay: if config.dev.demo { matches!(purpose, 3 | 4) }
            else { purpose == 3 && config.ui.game_logo },
        cursor_visible: config.ui.custom_cursor,
    })
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
    /// T10 gate shared by T11-T19. Loading and egui remain outside it.
    pub ui_visible: bool,
}

fn tail_missing(name: &str) -> rhapsody::Error {
    rhapsody::Error::invalid_usage(format!(
        "original compile_main 0x140161cd0 tail: missing {name}"
    ))
}

/// `final_post` binds its grading LUTs through `TextureFromDisk`; the raster
/// host resolves those hashes from the named texture table and panics when a
/// producer is missing. Check them first so the frame fails with the missing
/// asset and its startup producer instead of a generic "No such texture".
fn require_grading_lut_texture(world: &World, hash: u64, what: &str) -> rhapsody::Result<()> {
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
            "{what} grading LUT texture 0x{hash:016x} (startup load_glade_grading did not publish this asset)"
        )));
    }
    Ok(())
}

/// `stack0x1490` LUT selection (0x140161cd0.c:1126-1149). The native branches
/// are `bVar43 == 7 || ((b-2) != 3 && (b-2) < 5)` -> Custom(flattened name,
/// flattened +0x18, flag 0); otherwise the niche byte is `b & 1` (1 = Keep,
/// 0 = Default).
fn lut_selection(b: u8, cfg: &ImageFormationConfig) -> final_post::LutSelect {
    let k = b.wrapping_sub(2);
    if b == 7 || (k != 3 && k < 5) {
        final_post::LutSelect::Custom { name: cfg.name.clone(), blend: cfg.params_18[0], flag: 0 }
    } else if b & 1 == 1 { final_post::LutSelect::Keep }
    else { final_post::LutSelect::Default }
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
    shadows: bool,
) -> rhapsody::Result<(
    country_core::systems::render_loop::draw_passes::ui_2d_pass::MeiTextOutput,
    Option<AttachmentTexture>,
)> {
    use country_core::{
        asset_libraries::texture_library::AssetTextureLibrary, name_hash::NameHash,
    };
    if !world.contains_resource::<tg_text::TextRenderer>() {
        return Err(tail_missing("TextRenderer (Update44 text_flush_system)"));
    }
    let raster = frame.raster_resources();
    {
        // Native MEI binds the blue_noise disk texture (hash verified against
        // the shipped compiled asset). The font atlas is the separate raw
        // text.atlas binding; resolve this named noise texture from the same App.
        let textures = world
            .get_resource::<AssetTextureLibrary>()
            .ok_or_else(|| tail_missing("AssetTextureLibrary"))?;
        let source = textures
            .get_handle_and_texture_by_name(NameHash(0x579f_39ab_91a0_1865))
            .map(|(_, texture)| texture)
            .ok_or_else(|| tail_missing("MEI blue_noise texture 0x579f39ab91a01865"))?;
        raster
            .assets
            .lock()
            .textures_by_name
            .entry(0x579f_39ab_91a0_1865)
            .or_insert((ManagedHandle(source.id), AttachmentTexture::from(source)));
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
    let temp = if shadows { Some(frame.with_compute(|compute| {
        Ok(compute.create_raw_texture("mei temp", &TextureDesc::new_2d(out_extent, 0x2b)))
    })?) } else { None };
    Ok((
        text,
        temp.map(|temp| AttachmentTexture {
            id: temp.resource,
            extent: (out_extent[0], out_extent[1]),
            format: 0x2b,
            sample_count: 1,
        }),
    ))
}

/// T6 through the existing effect host. RNE163131/163144 retain both
/// real render_world auxiliaries. Native EscMenuState::Settings(Image) skips
/// before the mutable PainterlyState lookup and its changed tick.
fn run_painterly(
    frame: &mut CoreGraphFrame<'_>, world: &mut World, target: RgTexture,
    post: &WorldPostOutputs, controls: &WorldTailControls,
) -> rhapsody::Result<()> {
    use crate::frame_world::{FrameLibraries, FrameWorldAccess};
    use country_core::{asset_libraries::{placement_data_library::AssetPlacementDataLibrary,
        ssbo_library::AssetSsboLibrary, texture_library::AssetTextureLibrary,
        vao_library::AssetVAOLibrary}, resources::gui::EscMenuState};
    let menu = world.get_resource::<EscMenuState>().ok_or_else(|| tail_missing("EscMenuState"))?.view();
    if menu.tag == 7 && menu.flag == 1 { return Ok(()); }
    if !world.contains_resource::<painterly::PainterlyState>() { return Err(tail_missing("PainterlyState")); }
    let shaders = frame.shader_library();
    let backend = frame.shader_backend();
    world.resource_scope(|world, mut state: bevy_ecs::change_detection::Mut<painterly::PainterlyState>| {
        use bevy_ecs::change_detection::DetectChangesMut;
        state.set_changed();
        let instant_flag = world.get_resource::<country_core::resources::gui::DebugGui>()
            .ok_or_else(|| tail_missing("DebugGui"))?.painterly_instant;
        // The frame counter lookup is skipped when the debug byte is true.
        let count = if instant_flag { None } else {
            Some(world.get_resource::<country_core::render::frame_constants::FrameConstants>()
                .ok_or_else(|| tail_missing("FrameConstants"))?.frame_counter)
        };
        if !controls.painterly.enabled {
            state.phase = painterly::PainterlyPhase::Disabled;
            return Ok(());
        }
        // The native reset precedes temporal/shader/resource lookup failures.
        if state.phase == painterly::PainterlyPhase::Disabled || instant_flag || count == Some(0) {
            state.progress = 0.0;
            state.phase = painterly::PainterlyPhase::Pending;
            state.timer = 0.0;
        }
        let time = world.get_resource::<bevy_time::Time>().map(|time| painterly::TimeView {
            delta_secs_f64: time.delta_secs_f64(), delta_secs: time.delta_secs(),
        });
        let raster = frame.raster_resources();
        let access = FrameWorldAccess::new(FrameLibraries {
            shaders: shaders.as_ref(),
            textures: world.get_resource::<AssetTextureLibrary>().ok_or_else(|| tail_missing("AssetTextureLibrary"))?,
            ssbos: world.get_resource::<AssetSsboLibrary>().ok_or_else(|| tail_missing("AssetSsboLibrary"))?,
            vaos: world.get_resource::<AssetVAOLibrary>().ok_or_else(|| tail_missing("AssetVAOLibrary"))?,
            placement: world.get_resource::<AssetPlacementDataLibrary>().ok_or_else(|| tail_missing("AssetPlacementDataLibrary"))?,
            constants: raster.dynamic_constants(),
        }, frame.compute_device()).with_resource_graph_factory(frame.resource_graph_factory());
        let mut effects = crate::effects_host::EffectsHost::new(frame, &access, &*backend, shaders.as_ref(),
            Some(painterly::AppModeView { state: menu.tag, flag: menu.flag }), time, None, false);
        let color = effects.register_texture_with_samples(target, 4);
        let aux_b = effects.register_texture(post.depth);
        let aux_c = effects.register_texture(post.coc);
        painterly::painterly_pass(&mut effects, &mut state,
            painterly::PainterlyInputs { color, aux_b, aux_c, enabled: controls.painterly.enabled },
            || Some(instant_flag), || count);
        effects.finish()
    })
}

/// T7 `vermeer_pass` through the recovered Painterly effect host. The state and
/// debug resources are real gameplay resources (`run_game.rs` inserts both);
/// missing state is a hard error because the native system reads it directly.
fn run_vermeer(
    frame: &mut CoreGraphFrame<'_>,
    world: &mut World,
    target: RgTexture,
    _mode: u32,
    photomode: bool,
    _controls: &WorldTailControls,
) -> rhapsody::Result<bool> {
    use crate::frame_world::{FrameLibraries, FrameWorldAccess};
    use country_core::asset_libraries::{
        placement_data_library::AssetPlacementDataLibrary, ssbo_library::AssetSsboLibrary,
        texture_library::AssetTextureLibrary, vao_library::AssetVAOLibrary,
    };
    let shaders = frame.shader_library();
    let backend = frame.shader_backend();
    let debug = {
        use bevy_ecs::change_detection::DetectChangesMut;
        let mut debug = world.get_resource_mut::<vermeer::VermeerDebugControl>()
            .ok_or_else(|| tail_missing("VermeerDebugControl"))?;
        // The native mutable lookup writes its changed tick before phase gating.
        debug.set_changed();
        *debug
    };
    if !world.contains_resource::<vermeer::VermeerState>() {
        return Err(tail_missing("VermeerState"));
    }
    let skip_generate = world.get_resource::<FrameRenderConfig>().ok_or_else(|| tail_missing("FrameRenderConfig"))?.flag_20;
    let menu = world.get_resource::<country_core::resources::gui::EscMenuState>()
        .ok_or_else(|| tail_missing("EscMenuState"))?.view();
    // Original process static143634c9c survives each frame; never recreate
    // prev_active=false before the native temporal-reuse conjunction.
    world.init_resource::<vermeer::VermeerPassLocals>();
    world.resource_scope(
        |world, mut state: bevy_ecs::change_detection::Mut<vermeer::VermeerState>| {
        world.resource_scope(|world, mut locals: bevy_ecs::change_detection::Mut<vermeer::VermeerPassLocals>| {
        use bevy_ecs::change_detection::DetectChangesMut;
        state.set_changed();
        if state.phase == vermeer::VermeerPhase::Disabled {
            if debug.restart { state.activate(false, false); }
            else { locals.prev_active = false; }
            return Ok(false);
        }
        // Only an active reveal consumes Time and GPU/asset owners. The native
        // missing-Time unwrap remains after its temporal/acquire declarations.
        let time = world.get_resource::<bevy_time::Time>().map(|time| painterly::TimeView {
            delta_secs_f64: time.delta_secs_f64(), delta_secs: time.delta_secs(),
        });
        type Finished = bevy_ecs::event::Events<
            country_core::systems::save::load_game::StartupRevealFinished>;
        // Absence is observed only by send_startup_reveal_finished after the
        // core consumer has transitioned the completed reveal to Disabled.
        // An active but incomplete reveal does not require the event resource.
        let mut declare = |world: &World, finished: Option<&mut Finished>| {
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
                    state: menu.tag,
                    flag: menu.flag,
                }),
                time,
                Some(debug),
                // Static 0x143634d90 selects the alternate background name (U).
                false,
            );
            if let Some(finished) = finished {
                effects = effects.with_startup_reveal_finished_events(finished);
            }
            let target = effects.register_texture_with_samples(target, 4);
            let active = vermeer::vermeer_pass(
                &mut effects,
                &mut locals,
                &mut *state,
                vermeer::VermeerPassParams {
                    target,
                    photomode,
                    // RNE140163264 reads the real FrameRenderConfig+0x20 byte.
                    skip_generate,
                },
            );
            effects.finish()?;
            Ok(active)
        };
        match world.try_resource_scope(|world, mut finished: bevy_ecs::change_detection::Mut<Finished>| {
            let before = finished.len();
            let result = declare(world, Some(finished.bypass_change_detection()));
            // Passing the optional owner must not mark it changed on every
            // active frame. Only the core consumer's actual send changes it.
            if finished.len() != before { finished.set_changed(); }
            result
        }) {
            Some(result) => result,
            None => declare(world, None),
        }
        })
        },
    )
}

/// Snapshot the native screen-space tuple (stable sort happens in ui_2d_pass)
/// and the unsorted world-space tuple. Optional component absence is preserved.
fn collect_ui_2d(
    world: &mut World,
) -> (
    Vec<ui_2d_pass::Ui2dDrawElement>,
    Vec<ui_2d_pass::Ui2dWsElement>,
) {
    use country_core::{
        components::{
            Ui2dExtent, Ui2dImage, Ui2dWsExtent, UiDrawOrder, UiInactive, UiInteractionState,
        },
        resources::color_icon::CustomizationIcon,
    };
    let mut query = world.query::<(
        &Ui2dExtent,
        &Ui2dImage,
        Option<&CustomizationIcon>,
        Option<&UiInteractionState>,
        Option<&UiDrawOrder>,
        Option<&UiInactive>,
    )>();
    let elements = query
        .iter(world)
        .map(
            |(extent, image, icon, interaction, order, inactive)| ui_2d_pass::Ui2dDrawElement {
                extent: [extent.pos.x, extent.pos.y, extent.size.x, extent.size.y],
                image: image.into(),
                icon: icon.map(|icon| ui_2d_pass::CustomizationIconRead {
                    alpha: icon.alpha_18,
                    flag: icon.flag_21 != 0,
                }),
                interaction: ui_2d_pass::UiInteractionStateRead {
                    flag: interaction.is_some_and(|state| state.selected != 0),
                },
                order: order.map(|order| order.0),
                inactive: inactive.is_some(),
            },
        )
        .collect();
    let mut query = world.query::<(&Ui2dWsExtent, &Ui2dImage)>();
    let ws = query
        .iter(world)
        .map(|(extent, image)| ui_2d_pass::Ui2dWsElement {
            extent: extent.extent,
            position: extent.position,
            image: image.into(),
        })
        .collect();
    (elements, ws)
}

/// Owns a captured LUT preview's actual per-App texture-table entry. The
/// screenshot producer transfers its captured texture here after completion.
#[derive(bevy_ecs::resource::Resource)]
struct LutGalleryPreviewOwner {
    host: std::sync::Arc<crate::render_host::TextureHost>,
    texture: country_core::rhapsody_types::Texture,
}
impl Drop for LutGalleryPreviewOwner {
    fn drop(&mut self) {
        self.host.release(&self.texture);
    }
}

/// The native gather-screenshot producer allocates this shared optional slot
/// before scheduling its capture callback. It is the same Arc read by T12.
pub fn lut_gallery_capture_slot(
    world: &mut World,
) -> rhapsody::Result<
    std::sync::Arc<
        parking_lot::Mutex<
            Option<country_core::systems::render_loop::menu_background::ManagedTexture>,
        >,
    >,
> {
    let mut gallery = world
        .get_resource_mut::<country_core::systems::render_loop::lut_gallery::LutGalleryState>()
        .ok_or_else(|| tail_missing("LutGalleryState"))?;
    Ok(gallery
        .preview
        .get_or_insert_with(|| std::sync::Arc::new(parking_lot::Mutex::new(None)))
        .clone())
}

/// Transfer one completed capture destination from this exact TextureHost into
/// the LUT gallery. No texture is allocated or substituted by publication.
pub fn publish_lut_gallery_preview(
    world: &mut World,
    hosts: &crate::render_host::HostTables,
    texture: country_core::rhapsody_types::Texture,
) -> rhapsody::Result<()> {
    let slot = lut_gallery_capture_slot(world)?;
    publish_lut_gallery_preview_to_slot(world, hosts, &slot, texture)
}

/// Complete the exact request slot captured by gather_screenshot_requests.
/// Closing/reopening the gallery detaches the old Arc; an old completion must
/// release its destination instead of filling the newly opened gallery.
pub fn publish_lut_gallery_preview_to_slot(
    world: &mut World,
    hosts: &crate::render_host::HostTables,
    slot: &std::sync::Arc<
        parking_lot::Mutex<
            Option<country_core::systems::render_loop::menu_background::ManagedTexture>,
        >,
    >,
    texture: country_core::rhapsody_types::Texture,
) -> rhapsody::Result<()> {
    let actual = hosts
        .textures
        .get(&texture)
        .ok_or_else(|| tail_missing("captured LUT preview in the current TextureHost"))?;
    if actual.desc.sample_count != 1
        || !actual
            .desc
            .usage
            .contains(ash::vk::ImageUsageFlags::SAMPLED)
    {
        return Err(rhapsody::Error::invalid_usage(
            "LUT preview capture must be a single-sample sampled texture",
        ));
    }
    let current_slot = world
        .get_resource::<country_core::systems::render_loop::lut_gallery::LutGalleryState>()
        .and_then(|gallery| gallery.preview.as_ref());
    if !current_slot.is_some_and(|current| std::sync::Arc::ptr_eq(current, slot)) {
        hosts.textures.release(&texture);
        return Ok(());
    }
    *slot.lock() =
        Some(country_core::systems::render_loop::menu_background::ManagedTexture(texture.id));
    if world
        .get_resource::<LutGalleryPreviewOwner>()
        .is_some_and(|old| {
            old.texture.id == texture.id && std::sync::Arc::ptr_eq(&old.host, &hosts.textures)
        })
    {
        return Ok(());
    }
    world.insert_resource(LutGalleryPreviewOwner {
        host: hosts.textures.clone(),
        texture,
    });
    Ok(())
}

pub fn clear_lut_gallery_preview(world: &mut World) {
    if let Some(mut gallery) =
        world.get_resource_mut::<country_core::systems::render_loop::lut_gallery::LutGalleryState>()
    {
        gallery.preview = None;
    }
    world.remove_resource::<LutGalleryPreviewOwner>();
}

/// T12 uses the original optional preview slot, the current canonical String
/// configuration, and the actual loaded GradingLuts metadata. The old u128
/// model mistook the borrowed String ptr/len pair for a numeric name. No world
/// target is substituted when the preview capture producer has not filled its
/// slot: native 0x1408b0560 returns at that exact point.
fn run_lut_gallery(
    frame: &mut CoreGraphFrame<'_>,
    world: &mut World,
    target: AttachmentTexture,
) -> rhapsody::Result<()> {
    use country_core::{
        name_hash::NameHash,
        render::pass_builder::{
            draw_builder::{BindTexture, DrawBuilder},
            draw_stream::DrawPacket,
        },
        resources::{
            grading_lut::GradingLuts, gui::EscMenuState, texture_presets::TextureFromDisk,
        },
        systems::render_loop::{
            draw_passes::d_main_support::{ColorSpec, DepthSpec},
            lut_gallery::{self as gallery, LutGalleryState},
        },
    };
    // The same process demo flag already gates GpuLut and Vermeer setup.
    if world.resource::<GameConfig>().dev.demo {
        return Ok(());
    }
    let state = world
        .get_resource::<LutGalleryState>()
        .ok_or_else(|| tail_missing("LutGalleryState (LutGalleryPlugin::build)"))?;
    let metrics = state.state.clone();
    let preview = state.preview.clone();
    if !matches!(
        world
            .get_resource::<EscMenuState>()
            .ok_or_else(|| tail_missing("EscMenuState"))?,
        EscMenuState::Photomode(true)
    ) {
        return Ok(());
    }
    let Some(preview) = preview else {
        return Ok(());
    };
    let preview_guard = preview.lock();
    let Some(preview) = preview_guard.as_ref() else {
        return Ok(());
    };
    let preview_owner = frame
        .hosts()
        .textures
        .get_by_id(preview.0)
        .ok_or_else(|| tail_missing("LUT gallery preview texture in this App's TextureHost"))?;
    let logical_preview = frame.import_texture(
        &country_core::rhapsody_types::Texture {
            id: preview.0,
            width: preview_owner.desc.width,
            height: preview_owner.desc.height,
            format: preview_owner.desc.format.as_raw() as u32,
        },
        0x22,
    )?;
    let preview_desc = AttachmentTexture {
        id: logical_preview.raw(),
        extent: (preview_owner.desc.width, preview_owner.desc.height),
        format: preview_owner.desc.format.as_raw() as u32,
        sample_count: preview_owner.desc.sample_count,
    };
    let configurations = world
        .get_resource::<ImageFormationConfigs>()
        .ok_or_else(|| tail_missing("ImageFormationConfigs"))?;
    let active = configurations.current().name.clone();
    let initial = configurations
        .get(0)
        .expect("native initial grading configuration");
    let first = (initial.name.clone(), initial.params_18[0]);
    let names: Vec<_> = std::iter::once(first)
        .chain(
            world
                .get_resource::<GradingLuts>()
                .ok_or_else(|| tail_missing("GradingLuts (load_glade_grading)"))?
                .metas
                .iter()
                .map(|meta| (meta.name.clone(), 1.0)),
        )
        .collect();
    let state = *metrics.lock();
    let shaders = frame.shader_library();
    let elapsed = world
        .get_resource::<bevy_time::Time>()
        .map(|time| time.elapsed())
        .unwrap_or_default();
    frame.with_raster(|host| {
        let default_state =
            country_core::render::pass_builder::draw_stream::PassHost::default_raster_state(&host);
        let mut new_host =
            || -> Box<dyn country_core::render::pass_builder::draw_stream::PassHost> {
                Box::new(host.sibling())
            };
        let lookup = LibraryShaders(shaders.as_ref());
        let mut ctx = DrawPassCtx::new(world, elapsed, &mut new_host, &lookup);
        let shader = ctx
            .shader(gallery::SHADER_NAME)
            .ok_or_else(|| tail_missing("lut_gallery_thumbnail shader"))?;
        let mut pass = ctx.raster_pass(
            gallery::PASS_NAME,
            &[ColorSpec::tag2(target)],
            DepthSpec::None,
            target,
        );
        for (index, ((name, strength), position)) in names.iter().zip(state.positions()).enumerate()
        {
            let u =
                gallery::cell_uniforms(&state, index as u32, position.to_array(), *name == active);
            let packet = DrawPacket::new(shader.clone(), [0, 0], default_state);
            let asset = country_core::resources::grading_lut::lut_name_to_asset_name(name);
            let mut draw = DrawBuilder::new(&mut pass, packet, 0)
                .bind_texture_to_draw_builder(
                    BindTexture::RgHandle(&preview_desc),
                    gallery::PREVIEW_BINDING,
                    gallery::TEXTURE_ACCESS,
                    None,
                )
                .bind_texture_to_draw_builder(
                    BindTexture::FromDisk(TextureFromDisk(gallery::ICON_DISK_HASH)),
                    gallery::ICON_BINDING,
                    gallery::TEXTURE_ACCESS,
                    None,
                )
                .bind_texture_to_draw_builder(
                    BindTexture::FromDisk(TextureFromDisk(NameHash::compute(&asset).0)),
                    gallery::LUT_BINDING,
                    gallery::TEXTURE_ACCESS,
                    None,
                )
                .optional_uniform(gallery::U_TARGET_EXTENT, [target.extent.0, target.extent.1])
                .uniform(gallery::U_ALPHA, *strength)
                .uniform(gallery::U_CELL_POS, ui_2d_pass::Vec2Uniform(position))
                .uniform(
                    gallery::U_CELL_SIZE,
                    ui_2d_pass::Vec2Uniform(glam::Vec2::from_array(u.size)),
                )
                .uniform(gallery::U_HIGHLIGHT, u.highlighted)
                .uniform(gallery::U_IS_CURRENT, u.is_current)
                .uniform(gallery::U_IS_FIRST, u.is_first);
            let packet = draw.packet_mut();
            packet.state = packet.state.clear_byte0_bit7().set_byte3_field_2();
            draw.draw_no_vao(gallery::VERTICES_PER_CELL);
        }
        pass.render();
        Ok(())
    })
}

/// T13's first resource is the same native EscMenuState TypeId as T11/T12.
/// The FeedbackReportBg projection is mandatory only in ReportBug (tag 8),
/// then the original consumer owns visible/page gates, shader and draw rules.
fn run_feedback_report_bg(
    frame: &mut CoreGraphFrame<'_>,
    world: &mut World,
    target: AttachmentTexture,
) -> rhapsody::Result<()> {
    use country_core::{
        resources::gui::EscMenuState,
        systems::render_loop::draw_passes::feedback_report_bg::{self, FeedbackReportBg},
    };
    let tag = world
        .get_resource::<EscMenuState>()
        .ok_or_else(|| tail_missing("EscMenuState"))?
        .view()
        .tag;
    if tag != feedback_report_bg::GATE_STATE {
        return Ok(());
    }
    let report = world
        .get_resource::<FeedbackReportBg>()
        .ok_or_else(|| {
            tail_missing("FeedbackReportBg capture/state producer (EscMenuState::ReportBug)")
        })?
        .clone();
    let elapsed = world
        .get_resource::<bevy_time::Time>()
        .map(|time| time.elapsed())
        .unwrap_or_default();
    let shaders = frame.shader_library();
    frame.with_raster(|host| {
        let mut new_host =
            || -> Box<dyn country_core::render::pass_builder::draw_stream::PassHost> {
                Box::new(host.sibling())
            };
        let lookup = LibraryShaders(shaders.as_ref());
        let mut ctx = DrawPassCtx::new(world, elapsed, &mut new_host, &lookup);
        feedback_report_bg::render_feedback_report_bg(&mut ctx, &target, tag, &report);
        Ok(())
    })
}

/// Build the native compile_main tail after W54. Caller owns the enclosing
/// main graph, `with_main_prelude`, and the 0x22 export of the returned target.
pub fn build_world_tail(
    frame: &mut CoreGraphFrame<'_>,
    world: &mut World,
    lists: &DrawLists,
    post: &WorldPostOutputs,
    out_extent: [u32; 2],
    controls: &WorldTailControls,
) -> rhapsody::Result<WorldTailOutputs> {
    let taa = &post.taa;
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
    // T1: original purpose1/5 select the initial configuration for flattening;
    // the remaining purposes select current_mut. final_post independently
    // reads the current configuration for lens/contrast fields.
    // ColorGradingState::flattened@140b00b20 is CPU-only.
    let formation = world
        .get_resource::<ImageFormationConfigs>()
        .ok_or_else(|| tail_missing("ImageFormationConfigs"))?
        .current()
        .clone();
    let purpose = controls.bvar43;
    let flattened = if matches!(purpose, 1 | 5) {
        world.resource::<ImageFormationConfigs>().configs[0].flattened()
    } else { formation.flattened() };


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
    let native_msaa = rhapsody::resource::TextureDesc::new_2d(
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
    // default LUT (custom binding falls back to it). The fixed file key is
    // tony_mc_mapface; startup loads it alongside the directory grading LUTs.
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
        .map(|handle| DrawHandle(ManagedHandle(handle.id().raw()), core::marker::PhantomData))
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
        // TypeId c2b8... is AppMode, independently proved by registration.
        theme: mode,
        custom_lut: None, // no custom LUT texture producer is recovered (U)
    };
    let elapsed = world
        .get_resource::<bevy_time::Time>()
        .map(|time| time.elapsed())
        .unwrap_or_default();
    let shaders = frame.shader_library();
    frame.with_raster(|host| {
        let mut new_host =
            || -> Box<dyn country_core::render::pass_builder::draw_stream::PassHost> {
                Box::new(host.sibling())
            };
        let lookup = LibraryShaders(shaders.as_ref());
        let mut ctx = DrawPassCtx::new(world, elapsed, &mut new_host, &lookup);
        final_post::final_post(&mut ctx, &args, &msaa_att);
        Ok(())
    })?;

    // T4: the original shared-texture request opportunities, before overlays.
    for purpose in [0, 1, 2] {
        crate::screenshot::shared_texture_capture::opportunity(frame, world, purpose, msaa, 4)?;
    }

    // T5: original canonical EscMenuState gate and captured texture owner.
    crate::menu_background_host::render(frame, world, msaa_att)?;

    // T6: original r9=returned depth, stack20=returned CoC. Disabled also
    // calls the state consumer, which must clear an earlier Ready phase.
    run_painterly(frame, world, msaa, post, controls)?;

    // T7: `vermeer_pass` (0x1408a82f0) on the same MSAA target.
    let vermeer_active = run_vermeer(frame, world, msaa, mode, photomode, controls)?;

    // T8: logo overlay, gated by the explicit U condition inputs.
    if controls.logo_overlay {
        let shaders = frame.shader_library();
        frame.with_raster(|host| {
            let mut new_host =
                || -> Box<dyn country_core::render::pass_builder::draw_stream::PassHost> {
                    Box::new(host.sibling())
                };
            let lookup = LibraryShaders(shaders.as_ref());
            let mut ctx = DrawPassCtx::new(world, elapsed, &mut new_host, &lookup);
            logo_overlay::render_logo_overlay(&mut ctx, &msaa_att);
            Ok(())
        })?;
    }

    // T9 runs only in the screenshot-purpose branch that reaches it (the
    // same demo branch also draws the logo for purpose4).
    let purpose = controls.bvar43;
    if purpose == 3 || (world.resource::<GameConfig>().dev.demo && purpose == 4) {
        crate::screenshot::shared_texture_capture::opportunity(frame, world, 3, msaa, 4)?;
    }

    // T10: native UI block gate `(vermeer_active & mode != 0x13) == 0`.
    let ui_visible = !vermeer_active || photomode;
    if ui_visible {
        // T11 uses the canonical menu/saved-list/pool and real image host.
        session_preview_host::render(frame, world, msaa_att)?;
        // T12 native order: after session previews, before feedback/wireframe.
        run_lut_gallery(frame, world, msaa_att)?;
        // T13 feedback capture/report is explicitly excluded by the user.
        // The historical recovered consumer remains available for reference.

        // T16 snapshots the original two queries from actual ECS components.
        // Toolbar writers share the same canonical UiInteractionState TypeId.
        let (ui_elements, ui_ws_elements) = collect_ui_2d(world);
        // T17 prerequisites (MEI buffers + real text output + "mei temp").
        let shadows = world.get_resource::<country_core::resources::gui::EscMenuState>()
            .ok_or_else(|| tail_missing("EscMenuState"))?.cast_ui_shadows();
        let (mei_text, mei_temp) = prepare_mei(frame, world, out_extent, shadows)?;
        let window = window_dims(world)?;
        let shaders = frame.shader_library();
        frame.with_raster(|host| {
            let mut new_host =
                || -> Box<dyn country_core::render::pass_builder::draw_stream::PassHost> {
                    Box::new(host.sibling())
                };
            let lookup = LibraryShaders(shaders.as_ref());
            let mut ctx = DrawPassCtx::new(world, elapsed, &mut new_host, &lookup);
            // T14/T15 share the `stack0x1ac0 != 0` gate (O).
            if controls.flag_0x1ac0 {
                wireframe_pass::wireframe_pass(&mut ctx, &msaa_att, lists);
                ui_pass::ui_pass(&mut ctx, &msaa_att, lists);
            }
            // T16 is unconditional inside the native T10 UI gate.
            ui_2d_pass::ui_2d_pass(&mut ctx, &msaa_att, window, &ui_elements, &ui_ws_elements);
            // T17 MEI compositing on "mei temp" then the MSAA target.
            ui_2d_pass::mei_pass_with_shadows(&mut ctx, &msaa_att, mei_temp.as_ref(), window, &mei_text);
            // T18 custom cursor (visibility byte is U).
            custom_cursor_pass::custom_cursor_pass(
                &mut ctx,
                &msaa_att,
                window,
                controls.cursor_visible,
            );
            Ok(())
        })?;
        crate::screenshot::shared_texture_capture::opportunity(frame, world, 4, msaa, 4)?;
    }
    // T20 is outside the reveal/UI gate (RNE1401634af). The native loading
    // list and caller-owned EguiBackend still render during an active reveal.
    frame.with_raster(|host| {
        let mut new_host = || -> Box<dyn country_core::render::pass_builder::draw_stream::PassHost> { Box::new(host.sibling()) };
        let lookup = LibraryShaders(shaders.as_ref());
        let mut ctx = DrawPassCtx::new(world, elapsed, &mut new_host, &lookup);
        ui_pass::loading_screen_pass(&mut ctx, &msaa_att, lists);
        Ok(())
    })?;

    Ok(WorldTailOutputs {
        msaa,
        blur,
        vermeer_active,
        ui_visible,
    })
}

#[cfg(test)]
mod tail_tests {
    use super::*;

    #[test]
    fn ui_queries_share_toolbar_owner_and_preserve_all_optional_draw_fields() {
        use country_core::components::{
            Ui2dExtent, Ui2dImage, Ui2dWsExtent, UiDrawOrder, UiInactive,
        };
        use country_core::resources::color_icon::CustomizationIcon;
        let mut world = World::new();
        world.spawn((
            Ui2dExtent {
                pos: glam::Vec2::new(2.0, 3.0),
                size: glam::Vec2::new(4.0, 5.0),
            },
            Ui2dImage {
                name: "actual/image".into(),
                layer: 7,
            },
            // This is the toolbar's public owner, queried through country-core.
            toolbar::resouces_and_components::UiInteractionState {
                selected: 1,
                ..Default::default()
            },
            UiDrawOrder(9),
            UiInactive,
            CustomizationIcon {
                alpha_18: 0.375,
                flag_21: 1,
                ..Default::default()
            },
        ));
        world.spawn((Ui2dExtent::default(), Ui2dImage::new("plain")));
        world.spawn((
            Ui2dWsExtent {
                extent: glam::Vec2::new(8.0, 9.0),
                position: glam::Vec3::new(1.0, 2.0, 3.0),
            },
            Ui2dImage::new("world"),
        ));
        let (elements, ws) = collect_ui_2d(&mut world);
        assert_eq!(elements.len(), 2);
        let decorated = elements
            .iter()
            .find(|e| e.image.name == "actual/image")
            .unwrap();
        assert_eq!(decorated.extent, [2.0, 3.0, 4.0, 5.0]);
        assert_eq!(decorated.image.layer, 7);
        assert_eq!(
            decorated.icon,
            Some(ui_2d_pass::CustomizationIconRead {
                alpha: 0.375,
                flag: true
            })
        );
        assert!(decorated.interaction.flag && decorated.inactive);
        assert_eq!(decorated.order, Some(9));
        let plain = elements.iter().find(|e| e.image.name == "plain").unwrap();
        assert!(plain.icon.is_none() && plain.order.is_none());
        assert!(!plain.inactive && !plain.interaction.flag);
        assert_eq!(ws.len(), 1);
        assert_eq!(ws[0].extent, glam::Vec2::new(8.0, 9.0));
        assert_eq!(ws[0].position, glam::Vec3::new(1.0, 2.0, 3.0));
    }

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
