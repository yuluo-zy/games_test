//! Typed production consumers for the recovered world graph.
//!
//! These are callable portions of B render_world, not a substitute 54-method
//! FramePasses implementation. Every method declares actual core passes on the
//! enclosing CoreGraphFrame. Complex results retain their real types rather
//! than inventing an opaque-handle ABI. The caller owns main FrameSetup and the
//! original ordering; this module never installs resources or submits a frame.

use bevy_ecs::world::World;
use country_core::{
    asset_libraries::{
        placement_data_library::AssetPlacementDataLibrary, ssbo_library::AssetSsboLibrary,
        texture_library::AssetTextureLibrary, vao_library::AssetVAOLibrary,
    },
    render::{
        pass_builder::{pass_resource_access::RgTexture, rg_resources::TemporalTexture},
        ssbo::InstanceCounted,
    },
    resources::{
        render_ssbo::{LanternSsbo, SpatialResolveOffsetsSsbo}, shader_refactoring_state::ShaderRefactoringState,
        world_raster_defs,
    },
    systems::render_loop::compute_passes::{
        apply_screen_shadows::{self, ScreenShadowInputs},
        apply_volumetrics, generate_mips,
        bin_lights::{self, LightBinning},
        dof::{self, DofConfig, DofPrepared},
        dof_autofocus::DofAutofocusSsbo,
        eval_gi_screen_radiance,
        generate_hzb::{self, Hzb},
        half_res::{self, GBuffer},
        pass_kit::PassEnv,
        ray_trace_gi::{self, RestirTextures},
        reprojection::{self, PrevDepth},
        restir_di::{self, LightGrid},
        ssgi,
        taa::{self, TaaInputs, TgsrInputs},
        upload_world_raster::{self, LiveRasterKind},
    },
};

use crate::{
    compute_frame::ComputeUniformAllocator,
    frame_world::{FrameLibraries, FrameWorldAccess},
    render_bridge::CoreGraphFrame,
};

fn missing(name: &str) -> rhapsody::Error {
    rhapsody::Error::invalid_usage(format!("original render_world: missing {name} producer"))
}

