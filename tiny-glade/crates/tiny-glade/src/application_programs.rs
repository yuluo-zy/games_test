//! Original base raster-program registration on one App's existing owners.
//! Reading/registering the 27 programs does not imply pipeline compilation.
use std::{cell::Cell, sync::{Arc, atomic::{AtomicBool, Ordering}}, time::{Duration, Instant}};
use anyhow::{Context, Result};
use bevy_ecs::prelude::*;
use country_core::{asset_libraries::{Handle, shader_library::AssetShaderLibrary},
    render::shader::{RasterShaderProgram, boundary::*}};
use crate::render_host::ShaderHost;

#[derive(Resource)]
pub(super) struct RegisteredApplicationPrograms {
    pub library: Arc<AssetShaderLibrary>,
    pub backend: Arc<ShaderHost>,
    pub final_post: Handle<RasterShaderProgram>,
}

struct RegistrationBackend<'a> {
    backend: &'a dyn ShaderBackend,
    cancelled: &'a AtomicBool,
    timed: bool,
    reads: Cell<u64>,
    bytes: Cell<u64>,
    read_time: Cell<Duration>,
}
impl RegistrationBackend<'_> {
    fn check_cancel(&self) -> Result<()> {
        if self.cancelled.load(Ordering::Acquire) {
            return Err(crate::run_game::GameplayPreparationCancelled.into());
        }
        Ok(())
    }
}
impl ShaderBackend for RegistrationBackend<'_> {
    fn load_compiled_compute(&self, desc: &ComputePipelineDesc) -> Result<CompiledShaderPipeline, Error> {
        self.check_cancel()?;
        self.backend.load_compiled_compute(desc)
    }
    fn load_compiled_raster(&self, desc: &RasterPipelineDesc) -> Result<CompiledShaderPipeline, Error> {
        self.check_cancel()?;
        let started = self.timed.then(Instant::now);
        let result = self.backend.load_compiled_raster(desc);
        if let Some(started) = started {
            self.reads.set(self.reads.get().saturating_add(1));
            self.read_time.set(self.read_time.get().saturating_add(started.elapsed()));
            if let Ok(bytes) = &result { self.bytes.set(self.bytes.get().saturating_add(bytes.0.len() as u64)); }
        }
        result
    }
    fn raster_shader_variants(&self, desc: &RasterPipelineDesc) -> [RasterPipelineDesc; 2] {
        self.backend.raster_shader_variants(desc)
    }
    fn create_compute_pipeline(&self, name: &str, compiled: Arc<CompiledShaderPipeline>) -> rhapsody::Result<ComputePipeline> {
        self.backend.create_compute_pipeline(name, compiled)
    }
    fn create_raster_pipeline(&self, compiled: Arc<CompiledShaderPipeline>, key: &RasterPipelineKey) -> Result<RasterPipeline, Error> {
        self.backend.create_raster_pipeline(compiled, key)
    }
}

pub(super) fn register(
    library: &AssetShaderLibrary, backend: &dyn ShaderBackend, cancelled: &AtomicBool,
) -> Result<Handle<RasterShaderProgram>> {
    let timed = crate::game_loop::frame_timing_enabled();
    let started = timed.then(Instant::now);
    let backend = RegistrationBackend { backend, cancelled, timed,
        reads: Cell::new(0), bytes: Cell::new(0), read_time: Cell::new(Duration::ZERO) };
    let load = |path: &str, name: &'static str| country_core::startup::load_shader_into_library(
        name, RasterPipelineDesc { path: path.to_owned(), entry: String::new(), defines: Vec::new() },
        &backend, library).with_context(|| format!("register base raster program {name} ({path})"));
    let mut final_post = None;
    for (index, (path, name)) in country_core::startup::load_shaders::COMMON_SHADERS.into_iter().enumerate() {
        let handle = load(path, name)?;
        if index == 4 { final_post = Some(handle); }
    }
    for (path, name) in country_core::startup::load_shaders::UI_SHADERS { load(path, name)?; }
    for (path, name) in country_core::startup::startup_blit::BLIT_SHADERS { load(path, name)?; }
    struct Loader<'a> { load: &'a dyn Fn(&str, &'static str) -> Result<Handle<RasterShaderProgram>>, error: Option<anyhow::Error> }
    impl country_core::systems::render_loop::painterly::ShaderLoader for Loader<'_> {
        fn load_shader_into_library(&mut self, path: &'static str, name: &'static str) {
            if self.error.is_none() { if let Err(error) = (self.load)(path, name) { self.error = Some(error); } }
        }
    }
    let mut loader = Loader { load: &load, error: None };
    country_core::systems::render_loop::painterly::startup_painterly(&mut loader);
    country_core::systems::render_loop::vermeer::startup_vermeer(&mut loader);
    if let Some(error) = loader.error { return Err(error); }
    backend.check_cancel()?;
    let final_post = final_post.context("original COMMON_SHADERS did not register final_post")?;
    if let Some(started) = started {
        log::info!(target: "tiny_glade::frame_timing",
            "base_shader_registration thread={:?} library={:p} files={} bytes={} read_ms={:.3} total_ms={:.3}; raster registration only, pipeline decode/compile remains lazy",
            std::thread::current().name(), library, backend.reads.get(), backend.bytes.get(),
            backend.read_time.get().as_secs_f64() * 1000.0, started.elapsed().as_secs_f64() * 1000.0);
    }
    Ok(final_post)
}

