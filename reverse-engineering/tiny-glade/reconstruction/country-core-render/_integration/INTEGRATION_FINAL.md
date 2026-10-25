# Country-core — INTEGRATION FINAL (wp17–wp21 → `country-core`)

Target crate: `reconstruction/country-core/` — package `country-core`, edition 2024,
rust-version 1.86, standalone (not in the reconstruction workspace). This final batch
merges the five country-core replay / draw-shaders / stylized / camera-UI-time /
misc-resources work packages into the assembled render-domain crate, following the
INTEGRATION_BATCH4.md pattern.

## Status

- `cargo check --offline` — **PASSES** (184 warnings, 0 errors; all warnings are
  skeleton markers / unused).
- `cargo clippy --all-targets --offline` — **PASSES** (383 warnings, 0 errors; clippy
  style lints only).
- No remaining compile errors. 570 `.rs` files (was 428). wp17–wp21 directories
  untouched (0 edits; all changes live under `country-core/src/`).

## Module tree delta (merged in this batch)

```
src/systems/history/          wp17: finish_replay/replay_mode/replay_pop_bucket/
                              signifier_replay_active/replay_wall/replay_terrain/
                              replay_auto_clutter/replay_stairs/replay_raster/
                              replay_path/replay_water/replay_garden (12 modules);
                              mod.rs keeps wp13 add_history_* + gains the
                              replay-domain AppMode typed-gap boundary
src/resources/history/        wp17: auto_clutter_edit/color_edit/decorator_edit/
                              path_edit/stairs_history_edit/terrain_edit/tree_edit/
                              wall_history/replay_cmd_buffer (9 new) +
                              wall_compound_edit (file replaces the inline WP12
                              facade submodule)
src/systems/render_loop/
  draw_passes/draw_shaders/   wp18: 14 per-shader raster modules
                              (billboard_forest/clearing_shaders/clouds/
                              common_includes/draw_bricks/floor_shader/grass_shader/
                              instanced_critters/instanced_ivy/leaf_particle_draw/
                              path_pebble_shader/rocky_terrain_draw/stairs_draw/stars)
src/resources/{color_id_roof,texture_presets}.rs  wp18 carriers; critters.rs extended
src/systems/render_loop/      wp19: painterly/vermeer/menu_background/lut_gallery/
                              indirect_params + session_selection/ + resources/
src/resources/{gpu_lut,grading_lut,loading_screen,logo_animation}.rs  wp19
src/systems/backend/input/    wp20: 7 units (calculate_input_delta/input_record_replay/
                              input_replay_for_trailers/reset_input_state/
                              translate_input/translate_input_to_cursor_moved/
                              update_cursor_position)
src/systems/camera_rig/       wp12 facade file → wp20 directory (CameraBody trait +
                              Free/Normal/Walk/Butterfly + MainCamera 0x4e0 resource);
                              `crate::systems::camera_rig::MainCamera` still resolves
                              (ui_adjust_wall_height_mode uses Res<MainCamera>)
src/systems/                  wp20: main_camera/look_at_camera/time_of_day/time_update/
                              weather_system/world_raster/timelapse_replay/mode_manager/
                              prefab + ui_2d_follow_mouse/ui_camera_mode_gizmos/ui_clutter/
                              ui_edit_*/ui_move_*/ui_focus_*/ui_raise_shape +
                              arrow_gizmo_animation/brush_preview/clear_canvas/despawn/
                              mesh_follow_terrain/mod_key/utils/utils_intersect;
                              mod.rs gains GameSystemSet
src/resources/walls/          wp21: 9 residual modules (arches/dormers/wall_holes/
                              wall_stone_columns/wooden_gates/decorator_collision/
                              decorator_outline/decorator_subtype/elevation_support_stats)
src/resources/stairs/         wp21: stairs_state/stairs_assembly_params/stairs_collider/
                              structural_graph/stairs_removed_supports/invalidate +
                              local ColliderHandle boundary in mod.rs
src/resources/ivy/            wp16 ivy.rs split → ivy/ (ivy_storage/ivy_leaf_storage/
                              ivy_direction_proposer/wall_coord); WP21 IvyLeaf 0x68B
                              layout + PublicWallCurve grafted
src/resources/rt/             wp16 rt.rs split → rt/mod.rs; WP21 update_rt_world /
                              upload_rt_world + RtBvhBuilderOutput + idle flag grafted
src/resources/                wp21 misc: clutter_meshes/color_icon/daily/
                              feedback_report_resources/garden_mask/
                              glade_color_customization/glade_settings/
                              gothic_window_profile/image_formation/mouse_capture/
                              photo_mode/placement_data/preplaced_autoclutter/
                              raster_stroke/shader_refactoring_state/shape_intersections/
                              shutdown/sparkle/steam/time_of_day/toolbar/ui_settings/weather
                              + render/spatial_resolve_offsets
src/resources/stairs_walls.rs wp21 recovered container (replaces WP12 facade)
src/render/nani/mod.rs        wp21 GpuDrivenDrawable grafted
src/systems/save/             wp21 save_backup_metronome
src/systems/garden/           wp21 draw_garden/record_garden_history
```

