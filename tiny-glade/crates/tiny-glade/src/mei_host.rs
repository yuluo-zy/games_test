//! B toolbar text flush → shared MEI render-graph resources.
//!
//! Native facts and known adapter deviations are recorded in
//! TG-c80526daecad7e4117/meihost-20261018/CONTRACT.md. This adapter consumes a
//! real TextRenderer stream; it never manufactures a primitive count or atlas.
use std::{path::Path, sync::Arc};

use anyhow::Context;
use ash::vk;
use bevy_ecs::prelude::*;
use country_core::{
    asset_libraries::ssbo_library::AssetSsboLibrary,
    render::{
        load_texture::boundary::TextureDevice,
        pass_builder::{framebuffer_binding::AttachmentTexture, pass_resource_access::SsboType},
        ssbo::rhapsody_boundary::{Buffer, BufferDesc, GraphicsDevice},
    },
    resources::window::WindowSize,
    rhapsody_contract::ManagedHandle,
    rhapsody_types::{Texture, TextureDesc},
    systems::render_loop::draw_passes::ui_2d_pass::{MeiTextOutput, ShapeStreamSsbo, TextPrimSsbo},
};
use rhapsody::{
    command::record_texture_regions, device::Device, resource::Buffer as NativeBuffer,
    resource::BufferDesc as NativeBufferDesc, resource::ManagedBuffer,
    resource::with_setup_command,
};
use tg_text::{AtlasDescriptor, RenderBatch, TextFragment, TextRenderer};

use crate::{
    raster_host::RasterShared,
    render_bridge::CoreGraphFrame,
    render_host::{BufferHost, TextureHost},
};

pub const TEXT_PRIMITIVE_BYTES: usize = 0x600000;
pub const SHAPE_STREAM_BYTES: usize = 0x400000;

/// Shared gameplay/loading registration from B MeiPlugin (set6/7/44).
/// Text preparation precedes Taffy measurement; drawing follows layout; the
/// stream is consumed once, after both component and MEI node producers. Graph
/// compilation and its retries only read the uploaded output of this update.
/// Original MeiPlugin load system (see `install_mei_frame_systems`). Failure to
/// read the shipped table is fatal, as in the original `load_font_table` unwrap path.
pub fn load_mei_assets(mut mei: ResMut<toolbar::MeiContext>) -> bevy_ecs::error::Result {
    mei.load_configs(Path::new("assets/ui"));
    mei.load_font_table(Path::new("assets/fonts/table.ron"))?;
    Ok(())
}

pub fn install_mei_frame_systems(app: &mut bevy_app::App) {
    use bevy_ecs::schedule::IntoScheduleConfigs;
    use crate::run_game::GameSystemSet;

    app.init_resource::<toolbar::MeiContext>();
    // MeiPlugin::build @1406868d0 registers one load system right after
    // insert_resource(MeiContext) (system body @140fb5b90 = InfallibleSystemWrapper
    // run_unsafe): `load_configs()` then `load_font_table()`. Both paths are
    // compile-time constants inside the originals (@140682e70 "assets/ui", len 9;
    // @140683a20 "assets/fonts/table.ron", len 0x16), relative to the install root.
    // OPEN: the schedule label of that registration (Schedules::add_systems mono
    // @140fd50c0) is baked into the monomorph and not yet identified; PreStartup and
    // Startup are equivalent here because both finish before the first Update.
    app.add_systems(bevy_app::Startup, load_mei_assets);
    app.init_resource::<toolbar::resouces_and_components::taffy_layout::TaffyLayout>();
    app.init_resource::<toolbar::toolbar_update::ToolbarSizeMult>();
    app.init_resource::<country_core::systems::ui::resources::widget_params::UiNavState>();
    app.add_event::<country_core::systems::audio::PlayMenuUiSound>();
    app.add_systems(bevy_app::Update,
        toolbar::mei::systems::mei_pre_update
            .in_set(GameSystemSet(6))
            .after(country_core::systems::ui::ui_update::ui_update));
    app.add_systems(bevy_app::Update,
        update_text_renderer_scale.in_set(GameSystemSet(7)));
    app.add_systems(bevy_app::Update, (
        prepare_text_fragments,
        toolbar::taffy_update::taffy_finalize_element_size,
        toolbar::taffy_update::taffy_perform_layout,
        toolbar::taffy_update::taffy_update_drawables,
        toolbar::ui_interaction::debug_hide_ui,
        render_text_fragments,
    ).chain_ignore_deferred().in_set(GameSystemSet(44)));
    app.add_systems(bevy_app::Update, (
        toolbar::mei_post_update,
        crate::loading_graph::flush_mei_frame,
    ).chain().in_set(GameSystemSet(44)).after(render_text_fragments));
}


