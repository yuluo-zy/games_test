//! Shader program load / cache / wire surface (WP4-owned).
//!
//! The actual shader compiler is REUSED (`shader-compiler` / `shader-pipeline`
//! / `shader-variants`); WP4 implements the game-owned load/cache/wire layer:
//!   - `ComputeShaderProgram::new` / `RasterShaderProgram::new_draw`
//!   - `compile_shader_variants`
//!   - `RasterShaderProgramVariant::get_or_generate` (variant→pipeline cache)
//!
//! Skeleton bundle: `skeleton/B/crates/country-core/src/render/shader.rs`
//! (source_sha256 `d4da69a2…`). Decompiles `A_0x140937e10.c … A_0x140938b00.c`.

use std::collections::HashMap;
use std::sync::Arc;

use parking_lot::RwLock;

use crate::rhapsody_contract::{
    CompiledShaderPipeline, ComputePipelineDesc, GraphicsDevice, RasterPipeline, RasterPipelineDesc,
};
use crate::shader_pipeline_contract::ShaderCompilerArtifacts;

/// `ComputeShaderProgram` — 0xa0 = 160 bytes (O: `add_compute` boxes 0xa0).
/// Holds the compiled pipeline + source path/entry for the compute shader.
pub struct ComputeShaderProgram {
    pub pipeline: Arc<CompiledShaderPipeline>,
    /// Compiled pipeline desc / reflection (rhapsody).
    pub pipeline_desc: ComputePipelineDesc,
    pub source_path: String,
    pub entry_point: String,
}

impl ComputeShaderProgram {
    /// `ComputeShaderProgram::new` (O: 0x140937e10, 738B):
    /// 1. `ShaderCompilerArtifacts::load_compiled<ComputePipelineDesc>()` (reuse)
    /// 2. debug-log `"Creating pipeline"` (module `country_core::render::shader`)
    /// 3. `GraphicsDevice::create_compute_pipeline<Arc<CompiledShaderPipeline>>`
    /// Returns Err on any failure (callers `unwrap`).
    pub fn new(
        desc: ComputePipelineDesc,
        source_path: String,
        entry_point: String,
        device: &GraphicsDevice,
    ) -> Result<Self, crate::rhapsody_contract::PipelineError> {
        // (reused) compiled-artifact load for the compute desc.
        let artifacts = ShaderCompilerArtifacts::load_compiled_compute(&desc)
            .map_err(|_| crate::rhapsody_contract::PipelineError::Artifacts)?;
        log::debug!(target: "country_core::render::shader", "Creating pipeline");
        let pipeline = device
            .create_compute_pipeline(artifacts)
            .map_err(|_| crate::rhapsody_contract::PipelineError::Create)?;
        Ok(ComputeShaderProgram {
            pipeline,
            pipeline_desc: desc,
            source_path,
            entry_point,
        })
    }
}

/// `RasterShaderProgram` — 0xd8 = 216 bytes (O: `add_raster` boxes 0xd8).
pub struct RasterShaderProgram {
    pub pipeline: Arc<CompiledShaderPipeline>,
    pub source_name: Option<String>,
    pub compiled_variants: RasterShaderVariants,
}

impl RasterShaderProgram {
    /// `new_draw` (O: 0x140938240, 258B): compiles variants, then fills the
    /// struct; on error `unwrap_failed`.
    pub fn new_draw(
        pipeline_desc: RasterPipelineDesc,
        source_name: Option<String>,
        device: &GraphicsDevice,
    ) -> Result<Self, crate::rhapsody_contract::PipelineError> {
        let variants = compile_shader_variants(&pipeline_desc, device)?;
        Ok(RasterShaderProgram {
            pipeline: variants.base_pipeline.clone(),
            source_name,
            compiled_variants: variants,
        })
    }

    /// `desc` (O name; no standalone proc — thin accessor).
    pub fn desc(&self) -> &RasterPipelineDesc {
        &self.compiled_variants.desc
    }

    /// `recompile` (O name; no standalone proc — re-upload wrapper, U body).
    pub fn recompile(&mut self) {
        // U: not recovered (inline).
    }

