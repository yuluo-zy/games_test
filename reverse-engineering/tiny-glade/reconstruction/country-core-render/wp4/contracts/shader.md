# WP4 Contract: `country_core::render::shader`

PDB bundle: `skeleton/B/crates/country-core/src/render/shader.rs` (source_sha256 `d4da69a2…`, 6 procedures, modules 509/587).
Decompiles: `_analysis/decomp/A_0x140937e10.c`, `A_0x140938240.c`, `A_0x1409383b0.c`, `A_0x140938b00.c`.

Consumers: `AssetShaderLibrary::add_compute/add_raster`, `ComputePass::new` (0x1405876a0), `DrawBuilder::new` (0x140905be0). The shader compiler itself (`shader-compiler`/`shader-pipeline`/`shader-variants`) is a REUSED dependency — WP4 implements only the load/cache/wire surface.

## Types (O)

- `ComputeShaderProgram` — 0xa0 = 160 bytes (O: `add_compute` Box memcpy 0xa0). Fields: [0..0x10) pipeline desc/artifacts from `ComputeShaderProgram::new`, +0x90/+0x98 source path/entry (String copy of `in_RDX`/`in_R8`), +0x48..0x88 pipeline handle/reflection. `new(desc, …) -> Result<ComputeShaderProgram, Error>`:
  1. `shader_pipeline::compiled::ShaderCompilerArtifacts::load_compiled<ComputePipelineDesc>()` (reuse) → 0x140-byte artifacts.
  2. Log "Creating pipeline" (module `country_core::render::shader`) at debug level.
  3. Box the program (0x150 allocation incl. compiled artifacts) → `GraphicsDevice::create_compute_pipeline<Arc<CompiledShaderPipeline>>`.
  4. Fill 0xa0 struct from artifacts + source path/entry. O.
- `RasterShaderProgram` — 0xd8 = 216 bytes (O: `add_raster` Box memcpy 0xd8). Fields: +0x8/+0x10 source_name String, +0x20/+0x28 version/desc, +0x30 Option<String>, +0x38/+0x40 compiled source String, +0x48+ pipeline. `new_draw(…) -> Result<RasterShaderProgram, Error>` (0x140938240): calls `compile_shader_variants()`; on Ok copies the 64-byte arg + variants + 2 args into the struct (0xd0+); error → unwrap. O.
- `RasterShaderProgram::desc()` / `recompile()` — no standalone proc (I: accessors / re-upload wrapper, typed gaps).
- `RasterShaderProgramVariant` — `RwLock<HashMap<RasterPipelineDesc, Box<RasterPipeline>>>` (O: `get_or_generate`; +0x08 lock state, +0x10 buckets, +0x18 mask, +0x28 items).

## Functions

- `compile_shader_variants() -> Result<…, _>` (0x1409383b0, 1054B): calls `shader_variants::get_raster_shader_variants()` (reuse) then `ShaderCompilerArtifacts::load_compiled<RasterPipelineDesc>()` for each variant; produces a container of compiled variant artifacts (one per 16-hex variant; see `evidence/render-passes/shaders/catalog.md` two-variant pattern). O.
- `RasterShaderProgramVariant::get_or_generate(&self, &RasterPipelineDesc) -> Handle<RasterPipeline>` (0x140938b00, 1395B):
  1. shared-lock lookup by desc hash + field +0x18 + handle-eq → hit returns cached handle.
  2. miss → exclusive lock, `HashMap::rustc_entry<RasterPipelineDesc, Box<RasterPipeline>>` get-or-insert; on vacant: bump `Arc<CompiledShaderPipeline>` refcount, `GraphicsDevice::create_raster_pipeline<Arc<CompiledShaderPipeline>>`, Box the 0x38-byte `RasterPipeline`, store into entry.
  3. Releases lock; returns handle. O.
  This is the variant→pipeline cache that maps the two 16-hex variant compile products to runtime pipelines.

## Typed gaps
- Exact `RasterPipelineDesc` +0x18 field meaning (feature/binding selector int).
- `RasterShaderProgram::desc/recompile` bodies.
- `ComputeShaderProgram` field-level layout beyond the above.
