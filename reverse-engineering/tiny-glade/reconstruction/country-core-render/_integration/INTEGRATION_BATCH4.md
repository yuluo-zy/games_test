# Country-core — INTEGRATION BATCH4 (wp11–wp16 → `country-core`)

Target crate: `reconstruction/country-core/` — package `country-core`, edition 2024,
rust-version 1.86, standalone (not in the reconstruction workspace). Batch-4 merges the
six country-core content/app-shell work packages into the assembled render-domain crate.

## Status

- `cargo check --offline` — **PASSES** (176 warnings, all skeleton-markers / unused).
- `cargo clippy --all-targets --offline` — **PASSES** (331 warnings, 0 errors; all clippy
  lints such as `needless_range_loop` / `redundant_closure`).
- No remaining compile errors. 428 `.rs` files (was 157). wp11–wp16 directories untouched
  (all edits live under `country-core/src/`).

## Module tree delta (merged in this batch)

```
src/lib.rs                  + root mods: app_loading_state, audio_events, boundary,
                             game_config, game_key_bindings, game_profile, geometry,
                             input, meta_commander, reused, startup, ui; utils/resources
                             now FILE modules; Gap gained serde derives; typed_gap gained
                             GapEventPlaceholder; puffin_contract gained ProfilerScope/
                             are_scopes_on
src/app_loading_state.rs    wp15
src/audio_events.rs         wp14 (16×register_event rows; RequestWallMaterialAudio merged
                             to struct {wall,pos,raw} for wp12 construction)
src/boundary.rs             wp16 (kept as typed-gap boundary — see below)
src/game_{config,key_bindings,profile}.rs  wp15
src/meta_commander.rs       wp15
src/reused.rs               wp16 reused-dep call surfaces (contract module)
src/ui.rs                   wp12 wall-domain UI typed-gap boundary (AppMode / Modifiers /
                             bool-predicate UiState / builder signifiers / bevy_app App)
src/asset_libraries/        + audio_trigger_library (wp14), configwatch/meshwatch/
                             placement_data_library/texture_watch/vao_library (wp16);
                             Handle<T> derived Component (wp12 Query<&Handle<Mesh>>)
src/components/             + geometry/transient_mesh/utils (wp16), ~38 residual markers
                             (TestCube, SnappingTo*Preview, Tree*, Ui*, …)
src/geometry/               wp15 (cube/instanced_wall/plane/sphere); InstancedWall =
                             Component (wp12 gpu_flush query)
src/input/                  wp15 (action_set/input_stream/mod/panic_repro_state)
src/startup/                wp15 (~30 modules) with wp14 load_audio/load_glade_audio
                             overriding wp15's thinner duplicates
src/systems/audio/          wp14 (31 modules; wp14 systems/wall/snapping/snapping_audio
                             dropped — duplicate of wp12's systems::wall::snapping)
src/systems/wall/           wp12 (57 units)
src/systems/{backend,collision,debug,steam,ivy}/  wp16
src/systems/history/, save/ wp13
src/systems/ui/             wp15 real UI domain (replaces the old systems/ui.rs GlobalId gap;
                             GlobalId recovered as u64 name-hash)
src/systems/camera_rig.rs   wp12 facade (MainCamera)
src/resources/{session,prefab,history}/  wp13 (authoritative; replaces wp7 session.rs slice)
src/resources/{audio_resources,audio_settings}.rs  wp14
src/resources/{ivy,rt}.rs   wp16
src/resources/walls/        wp11 state types + wp12 facade method surface + facade-only
                             modules (decorator_storage, decorator_archivist,
                             cached_decorator_transforms, wall_trigger_volume, material_stats)
src/resources/{events,gui,in_progress_curve,drawing_input_smoother,glade_border,
             editing_highlight,read_mouse_depth_value,onboarding,stairs_walls,time,window}.rs
                             wp12 typed-gap facade families
src/utils/                  index_condenser/float_grid/components (kept) + wp15 helpers
                             (child_wall, debug_viz, dirty_tiles, geo, load_json,
                             project_cursor_onto_ray, free fns) + wp12 curve/geometry/
                             swizzlers/comparative_space/rdp/lerp + wp14 audio_utils
```

## Facade → real swaps performed

- **wp12 `crate::audio::*` → `crate::audio_events::*`** (RequestWallMaterialAudio,
  PlayBuildingAudio, PlayWallUiSound) across 6 wall-system files; event shapes merged.
- **wp12 `crate::render::Handle` → `crate::asset_libraries::Handle`** (4 signifier files);
  Handle gained `Component` derive (O: entities carry `Handle<Mesh>`).
- **wp12 `crate::render::{mesh::Mesh, GraphicsDeviceResource, geometry::instanced_wall::*}`
  → real** `render::mesh::Mesh`, `render::GraphicsDeviceResource` (gained `Resource`),
  `geometry::instanced_wall::{InstancedWall, BrickTransformSsbo}` (InstancedWall gained
  `Component`). `upload_instanced_wall_ssbo.rs` rewritten to the real surface
  (typed-gap body; SparseSsbo/AssetSsboLibrary SsboId wiring recorded as gap).
- **wp12 `crate::utils::{components::transform, debug_viz, child_wall, project_cursor_onto_ray}`
  → real wp15 types**; `crate::utils::curve/geometry/swizzlers/rdp/comparative_space`
  kept as wp12 facade modules (genuinely not recovered elsewhere in the crate).
- **wp12 `crate::resources::{cursor_terrain_raycast, terrain_height, mouse_terrain_raycast,
  roofs, history, session}` → real** wp8/wp11/wp13 types; call sites adapted
  (see Collision resolutions). wp13 `History` extended with `mark_changed()` (wall-domain
  marker) + `wall_compound_edit` submodule; `deferred_wall_history` re-wired to
  `History::add(EditType)`.
