# WP15 Contract — `geometry` + `utils` (game-owned geometry/util helpers)

Source: `skeleton/B/crates/country-core/src/geometry/{cube,instanced_wall,plane,sphere}.rs` and `utils/{mod,audio_utils,child_wall,debug_viz,dirty_tiles,geo,load_json,project_cursor_onto_ray}.rs`
Evidence: Ghidra decompiles in `analysis/b8_geom/`, `b9_utils/`.

> Reconcile: `utils::components::transform::Transform` (40 B `{translation,rotation,scale}`) is **owned by wp6** — not redefined here. `resources::mouse_terrain_raycast::CameraCursorRay` (32 B) is **owned by wp8** — not redefined. This package only defines geometry/util helpers wp6/wp8 do not own.

## Observed (O)
- **`DirtyTiles`** = 0x200 B = 64×64 bit grid (`[u64; 64]`); `clear`/`new_clear` @0x140960c20/@0x140960ac0 (memset 0x200), `extend` @0x140960c30 (OR merge), `cell_from_world_space` @0x140960b20 (fmaf world→cell scale; valid when both coords < 0x40), `rasterize_world_space_rect` @0x140960ae0 / `intersects_world_space_rect` @0x140960af0 (via `iter_world_space_rect_cells` @0x140ade8e0), `rasterize_world_space_stroke` @0x140ade620 (28 callers; `conservative_line_raster::conservative_plot_line`), `iter_tiles` @0x140adeea0, `retain` @0x140adfcb0; `dilate`/`bitmap_set`/`bitmap_intersects`/`is_set` inline.
- **`ChildWall`** = 16 B `{wall_id u64@0, t0 f32@8, t1 f32@0xc}`; `slice` @0x140a24000 = two `utils::lerp` + **[0,1] panic**.
- **`AudioLoop`** = `{instances: Vec<AudioInstance> (stride 0x30), last-stop Instant@0x18, marker u32@0x20 (=1000000000), current_index@0x28, cooldown_ms@0x30, is_playing@0x34, has_parameter@0x35, bool@0x36}`; `current` @0x140a593f0 (OOB panic), `start_if_not_playing` @0x140a596b0 (fmod `EventInstance::start`), `stop` @0x140a59430 (cooldown-gated stop + advance index modulo count); `AudioLoopDesc` cooldown @+0x18 (`AudioCore::instantiate_loop` @0x140a59220).
- **`geometry::instanced_wall`**: `InstancedWall` = 48 B `{id u64, brick_transforms Vec<BrickTransformSsbo>, writer Option, storage Option}`; `new` @0x140858800, `upload_to_storage_buffer` @0x140858880 (sparse-writer clear + push per **0x60 B** `BrickTransformSsbo`); `BrickSourceId::from_wall_id` @0x140858750 / `with_transient_stair_edge_id` @0x140858780; `WallBrickInstancedDrawProxy` `{kind u16, sub u8}` (from `glade_startup_walls`); SSBO families + flag constants `DISCARD_ABOVE/FORCE_LOD0/LESS_SNOWY/NO_ERASER_HIGHLIGHT/ROOF_STITCH/SLIGHT_OVERLAP/STEPS`.
- **`geometry::{cube,plane,sphere}`** → `Mesh` conversions: `Box` (AABB two corners) `From<Box>` @0x140a05530, `Plane From` @0x140a06370, `Sphere TryFrom` @0x140a06b00 (Vec3A vertex iteration, Result); `Sphere::new` @0x140b4af60. Procedural vertex data I.
- **`utils::geo`**: `mesh_from_pts` @0x140a6c120 (positions `set_attribute<Vec<[f32;3]>>` + identity indices 0..n + transient name-hash `anon_cc634b…` + thread-local instance counter); `update_curve_mesh<It>` @0x14157eac0; `rewire_enclosed_curve` @0x140a6ba50.
- **`utils::load_json`**: `read_attribute_from_json` @0x14083e210 (numeric attr; array shapes Vec<f32>, Vec<i32>, [f32;2], [f32;3], [f32;4]); `load_json_as_mesh<&str>` @0x140b0eeb0 (68 callers; PathBuf/String/&PathBuf instantiations).
- **`utils::project_cursor_onto_ray`**: `ProportionalScreenSpace` = 16 B `{w,h,aspect=w/h,1.0}` (`from_window_size` @0x140ac8420); `transform_screen_space` @0x140ac8450 (`utils::mouse_ss_to_cs`); `ProportionalScreenSpaceCoord::project_onto_ray` @0x140ac84a0 (two-depth 4×4 camera transform + perspective divide + lerp + **ray-param min clamp 1e-10**).
- **`utils::debug_viz`**: `DebugLine`/`DebugSphere`/`DebugRectangle2d/3d`/`DebugColor`/`DebugTransform` builders (`new`/`color`/`line_width`/`dashed`/`solid`/`set_ui_pass`/`show` @0x140b02490..0x140b04f70); rendered through the `_dashed_line` UI draw pass family.
- **`utils/mod.rs`**: `get_res`/`get_res_mut` (inline into rhapsody compile / render loop), `drain_events<E>` @0x1402b47f0 / `get_last_event<E>` @0x1402b4910 (bevy `Events<E>` buffer drain; returns all / last), `world_space_to_screen_space` @0x140ac8780 (typed gap), `is_inside_map_bounds` (inline), `convert_function_expr_to_title`; `calculate_hash` (O call site of `WidgetId::from_name`).

## Inferred (I)
- `Mesh` placeholder here (owner re-points to wp4 `render::mesh::Mesh`).
- `grid_gen_*` corner order; `AudioInstance`/`AudioLoopDesc` full layout; `BrickTransformSsbo` internal packing (stride 0x60 O).

## Unknown (U / typed gaps)
- `calculate_hash` exact algorithm (DefaultHasher marked I — integration must confirm so `WidgetId` cross-matching holds).
- `DebugLine/Sphere/Rectangle` layouts; `keycode_to_str`/`mouse_button_to_string` tables; `world_space_to_screen_space` body; `Triangle`-family AABB/intersect helpers in `utils::geometry`.

Reused deps: glam, serde_json, bevy_ecs (events), fmod/libfmod (audio instance), `conservative_line_raster` (stroke raster), puffin. No re-implementation of these.
