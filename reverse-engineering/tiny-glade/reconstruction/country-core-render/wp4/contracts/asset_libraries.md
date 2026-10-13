# WP4 Contract: `country_core::asset_libraries` (render-facing surface)

Decompiles: `_analysis/decomp/C_0x14*.c`, `D_0x14*.c`, `E_0x14*.c`, plus `evidence/building-gates/B-mesh-helpers/*.c`.

## `asset_libraries::mod` (source `b86638af…`, 3 procs)

- `Asset { name: String, … }`, `Handle<T> { id: HandleId }`, `HandleId::new/from_raw`, `Handle::new/is_valid` — core id types (O names; `add` uses a global atomic `HANDLE_COUNT` counter → `HandleId`). `impl Asset::name()`. Layouts I.

## `shader_library` (source `16e9fe00…`, 6 procs, modules 485/503/509/1655)

`AssetShaderLibrary` (O from `new`/`add_raster`/`get_compute_by_name`):
- `raster_shaders: FrozenMap<Handle<RasterShaderProgram>, Box<RasterShaderProgram>>` (≈+0x00)
- `compute_shaders: FrozenMap<Handle<ComputeShaderProgram>, Box<ComputeShaderProgram>>` (≈+0x38)
- `raster_names: RwLock<HashMap<String, Handle<RasterShaderProgram>>>` (lock +0x80, buckets +0x88, mask +0x90, items +0xa0) — O
- `compute_names: RwLock<HashMap<ComputeShaderSourceHash, Handle<ComputeShaderProgram>>>` keyed by source hash, `NoHashHasher` (lock +0xb8, buckets +0xc0, mask +0xc8, items +0xd8) — O
  (New() also seeds several `&'static` empty FrozenMap arrays + RandomState; exact container offsets Inferred.)

- `new()` (0x140936c60): seeds RandomStates + empty FrozenMaps. O.
- `add_raster(name: &str, program: RasterShaderProgram)` (0x140936ef0): under lock, if name present in `raster_names` → dedup: compare existing program source (fields +0x8/+0x10, +0x38/+0x40) + version (+0x28); identical → return; differing → panic `"AssetShaderLibrary: the shader …"`. Else new HandleId, insert name→handle, Box the 0xd8-byte program into FrozenMap. O.
- `add_compute(source: ComputeShaderSourceHash, program: ComputeShaderProgram)` (0x140937560): under lock, duplicate hash → panic `"the shader … already in library"`; else new HandleId, insert hash→handle into `compute_names` (NoHashHasher), Box 0xa0 program into FrozenMap. O.
- `get_compute_by_name(&self, hash: ComputeShaderSourceHash) -> Handle<ComputeShaderProgram>` (0x1409378e0): shared-lock lookup by u64 hash in `compute_names`; hit → `FrozenMap::get` → handle; miss → returns default handle (U what; caller `ComputePass::new` treats it as cache lookup). O.
- `get_raster_handle_by_name(&self, name: &str) -> Handle<RasterShaderProgram>` (0x140937aa0): shared-lock string lookup; returns handle (miss → default). O.
- `debug_raster_get_name(&self, handle) -> Option<&str>` (0x140937cb0): iterate raster FrozenMap entries; returns the source name for handle. O.
- `get_compute`/`get_raster`/`get_raster_mut`/`get_raster_by_name`/`get_compute_handle_by_name`/`extract_raster_shader_names`/`get_compute_shader_name` — PDB names only, no standalone procs (I: thin wrappers; typed gaps).
- `get_rg_buffer_by_name` is called by render pass code (INDEX.md), implemented via `get_raster_handle_by_name` + `get_raster` (I).

## `ssbo_library` (source `d783c195…`, 336 procedures, heavily monomorphized)

- `SsboId` — u64 type id (type-name hash), keyed in `FrozenMap` with `NoHashHasher`.
- `BufferState` — 0x38+ bytes (O): +0x00 buffer handle/ptr, +0x08 size, +0x10 data ptr, +0x18 len, +0x20 RwLock state, +0x28/+0x30 RG handle fields, +0x36 state byte (5 = exported-to-CPU).
  `BufferState::new(ptr, size, data, len)` (0x14091eb30, 36B). O.