/// Build a real compute consumer with the six confirmed PassResources owners.
/// No compiler-specific Bevy TypeId or source-build resource layout is cast.
pub(crate) fn with_env<T>(
    frame: &mut CoreGraphFrame<'_>,
    world: &World,
    build: impl FnOnce(&mut PassEnv<'_>) -> rhapsody::Result<T>,
) -> rhapsody::Result<T> {
    let shaders = frame.shader_library();
    let backend = frame.shader_backend();
    let raster = frame.raster_resources();
    let access = FrameWorldAccess::new(
        FrameLibraries {
            shaders: &shaders,
            textures: world
                .get_resource::<AssetTextureLibrary>()
                .ok_or_else(|| missing("AssetTextureLibrary"))?,
            ssbos: world
                .get_resource::<AssetSsboLibrary>()
                .ok_or_else(|| missing("AssetSsboLibrary"))?,
            vaos: world
                .get_resource::<AssetVAOLibrary>()
                .ok_or_else(|| missing("AssetVAOLibrary"))?,
            placement: world
                .get_resource::<AssetPlacementDataLibrary>()
                .ok_or_else(|| missing("AssetPlacementDataLibrary"))?,
            constants: raster.dynamic_constants(),
        },
        frame.compute_device(),
    )
    .with_resource_graph_factory(frame.resource_graph_factory());
    frame.with_compute(|compute| {
        let mut uniforms = ComputeUniformAllocator;
        build(&mut PassEnv::new(
            &access,
            &*backend,
            &mut uniforms,
            compute,
        ))
    })
}

/// W1's actual ECS mutation and mapped SSBO owner. The native consumer marks
/// changed only when an upload is pending and after format admission, before
/// declaring the shader pass. The shared live helper preserves those points.
fn upload<R: LiveRasterKind>(
    frame: &mut CoreGraphFrame<'_>,
    world: &mut World,
    preset: u8,
) -> rhapsody::Result<Option<u32>> {
    if !world.contains_resource::<R>() {
        return Err(missing(std::any::type_name::<R>()));
    }
    let tick = world.read_change_tick().get();
    world.resource_scope(|world, mut raster: bevy_ecs::change_detection::Mut<R>| {
        with_env(frame, world, |env| {
            Ok(upload_world_raster::upload_live_world_raster(
                env,
                &mut raster,
                tick,
                preset,
            ))
        })
    })
}

/// One real W1 call. Slot is the original texture preset, not an ordinal.
pub fn upload_raster_staging_to_texture(
    frame: &mut CoreGraphFrame<'_>,
    world: &mut World,
    slot: u8,
) -> rhapsody::Result<Option<u32>> {
    match slot {
        1 => upload::<world_raster_defs::PathRaster>(frame, world, slot),
        2 => upload::<world_raster_defs::Cobblestone1Raster>(frame, world, slot),
        3 => upload::<world_raster_defs::Cobblestone2Raster>(frame, world, slot),
        4 => upload::<world_raster_defs::Cobblestone3Raster>(frame, world, slot),
        0x13 => upload::<world_raster_defs::WaterRaster>(frame, world, slot),
        0x14 => upload::<world_raster_defs::GardenRaster>(frame, world, slot),
        _ => Err(rhapsody::Error::invalid_usage(
            "unknown original world-raster preset",
        )),
    }
}

/// Original six calls in order. This does not run W2 or any other missing scene
/// stage, and is not installed as a successful production world renderer.
pub fn upload_world_rasters(
    frame: &mut CoreGraphFrame<'_>,
    world: &mut World,
) -> rhapsody::Result<[Option<u32>; 6]> {
    let mut result = [None; 6];
    for (i, slot) in country_core::systems::render_loop::render_world::UPLOAD_RASTER_SLOTS
        .into_iter()
        .enumerate()
    {
        result[i] = upload_raster_staging_to_texture(frame, world, slot)?;
    }
    Ok(result)
}

/// W2, on the actual WaterRaster mutated by W1 in this same frame.
pub fn water_rasterize_from_cpu(
    frame: &mut CoreGraphFrame<'_>, world: &mut World,
) -> rhapsody::Result<Option<u32>> {
    if !world.contains_resource::<world_raster_defs::WaterRaster>() {
        return Err(missing("WaterRaster"));
    }
    let tick = world.read_change_tick().get();
    world.resource_scope(|world, mut raster: bevy_ecs::change_detection::Mut<world_raster_defs::WaterRaster>| {
        with_env(frame, world, |env| Ok(country_core::systems::render_loop::compute_passes::compute_water::water_rasterize_live(env, &mut raster, tick)))
    })
}

/// W12, called at its original point after buildings, using the same GardenRaster.
pub fn garden_rasterize_from_cpu(
    frame: &mut CoreGraphFrame<'_>, world: &mut World,
) -> rhapsody::Result<Option<u32>> {
    if !world.contains_resource::<world_raster_defs::GardenRaster>() {
        return Err(missing("GardenRaster"));
    }
    let tick = world.read_change_tick().get();
    world.resource_scope(|world, mut raster: bevy_ecs::change_detection::Mut<world_raster_defs::GardenRaster>| {
        with_env(frame, world, |env| Ok(country_core::systems::render_loop::compute_passes::compute_garden::garden_rasterize_live(env, &mut raster, tick)))
    })
}

/// Retain the same App's actual allocation, with full handle-generation
/// validation. This is a physical range projection, not an upload or new buffer.
fn retain_nani_cull_read(
    buffers: &crate::render_host::BufferHost,
    handle: &country_core::render::ssbo::rhapsody_boundary::Buffer,
    bytes: u64,
) -> rhapsody::Result<rhapsody::resource::DynamicConstantsChunk> {
    let owner = buffers.managed_buffer(handle).ok_or_else(|| {
        rhapsody::Error::invalid_usage("Nani cull input is stale or belongs to another App")
    })?;
    if bytes == 0 || bytes > owner.size || handle.byte_size != owner.size {
        return Err(rhapsody::Error::invalid_usage("Nani cull input exceeds its actual owner"));
    }
    let read = rhapsody::resource::DynamicConstantsChunk {
        buffer: owner.shared(), offset: 0, size: bytes,
    };
    Ok(read)
}

/// Original W22 / W28 over the actual current GpuWorld and the same frame's
/// SSBO graph cache. The enclosing original main prelude must already have
/// imported owned instance data. This does not run or replace the other scene
/// stages and does not turn the production world entry into a success path.
pub fn generate_nani_draw_lists(
    frame: &mut CoreGraphFrame<'_>,
    world: &mut World,
    hzb: Option<
        country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::NaniHzb,
    >,
) -> rhapsody::Result<
    country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::NaniGraphOutputs,
> {
    use bevy_ecs::change_detection::DetectChangesMut;
    use country_core::render::nani::GpuWorld;
    if !world.contains_resource::<GpuWorld>() {
        return Err(missing("GpuWorld (gpu_generate_draw_lists@0x140987dd0)"));
    }
    let buffers = frame.hosts().buffers;
    world.resource_scope(|world, mut gpu_world: bevy_ecs::change_detection::Mut<GpuWorld>| {
        // Native 140987e80 writes the actual resource row's changed tick before
        // declaring the first RG allocation, even when instance counts are zero.
        gpu_world.set_changed();
        with_env(frame, world, |env| {
            country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::gpu_generate_draw_lists(
                env, &gpu_world, &*buffers, hzb,
                &|handle, bytes| retain_nani_cull_read(&buffers, handle, bytes),
            )
        })
    })
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HalfDepthNormals {
    pub depth: RgTexture,
    pub normals: RgTexture,
}

/// Real postprocessing output plus optional original multires graph buffers.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DofFrameOutput {
    pub color: RgTexture,
    pub tile_buffers: Option<dof::DofTileBuffers>,
}

/// Typed world consumers borrowing one current App and frame. All mandatory
/// resources must already have been produced by the real startup/ECS chain.
pub struct SceneComputeHost<'f, 's, 'w> {
    frame: &'f mut CoreGraphFrame<'s>,
    world: &'w World,
}