    /// `default_handle` — default managed shader handle (`shader::impl$7::default()`,
    /// used by wp1 `DrawPacket::new`). Typed gap (assembly-completed static).
    pub fn default_handle() -> crate::rhapsody_contract::Handle<RasterShaderProgram> {
        crate::rhapsody_contract::Handle::gap_new()
    }
}

/// One raster pipeline (56 bytes / Box, O: `get_or_generate` boxes 0x38).
#[allow(dead_code)] // opaque GPU handle; exposed via the wrapped ManagedHandle
pub struct RasterPipelineHandle(pub crate::rhapsody_contract::ManagedHandle);

/// Compiled variant set for a raster shader (two 16-hex variant products; see
/// `evidence/render-passes/shaders/catalog.md`).
pub struct RasterShaderVariants {
    pub desc: RasterPipelineDesc,
    pub base_pipeline: Arc<CompiledShaderPipeline>,
    /// O: variants are compiled in pairs; the second variant product.
    pub second_variant: Option<Arc<CompiledShaderPipeline>>,
}

/// `compile_shader_variants` (O: 0x1409383b0, 1054B): calls
/// `shader_variants::get_raster_shader_variants()` then
/// `ShaderCompilerArtifacts::load_compiled<RasterPipelineDesc>()` per variant.
pub fn compile_shader_variants(
    desc: &RasterPipelineDesc,
    device: &GraphicsDevice,
) -> Result<RasterShaderVariants, crate::rhapsody_contract::PipelineError> {
    // (reused) enumerate + load compiled artifacts for the raster variants.
    let variants = crate::shader_variants_contract::get_raster_shader_variants(desc);
    let base_pipeline = ShaderCompilerArtifacts::load_compiled_raster(&variants[0])
        .map_err(|_| crate::rhapsody_contract::PipelineError::Artifacts)?;
    let second_variant = if variants.len() > 1 {
        Some(
            ShaderCompilerArtifacts::load_compiled_raster(&variants[1])
                .map_err(|_| crate::rhapsody_contract::PipelineError::Artifacts)?,
        )
    } else {
        None
    };
    let _ = device; // used by consumers via `create_raster_pipeline`
    Ok(RasterShaderVariants {
        desc: desc.clone(),
        base_pipeline,
        second_variant,
    })
}

/// `RasterShaderProgramVariant` — variant→pipeline cache (O: `get_or_generate`,
/// 0x140938b00): `RwLock<HashMap<RasterPipelineDesc, Box<RasterPipeline>>>`.
pub struct RasterShaderProgramVariant {
    inner: RwLock<HashMap<RasterPipelineDesc, Box<RasterPipeline>>>,
}

impl RasterShaderProgramVariant {
    pub fn new() -> Self {
        RasterShaderProgramVariant {
            inner: RwLock::new(HashMap::new()),
        }
    }

    /// `get_or_generate`:
    /// 1. shared-lock lookup by desc (hash + desc field +0x18 + handle-eq) → hit
    ///    returns cached pipeline.
    /// 2. miss → exclusive lock, `rustc_entry` get-or-insert; on vacant bump the
    ///    `Arc<CompiledShaderPipeline>` refcount, `GraphicsDevice::create_raster_pipeline`,
    ///    Box the new `RasterPipeline`, store.
    pub fn get_or_generate(
        &self,
        desc: &RasterPipelineDesc,
        compiled: &Arc<CompiledShaderPipeline>,
        device: &GraphicsDevice,
    ) -> RasterPipelineHandle {
        if let Some(hit) = self.inner.read().get(desc) {
            return RasterPipelineHandle(hit.1.clone());
        }
        let mut map = self.inner.write();
        if let Some(hit) = map.get(desc) {
            return RasterPipelineHandle(hit.1.clone());
        }
        let pipeline: Box<RasterPipeline> = device.create_raster_pipeline(compiled.clone());
        let handle = RasterPipelineHandle(pipeline.1.clone());
        map.insert(desc.clone(), pipeline);
        handle
    }
}

impl Default for RasterShaderProgramVariant {
    fn default() -> Self {
        Self::new()
    }
}