- `AssetSsboLibrary` (O): `FrozenMap<SsboId, Box<BufferState>>` + storage-buffer descriptor map (per-T vtable: +0x18 = get ptr, +0x20 = get_mut, +0x30 = get value; name string + length for assert).
- `Default` (0x14091eb60): empty FrozenMap + name. O.
- `add_new<T>()` (0x1412bc470 rep): `render::ssbo::new_storage_buffer<T>()` (game-owned, wraps rhapsody buffer), build `BufferState`, insert `FrozenMap<SsboId, Box<BufferState>>`; collision → if same type-name panic `"Tried to register the same storage buffer …"` else `"…two storage buffers with the same id"`. Debug label e.g. `"Firefly_Vis_Buffer"`. O.
- `add_new_with_data<T>(…)` (0x1412badf0 rep): same via `new_storage_buffer<T>` (data variant), label e.g. `"NaniNextVisibilityBitmaskSsbo"`. O.
- `export_from_rg(&mut self)` (0x14091ebb0): for each buffer, lock +0x20; if state != 5 → `rhapsody::rg::import_export::export()`, set state 5. O.
- `get_rg_buffer<T>() -> Handle<…>` (0x1402b31a0 rep): FrozenMap lookup by SsboId (miss → panic); if state==5 → re-`import_unowned()`, refresh +0x28/+0x30/+0x38, return handle; else return current +0x28/+0x30/+0x38. O. (This is the render-facing entry used by `pass_resource_access::*`.)
- `get<T>()`/`get_mut<T>()`/`get_dyn`/`get_mut_dyn` — FrozenMap lookup + vtable call (0x1412098a0 rep: assert type-name, call +0x20). O. `get_mut` also records handle in a pending list (O; semantics I).
- `add_existing`, `get`/`get_dyn` variants, `StorageBufferType::default_value` — PDB names; typed gaps.

## `mesh_library` (source `312b9e63…`, 12 procs)

`AssetMeshLibrary` (O): `FrozenMap<Handle<Mesh>, Box<Mesh>>` (primary, ≈+0x00) + hashbrown table for transient meshes (remove path), `RwLock<HashMap<String, Handle<Mesh>>>` name map (lock +0x60, buckets +0x68, mask +0x70, items +0x80), `RwLock<HashMap<PathBuf, Handle<Mesh>>>` source-path map (lock +0x98, buckets +0xa0, mask +0xa8, items +0xb8), +0x28 disposed flag.

- `new()` (0x14092c920): seeds RandomStates, empty maps, sentinel box. O.
- `add(name: String, mesh: Mesh) -> Handle<Mesh>` (0x14092cc60): new HandleId (global counter), Box the 120-byte Mesh into FrozenMap, insert name→handle, insert source-path→handle. O.
- `get(handle) -> &Mesh` (0x14092d2e0): FrozenMap::get. O.
- `get_mut(handle) -> &mut Mesh` (0x14092d300): push handle to pending list, FrozenMap lookup (O; container interior-mutability approach U).
- `get_by_name(name) -> &Mesh` (0x14092d450): try_get_handle_by_name → FrozenMap::get; miss → panic (`"… not found"`).
- `get_handle_by_name(name) -> Handle<Mesh>` (0x14092d4f0): try_get → unwrap; miss → panic.
- `try_get_handle_by_name(name) -> Option<Handle<Mesh>>` (0x14092d570): shared-lock string lookup. O.
- `remove(handle)` (0x14092d750): if disposed flag → unwrap; remove from transient table (drop Box<Mesh>), `retain` name map to drop stale name entries. O.
- `reload_mesh_src(&self, path: &Path)` (0x14092d8d0): source-path map lookup → `get_mut`; if extension == `"json"` → `utils::load_json::load_json_as_mesh<&str>` and replace Mesh contents (log error on failure); other extensions → assert-fail. O.
- `transient_meshes` — bevy system wrapper (`InfallibleSystemWrapper::component_access`, 0x140002450): ECS resource system hook, not a library method. O.
- `is_dirty` — PDB name; typed gap (likely reload watcher state).

## `texture_library` (source `c5db910c…`, 13 procs, modules 510/559/562/595/1610/3098)

`AssetTextureLibrary` (O): `Vec<NamedTexture>` textures (+0x00, stride 0x38), name map `HashMap<utils::name_hash::NameHash, NamedTexture>` (buckets +0x30, mask +0x38, items +0x48, NoHashHasher), `bindless_textures_dset` persistent descriptor set (+0x18), futex RwLock +0x80, dirty flag +0x88, pending-handle Vec +0x90/+0x98/+0xa0, bindless index table +0xb0/+0xb8, count +0xc0.

