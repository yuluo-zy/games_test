//! Device consumer of B `EguiBackend::draw` (0x1400ddc80).
//!
//! The original packs 20-byte egui vertices and u32 indices into separate mapped
//! buffers, puts `extent / pixels_per_point` into DrawPacket's push bytes, binds
//! color_tex/vertices, then issues the core indexed draw. This uses that existing
//! consumer path and its frame-owned resource imports. macOS is only a device
//! adaptation; it does not replace the game's shader or draw rules.
use std::{collections::HashMap, sync::Arc};

use anyhow::{Context, Result, bail};
use ash::vk;
use country_core::{
    asset_libraries::shader_library::AssetShaderLibrary,
    render::{
        load_texture::boundary::TextureDevice,
        pass_builder::{
            draw_builder::{BindTexture, DrawBuilder},
            draw_stream::{DrawPacket, RasterPass},
            framebuffer_binding::{AttachmentTexture, ColorAttachment},
        },
        ssbo::rhapsody_boundary::{Buffer, BufferDesc, GraphicsDevice},
    },
    rhapsody_contract::ManagedHandle,
    rhapsody_types::{Texture, TextureDesc},
};

use crate::{
    egui_backend::{EguiBackend, EguiRenderer},
    render_bridge::CoreGraphFrame,
    render_host::{BufferHost, HostTables},
};

pub struct EguiGpu {
    tables: HostTables,
    textures: HashMap<egui::TextureId, Texture>,
}

impl EguiGpu {
    pub fn new(tables: HostTables) -> Self {
        Self {
            tables,
            textures: HashMap::new(),
        }
    }

    /// Called by the original main-graph UI consumer. The loading and second
    /// presentation graphs do not draw egui. Missing shader/resources are errors.
    pub fn draw(
        &mut self,
        frame: &mut CoreGraphFrame<'_>,
        backend: &mut EguiBackend,
        shaders: &AssetShaderLibrary,
        target: AttachmentTexture,
    ) -> Result<()> {
        let shader = shaders
            .get_raster_handle_by_name("egui")
            .context("EguiBackend::draw: missing original egui shader")?;
        // The reconstructed library and draw facade use distinct handle wrapper
        // types. Preserve the library's actual identity, as RasterShared resolves
        // that exact raw key in the same library rather than allocating a slot.
        let shader = country_core::rhapsody_contract::Handle(
            ManagedHandle(shader.id().raw()),
            std::marker::PhantomData,
        );
        let pixels_per_point = backend.pixels_per_point();
        if !pixels_per_point.is_finite() || pixels_per_point <= 0.0 {
            bail!("invalid egui pixels_per_point");
        }
        let mut result = Ok(());
        let buffers_host = frame.hosts().buffers;
        frame.with_raster(|host| {
            let mut renderer = EguiFrame {
                gpu: self,
                buffers_host,
                pass: Some(
                    RasterPass::new(
                        "egui",
                        &[ColorAttachment::Texture(target)],
                        None,
                        Box::new(host),
                    )
                    .with_viewport([target.extent.0, target.extent.1]),
                ),
                shader,
                pixels_per_point,
                buffers: Vec::new(),
            };
            result = backend.draw([target.extent.0, target.extent.1], &mut renderer);
            // RasterPass's Drop contract requires render even after a rejected
            // input. The enclosing frame is cancelled on the returned error.
            renderer.pass.take().expect("one egui pass").render();
            Ok(())
        })?;
        result
    }
}

impl Drop for EguiGpu {
    fn drop(&mut self) {
        for (_, texture) in self.textures.drain() {
            self.tables.textures.release(&texture);
        }
    }
}

struct EguiFrame<'a> {
    gpu: &'a mut EguiGpu,
    buffers_host: Arc<BufferHost>,
    pass: Option<RasterPass>,
    shader:
        country_core::rhapsody_contract::Handle<country_core::render::shader::RasterShaderProgram>,
    pixels_per_point: f32,
    buffers: Vec<Buffer>,
}

impl Drop for EguiFrame<'_> {
    fn drop(&mut self) {
        // submit_raster_pass has imported/retained the same actual buffer owners.
        // Their timeline use cells defer physical destruction past GPU completion.
        for buffer in self.buffers.drain(..) {
            self.buffers_host.release(&buffer);
        }
    }
}

fn image_bytes(image: &egui::ImageData) -> Vec<u8> {
    match image {
        egui::ImageData::Color(image) => image
            .pixels
            .iter()
            .flat_map(|pixel| pixel.to_array())
            .collect(),
        egui::ImageData::Font(image) => image
            .srgba_pixels(None)
            .flat_map(|pixel| pixel.to_array())
            .collect(),
    }
}