impl<'f, 's, 'w> SceneComputeHost<'f, 's, 'w> {
    pub fn new(frame: &'f mut CoreGraphFrame<'s>, world: &'w World) -> Self {
        Self { frame, world }
    }

    pub fn generate_hzb(&mut self, depth: &RgTexture, name: &str) -> rhapsody::Result<Hzb> {
        with_env(self.frame, self.world, |env| {
            Ok(generate_hzb::generate_hzb(env, &depth, &depth.desc, name))
        })
    }

    pub fn extract_half_res_depth_normals(
        &mut self,
        gbuffer: &GBuffer,
    ) -> rhapsody::Result<HalfDepthNormals> {
        with_env(self.frame, self.world, |env| {
            let out = half_res::extract_half_res_depth_normals(env, gbuffer);
            env.record(out.pass);
            Ok(HalfDepthNormals {
                depth: out.depth,
                normals: out.normals,
            })
        })
    }

    pub fn calculate_reprojection_map(
        &mut self,
        depth: &RgTexture,
        velocity: &RgTexture,
        previous: &TemporalTexture,
    ) -> rhapsody::Result<RgTexture> {
        with_env(self.frame, self.world, |env| {
            let out = reprojection::calculate_reprojection_map(
                env,
                depth,
                velocity,
                &PrevDepth {
                    texture: previous.texture,
                    valid: previous.flag,
                },
            );
            env.record(out.map_pass);
            env.record(out.copy_pass);
            Ok(out.map)
        })
    }

