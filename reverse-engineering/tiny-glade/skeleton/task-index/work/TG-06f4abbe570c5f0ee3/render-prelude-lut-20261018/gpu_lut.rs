//! Original B GPU lookup-table producer and its once-per-tracker graph consumer.
//! Native anchors: build 0x14090e080, render_luts 0x14090e280, compute_impl
//! 0x14090e310; BRDF callback 0x140a71730, water callback 0x140a713b0.
//! See TG-06f4abbe570c5f0ee3/render-prelude-lut-20261018/CONTRACT.md.

use crate::{
    render::pass_builder::{
        compute_pass_builder::{ComputePass, RecordedComputePass},
        shader_uri::AsComputeShaderSource,
    },
    resources::texture_presets::{TextureFromDisk, TexturePreset},
    systems::render_loop::compute_passes::pass_kit::{DynSource, PassEnv},
};
use bevy_app::{App, Plugin};
use bevy_ecs::resource::Resource;

pub const BRDF_SOURCE: &str = "/lut/brdf_fg.cs.hlsl";
pub const WATER_SOURCE: &str = "/lut/water_noise.cs.hlsl";
const OUTPUT_TEX: u64 = 0x8821_9a3a_e698_92b2;
const NOISE_TEX: u64 = 0x815b_f503_6966_978c;
const NOISE_IMAGE: TextureFromDisk = TextureFromDisk(0x6ec9_5f7a_9d9f_0faf);

type ComputeBody = Box<dyn FnOnce(ComputePass) -> RecordedComputePass>;

/// The object-safe original API. The recorded pass is handed straight to the
/// enclosing frame because this reconstruction's dispatch returns its callback.
pub trait GpuLutApi {
    fn compute_impl(
        &mut self,
        source: &dyn AsComputeShaderSource,
        body: ComputeBody,
    ) -> rhapsody::Result<()>;
}

type LutBody = Box<dyn Fn(&mut dyn GpuLutApi) -> rhapsody::Result<()> + Send + Sync>;

pub struct GpuLutDesc {
    source_hashes: Vec<u64>,
    render: LutBody,
    rendered: bool,
}

impl GpuLutDesc {
    pub fn new(
        render: impl Fn(&mut dyn GpuLutApi) -> rhapsody::Result<()> + Send + Sync + 'static,
    ) -> Self {
        Self {
            source_hashes: Vec::new(),
            render: Box::new(render),
            rendered: false,
        }
    }
    pub fn source_hashes(&self) -> &[u64] {
        &self.source_hashes
    }
    pub fn rendered(&self) -> bool {
        self.rendered
    }
}

#[derive(Resource)]
pub struct GpuLutTracker {
    luts: Vec<GpuLutDesc>,
}

impl GpuLutTracker {
    /// Original build's process-global demo flag is supplied by the executable's
    /// actual config producer; loading App does not install this plugin.
    pub fn new(demo: bool) -> Self {
        let mut luts = vec![GpuLutDesc::new(|api| {
            api.compute_impl(
                &BRDF_SOURCE,
                Box::new(|pass| {
                    pass.bind_texture(OUTPUT_TEX, &TexturePreset(0), 0x32)
                        .dispatch_2d(64, 64)
                }),
            )
        })];
        if !demo {
            luts.push(GpuLutDesc::new(|api| {
                api.compute_impl(
                    &WATER_SOURCE,
                    Box::new(|pass| {
                        pass.bind_texture(OUTPUT_TEX, &TexturePreset(0x18), 0x32)
                            .bind_texture(NOISE_TEX, &NOISE_IMAGE, 0x19)
                            .dispatch_2d(256, 256)
                    }),
                )
            }));
        }
        Self { luts }
    }

    pub fn descriptors(&self) -> &[GpuLutDesc] {
        &self.luts
    }

    /// O: 0x14090e2f6 invokes the CPU graph-declaration callback; 0x14090e2f9
    /// immediately writes rendered=true. This precedes graph compilation and
    /// submission. Preserve that timing, including graph cancellation/OOM; no
    /// execution-time flag or speculative rollback is added.
    pub fn render_luts(&mut self, env: &mut PassEnv<'_>) -> rhapsody::Result<()> {
        for lut in &mut self.luts {
            if !lut.rendered {
                lut.source_hashes.clear();
                let mut api = FrameLutApi {
                    env,
                    source_hashes: &mut lut.source_hashes,
                };
                (lut.render)(&mut api)?;
                lut.rendered = true;
            }
        }
        Ok(())
    }
}

/// The native plugin is a ZST reading a process flag. The source host carries
/// that exact flag explicitly, avoiding a second global config authority.
pub struct GpuLutPlugin {
    pub demo: bool,
}
impl Plugin for GpuLutPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GpuLutTracker::new(self.demo));
    }
}

struct FrameLutApi<'a, 'b> {
    env: &'a mut PassEnv<'b>,
    source_hashes: &'a mut Vec<u64>,
}
impl GpuLutApi for FrameLutApi<'_, '_> {
    fn compute_impl(
        &mut self,
        source: &dyn AsComputeShaderSource,
        body: ComputeBody,
    ) -> rhapsody::Result<()> {
        // O: source_hash then Vec::push before ComputePass::new (0x14090e361..3cb).
        self.source_hashes.push(source.source_hash());
        let pass = self.env.try_new_pass("LUT", &DynSource(source))?;
        self.env.record(body(pass));
        Ok(())
    }
}