/// Render-thread extraction: what this module reads from the World.
pub fn render_extract(m: &mut impl country_core::render::extract::RenderExtract) {
    m.resource::<toolbar::toolbar_update::ToolbarSizeMult>();
    m.resource::<country_core::systems::ui::resources::widget_params::UiNavState>();
}

#[cfg(test)]
mod registration_tests {
    #[test]
    fn shared_mei_schedule_initializes_with_native_stages() {
        use bevy_ecs::schedule::IntoScheduleConfigs;
        let mut app = bevy_app::App::new();
        for stage in 0..44 {
            app.configure_sets(bevy_app::Update,
                crate::run_game::GameSystemSet(stage)
                    .before(crate::run_game::GameSystemSet(stage + 1)));
        }
        super::install_mei_frame_systems(&mut app);
        app.world_mut().schedule_scope(bevy_app::Update, |world, schedule| {
            schedule.initialize(world).expect("shared MEI stage order");
        });
    }
}

/// Original toolbar Startup producer. WalkDir error entries are skipped by the
/// original filter; a selected ttf/otf file failing to load is fatal.
pub fn load_text_fonts(renderer: &mut TextRenderer, installed: &Path) -> anyhow::Result<()> {
    for entry in walkdir::WalkDir::new(installed.join("assets/fonts"))
        .into_iter()
        .filter_map(Result::ok)
    {
        if matches!(
            entry.path().extension().and_then(|s| s.to_str()),
            Some("ttf" | "otf")
        ) {
            log::trace!("Loading font file {}", entry.path().display());
            renderer
                .load_font_from_file(entry.path())
                .with_context(|| format!("Failed to load font {}", entry.path().display()))?;
        }
    }
    Ok(())
}

/// B 0x141e9d960: logical UI height is exactly 1000, independent of OS DPI.
pub fn update_text_renderer_scale(mut renderer: ResMut<TextRenderer>, window: Res<WindowSize>) {
    renderer
        .set_pixels_per_point(window.height as f32 / 1000.0)
        .expect("update text renderer scale");
}

/// B 0x141e9dd20: prepare each genuine TextFragment component.
pub fn prepare_text_fragments(
    mut renderer: ResMut<TextRenderer>,
    mut fragments: Query<&mut TextFragment>,
) {
    for mut fragment in &mut fragments {
        fragment
            .prepare(&mut renderer)
            .expect("prepare text fragment");
    }
}

/// B 0x141e9df60: Ui2dExtent supplies packed center/extent; native R9D is -1.
pub fn render_text_fragments(
    mut renderer: ResMut<TextRenderer>,
    mut fragments: Query<(&mut TextFragment, &country_core::components::Ui2dExtent)>,
) {
    for (mut fragment, extent) in &mut fragments {
        fragment
            .render(
                &mut renderer,
                extent.pos.to_array(),
                extent.size.to_array(),
                u32::MAX,
            )
            .expect("render text fragment");
    }
}

