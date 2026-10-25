//! Execute the recovered Painterly/Vermeer pass descriptions on the core bridge.
//!
//! The descriptions come from B `painterly_pass` 0x1408a6410 and
//! `vermeer_render_from_target` 0x1408a8d70. This host adds no effects or fallback
//! draws: each existing description operation is forwarded to its actual core
//! builder. Resource identities and metadata must be supplied by their producers.
use std::collections::HashMap;

use country_core::{
    asset_libraries::{shader_library::AssetShaderLibrary, Handle as AssetHandle},
    render::{
        pass_builder::{
            compute_pass_builder::{ComputePass, DispatchMode, RgBuffer, WorldAccess},
            draw_builder::{BindTexture, DrawBuilder},
            draw_stream::{DrawPacket, RasterPass},
            framebuffer_binding::{AttachmentTexture, ColorAttachment},
            pass_resource_access::{RgTexture, WriteTextureUnchecked},
            rg_resources::RgResources,
        },
        shader::{boundary::ShaderBackend, RasterShaderProgram},
        ssbo::rhapsody_boundary::BufferDesc,
    },
    resources::texture_presets::TextureFromDisk,
    rhapsody_types::TextureDesc,
    systems::render_loop::{
        compute_passes::{blur_pyramid, pass_kit::PassEnv},
        painterly::{self as p, ComputeOp, ComputeSource, RasterOp, Uniform},
        vermeer::{self, VermeerDebugControl},
    },
};

use crate::{compute_frame::ComputeUniformAllocator, render_bridge::CoreGraphFrame};

pub struct EffectsHost<'f, 's, 'a> {
    frame: &'f mut CoreGraphFrame<'s>,
    access: &'a dyn WorldAccess,
    backend: &'a dyn ShaderBackend,
    shaders: &'a AssetShaderLibrary,
    textures: HashMap<u32, RgTexture>,
    named_textures: HashMap<u64, p::TexRef>,
    app_mode: Option<p::AppModeView>,
    time: Option<p::TimeView>,
    debug: Option<VermeerDebugControl>,
    alt_background: bool,
    reveal_finished_events: Option<&'a mut bevy_ecs::event::Events<country_core::systems::save::load_game::StartupRevealFinished>>,
    error: Option<rhapsody::Error>,
}

impl<'f, 's, 'a> EffectsHost<'f, 's, 'a> {
    pub fn new(
        frame: &'f mut CoreGraphFrame<'s>, access: &'a dyn WorldAccess,
        backend: &'a dyn ShaderBackend, shaders: &'a AssetShaderLibrary,
        app_mode: Option<p::AppModeView>, time: Option<p::TimeView>,
        debug: Option<VermeerDebugControl>, alt_background: bool,
    ) -> Self {
        Self { frame, access, backend, shaders, textures: HashMap::new(),
            named_textures: HashMap::new(), app_mode, time, debug, alt_background,
            reveal_finished_events: None, error: None }
    }

    pub fn with_startup_reveal_finished_events(mut self, events: &'a mut bevy_ecs::event::Events<country_core::systems::save::load_game::StartupRevealFinished>) -> Self {
        self.reveal_finished_events = Some(events);
        self
    }

    /// Record the actual graph handle returned by its producer. A logical index
    /// is never replaced with a fabricated GPU handle.
    pub fn register_texture(&mut self, texture: RgTexture) -> p::TexRef {
        self.textures.insert(texture.resource, texture);
        p::TexRef(texture.resource)
    }

    pub fn register_named_texture(&mut self, name: u64, texture: RgTexture) {
        let texture = self.register_texture(texture);
        self.named_textures.insert(name, texture);
    }

    pub fn texture(&self, id: p::TexRef) -> rhapsody::Result<RgTexture> {
        self.textures.get(&id.0).copied().ok_or_else(||
            rhapsody::Error::invalid_usage("effect references a texture absent from this frame"))
    }

    pub fn finish(self) -> rhapsody::Result<()> {
        self.error.map_or(Ok(()), Err)
    }