pub(super) fn initialize(
    world: &mut World, library: Arc<AssetShaderLibrary>, backend: Arc<ShaderHost>, cancelled: &AtomicBool,
) -> Result<()> {
    anyhow::ensure!(!world.contains_resource::<RegisteredApplicationPrograms>(), "base shader initializer ran twice");
    let actual = &world.get_resource::<crate::run_game::ApplicationShaderLibrary>()
        .context("gameplay base shader initialization: canonical library is missing")?.0;
    anyhow::ensure!(Arc::ptr_eq(actual, &library), "gameplay base shader initialization: library owner differs");
    anyhow::ensure!(library.raster_shaders.is_empty() && library.compute_shaders.is_empty(),
        "deferred gameplay base shader initialization requires its original unregistered library");
    let final_post = register(&library, backend.as_ref(), cancelled)?;
    world.insert_resource(RegisteredApplicationPrograms { library, backend, final_post });
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    // Registration-only CPU backend: payloads intentionally never become GPU
    // pipelines. A pipeline request here is a violation of this narrow seam.
    struct RecordingBackend { reads: RefCell<Vec<RasterPipelineDesc>>, fail_at: Option<usize> }
    impl ShaderBackend for RecordingBackend {
        fn load_compiled_compute(&self, _: &ComputePipelineDesc) -> Result<CompiledShaderPipeline, Error> { panic!("base raster registration requested compute") }
        fn load_compiled_raster(&self, desc: &RasterPipelineDesc) -> Result<CompiledShaderPipeline, Error> {
            let mut reads = self.reads.borrow_mut();
            reads.push(desc.clone());
            if self.fail_at == Some(reads.len()) { return Err(std::io::Error::from(std::io::ErrorKind::NotFound).into()); }
            Ok(CompiledShaderPipeline(vec![0]))
        }
        fn raster_shader_variants(&self, desc: &RasterPipelineDesc) -> [RasterPipelineDesc; 2] {
            [desc.clone(), desc.clone()]
        }
        fn create_compute_pipeline(&self, _: &str, _: Arc<CompiledShaderPipeline>) -> rhapsody::Result<ComputePipeline> { panic!("registration compiled compute") }
        fn create_raster_pipeline(&self, _: Arc<CompiledShaderPipeline>, _: &RasterPipelineKey) -> Result<RasterPipeline, Error> { panic!("registration compiled raster") }
    }
    #[test]
    fn original_27_program_order_reads_both_variants_without_compiling() {
        let library = AssetShaderLibrary::new();
        let backend = RecordingBackend { reads: RefCell::default(), fail_at: None };
        let final_post = register(&library, &backend, &AtomicBool::new(false)).unwrap();
        let mut paths: Vec<&str> = country_core::startup::load_shaders::COMMON_SHADERS.into_iter()
            .chain(country_core::startup::load_shaders::UI_SHADERS)
            .chain(country_core::startup::startup_blit::BLIT_SHADERS).map(|(path, _)| path).collect();
        use country_core::systems::render_loop::{painterly, vermeer};
        paths.extend([painterly::PATH_RASTER_STROKES, painterly::PATH_DEBUG_TARGET,
            vermeer::PATH_RASTER_STROKES, vermeer::PATH_COMBINE]);
        let reads = backend.reads.borrow();
        assert_eq!(paths.len(), 27);
        assert_eq!(reads.len(), 54);
        for (pair, path) in reads.chunks_exact(2).zip(paths) {
            assert!(pair.iter().all(|desc| desc.path == path && desc.entry.is_empty() && desc.defines.is_empty()));
        }
        assert_eq!(library.get_raster_handle_by_name("final_post"), Some(final_post));
    }
    #[test]
    fn missing_variant_preserves_error_and_stops_all_later_reads() {
        let library = AssetShaderLibrary::new();
        let backend = RecordingBackend { reads: RefCell::default(), fail_at: Some(2) };
        let error = register(&library, &backend, &AtomicBool::new(false)).unwrap_err();
        assert_eq!(error.downcast_ref::<std::io::Error>().unwrap().kind(), std::io::ErrorKind::NotFound);
        assert_eq!(backend.reads.borrow().len(), 2);
        assert!(library.raster_shaders.is_empty());
    }
    #[test]
    fn cancellation_before_read_performs_no_io_or_registration() {
        let library = AssetShaderLibrary::new();
        let backend = RecordingBackend { reads: RefCell::default(), fail_at: None };
        let error = register(&library, &backend, &AtomicBool::new(true)).unwrap_err();
        assert!(error.is::<crate::run_game::GameplayPreparationCancelled>());
        assert!(backend.reads.borrow().is_empty());
        assert!(library.raster_shaders.is_empty());
    }
}