/// Exact native fixed-capacity TextPrim/ShapeStream buffers and glyph atlas on
/// the same device/host namespaces as the main graph. The owner must outlive all
/// frames that use it; rhapsody's managed handles retain actual GPU ownership.
enum MeiBufferOwnership {
    Private,
    // Real ManagedBuffer leases retain both physical allocations; a library
    // consumer must not invalidate their shared BufferHost slot on drop.
    Library {
        _primitives: ManagedBuffer,
        _shapes: ManagedBuffer,
    },
    /// A render-thread snapshot: it names the owner's buffers and atlas but
    /// releases nothing.
    Borrowed,
}

pub struct MeiHost {
    device: Arc<Device>,
    buffers: Arc<BufferHost>,
    textures: Arc<TextureHost>,
    primitives: Buffer,
    shapes: Buffer,
    buffer_ownership: MeiBufferOwnership,
    atlas: Option<(AtlasDescriptor, Texture)>,
    last_output: Option<MeiTextOutput>,
}

impl MeiHost {
    pub fn new(
        device: Arc<Device>,
        buffers: Arc<BufferHost>,
        textures: Arc<TextureHost>,
    ) -> rhapsody::Result<Self> {
        let primitive_desc = BufferDesc::new(TEXT_PRIMITIVE_BYTES, 0x22, 1).alignment(4);
        let primitives = buffers.create_buffer(&primitive_desc, "TextPrim", None)?;
        let shape_desc = BufferDesc::new(SHAPE_STREAM_BYTES, 0x22, 1).alignment(4);
        let shapes = match buffers.create_buffer(&shape_desc, "ShapeStream", None) {
            Ok(buffer) => buffer,
            Err(error) => {
                buffers.release(&primitives);
                return Err(error);
            }
        };
        Ok(Self {
            device,
            buffers,
            textures,
            primitives,
            shapes,
            buffer_ownership: MeiBufferOwnership::Private,
            atlas: None,
            last_output: None,
        })
    }

    /// Original flush_text_render@141e9e1f0 reads the actual TextPrim and
    /// ShapeStream entries from this App's AssetSsboLibrary. Use this entry
    /// after its real minimal/full init_populate producer, never allocate replacement buffers
    /// when the mandatory library owner is absent.
    pub fn from_library(
        device: Arc<Device>,
        buffers: Arc<BufferHost>,
        textures: Arc<TextureHost>,
        library: &AssetSsboLibrary,
    ) -> rhapsody::Result<Self> {
        if !Arc::ptr_eq(&device, buffers.graphics_device()) {
            return Err(rhapsody::Error::invalid_usage(
                "MEI library BufferHost belongs to another device",
            ));
        }
        let primitives = library
            .get::<TextPrimSsbo>()
            .ok_or_else(|| {
                rhapsody::Error::invalid_usage(
                    "original text flush: missing canonical TextPrimSsbo producer",
                )
            })?
            .buffer;
        let shapes = library
            .get::<ShapeStreamSsbo>()
            .ok_or_else(|| {
                rhapsody::Error::invalid_usage(
                    "original text flush: missing canonical ShapeStreamSsbo producer",
                )
            })?
            .buffer;
        let primitive_lease = buffers.managed_buffer(&primitives).ok_or_else(|| {
            rhapsody::Error::invalid_usage("TextPrimSsbo has no live owner in this BufferHost")
        })?;
        let shape_lease = buffers.managed_buffer(&shapes).ok_or_else(|| {
            rhapsody::Error::invalid_usage("ShapeStreamSsbo has no live owner in this BufferHost")
        })?;
        for (lease, handle, expected) in [
            (&primitive_lease, &primitives, TEXT_PRIMITIVE_BYTES),
            (&shape_lease, &shapes, SHAPE_STREAM_BYTES),
        ] {
            if !Arc::ptr_eq(&lease.device(), &device)
                || lease.size != expected as u64
                || handle.byte_size != expected as u64
                || !lease.usage.contains(
                    vk::BufferUsageFlags::STORAGE_BUFFER | vk::BufferUsageFlags::TRANSFER_DST,
                )
            {
                return Err(rhapsody::Error::invalid_usage(
                    "canonical MEI buffer descriptor/device mismatch",
                ));
            }
        }
        Ok(Self {
            device,
            buffers,
            textures,
            primitives,
            shapes,
            buffer_ownership: MeiBufferOwnership::Library {
                _primitives: primitive_lease,
                _shapes: shape_lease,
            },
            atlas: None,
            last_output: None,
        })
    }