impl EguiRenderer for EguiFrame<'_> {
    fn update_texture(
        &mut self,
        id: egui::TextureId,
        delta: &egui::epaint::ImageDelta,
    ) -> Result<()> {
        let bytes = image_bytes(&delta.image);
        let [width, height] = delta.image.size().map(u32::try_from);
        let (width, height) = (width?, height?);
        if width == 0 || height == 0 {
            bail!("zero-sized egui texture");
        }
        if let Some(texture) = self.gpu.textures.get(&id) {
            let [x, y] = delta.pos.unwrap_or([0, 0]);
            let region = vk::BufferImageCopy::default()
                .image_subresource(
                    vk::ImageSubresourceLayers::default()
                        .aspect_mask(vk::ImageAspectFlags::COLOR)
                        .layer_count(1),
                )
                .image_offset(vk::Offset3D {
                    x: i32::try_from(x)?,
                    y: i32::try_from(y)?,
                    z: 0,
                })
                .image_extent(vk::Extent3D {
                    width,
                    height,
                    depth: 1,
                });
            self.gpu.tables.textures.upload_regions(texture, bytes, vec![region])?;
        } else {
            if delta.pos.is_some() {
                bail!("egui partial update refers to a missing texture");
            }
            // O 0x1400de535: R8D=0x2b (R8G8B8A8_SRGB), then sampled().
            let texture = self.gpu.tables.textures.create_texture(
                "egui",
                &TextureDesc {
                    extent: [width, height, 1],
                    kind: 2,
                    sampled: true,
                    mip_levels: 1,
                    array_elements: 1,
                    format: 0x2b,
                    ..Default::default()
                },
                &bytes,
            )?;
            self.gpu.textures.insert(id, texture);
        }
        Ok(())
    }

    fn free_texture(&mut self, id: egui::TextureId) -> Result<()> {
        if let Some(texture) = self.gpu.textures.remove(&id) {
            self.gpu.tables.textures.release(&texture);
        }
        Ok(())
    }

    fn draw_primitives(
        &mut self,
        primitives: &[egui::ClippedPrimitive],
        extent: [u32; 2],
    ) -> Result<()> {
        let pass = self.pass.as_mut().expect("one egui pass");
        for primitive in primitives {
            // O: the native callback variant goes to panic; only mesh is valid.
            let egui::epaint::Primitive::Mesh(mesh) = &primitive.primitive else {
                bail!("Egui paint callbacks are unsupported by the original renderer");
            };
            if mesh.vertices.is_empty() || mesh.indices.is_empty() {
                continue;
            }
            let texture = self
                .gpu
                .textures
                .get(&mesh.texture_id)
                .context("egui mesh texture is absent")?;
            let vertices = pack_vertices(&mesh.vertices);
            let indices: Vec<u8> = mesh
                .indices
                .iter()
                .flat_map(|index| index.to_le_bytes())
                .collect();
            let ib = self.buffers_host.create_buffer(
                &BufferDesc::new(indices.len(), 0x40, 2),
                "egui index buffer",
                Some(&indices),
            )?;
            self.buffers.push(ib);
            let vb = self.buffers_host.create_buffer(
                &BufferDesc::new(vertices.len(), 0x20, 2),
                "egui vertex buffer",
                Some(&vertices),
            )?;
            self.buffers.push(vb);
            let mut packet = DrawPacket::new(
                self.shader.clone(),
                [0, 0],
                pass.host_mut().default_raster_state(),
            );
            packet.unknown_bytes = extent
                .into_iter()
                .flat_map(|size| (size as f32 / self.pixels_per_point).to_le_bytes())
                .collect();
            let mut draw = DrawBuilder::new(pass, packet, 0)
                .bind_texture_to_draw_builder(
                    BindTexture::Raw {
                        texture: &ManagedHandle(texture.id),
                        desc: &AttachmentTexture::from(texture),
                    },
                    0xd997_8e54_1ce9_a95e,
                    0x1d,
                    None,
                )
                .bind_external_storage_buffer(0x12ef_39a9_447f_1f0e, &ManagedHandle(vb.id));
            draw.packet_mut().state = draw.packet().state.clear_byte0_bit7().set_byte3_field_2();
            draw.draw_no_vao_indexed(&ManagedHandle(ib.id), u32::try_from(mesh.indices.len())?);
        }
        Ok(())
    }
}

fn pack_vertices(vertices: &[egui::epaint::Vertex]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(vertices.len() * 20);
    for vertex in vertices {
        for value in [vertex.pos.x, vertex.pos.y, vertex.uv.x, vertex.uv.y] {
            bytes.extend_from_slice(&value.to_le_bytes());
        }
        bytes.extend_from_slice(&vertex.color.to_array());
    }
    bytes
}