## Facade → real swaps performed

- **wp17 `systems/history/boundary.rs` dropped.** `HistoryFacade` → real
  `resources::history::History`; `HistoryCmdFacade` → real
  `systems::history::history_process::HistoryCmd`; `PendingHistoryCommandsFacade` →
  real `systems::history::history_process::PendingHistoryCommands`; `WallId` →
  `resources::walls::WallId`; `DecoratorId` → `resources::walls::decorator::DecoratorId`;
  `StairGraphEdge` → `resources::stairs::stairs_state::StairGraphEdge`. **Kept as a
  boundary (documented):** the replay-domain `AppMode` (`{Unknown,Replay,Other(u32)}`,
  REPLAY_TAG 0xd) — wp20 `mode_manager::AppMode` (None/Timelapse/Wall/Focus/Tool, serde
  2-qword) has no Replay discriminant or `Other(u32)`; merging would fabricate a variant
  set, so the replay `AppMode` lives at `systems::history::AppMode`.
- **wp18 `country_core::…` → `crate::…`** in all 14 draw_shaders + carriers; the shared
  DrawBuilder/NaniDrawExt/SSBO/FrameConstants paths already exist in the crate.
- **wp19 `consume::core` extended** with real re-exports for the wp19-consumed shared
  types: `AudioCore`, `GradingLuts`, `LutGalleryState`, `LogoAnimation`, `SavedSession`,
  `rhapsody_contract::Texture`, `utils::timer::FilteredWallTime` (new typed-gap
  `utils::timer` module). `AsComputeShaderSource::as_compute_pipeline` impls re-pointed
  to the real `ComputeShaderSource<'_>` return type.
- **wp20 facade marker files not copied** (`src/{app_loading_state,asset_libraries,
  game_config,input,meta_commander,resources,reused,systems/backend,systems/ui,utils}.rs`):
  their surfaces already exist in the crate. The real `reused.rs` was extended with the
  wp20 bevy-family contract modules (bevy_app/bevy_state/bevy_input/bevy_time/egui/
  enum-iterator/append-only-vec/puffin/suncalc/time/hexasphere). `UiAppendSignifier`
  added to `systems::ui::signifiers`.
- **wp21 `boundary.rs` not copied.** `crate::boundary::{WallId,DecoratorId,Aabb2,
  RaycastMeshData,StairsVisualState}` → real types. **Kept as a boundary (documented):**
  `ColliderHandle` (`{Empty,Handle(u64)}`, EMPTY=0x11) — the real
  `collision::world::SubworldColliderHandle` is `{subworld,index}` without an Empty
  discriminant; the stairs `GraphMap` node key keeps the local enum at
  `resources::stairs::ColliderHandle`.

## Name-collision / extension resolutions

- **`wall_compound_edit`**: wp17 file (WallHistoryEdit/ConcurrentEdits/CompoundEditBuilder
  with real methods) replaces the WP12 inline facade submodule; wp12's 3
  `WallHistoryCompoundEdit::new()` call sites adapted to `new(ConcurrentEdits::default())`.
- **`texture_presets`**: wp18 tuple shapes (`TexturePreset(pub u8)`/`TextureFromDisk(pub u64)`)
  win for the draw path; wp21's `From<u8>` mapping + `usage()` grafted onto the newtype.
  wp21's `TextureFromDisk` Event form dropped (conflicts).
- **ivy/rt subpaths**: wp16 assembled `ivy.rs`/`rt.rs` are authoritative (consumed by
  `systems::ivy`, draw_shaders); split to `ivy/` + `rt/` subpaths. wp21's subpath files
  (ivy_storage/ivy_leaf_storage/ivy_direction_proposer/wall_coord, rt/bvh, rt/bvh_builder)
  duplicate wp16 types with conflicting signatures — only the genuinely-new recovery was
  grafted (IvyLeaf 0x68B layout, PublicWallCurve, update/upload_rt_world,
  RtBvhBuilderOutput, idle flag).