    /// Original flush order: update primitive prefix, update shape prefix,
    /// upload pending glyphs; render then reports this exact consumed count.
    /// The CPU stream belongs to the supplied TextRenderer world resource.
    pub fn prepare(
        &mut self,
        renderer: &mut TextRenderer,
        window: [u32; 2],
        resolver_image_count: usize,
    ) -> anyhow::Result<MeiTextOutput> {
        let atlas = renderer.atlas();
        let batch = renderer.consume()?;
        self.upload_batch(&batch, atlas, window, resolver_image_count)?;
        let (actual_atlas, primitive_count) = renderer.render();
        anyhow::ensure!(
            *actual_atlas == atlas && primitive_count == batch.primitives.len() as u64,
            "text render output differs from the just-consumed stream"
        );
        let texture = self
            .atlas
            .as_ref()
            .context("glyph atlas was not uploaded")?
            .1
            .clone();
        let output = MeiTextOutput {
            prim_count: u32::try_from(primitive_count)?,
            atlas: ManagedHandle(texture.id),
            atlas_desc: AttachmentTexture {
                id: texture.id as u32,
                extent: (texture.width, texture.height),
                format: texture.format,
                sample_count: 1,
            },
        };
        self.last_output = Some(output.clone());
        Ok(output)
    }

    fn upload_batch(
        &mut self,
        batch: &RenderBatch,
        atlas: AtlasDescriptor,
        window: [u32; 2],
        resolver_image_count: usize,
    ) -> anyhow::Result<()> {
        let primitive_bytes = batch.primitive_bytes();
        let shape_bytes: &[u8] = bytemuck::cast_slice(&batch.shape_words);
        anyhow::ensure!(
            primitive_bytes.len() <= TEXT_PRIMITIVE_BYTES
                && shape_bytes.len() <= SHAPE_STREAM_BYTES,
            "text stream exceeds original fixed SSBO capacity"
        );
        tg_text::validation::validate_batch(
            window[0],
            window[1],
            batch,
            atlas,
            &batch.glyph_uploads,
            resolver_image_count,
            &tg_text::validation::RenderLimits::default(),
        )?;
        let fresh_atlas = self
            .atlas
            .as_ref()
            .is_none_or(|(descriptor, _)| *descriptor != atlas);
        if fresh_atlas {
            // A new host cannot recover old glyph pixels from CPU allocation IDs.
            // Every sampled glyph must have a true upload in this generation.
            for primitive in &batch.primitives {
                if primitive.stream == 0 {
                    let [x, y] = primitive.uv_min;
                    let [width, height] = primitive.uv_extent;
                    anyhow::ensure!(
                        batch.glyph_uploads.iter().any(|upload| x >= upload.x as f32
                            && y >= upload.y as f32
                            && x + width <= (upload.x + upload.width) as f32
                            && y + height <= (upload.y + upload.height) as f32),
                        "fresh text atlas is missing pixels for an existing glyph; rebuild the source text frame"
                    );
                }
            }
        }
        if !primitive_bytes.is_empty() {
            self.buffers.update_buffer_bytes(
                &self.primitives,
                primitive_bytes,
                &[0..primitive_bytes.len()],
                0,
            )?;
        }
        if !shape_bytes.is_empty() {
            self.buffers.update_buffer_bytes(
                &self.shapes,
                shape_bytes,
                &[0..shape_bytes.len()],
                0,
            )?;
        }
        if fresh_atlas {
            // Native TextRenderer::new requests zero-init variant 1 for format
            // 0x2b. The real TextureHost forwards an empty sampled initializer
            // to Device::create_texture_zeroed; glyph pixels follow below.
            let texture = self.textures.create_texture(
                "Glyph atlas",
                &TextureDesc::new_2d([atlas.width, atlas.height], 0x2b).sampled(),
                &[],
            )?;
            if let Some((_, old)) = self.atlas.replace((atlas, texture)) {
                self.textures.release(&old);
            }
        }
        if !batch.glyph_uploads.is_empty() {
            let texture = self
                .textures
                .get(&self.atlas.as_ref().expect("created atlas").1)
                .context("glyph atlas host handle is stale")?
                .shared();
            let mut bytes = Vec::new();
            let mut regions = Vec::with_capacity(batch.glyph_uploads.len());
            for upload in &batch.glyph_uploads {
                regions.push(
                    vk::BufferImageCopy::default()
                        .buffer_offset(bytes.len() as u64)
                        .image_subresource(
                            vk::ImageSubresourceLayers::default()
                                .aspect_mask(vk::ImageAspectFlags::COLOR)
                                .layer_count(1),
                        )
                        .image_offset(vk::Offset3D {
                            x: upload.x.try_into()?,
                            y: upload.y.try_into()?,
                            z: 0,
                        })
                        .image_extent(vk::Extent3D {
                            width: upload.width,
                            height: upload.height,
                            depth: 1,
                        }),
                );
                bytes.extend_from_slice(&upload.rgba);
            }
            let staging = Arc::new(NativeBuffer::new(
                self.device.clone(),
                &NativeBufferDesc::new_cpu_to_gpu(
                    bytes.len() as u64,
                    vk::BufferUsageFlags::TRANSFER_SRC,
                ),
            )?);
            staging.write_bytes_unsynchronized(0, &bytes)?;
            let queue = self.device.queue(rhapsody::device::QueueClass::Graphics)?;
            with_setup_command(&self.device, &[], |command| unsafe {
                record_texture_regions(
                    command,
                    queue,
                    texture,
                    staging,
                    &regions,
                    vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
                )
            })?
            .into_value();
        }
        Ok(())
    }