    fn compute<T>(&mut self, body: impl FnOnce(&mut PassEnv<'_>) -> rhapsody::Result<T>) -> rhapsody::Result<T> {
        let (access, backend) = (self.access, self.backend);
        self.frame.with_compute(|frame| {
            let mut uniforms = ComputeUniformAllocator;
            let mut env = PassEnv::new(access, backend, &mut uniforms, frame);
            body(&mut env)
        })
    }

    fn submit_compute_impl(&mut self, desc: p::ComputePassDesc) -> rhapsody::Result<()> {
        let textures = &self.textures;
        let (access, backend) = (self.access, self.backend);
        self.frame.with_compute(|frame| {
            let mut uniforms = ComputeUniformAllocator;
            let mut env = PassEnv::new(access, backend, &mut uniforms, frame);
            let mut pass = match desc.source {
                ComputeSource::Uri(uri) => env.try_new_pass(desc.name, &uri)?,
                ComputeSource::GenerateStrokes { photomode } =>
                    env.try_new_pass(desc.name, &vermeer::GenerateStrokes { photomode })?,
            };
            for op in desc.ops {
                pass = match op {
                    ComputeOp::BindTexture { key, access, tex, write } => {
                        let tex = textures.get(&tex.0).ok_or_else(||
                            rhapsody::Error::invalid_usage("effect compute texture was not produced"))?;
                        if write { pass.bind_texture(key, &WriteTextureUnchecked(*tex), access) }
                        else { pass.bind_texture(key, &tex, access) }
                    }
                    ComputeOp::WriteRgStorageBuffer { key, buf } =>
                        pass.write_rg_storage_buffer(key, &RgBuffer { resource: buf.0 }),
                    ComputeOp::WriteUniform { key, value } => compute_uniform(pass, key, value),
                };
            }
            // Original descriptions carry thread counts (not workgroup counts).
            let [x, y, z] = desc.dispatch_threads;
            let pass = pass.push_constants(&desc.push_constants);
            let mut record = pass.dispatch_2d(x, y);
            record.dispatch = DispatchMode::Threads { x, y, z };
            env.record(record);
            Ok(())
        })
    }

    fn submit_raster_impl(&mut self, desc: p::RasterPassDesc) -> rhapsody::Result<()> {
        let shader: AssetHandle<RasterShaderProgram> = self.shaders
            .get_raster_handle_by_name(desc.shader_name)
            .ok_or_else(|| rhapsody::Error::invalid_usage("effect raster shader is not loaded"))?;
        if shader.id().raw() != desc.shader.0 {
            return Err(rhapsody::Error::invalid_usage("effect shader identity changed during this frame"));
        }
        let target = match desc.color { p::RasterColor::Texture(t) | p::RasterColor::View(t) => self.texture(t)? };
        let target_attachment = attachment(target);
        let color = match desc.color {
            p::RasterColor::Texture(_) => ColorAttachment::Texture(target_attachment),
            p::RasterColor::View(_) => ColorAttachment::View { view: [0; 4], texture: target_attachment },
        };
        let textures = &self.textures;
        self.frame.with_raster(|host| {
            let mut pass = RasterPass::new(desc.name, &[color], None, Box::new(host))
                .with_viewport(desc.viewport);
            let state = pass.host_mut().default_raster_state();
            let handle = country_core::rhapsody_contract::Handle(
                country_core::rhapsody_contract::ManagedHandle(shader.id().raw()),
                core::marker::PhantomData,
            );
            let mut draw = DrawBuilder::new(&mut pass, DrawPacket::new(handle, [1, 0], state), 0);
            for op in desc.ops {
                draw = match op {
                    RasterOp::OptionalUniform { key, value } => draw.optional_uniform(key, value),
                    RasterOp::Uniform { key, value } => raster_uniform(draw, key, value),
                    RasterOp::ReadRgStorageBuffer { key, buf } => draw.read_rg_storage_buffer(key, buf.0),
                    RasterOp::BindTexture { key, access, tex: p::TexBinding::FromDisk(hash) } =>
                        draw.bind_texture_to_draw_builder(BindTexture::FromDisk(TextureFromDisk(hash)), key, access, None),
                    RasterOp::BindTexture { key, access, tex: p::TexBinding::Rg(id) } => {
                        let tex = *textures.get(&id.0).ok_or_else(||
                            rhapsody::Error::invalid_usage("effect raster texture was not produced"))?;
                        draw.bind_texture_to_draw_builder(BindTexture::RgHandle(&attachment(tex)), key, access, None)
                    }
                };
            }
            if desc.raster_state_edit {
                draw.packet_mut().state = draw.packet().state.clear_byte0_bit7().set_byte3_field_2();
            }
            draw.draw_no_vao(desc.vertex_count);
            pass.render();
            Ok(())
        })
    }
}

impl p::PassHost for EffectsHost<'_, '_, '_> {
    fn app_mode(&self) -> Option<p::AppModeView> { self.app_mode }
    fn time(&self) -> Option<p::TimeView> { self.time }
    fn raster_shader(&self, name: &str) -> Option<p::ShaderHandle> {
        self.shaders.get_raster_handle_by_name(name).map(|h| p::ShaderHandle(h.id().raw()))
    }
    fn texture_extent(&self, tex: p::TexRef) -> [u32; 2] {
        self.texture(tex).expect("effect texture").desc.extent_2d()
    }
    fn texture_desc(&self, tex: p::TexRef) -> p::TextureDesc {
        let d = self.texture(tex).expect("effect texture").desc;
        p::TextureDesc { extent: d.extent_2d(), format: d.format, storage: d.storage, sampled: d.sampled }
    }
    fn temporal_texture(&mut self, name: &str, desc: p::TextureDesc) -> Result<p::TemporalTexture, p::HostError> {
        let desc = texture_desc(desc);
        let result = self.compute(|env| env.try_temporal_texture(name, desc));
        result.map(|t| p::TemporalTexture { tex: self.register_texture(t.texture), reused: t.flag == 0 })
            .map_err(|e| p::HostError(e.to_string()))
    }
    fn temporal_buffer(&mut self, name: &str, desc: p::BufferDesc) -> Result<p::TemporalBuffer, p::HostError> {
        let result = self.frame.with_compute(|frame| {
            // Original rhapsody BufferDesc::new_gpu_only uses storage access;
            // alignment and the original usage byte remain explicit.
            let desc = BufferDesc::new(desc.size as usize, desc.align, u8::from(desc.flag));
            frame.get_or_create_temporal_buffer(name, &desc).map_err(|e|
                rhapsody::Error::invalid_usage(format!("effect temporal buffer: {e}")))
        });
        result.map(|b| p::TemporalBuffer { buf: p::BufRef(b.buffer.resource), reused: b.flag == 0 })
            .map_err(|e| p::HostError(e.to_string()))
    }
    fn create_raw_texture(&mut self, name: &str, desc: p::TextureDesc) -> p::TexRef {
        let t = self.frame.with_compute(|frame| Ok(frame.create_raw_texture(name, &texture_desc(desc))))
            .expect("effect texture creation failed");
        self.register_texture(t)
    }
    fn log(&mut self, level: p::LogLevel, target: &str, _file: &str, message: &str) {
        match level {
            p::LogLevel::Info => log::info!(target: target, "{message}"),
            p::LogLevel::Trace => log::trace!(target: target, "{message}"),
        }
    }
    fn submit_compute(&mut self, desc: p::ComputePassDesc) {
        if self.error.is_none() {
            if let Err(e) = self.submit_compute_impl(desc) { self.error = Some(e); }
        }
    }
    fn submit_raster(&mut self, desc: p::RasterPassDesc) {
        if self.error.is_none() {
            if let Err(e) = self.submit_raster_impl(desc) { self.error = Some(e); }
        }
    }
}