- **prefab**: wp13 `resources::prefab/mod.rs` authoritative; wp21's prefab.rs duplicates
  (PrefabName String form) — only `PrefabInstanceData` (wp20 `update_instance_data`
  consumer) grafted in.
- **stairs_walls**: wp21 recovered `StairsPillarProposals` container replaces the WP12
  `{raw:Gap}` facade (ui_edit_rectangle_orientation keeps `ResMut<StairsPillarProposals>`).
- **StairsVisualState** gained `remove_edge` (wp21 invalidate surface).
- **`Gap` now derives bevy `Resource`/`Component`/`Event`** (lib.rs) — required by the
  ~40 `Res<Gap>`/`Query<…Gap…>`/`EventWriter<Gap>` typed-gap system params in wp20's
  camera/UI/time systems (wp20's standalone gate used the same derives; the batch4 doc's
  "stays unadorned" predates wp20). Inert otherwise; placeholders documented as gaps.
- **Existing types gained the trait impls wp17/wp20 system params require**
  (derive additions, not shape changes): `WallId`/`DecoratorId`/`StairGraphEdge` +
  serde; `DecoratorId` + PartialOrd/Ord (petgraph GraphMap node); `WorldRaster` →
  `WorldRaster<const N: usize>` (wp20 `WorldRaster<512>`); `InputRecordReplayState`,
  `ActionSetStates`/`ActionSetValues`, `PlacementDataSsbo`, `AudioCoreInner`,
  `systems::ui::resources::ui_state::UiState`, `AllWorldRasterWrite` + Resource;
  `PrefabInstance`/`PrefabInstanceData` + Component; `Aabb2` → recovered `{min,max}`
  16B shape + `from_center_dims`/`intersect_ray` + `RayBoxIntersection` (wp21
  wall_holes / wp20 utils_intersect).

## New dependencies (Cargo.toml)

- `dolly = "=0.6.0"` (wp20 camera rig; the game's locked camera-rig dep).
- `petgraph = "=0.8.3"`, `indexmap = "=2.7.1"`, `foldhash = "=0.1.4"` (wp21 stairs
  structural graph / ordered maps; original-lock baseline).
- `glam` gained the `mint` feature (dolly 0.6.0 converts through mint; both glam 0.29.3
  direct and dolly's transitive glam 0.28 are in the lock).
- All resolve in the offline registry cache. steamworks / bracket-noise / obvhs /
  parry3d / perchance remain UNAVAILABLE offline → kept as typed-gap / `reused` contract
  surfaces (`rng`, `noise`, `bvh`, `rapier`), NOT re-versioned.

## Remaining errors / warnings (categorized)

**0 compile errors** in both gates. 184 check warnings / 383 clippy warnings, all
skeleton markers: unused imports/fields/functions, `todo!()`/`unimplemented!()` bodies,
`non_camel_case_types`, clippy style lints (needless_range_loop, redundant_closure,
needless_lifetimes …). No error category remains.

## Exact tails

```
$ cargo check --offline
warning: `country-core` (lib) generated 184 warnings (run `cargo fix --lib -p country-core` to apply 149 suggestions)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.28s

$ cargo clippy --all-targets --offline
warning: `country-core` (lib test) generated 383 warnings (run `cargo clippy --fix --lib -p country-core --tests -- ` to apply 210 suggestions)
warning: `country-core` (lib) generated 383 warnings (383 duplicates)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.35s
```

## Biggest remaining blocker

The wp20 camera/UI/time systems were recovered against a `Gap`-derives-
`Resource/Component/Event` placeholder surface and numerous thin typed-gap boundaries
(replay-domain `AppMode`, stairs `ColliderHandle`, `MetaCommanderBoundary`,
`ToolbarBoundary`, `MainCameraBoundary`…). Reconciling each placeholder to the real
recovered type — especially wiring wp20's `main_camera_update` to the real
`CameraMatrices`/`FrameConstants` write path, wp20's camera bodies to the rapier
collision glue, and the replay-domain `AppMode` to the authoritative mode FSM — is the
next integration pass once the runtime surfaces are finalised. The crate is a
skeleton-aligned surface closure, not original-behaviour equivalence.
