    // The dependency crate's cfg(test) TestDevice is deliberately unavailable
    // here. This local fixture only admits CPU shader artifacts, never a GPU.
    struct CpuShaders;
    use country_core::render::shader::boundary::{ShaderBackend, CompiledShaderPipeline,
        ComputePipelineDesc, ComputePipeline, RasterPipelineDesc, RasterPipelineKey, RasterPipeline, Error};
    impl ShaderBackend for CpuShaders {
        fn load_compiled_compute(&self,_:&ComputePipelineDesc)->Result<CompiledShaderPipeline,Error> { unreachable!() }
        fn load_compiled_raster(&self,_:&RasterPipelineDesc)->Result<CompiledShaderPipeline,Error> { Ok(CompiledShaderPipeline::default()) }
        fn raster_shader_variants(&self,d:&RasterPipelineDesc)->[RasterPipelineDesc;2] { [d.clone(),d.clone()] }
        fn create_compute_pipeline(&self,_:&str,_:std::sync::Arc<CompiledShaderPipeline>)->Result<ComputePipeline,country_core::rhapsody_types::BackendError> { unreachable!() }
        fn create_raster_pipeline(&self,_:std::sync::Arc<CompiledShaderPipeline>,_:&RasterPipelineKey)->Result<RasterPipeline,Error> { unreachable!() }
    }
