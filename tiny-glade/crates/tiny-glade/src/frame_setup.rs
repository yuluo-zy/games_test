//! Shared setup around the original loading/main/presentation closures.
//!
//! B compile-present @0x140166736 imports presets, @6757 publishes the texture
//! array, @6801 copies the actual 0x1090 FrameConstants, @68f8 allocates its
//! DynamicConstants chunk; @6c00 exports presets. The `handle<Halftimber...>`
//! symbol at @689d is an ICF alias: its entire body is `mov rax,rcx; ret`.
//! The preceding TypeId is DynamicConstantsResource, so it is not a separate
//! Halftimber buffer dependency. Shader heap lowering below is a host adapter;
//! original array indices and actual resource owners are preserved.
use crate::render_bridge::CoreGraphFrame;
use ash::vk;
use bevy_ecs::world::World;
use country_core::{
    asset_libraries::{
        ssbo_library::AssetSsboLibrary,
        texture_library::{AssetTextureLibrary, ImportedTexture, PRESET_COUNT, TextureRenderGraph},
    },
    render::{
        frame_constants::FrameConstants, pass_builder::framebuffer_binding::AttachmentTexture,
    },
    resources::texture_presets::TexturePreset,
    rhapsody_types::{Texture, TextureDesc},
};
use rhapsody::{
    bindless::BindlessParameters, bindless::BindlessResource, bindless::BindlessShaderAbi,
    resource::DynamicConstantsChunk, resource::TextureView,
};
use shader_pipeline::{reflection::BindingKind, reflection::BindingLocation};
use std::sync::Arc;

/// Immutable per-frame owners, captured by actual shader parameter bindings.
pub struct FrameGlobals {
    constants: DynamicConstantsChunk,
    sampled: Vec<(u32, Arc<TextureView>)>,
}
impl FrameGlobals {
    /// Physical owner and range of the FrameConstants block bound through the
    /// storage view at original set 1/b1 (see `write_to`).
    pub fn storage_view(&self) -> (Arc<rhapsody::resource::Buffer>, u64, u64) {
        (
            self.constants.buffer.clone(),
            self.constants.offset,
            self.constants.size,
        )
    }

    pub fn write_to(
        &self,
        parameters: &mut BindlessParameters,
        abi: &BindlessShaderAbi,
    ) -> rhapsody::Result<()> {
        // Original global set 1/b0 is the FrameConstants block. The lowered
        // shader may use a uniform or storage representation of the same bytes.
        let frame = BindingLocation { set: 1, binding: 0 };
        if let Some(slot) = abi.slot(frame) {
            if !matches!(
                slot.kind,
                BindingKind::UniformBuffer | BindingKind::StorageBuffer
            ) {
                return Err(rhapsody::Error::invalid_usage(
                    "original frame constants binding has the wrong kind",
                ));
            }
            parameters.write(
                frame,
                0,
                BindlessResource::Buffer {
                    buffer: self.constants.buffer.clone(),
                    offset: self.constants.offset,
                    size: self.constants.size,
                },
            )?;
        }
        // Original global set 1/b1: the wall/roof chain reads the very same FrameConstants
        // bytes through a storage view (`frame_constants_storage_buffer`,
        // `StructuredBuffer`/`ByteAddressBuffer` register(t1, space1) in
        // wall/generate_draw_lists, nani_instanced_roof, roof_tiles_gravity).
        let frame_storage = BindingLocation { set: 1, binding: 1 };
        if let Some(slot) = abi.slot(frame_storage) {
            if slot.kind != BindingKind::StorageBuffer {
                return Err(rhapsody::Error::invalid_usage(
                    "original frame constants storage binding has the wrong kind",
                ));
            }
            parameters.write(
                frame_storage,
                0,
                BindlessResource::Buffer {
                    buffer: self.constants.buffer.clone(),
                    offset: self.constants.offset,
                    size: self.constants.size,
                },
            )?;
        }
        let images = BindingLocation { set: 2, binding: 0 };
        if let Some(slot) = abi.slot(images) {
            if slot.kind != BindingKind::SampledImage {
                return Err(rhapsody::Error::invalid_usage(
                    "original texture array binding has the wrong kind",
                ));
            }
            for (index, view) in &self.sampled {
                if *index >= slot.parameter_count {
                    return Err(rhapsody::Error::unsupported(
                        "original texture array exceeds this shader host's admitted runtime array capacity",
                    ));
                }
                parameters.write(
                    images,
                    *index,
                    BindlessResource::SampledImage {
                        view: view.clone(),
                        layout: vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
                    },
                )?;
            }
        }
        Ok(())
    }
}