- **wp16 `boundary.rs` kept as typed-gap boundary** (documented): the wp4/wp2 recovered
  shapes don't expose the wp16-required surfaces (`FrameConstants.frame_index`,
  `WorldRaster<512>` generic, `Mesh` construction fields, `Vao` construction shape,
  `SparseInstanceBufferWriter` without `SparseSsbo`); re-pointing would force inventing
  fields on the real types, which the rules forbid. `reused.rs` kept as the reused-dep
  contract surface (rapier/parry, steamworks, hotwatch/notify, bracket-noise, perchance,
  obvhs, tabloid, append-only-vec, conservative-line-raster, frozen/nohash-hasher,
  priority-queue — none available offline).
- **Terrain-local `UiState`/`AppMode`/`Modifiers` facades (wp8) kept** at
  `systems::terrain` — the terrain systems only use them as bare `&`-params; aliasing to
  wp15's real widget `UiState` would break `Clone/Copy/Default` usage and isn't required.

## Name-collision resolutions

- **UiState**: wp15's real `systems::ui::resources::ui_state::UiState` is the canonical
  type; wp12's wall-domain UI surface (`clicked/dragged/clearing_canvas` bool predicates,
  AppMode, Modifiers, builder signifiers) kept as a **distinct `crate::ui` typed-gap
  boundary** — the two are different layers in the game (wall UI facade vs widget state),
  and the wp15 real builder's `position(Vec2)`/`build()` differs fundamentally from the
  wp12 systems' `position(Vec3)`/`spawn()` surface; merging would fabricate a widget API.
- **walls state types**: wp11 authoritative layouts win; wp12 system-facing projections
  grafted as typed-gap fields/methods — `WallState` merged to
  `{raw, height_ty, curve, shape}` (systems construct it), `InnerWalls` gained
  `next_id` + the `update_*`/`get`/`wall_ids`/`new_wall` surface, `PublicWalls` gained
  `wall_entities_map`/`roofs` + `get`/`wall_ids`, `PublicWallState` gained
  `point_count`/`source_curve_length`/`published_curve_points` (+0x18 vs +0x10 offset
  ambiguity documented), `WallHighlight` became a Resource `{wall_entity, wall_id, ty}`,
  `InProgressWallResource` gained `wall: Option<InProgressWall>`.
- **`WallStyle::Plain` (wp12) → `WallStyle::Stone`** (wp11 authoritative variant set;
  deserialize evidence maps "Stone"→0; 2 call sites).
- **`WallShapeSnapshot`** added as the 28-B systems-facing snapshot next to wp11's
  `WallShape` enum (re-exported at `resources::walls::` root along with WallState/WallStyle/
  StructureType for the B-skeleton import surface).
- **GladeType/ActiveSession**: wp13 authoritative (real 1/2/3/4 discriminants); water
  domain code re-points cleanly.
- **`sample_at_world_xz(Vec3)` → `(x, z)`**, **`CameraCursorRay::intersect_with_xz_plane`
  `Option<Vec3>` → `Vec3`**, **`is_circle()` bool → `Option<CircleShape>` predicates**,
  **`ChildWall::new()` → `new(0,0.0,0.0)`**, **`DebugRectangle2d` → `DebugRectangle3d`**,
  **`ResMut<Option<WipRectangle>>` → `Option<ResMut<WipRectangle>>`** — wp12 call sites
  adapted to the real recovered APIs (all bodies typed-gap skeletons).
- **audio_utils / load_audio / load_glade_audio / systems/wall/snapping/snapping_audio**:
  wp14 (audio domain owner) wins over wp15's thinner duplicates / wp12's wall copy.

## New dependencies (Cargo.toml)

- `fastrand = "=1.8.0"` (wp13 HistoryIdentity / wp14), `uuid = { "=1.15.1", features=["v4"] }`
  (wp13 SessionId), `slotmap = "=1.0.6"` (wp14 AudioCore instance pool). All resolve in the
  offline cache and lock to original-baseline versions. steamworks / hotwatch / notify /
  bracket-noise / perchance / obvhs / tabloid / append-only-vec / conservative-line-raster /
  priority-queue / libfmod / lcms2 / rapier3d are NOT added — unavailable offline; served by
  `crate::reused` contract modules.

## Remaining errors / warnings (categorized)

**0 compile errors** in both gates. 176 check warnings / 331 clippy warnings, all
skeleton markers: unused imports/fields/functions, `todo!()`/`unimplemented!()` bodies,
`non_camel_case_types`, clippy style lints. No error category remains.

## Exact tails

```
$ cargo check --offline
warning: `country-core` (lib) generated 176 warnings (run `cargo fix --lib -p country-core` to apply 141 suggestions)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.23s

$ cargo clippy --all-targets --offline
warning: `country-core` (lib) generated 331 warnings (run `cargo clippy --fix --lib -p country-core -- ` to apply 179 suggestions)
warning: `country-core` (lib test) generated 331 warnings (331 duplicates)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
```

## Biggest remaining blocker

The wall systems (wp12) were recovered against a typed-gap UI/state facade surface that
differs structurally from the wp15-recovered real UI and the wp11 state layouts; the
integration keeps `crate::ui` and `crate::boundary` as documented typed-gap boundaries
rather than fabricating merged widget/render APIs. Fully reconciling wp12's systems to
wp15's real `SignifierEntityBuilder` (Vec2-position builder, `build()`→()` vs
Vec3-position builder + `spawn()`→Entity) and wp16's systems to the real
`FrameConstants`/`Mesh`/`SparseInstanceBuffer` APIs is the next integration pass when the
runtime surfaces are finalised.