impl vermeer::VermeerHost for EffectsHost<'_, '_, '_> {
    fn debug_control(&self) -> Option<VermeerDebugControl> { self.debug }
    fn alt_background(&self) -> bool { self.alt_background }
    fn library_texture(&mut self, name: u64) -> p::TexRef {
        *self.named_textures.get(&name).expect("Vermeer source texture has not been imported")
    }
    fn blur_pyramid(&mut self, input: p::TexRef) -> p::TexRef {
        let input = self.texture(input).expect("Vermeer source texture");
        let output = self.compute(|env| Ok(blur_pyramid::blur_pyramid(env, &input)))
            .expect("Vermeer blur pyramid failed");
        self.register_texture(output)
    }
    fn send_startup_reveal_finished(&mut self) -> Option<()> {
        self.reveal_finished_events.as_deref_mut()?.send(country_core::systems::save::load_game::StartupRevealFinished);
        Some(())
    }
}

fn texture_desc(d: p::TextureDesc) -> TextureDesc {
    let mut out = TextureDesc::new_2d(d.extent, d.format);
    out.storage = d.storage;
    out.sampled = d.sampled;
    out
}

pub fn attachment(t: RgTexture) -> AttachmentTexture {
    AttachmentTexture { id: t.resource, extent: (t.desc.extent[0], t.desc.extent[1]), format: t.desc.format, sample_count: 1 }
}

fn compute_uniform(p: ComputePass, key: u64, value: Uniform) -> ComputePass {
    match value {
        Uniform::F32(v) => p.uniform(key, v), Uniform::I32(v) => p.uniform(key, v),
        Uniform::Bool(v) => p.uniform(key, v), Uniform::U32x2(v) => p.uniform(key, v),
    }
}

fn raster_uniform<'a>(p: DrawBuilder<'a>, key: u64, value: Uniform) -> DrawBuilder<'a> {
    match value {
        Uniform::F32(v) => p.uniform(key, v), Uniform::I32(v) => p.uniform(key, v),
        Uniform::Bool(v) => p.uniform(key, v), Uniform::U32x2(v) => p.uniform(key, v),
    }
}