    pub fn calculate_ssgi(
        &mut self,
        half: &HalfDepthNormals,
        gbuffer: &GBuffer,
        reprojection: &RgTexture,
    ) -> rhapsody::Result<RgTexture> {
        with_env(self.frame, self.world, |env| {
            let out = ssgi::calculate_ssgi(env, &half.depth, &half.normals, gbuffer, reprojection);
            for pass in out.passes {
                env.record(pass);
            }
            Ok(out.output)
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub fn ray_trace_gi(
        &mut self,
        gbuffer: &GBuffer,
        half: &HalfDepthNormals,
        history: &RgTexture,
        history_valid: u8,
        reprojection: &RgTexture,
        ssgi: &RgTexture,
    ) -> rhapsody::Result<RestirTextures> {
        with_env(self.frame, self.world, |env| {
            let out = ray_trace_gi::ray_trace_gi(
                env,
                gbuffer,
                &half.depth,
                &half.normals,
                history,
                history_valid,
                reprojection,
                ssgi,
            );
            env.record(out.trace);
            env.record(out.update);
            Ok(out.output)
        })
    }

    pub fn eval_gi_screen_radiance(
        &mut self,
        half: &HalfDepthNormals,
        ssgi: &RgTexture,
        restir: &RestirTextures,
        reprojection: &RgTexture,
    ) -> rhapsody::Result<RgTexture> {
        let state = self
            .world
            .get_resource::<ShaderRefactoringState>()
            .ok_or_else(|| missing("ShaderRefactoringState"))?
            .disc;
        with_env(self.frame, self.world, |env| {
            let out = eval_gi_screen_radiance::eval_gi_screen_radiance(
                env,
                &half.depth,
                &half.normals,
                ssgi,
                restir,
                reprojection,
                state,
            );
            env.record(out.eval);
            env.record(out.reproject);
            env.record(out.filter);
            Ok(out.output)
        })
    }

    pub fn bin_lights(&mut self) -> rhapsody::Result<LightBinning> {
        with_env(self.frame, self.world, |env| {
            Ok(bin_lights::bin_lights::<LanternSsbo>(env))
        })
    }

    pub fn restir_sample_direct_lights(
        &mut self,
        half: &HalfDepthNormals,
        reprojection: &RgTexture,
        bins: &LightBinning,
    ) -> rhapsody::Result<RgTexture> {
        let count = self
            .world
            .get_resource::<AssetSsboLibrary>()
            .ok_or_else(|| missing("AssetSsboLibrary"))?
            .get::<LanternSsbo>()
            .ok_or_else(|| missing("LanternSsbo"))?
            .instance_count() as u32;
        let grid = LightGrid {
            light_indirection_grid: bins.grid,
            grid_light_indices: bins.indices,
        };
        with_env(self.frame, self.world, |env| {
            let out = restir_di::restir_sample_direct_lights::<LanternSsbo>(
                env,
                &half.depth,
                &half.normals,
                reprojection,
                &grid,
                count,
            );
            if let Some(pass) = out.pass {
                env.record(pass);
            }
            Ok(out.reservoir)
        })
    }

    pub fn apply_screen_shadows(
        &mut self,
        inputs: &ScreenShadowInputs<'_>,
    ) -> rhapsody::Result<RgTexture> {
        with_env(self.frame, self.world, |env| {
            Ok(apply_screen_shadows::apply_screen_shadows(env, inputs))
        })
    }

    pub fn compute_shadow_mask_mean(&mut self) -> rhapsody::Result<()> {
        with_env(self.frame, self.world, |env| {
            apply_screen_shadows::compute_shadow_mask_mean(env);
            Ok(())
        })
    }

    /// W48 reads the real MainCamera mode, not a new default camera.
    pub fn apply_volumetrics(&mut self, color: &RgTexture, depth: &RgTexture) -> rhapsody::Result<()> {
        let mode = self.world.get_resource::<country_core::systems::camera_rig::main_camera::MainCamera>()
            .ok_or_else(|| missing("MainCamera"))?.mode_byte();
        with_env(self.frame, self.world, |env| {
            let pass = apply_volumetrics::apply_volumetrics(env, mode, color, depth);
            env.record(pass);
            Ok(())
        })
    }

    /// W50 reads the actual source description's mip count. The native stack
    /// value8 at1404eccd0 is the length of `dof_mips`, not a mip-count argument.
    pub fn generate_mips(&mut self, color: &RgTexture, name: &str) -> rhapsody::Result<()> {
        with_env(self.frame, self.world, |env| {
            generate_mips::generate_mips(env, &color.desc, &country_core::render::pass_builder::pass_resource_access::WriteTextureUnchecked(*color), name);
            Ok(())
        })
    }

    /// Whole W51, including the original shared offsets upload and nine-pass
    /// indirect multires branch. Global multires and photomode quality differ.
    pub fn calc_dof(
        &mut self, color: &RgTexture, depth: &RgTexture, prepared: &DofPrepared,
        config: &DofConfig, multires: bool, photomode: bool,
    ) -> rhapsody::Result<DofFrameOutput> {
        with_env(self.frame, self.world, |env| {
            let out = dof::calc_dof::<SpatialResolveOffsetsSsbo>(
                env, multires, color, depth, prepared, None, config, u8::from(photomode));
            for pass in out.passes { env.record(pass); }
            Ok(DofFrameOutput { color: out.output, tile_buffers: out.tile_buffers })
        })
    }

    pub fn dof_prepare(&mut self, depth: &RgTexture) -> rhapsody::Result<DofPrepared> {
        with_env(self.frame, self.world, |env| {
            let (out, pass) = dof::dof_prepare::<DofAutofocusSsbo>(env, depth);
            env.record(pass);
            Ok(out)
        })
    }

    /// Explicit fused-only entry for bounded callers. The complete original
    /// multires branch is available above through calc_dof.
    pub fn calc_dof_fullres(
        &mut self,
        color: &RgTexture,
        depth: &RgTexture,
        prepared: &DofPrepared,
        config: &DofConfig,
    ) -> rhapsody::Result<RgTexture> {
        with_env(self.frame, self.world, |env| {
            let out = dof::calc_dof_fullres(env, color, depth, prepared, config);
            for pass in out.passes {
                env.record(pass);
            }
            Ok(out.output)
        })
    }

    pub fn calculate_taa(&mut self, inputs: &TaaInputs<'_>) -> rhapsody::Result<RgTexture> {
        with_env(self.frame, self.world, |env| {
            let (out, pass) = taa::calculate_taa(env, inputs);
            env.record(pass);
            Ok(out)
        })
    }

    pub fn calculate_tgsr(&mut self, inputs: &TgsrInputs<'_>) -> rhapsody::Result<RgTexture> {
        with_env(self.frame, self.world, |env| {
            let (out, passes) = taa::calculate_tgsr(env, inputs);
            for pass in passes {
                env.record(pass);
            }
            Ok(out)
        })
    }
}


/// Render-thread extraction: what this module reads from the World.
pub fn render_extract(m: &mut impl country_core::render::extract::RenderExtract) {
    m.resource_with_revision::<AssetTextureLibrary>(<AssetTextureLibrary>::descriptor_revision);
    m.resource_incremental::<AssetSsboLibrary>(AssetSsboLibrary::snapshot_revision, AssetSsboLibrary::update_render_snapshot);
    m.resource_incremental::<AssetVAOLibrary>(AssetVAOLibrary::snapshot_revision, AssetVAOLibrary::update_render_snapshot);
    m.resource::<AssetPlacementDataLibrary>();
    m.resource_projected::<world_raster_defs::WaterRaster>(world_raster_defs::WaterRaster::declaration_snapshot);
    m.resource_projected::<world_raster_defs::GardenRaster>(world_raster_defs::GardenRaster::declaration_snapshot);
    m.resource::<country_core::render::nani::gpu_world::GpuWorld>();
    m.resource::<country_core::resources::shader_refactoring_state::ShaderRefactoringState>();
    m.resource_projected::<country_core::systems::camera_rig::main_camera::MainCamera>(country_core::systems::camera_rig::main_camera::MainCamera::render_snapshot);
}

#[cfg(test)]
mod nani_cull_owner_tests {
    use super::retain_nani_cull_read;
    use country_core::render::ssbo::rhapsody_boundary::{BufferDesc, GraphicsDevice};
    use std::sync::Arc;

    #[test]
    fn cull_range_uses_real_generation_checked_owner_and_survives_slot_release() {
        let device = match rhapsody::device::Device::new_headless(
            &rhapsody::device::DeviceConfig::headless(),
        ) {
            Ok(device) => device,
            Err(error) => {
                assert!(!std::env::var("RHAPSODY_REQUIRE_VULKAN").is_ok_and(|v| v == "1"),
                    "strict Nani cull-owner Vulkan initialization: {error}");
                eprintln!("SKIP Nani cull-owner Vulkan initialization: {error}");
                return;
            }
        };
        let buffers = crate::render_host::BufferHost::new(device);
        let handle = buffers.create_buffer(
            &BufferDesc::new(96, 0x20022, 1).alignment(4),
            "bounded Nani cull owner", Some(&[0x7f; 96]),
        ).unwrap();
        for bytes in [0, 97] {
            let error = retain_nani_cull_read(&buffers, &handle, bytes).err().unwrap();
            assert_eq!(error.kind(), rhapsody::ErrorKind::InvalidUsage);
        }
        let read = retain_nani_cull_read(&buffers, &handle, 32).unwrap();
        assert_eq!((read.offset, read.size), (0, 32));
        assert_eq!(read.device_address().unwrap(), buffers.device_address(&handle));
        let owner = buffers.managed_buffer(&handle).unwrap().shared();
        assert!(Arc::ptr_eq(&owner, &read.buffer));
        let alive = Arc::downgrade(&owner);
        drop(owner);
        assert!(buffers.release(&handle));
        let replacement = buffers.create_buffer(
            &BufferDesc::new(96, 0x20022, 1).alignment(4),
            "replacement Nani cull owner", Some(&[0x32; 96]),
        ).unwrap();
        assert_eq!(replacement.id & 0xffff_ffff, handle.id & 0xffff_ffff, "exercise actual slot reuse");
        assert_ne!(replacement.id, handle.id);
        assert_ne!(replacement.generation, handle.generation);
        assert!(buffers.managed_buffer(&handle).is_none());
        let error = retain_nani_cull_read(&buffers, &handle, 32).err().unwrap();
        assert_eq!(error.kind(), rhapsody::ErrorKind::InvalidUsage);
        let replacement_read = retain_nani_cull_read(&buffers, &replacement, 32).unwrap();
        assert!(!Arc::ptr_eq(&read.buffer, &replacement_read.buffer));
        assert!(alive.upgrade().is_some(), "the retained physical owner survives slot release");
        drop(read);
    }
}
