//! Production landing point for country-core's compute PassEnv.
//!
//! The program handle comes from the core AssetShaderLibrary; its pipeline is
//! resolved by GraphPasses through the real ShaderHost. Resource declarations,
//! upload sources and compute callbacks therefore share one graph and session.
//! This is application wiring, not an additional native equivalence claim.

use crate::render_host::GraphPasses;
use country_core::{
    asset_libraries::{Handle, shader_library::AssetShaderLibrary},
    render::{
        pass_builder::{
            compute_pass_builder::{RecordedComputePass, RgBuffer},
            pass_resource_access::RgTexture,
            rg_resources::{
                RgError, RgResources, TemporalBuffer, TemporalTexture, TemporalTexturePair,
            },
            uniform_binder::UniformAllocator,
        },
        rg_helpers::rg_boundary::{RenderFn, RenderGraph, StagingAlloc, StagingCopy},
        shader::ComputeShaderProgram,
        ssbo::rhapsody_boundary::BufferDesc,
    },
    rhapsody_types::{ComputePipeline, TextureDesc},
    systems::render_loop::compute_passes::pass_kit::PassFrame,
};

/// Declaration-time parameter storage for PassEnv. These bytes are CPU-owned;
/// GraphPasses::add_compute uploads the completed binder into its real dynamic
/// GPU buffer while recording the pass. No GPU buffer or offset is fabricated.
#[derive(Default)]
pub struct ComputeUniformAllocator;

impl UniformAllocator for ComputeUniformAllocator {
    fn allocate_uniform(&mut self, size: u32) -> Vec<u8> {
        vec![0; size as usize]
    }
}

/// Holds the declaration host until the caller has checked every recorded pass.
/// Always call `finish` before building or submitting the shared graph: the
/// recovered PassFrame::record contract cannot return an error itself.
#[must_use = "finish the compute frame before building or submitting its graph"]
pub struct CoreComputeFrame<'session, 'library> {
    passes: GraphPasses<'session>,
    shaders: &'library AssetShaderLibrary,
    first_error: Option<rhapsody::Error>,
    import_graph: Option<country_core::rhapsody_types::RenderGraph>,
}

impl<'session, 'library> CoreComputeFrame<'session, 'library> {
    pub fn new(passes: GraphPasses<'session>, shaders: &'library AssetShaderLibrary) -> Self {
        Self {
            passes,
            shaders,
            first_error: None,
            import_graph: None,
        }
    }

    pub(crate) fn with_resource_graph(
        mut self,
        graph: country_core::rhapsody_types::RenderGraph,
    ) -> Self {
        self.import_graph = Some(graph);
        self
    }

    /// Consuming the host releases its declaration handles. Shared production
    /// sessions are borrowed only by each concrete graph operation.
    pub fn finish(self) -> rhapsody::Result<()> {
        self.first_error.map_or(Ok(()), Err)
    }
}

/// B main prelude calls reset_indirect_args @0x1401622fd before render_world.
/// Its original implementation is a four-buffer transfer pass, not a shader
/// dispatch. Reuse that consumer on this same frame's real lazy import facade.
impl country_core::systems::render_loop::compute_passes::reset_indirect_args::IndirectResetGraph
    for CoreComputeFrame<'_, '_>
{
    fn import_graph(&mut self) -> &mut country_core::rhapsody_types::RenderGraph {
        self.import_graph
            .as_mut()
            .expect("indirect reset requires the enclosing frame's actual resource graph")
    }
}

fn resolve_pipeline(
    shaders: &AssetShaderLibrary,
    program: Option<Handle<ComputeShaderProgram>>,
) -> rhapsody::Result<&ComputePipeline> {
    let handle = program.ok_or_else(|| {
        rhapsody::Error::invalid_usage("recorded compute pass has no shader program")
    })?;
    shaders
        .get_compute(handle)
        .map(|program| &program.pipeline)
        .ok_or_else(|| {
            rhapsody::Error::invalid_usage(
                "recorded compute program is absent from the frame shader library",
            )
        })
}

impl PassFrame for CoreComputeFrame<'_, '_> {
    fn record(&mut self, pass: RecordedComputePass) {
        admit_compute(
            self.shaders,
            &mut self.first_error,
            pass,
            |pass, pipeline| self.passes.add_compute(pass, pipeline),
        );
    }
}

fn admit_compute(
    shaders: &AssetShaderLibrary,
    first_error: &mut Option<rhapsody::Error>,
    pass: RecordedComputePass,
    submit: impl FnOnce(RecordedComputePass, &ComputePipeline) -> rhapsody::Result<()>,
) {
    if first_error.is_some() {
        return;
    }
    if let Err(error) =
        resolve_pipeline(shaders, pass.program).and_then(|pipeline| submit(pass, pipeline))
    {
        *first_error = Some(error);
    }
}