/// Safe field serialization of the exact reflected UBO, including all five
/// views. No native padding or uninitialized memory is read.
fn frame_bytes(c: &FrameConstants) -> Vec<u8> {
    let mut words = Vec::<u32>::with_capacity(0x1090 / 4);
    for value in [
        c.sun_color,
        c.sky_low_color,
        c.sky_high_color,
        c.fog_tint_thickness,
        c.fake_light_color,
        c.fake_light_dir,
        c.fake_light2_color,
        c.fake_light2_dir,
    ] {
        words.extend(value.map(f32::to_bits));
    }
    words.extend([
        c.frame_counter,
        c.delta_time_seconds.to_bits(),
        c.ev_mult.to_bits(),
        c.glade_theme_id,
        c.inner_rendering_resolution[0],
        c.inner_rendering_resolution[1],
        c.game_time_sec,
        c.game_time_frac.to_bits(),
    ]);
    words.extend(c.editing_highlight_color.map(f32::to_bits));
    words.push(c.time_of_day.to_bits());
    words.extend(c.dir_to_sun.map(f32::to_bits));
    words.push(c.camera_idx);
    words.extend(c.dir_to_moon.map(f32::to_bits));
    words.push(c.render_overrides);
    words.extend([
        c.global_mip_bias.to_bits(),
        c.highlighted_transient_stair_edge_id,
        c.pad1,
        c.pad2,
    ]);
    words.extend(c.debug_tweakers.map(f32::to_bits));
    for v in &c.views {
        for m in [
            v.clip_from_view,
            v.view_from_clip,
            v.clip_from_view_jittered,
            v.view_from_clip_jittered,
            v.view_from_world,
            v.world_from_view,
            v.clip_from_world_jittered,
            v.prev_clip_from_clip,
            v.prev_clip_from_view,
            v.prev_view_from_clip,
            v.prev_view_from_world,
            v.prev_world_from_view,
        ] {
            words.extend(m.to_cols_array().map(f32::to_bits));
        }
        for v in [
            v.pixel_offset,
            v.jitter_ndc,
            v.prev_pixel_offset,
            v.prev_jitter_ndc,
        ] {
            words.extend(v.to_array().map(f32::to_bits));
        }
    }
    assert_eq!(
        words.len() * 4,
        0x1090,
        "native FrameConstants serialization"
    );
    words.into_iter().flat_map(u32::to_le_bytes).collect()
}

struct Textures<'a, 's>(&'a mut CoreGraphFrame<'s>);
impl TextureRenderGraph for Textures<'_, '_> {
    fn import_texture(
        &mut self,
        texture: &Texture,
        desc: TextureDesc,
        access: u8,
    ) -> ImportedTexture {
        let id = self
            .0
            .import_texture(texture, access)
            .expect("original preset import failed");
        ImportedTexture {
            resource: u64::from(id.raw()),
            generation: 0,
            access,
            desc,
        }
    }
    fn export_texture(&mut self, texture: ImportedTexture, access: u8) {
        self.0
            .with_graph(|graph| {
                let id = graph.resource_handle(texture.resource as u32)?;
                graph.export(id, access)?;
                Ok(())
            })
            .expect("original preset export failed");
    }
}

