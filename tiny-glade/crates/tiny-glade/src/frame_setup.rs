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
    /// 每页缓存保持实际 heap owner；没有 heap→FrameGlobals 的反向引用。
    page_leases: parking_lot::Mutex<Vec<Arc<FrameGlobalsPageSlot>>>,
}
struct FrameGlobalsPageSlot {
    heap: Arc<rhapsody::bindless::BindlessHeap>,
    cache: parking_lot::Mutex<Option<Arc<FrameGlobalsPageCache>>>,
}
struct FrameGlobalsPageCache {
    storage: Option<Arc<rhapsody::bindless::BindlessLease>>,
    sampled: Option<Arc<Vec<(u32, Arc<rhapsody::bindless::BindlessLease>)>>>,
}
pub(crate) struct FrameGlobalsPreparation {
    slot: Arc<FrameGlobalsPageSlot>,
    cached: Option<Arc<FrameGlobalsPageCache>>,
    storage: bool,
    sampled: bool,
    sampled_count: usize,
}
pub(crate) struct FrameGlobalsPublication {
    slot: Arc<FrameGlobalsPageSlot>,
    cache: Arc<FrameGlobalsPageCache>,
}
impl FrameGlobalsPublication {
    /// 参数组已完整生成；缓存槽事先存在，发布阶段不能再分配或失败。
    pub fn commit(self) {
        let old = { self.slot.cache.lock().replace(self.cache) };
        drop(old);
    }
}
impl FrameGlobalsPreparation {
    pub fn requirements(&self) -> [usize; 4] {
        let storage_missing = self.storage
            && self
                .cached
                .as_ref()
                .is_none_or(|cache| cache.storage.is_none());
        let sampled_missing = self.sampled
            && self
                .cached
                .as_ref()
                .is_none_or(|cache| cache.sampled.is_none());
        [
            if sampled_missing {
                self.sampled_count
            } else {
                0
            },
            0,
            usize::from(storage_missing),
            0,
        ]
    }
    pub fn write_to(
        &self,
        globals: &FrameGlobals,
        parameters: &mut BindlessParameters,
    ) -> rhapsody::Result<FrameGlobalsPublication> {
        let mut storage = self.cached.as_ref().and_then(|cache| cache.storage.clone());
        let mut sampled = self.cached.as_ref().and_then(|cache| cache.sampled.clone());
        let mut changed = self.cached.is_none();
        if self.storage && storage.is_none() {
            // b0/b1 始终使用已声明的同一存储角色，不能改成 private upload。
            storage = Some(self.slot.heap.storage_buffer(
                globals.constants.buffer.clone(),
                globals.constants.offset,
                globals.constants.size,
            )?);
            changed = true;
        }
        if self.sampled && sampled.is_none() {
            let mut leases = Vec::new();
            leases
                .try_reserve(globals.sampled.len())
                .map_err(|_| rhapsody::Error::OutOfHostMemory)?;
            for (index, view) in &globals.sampled {
                leases.push((
                    *index,
                    self.slot
                        .heap
                        .sampled_image(view.clone(), vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)?,
                ));
            }
            sampled = Some(Arc::new(leases));
            changed = true;
        }
        for binding in [0, 1] {
            let location = BindingLocation { set: 1, binding };
            if parameters.abi().slot(location).is_some() {
                parameters.write_lease(
                    location,
                    0,
                    storage
                        .as_ref()
                        .expect("validated frame storage role")
                        .clone(),
                )?;
            }
        }
        if self.sampled {
            for (index, lease) in sampled.as_ref().expect("prepared sampled table").iter() {
                parameters.write_lease(
                    BindingLocation { set: 2, binding: 0 },
                    *index,
                    lease.clone(),
                )?;
            }
        }
        let cache = if changed {
            Arc::new(FrameGlobalsPageCache { storage, sampled })
        } else {
            self.cached
                .as_ref()
                .expect("unchanged complete cache")
                .clone()
        };
        Ok(FrameGlobalsPublication {
            slot: self.slot.clone(),
            cache,
        })
    }
}
impl FrameGlobals {
    pub fn storage_view(&self) -> (Arc<rhapsody::resource::Buffer>, u64, u64) {
        (
            self.constants.buffer.clone(),
            self.constants.offset,
            self.constants.size,
        )
    }
    pub fn sampled_table(&self) -> &[(u32, Arc<TextureView>)] {
        &self.sampled
    }
    /// Intended long-lived descriptor owners, sampled only after GPU drain.
    pub(crate) fn retained_bindless_counts(&self) -> [usize; 4] {
        let mut counts = [0; 4];
        for page in self.page_leases.lock().iter() {
            if let Some(cache) = page.cache.lock().as_ref() {
                counts[0] += cache.sampled.as_ref().map_or(0, |sampled| sampled.len());
                counts[2] += usize::from(cache.storage.is_some());
            }
        }
        counts
    }
    /// 只预备页身份和元数据，不分配任何描述符；实际 writer 在组 guard 内调用。
    pub(crate) fn prepare_page(
        &self,
        abi: &BindlessShaderAbi,
        heap: &Arc<rhapsody::bindless::BindlessHeap>,
    ) -> rhapsody::Result<FrameGlobalsPreparation> {
        let mut storage = false;
        for binding in [0, 1] {
            if let Some(slot) = abi.slot(BindingLocation { set: 1, binding }) {
                let valid = if binding == 0 {
                    matches!(
                        slot.kind,
                        BindingKind::UniformBuffer | BindingKind::StorageBuffer
                    )
                } else {
                    slot.kind == BindingKind::StorageBuffer
                };
                if !valid {
                    return Err(rhapsody::Error::invalid_usage(
                        "original frame storage binding has the wrong kind",
                    ));
                }
                storage = true;
            }
        }
        let sampled = if let Some(slot) = abi.slot(BindingLocation { set: 2, binding: 0 }) {
            if slot.kind != BindingKind::SampledImage {
                return Err(rhapsody::Error::invalid_usage(
                    "original texture array binding has the wrong kind",
                ));
            }
            if self
                .sampled
                .iter()
                .any(|(index, _)| *index >= slot.parameter_count)
            {
                return Err(rhapsody::Error::unsupported(
                    "original texture array exceeds this shader host's admitted runtime array capacity",
                ));
            }
            true
        } else {
            false
        };
        let slot = {
            let mut pages = self.page_leases.lock();
            if let Some(slot) = pages.iter().find(|slot| Arc::ptr_eq(&slot.heap, heap)) {
                slot.clone()
            } else {
                pages
                    .try_reserve(1)
                    .map_err(|_| rhapsody::Error::OutOfHostMemory)?;
                let slot = Arc::new(FrameGlobalsPageSlot {
                    heap: heap.clone(),
                    cache: parking_lot::Mutex::new(None),
                });
                pages.push(slot.clone());
                slot
            }
        };
        let cached = slot.cache.lock().clone();
        Ok(FrameGlobalsPreparation {
            slot,
            cached,
            storage,
            sampled,
            sampled_count: self.sampled.len(),
        })
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
    let constants = {
        let mut constants = raster.dynamic_constants().lock();
        constants.begin_frame();
        constants.allocate_with_data(&bytes)?
    };
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
        let tables = Arc::make_mut(&mut tables.collect);
        tables.presets.clear();
        tables.textures_by_name.clear();
        tables.rg_buffers.clear();
        tables.ssbo_leases = Some(Default::default());
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
        let tables = Arc::make_mut(&mut tables.collect);
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
        let tables = Arc::make_mut(&mut tables.collect);
        tables.placement.clear();
        if let Some(placements) = world.get_resource::<
            country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary,
        >() {
            for (handle, (storage, count)) in placements.buffers() {
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
        .set_frame_globals(Arc::new(FrameGlobals {
            constants,
            sampled,
            page_leases: parking_lot::Mutex::new(Vec::new()),
        }));
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


/// Render-thread extraction: what this module reads from the World.
pub fn render_extract(m: &mut impl country_core::render::extract::RenderExtract) {
    m.resource::<FrameConstants>();
    m.asset_library::<AssetTextureLibrary>();
    m.asset_library::<AssetSsboLibrary>();
    m.asset_library::<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>();
}