impl RgResources for CoreComputeFrame<'_, '_> {
    fn create_raw_texture(&mut self, name: &str, desc: &TextureDesc) -> RgTexture {
        self.passes.create_raw_texture(name, desc)
    }
    fn create_raw_buffer(&mut self, name: &str, desc: &BufferDesc) -> RgBuffer {
        self.passes.create_raw_buffer(name, desc)
    }
    fn get_or_create_temporal_texture(
        &mut self,
        name: &str,
        desc: &TextureDesc,
    ) -> Result<TemporalTexture, RgError> {
        self.passes.get_or_create_temporal_texture(name, desc)
    }
    fn get_or_create_temporal_texture_pair(
        &mut self,
        name: &str,
        desc: &TextureDesc,
    ) -> Result<TemporalTexturePair, RgError> {
        self.passes.get_or_create_temporal_texture_pair(name, desc)
    }
    fn get_or_create_temporal_buffer(
        &mut self,
        name: &str,
        desc: &BufferDesc,
    ) -> Result<TemporalBuffer, RgError> {
        self.passes.get_or_create_temporal_buffer(name, desc)
    }
}

impl RenderGraph for CoreComputeFrame<'_, '_> {
    fn add_pass(&mut self, name: &'static str, resource: u32, access: u8, render: RenderFn) {
        if self.first_error.is_none() {
            self.passes.add_pass(name, resource, access, render);
        }
    }
    fn allocate_dynamic(&mut self, size: usize) -> rhapsody::Result<StagingAlloc> {
        self.passes.allocate_dynamic(size)
    }
    fn copy_from_staging(
        &mut self,
        name: &'static str,
        copies: Vec<StagingCopy>,
    ) -> rhapsody::Result<()> {
        self.passes.copy_from_staging(name, copies)
    }
    fn upload_complete_bytes(
        &mut self,
        buffer: u32,
        bytes: &[u8],
        source: u64,
        version: u64,
    ) -> rhapsody::Result<bool> {
        self.passes
            .upload_complete_bytes(buffer, bytes, source, version)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use country_core::asset_libraries::shader_library::ComputeShaderSourceHash;

    fn pass(program: Option<Handle<ComputeShaderProgram>>) -> RecordedComputePass {
        RecordedComputePass {
            name: "admission",
            builder: Default::default(),
            bindings: Vec::new(),
            access_ranges: Vec::new(),
            inactive_texture_bindings: Vec::new(),
            preserving_buffers: Vec::new(),
            preserving_texture_bindings: Vec::new(),
            external_bindings: Vec::new(),
            uniform_binder: None,
            push_constants: Vec::new(),
            program,
            dispatch:
                country_core::render::pass_builder::compute_pass_builder::DispatchMode::Groups {
                    x: 1,
                    y: 1,
                    z: 1,
                },
        }
    }

    #[test]
    fn recorded_program_must_exist_in_the_frame_library() {
        let library = AssetShaderLibrary::new();
        assert_eq!(
            resolve_pipeline(&library, None).unwrap_err().kind(),
            rhapsody::ErrorKind::InvalidUsage
        );
        assert_eq!(
            resolve_pipeline(&library, Some(Handle::new()))
                .unwrap_err()
                .kind(),
            rhapsody::ErrorKind::InvalidUsage
        );
        let handle = library.add_compute(
            ComputeShaderProgram {
                desc: Default::default(),
                pipeline: ComputePipeline { id: 13 },
                name: "registered",
            },
            ComputeShaderSourceHash::from_hash(7),
        );
        assert_eq!(resolve_pipeline(&library, Some(handle)).unwrap().id, 13);
        // A handle from another frame library must not resolve by pipeline id.
        assert!(resolve_pipeline(&AssetShaderLibrary::new(), Some(handle)).is_err());
    }

    #[test]
    fn admission_retains_first_backend_failure_and_stops_later_passes() {
        let library = AssetShaderLibrary::new();
        let handle = library.add_compute(
            ComputeShaderProgram {
                desc: Default::default(),
                pipeline: ComputePipeline { id: 29 },
                name: "registered",
            },
            ComputeShaderSourceHash::from_hash(11),
        );
        let mut error = None;
        let mut submitted = 0;
        admit_compute(&library, &mut error, pass(Some(handle)), |_, pipeline| {
            assert_eq!(pipeline.id, 29);
            submitted += 1;
            Err(rhapsody::Error::DeviceLost)
        });
        admit_compute(&library, &mut error, pass(None), |_, _| {
            submitted += 1;
            Ok(())
        });
        assert_eq!(submitted, 1);
        assert_eq!(error.unwrap().kind(), rhapsody::ErrorKind::DeviceLost);
        let mut missing = None;
        admit_compute(&library, &mut missing, pass(None), |_, _| {
            panic!("missing program must not enter graph")
        });
        assert_eq!(missing.unwrap().kind(), rhapsody::ErrorKind::InvalidUsage);
    }
}