fn begin(frame: &mut CoreGraphFrame<'_>, world: &mut World) -> rhapsody::Result<()> {
    let fail =
        |name| rhapsody::Error::invalid_usage(format!("original frame setup: missing {name}"));
    let bytes = frame_bytes(
        world
            .get_resource::<FrameConstants>()
            .ok_or_else(|| fail("FrameConstants"))?,
    );
    let raster = frame.raster_resources();
    let constants = raster
        .dynamic_constants()
        .lock()
        .allocate_with_data(&bytes)?;
    let hosts = frame.hosts();
    let mut library = world
        .get_resource_mut::<AssetTextureLibrary>()
        .ok_or_else(|| fail("AssetTextureLibrary"))?;
    // Publish real pending image requests before cloning the original descriptor
    // table. DescriptorSet snapshots own their actual TextureView resources.
    let sampled = library
        .try_bindless_textures_dset(hosts.textures.as_ref())?
        .sampled_views()?;
    library.import_texture_presets_to_rg(&mut Textures(frame));
    {
        let mut tables = raster.assets.lock();
        tables.presets.clear();
        tables.textures_by_name.clear();
        tables.rg_buffers.clear();
        tables.ssbo_leases = Some(std::collections::HashMap::new());
        tables.ssbo_instance_counts.clear();
        for preset in 0..PRESET_COUNT {
            let t = library.get_rg_texture(TexturePreset(preset));
            tables.presets.insert(
                preset,
                AttachmentTexture {
                    id: t.resource as u32,
                    extent: (t.desc.extent[0], t.desc.extent[1]),
                    format: t.desc.format,
                    sample_count: 1,
                },
            );
        }
        for (name, t) in library.named_textures() {
            tables.textures_by_name.insert(
                name.0,
                (
                    country_core::rhapsody_contract::ManagedHandle(t.id),
                    AttachmentTexture::from(t),
                ),
            );
        }
    }
    drop(library);
    if let Some(ssbos) = world.get_resource::<AssetSsboLibrary>() {
        use country_core::asset_libraries::ssbo_library::SsboId;
        use country_core::resources::rt::{RtBvhSsbo, RtTrianglesSsbo};
        // Original draw bind reads the same typed InstanceBuffer +00 count
        // as compute bind. Publishing counts neither imports GPU resources
        // nor manufactures absent entries; actual draw accesses remain lazy.
        let mut tables = raster.assets.lock();
        tables.ssbo_leases = Some(
            ssbos
                .snapshot_graph_leases()
                .into_iter()
                .map(|lease| (lease.id.0.0, lease))
                .collect(),
        );
        if let Some(storage) = ssbos.get::<RtTrianglesSsbo>() {
            tables.ssbo_instance_counts.insert(
                SsboId::of::<RtTrianglesSsbo>().0.0,
                storage.instance_count as u32,
            );
        }
        if let Some(storage) = ssbos.get::<RtBvhSsbo>() {
            tables
                .ssbo_instance_counts
                .insert(SsboId::of::<RtBvhSsbo>().0.0, storage.instance_count as u32);
        }
    }
    // The placement draw consumer indexes the exact gameplay library handle
    // and forwards its stored instance count. Publish this App's actual buffer
    // owner at every frame, rather than leaving RasterAssets::placement empty.
    {
        let mut tables = raster.assets.lock();
        tables.placement.clear();
        if let Some(placements) = world.get_resource::<
            country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary,
        >() {
            for (handle, (storage, count)) in &placements.buffers {
                hosts.buffers.managed_buffer(&storage.buffer)
                    .ok_or_else(|| fail("placement buffer does not belong to this App's BufferHost"))?;
                let count = u32::try_from(*count)
                    .map_err(|_| fail("placement instance count exceeds native u32"))?;
                tables.placement.insert(handle.id().raw(),
                    (country_core::rhapsody_contract::ManagedHandle(storage.buffer.id), count));
            }
        }
    }
    frame
        .shader_backend()
        .set_frame_globals(Arc::new(FrameGlobals { constants, sampled }));
    Ok(())
}

/// Always clear the library's graph-local preset cache before compilation or
/// retry. The original target export remains the consumer's responsibility.
pub fn with_frame_resources<T>(
    frame: &mut CoreGraphFrame<'_>,
    world: &mut World,
    build: impl FnOnce(&mut CoreGraphFrame<'_>, &mut World) -> rhapsody::Result<T>,
) -> rhapsody::Result<T> {
    begin(frame, world)?;
    let result = build(frame, world);
    world
        .resource_mut::<AssetTextureLibrary>()
        .export_preset_textures_from_rg(&mut Textures(frame));
    // Original SSBO entries import lazily in shader-binding consumers. Export
    // only those entries, then clear their graph-local IDs before the next
    // loading/main/presentation graph reuses the App's persistent library.
    if let Some(ssbos) = world.get_resource::<AssetSsboLibrary>() {
        ssbos.export_from_rg(&mut frame.resource_graph());
    }
    result
}