    /// Register real graph resources under the original game SSBO hashes. Must
    /// run for each new frame, because logical graph resource IDs are per frame.
    pub fn import_into_frame(
        &self,
        frame: &mut CoreGraphFrame<'_>,
        raster: &RasterShared,
    ) -> rhapsody::Result<()> {
        let primitives = frame.import_buffer(&self.primitives, 0x37)?;
        let shapes = frame.import_buffer(&self.shapes, 0x37)?;
        let mut assets = raster.assets.lock();
        assets
            .rg_buffers
            .insert(TextPrimSsbo::rg_buffer_hash(), primitives.raw());
        assets
            .rg_buffers
            .insert(ShapeStreamSsbo::rg_buffer_hash(), shapes.raw());
        Ok(())
    }

    /// No default count: absence means no producer flushed this renderer yet.
    pub fn output(&self) -> Option<&MeiTextOutput> {
        self.last_output.as_ref()
    }
}

impl Drop for MeiHost {
    fn drop(&mut self) {
        if matches!(&self.buffer_ownership, MeiBufferOwnership::Borrowed) {
            return;
        }
        if matches!(&self.buffer_ownership, MeiBufferOwnership::Private) {
            self.buffers.release(&self.primitives);
            self.buffers.release(&self.shapes);
        }
        if let Some((_, atlas)) = self.atlas.take() {
            self.textures.release(&atlas);
        }
    }
}

impl MeiHost {
    /// The render thread's view of this frame's text: same buffers, atlas and
    /// output, owning none of them.
    pub(crate) fn render_snapshot(&self) -> Self {
        Self {
            device: self.device.clone(),
            buffers: self.buffers.clone(),
            textures: self.textures.clone(),
            primitives: self.primitives.clone(),
            shapes: self.shapes.clone(),
            buffer_ownership: MeiBufferOwnership::Borrowed,
            atlas: self.atlas.clone(),
            last_output: self.last_output.clone(),
        }
    }
}