- `new()` (0x14093bb10): allocate `"bindless_textures"` name, `GraphicsDevice::create_persistent_descriptor_set()`, seed empty maps. O.
- `add(&mut self, NamedTexture)` (0x14093c200): duplicate name → panic `"the name is already in the texture library"`; else push to textures Vec, insert NameHash→NamedTexture map, debug-log `"Loaded texture as …"`. O.
- `create_name_mapping(name, texture)` (0x14093c720): registers preset name→texture; duplicate → panic `"Onoz, already exists…"`; missing target → panic `"Was asked to map to …"`. O.
- `get_rg_texture(&self, handle) -> Texture` (0x14093cf30): texture map lookup (stride 0x68) by handle; missing → expect-fail. O.
- `handle_to_bindless_index(&mut self, handle) -> Option<DescriptorIndex>` (0x14093d310): if handle already indexed → return; else if count ≥ 0x1fdf(8191) → panic (bindless limit); push handle to pending Vec, mark dirty (+0x88=1), allocate index. O.
- `bindless_textures_dset(&mut self)` (0x14093d040): if dirty → unwrap-fail; drain pending handles, `bumpalo::Bump` temp arena, `GraphicsDevice::update_persistent_descriptor_set<Map<Drain<AssetIndex>, closure>>`, free arena. O. (This flushes bindless table into the descriptor set — matches `bindless_textures_dyn@2.0` variant note in INDEX.md.)
- `create_preset_textures` (0x14093bd80), `clear_preset_texture` (0x14093bee0), `import_texture_presets_to_rg` (0x14093cd30), `export_preset_textures_from_rg` (0x14093ce30) — preset texture lifecycle; import/export use `rhapsody::rg::import_export`. O at call-graph level; details I.
- `get_by_name(name)` (0x14093ca90), `get_handle_and_texture_by_name` (0x14093c970), `get_handle_by_name`, `get_mut`, `get_mut_by_name`, `update_bindless_texture_table` — name lookups + bindless refresh (I).
- `DescriptorIndex` — u32 bindless slot; `allocate()` (O name; U body).
- `create_name_mapping`/`clear_preset_texture`/`update_bindless_texture_table` internal bits U.

## `color_library` (source `f8a54c0d…`, 6 procs, modules 510/1600/1618)

`ColorLibrarySsbo` — 0x250 = 592 bytes = array of Vec4 color entries (O: `deserialize_from` writes at 16-byte stride; `update_color_library` memcpy 0x250). Getters (`grass_1`, `terrain_1`, `sky_base`, `roof_color_1`, … 40 symbols) = array index accessors (I).

- `deserialize_from<R>(&mut self, r: &mut R) -> Result<…, R::Error>` (0x14093e510, 2522B): RON deserializer mapping string field names to Vec4 slots by byte-compare: `"path"`→+0x50, `"wall_1"/"wall_2"`→+0x70/+0x74, `"grass_1"/"grass_2"`→+0x48/+0x4c, `"sky_base"`→+0x54, `"terrain_1/terrain_2/wall_moss/wall_bricks/terrain_highlight/night_tint"`→+0x64/…, `"ivy_branch"/"ivy_leaf_1"`→+0x80/+0x84, … unknown name → panic `"Init Color Library failed: entry is missing from ColorLibrarySSBO"`. O.
- `deserialize_from_path(&self, path)` (0x14155a170/0x14155a320): open RON file + `deserialize_from`. O (call graph).
- `update_color_library` bevy system (0x14093b530, 1214B): default white (1.0) library; try `deserialize_from_path` (missing file → keep defaults); locate `ColorLibrarySsbo` in `AssetSsboLibrary` (type-name assert 0x3e chars); vtable +0x18 → data ptr; `GraphicsDevice::update_buffer_bytes` upload 0x250 bytes; error → fallback CPU copy. O.

## Typed gaps
- Non-render consumer surface of all four libraries (meshwatch/vao/placement-data/configwatch/audio-trigger libraries are NOT implemented — deferred).
- `SsboId` exact hash function; `StorageBufferType` enum; `BufferState` +0x36 other states.
- `NamedTexture` full field layout; bindless table base-index policy; `DescriptorIndex::allocate` body.
- `AvailableGraphicsDevice` (160 B) field layout.
