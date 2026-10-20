# Current source and original scheduling contract

This is a source audit and candidate delivery. Root alone applies shared-source
changes and runs Cargo. `candidates/manifest.json` records all12 candidates'
source checksum. No candidate file is evidence that the App runs that chain.

## Original registration

Original B `add_wall_systems140493120` installs these tuples. Full instruction
bounds are the `register-140*.asm` files; names are resolved from PDB and each
individual inline function-name length, not adjacent concatenated strings.

- Set19: sixteen systems, `chain_ignore_deferred` in helper140335b30. Exact
  order: ui_adjust_wall_height_signifier_animation, ui_adjust_wall_height_mode,
  adjust_wall_height_impl, do_ui_arch_controls, ui_lintel_controls,
  wall_highlight, publish_public_wall_state, delete_wall, spatial_hash_update,
  spatial_hash_debug, calc_wall_bend, debug_force_update_arches,
  gpu_clear_wall_gate_pieces, sample_wall_path_intersections,
  calculate_path_wall_segmentation, init_wall_height_and_arch_segment_height.
  Registration call140493197; numeric set0x13 at140493246. The no-op
  debug_force_update_arches is an actual B RET, not an unimplemented algorithm.
- Set22: wall_roof_hashmap, process_invalid_roof_notifications,
  split_wall_path_segment_into_platform_stairs, construct_arches,
  construct_lintels, `chain_ignore_deferred` in helper1405389b0. Original
  run_game_thread14059fb63 calls it;14059fb68 stores0x16. Cross-check with
  roof agent's `round12/roof-visual/roof22-native-contract.json`.
- Set28: update_gates_hashmap, construct_gates, construct_fences,
  construct_platform_stairs, place_pillars, construct_walls, construct_floors,
  debug_viz_threshold_curve, structure_outline, `chain_ignore_deferred` in
  helper14033def0. Registration call14049338e; numeric set0x1c at140493393.
- Set44: gpu_flush_brick_buffers, gpu_flush_plank_buffers, **unchained** tuple
  helper1400f50e0. Set0x2c at1404934ce, call1404934d0. These flush the existing
  shared physical sparse owners; they must not allocate a second library.

Do not install `system_wall_constructor::add_wall_constructor_systems`: its
current WallCpuStage order is not this original schedule and includes TODOs.

## Current producer readiness and candidate boundaries

| Owner | Actual current source / candidate | Required resources and dependency |
|---|---|---|
| Publication and original path sampling | `publish_public_wall_state` consumes actual four WorldRaster current buffers and calls `PublicWallState::prepare_paths`; empty legacy `sample_wall_path_intersections` must not run | `InnerWalls`, `WallPublicationTerrain`, actual `PathRaster/Cobblestone1/2/3Raster`, `WallPublicationDirty`, `WallSpatialHash`, `AnimateWallBulldozeBuffer`; emits actual changed/deleted |
| Height caches | `prepare_paths` creates regular/freeform cache from actual prepared positions and initializes independent third array; candidate19 init restores rectangle-only association lookup and panics on a present stale/missing roof component | `PublicWalls`, changed events, `Query<Roof>`; remaining base max_y_gable cache bug is described below |
| Bend | existing `country_core::systems::wall::calc_wall_bend::calc_wall_bend` reads terrain-mapped outline | `PublicWalls`, changed events, `Events<WallBendChangedEvent>`; original set19 after spatial hash |
| Raw masks | core `wall_path_segmentation` is the recovered full algorithm; candidate wrapper adds real stats init/count, fixes special byte294→296, fails at missing public input instead of silently skipping | changed events, mutable `PublicWalls`, mutable `WallPathSegmentationMasks`, `UiSignifierStream`; runs19 before height init |
| Processed masks and steps | candidate complete wrapper invokes existing `split_platform_wall_masks` and writes separate two maps | changed/create/switch events, `PublicWalls`, `Query<Roof>`, raw masks, `WallPathSegmentationMasksMinusStairs`, `PlatformStairsStorage`, `UiSignifierStream`; runs22 after real associations and invalid-roof consumer |
| Arches | candidate fixes raw→MinusStairs resource. Existing core regular/freehand CPU profile/walker/third/resmoothing output is real96B and semantic gate120B | `ArchSegments`, `WallConstructorOutput`, same physical `AssetSsboLibrary`, gate event; style==2 outer branch, stats, BuildingHadArchesLastFrame and recalc-roof side effects remain |
| Lintels | current actual prepared/U/arch cache→64B and independent third writeback | processed masks, `PublicWalls`, actual output/SSBO; `LintelSegments` retained segment map and stats are missing |
| Gates | current semantic command→real96B/TransientMesh/Nani and lifecycle; candidate borrows canonical shared shader Arc | `Gates`, mesh library/runtime, same shader library, actual terrain, PublicWalls, output; must test real nonempty mesh/Nani definitions and deferred entity visibility |
| Main wall | existing core `construct_wall_bricks` is the canonical full authority→seeded production→96B resource/RT producer | root installs once in28 after prior constructors and actual hole finalization; existing AssetMaintenance already synchronizes real main buffer |
| Fences | current core output includes posts and rails64B; no missing fence-post Nani | current wrapper discards stats seed/count; CPU says UNVERIFIED and source threshold error branch remains; finite shared pool source policy differs |
| Platform creator | actual core segmentation/blockers/materializer/rows/holes/geometry algorithms exist; system wrapper remains TODO | not made complete by candidate splitter; needs actual decorator80B+terrain, collision owner lifecycle, holes and96B sparse writer integration |

Original splitter stores **32B threshold step ranges**, not104B materialized
segments. Current `PlatformStairsStorage::segments: Vec<[u8;104]>` was at the
wrong layer. Candidate replaces it with typed `WallPlatformStepRange`. Native
751960 accumulates start/end as64-bit and LoopRange12B, memcpy stride0x20;
native current processed mask memcpy stride0x88. The28 creator materializes104B.
Original event union sorts unsigned WallIds and deduplicates before processing;
missing raw/public rows panic. On every present row it clears this wall's old
step list before either flat thresholding or non-flat/shape>=2 raw-mask clone.

Additional height-cache defect remains outside this narrow relationship
candidate: native78b550 sets max_y_gable=base and flag1 for every initialized
wall before its conditional rectangle/gable modification. Current
`update_gable_heights(None)` clears the only gable cache, but `max_y_gable()`
requires that cache; ordinary no-roof walls can therefore panic in actual critter
consumers (`systems/critters/src/shared.rs`). `building_projection` meanwhile
falls back to base, so the getter and projection disagree. Correct recovery is
an explicit initialized base gable-cache state at the owner, not a silent getter
fallback when initialization has never run. Roof agent confirms no competing
candidate for this field; central owner has been informed.

Retained `building-segmentation/B-platform-masks-verification.json` has384
original fullcalc/platform cases (941 processed masks,432 steps). That earlier
report explicitly excludes ECS/relationship/schedule. It is reused as numerical
evidence, not presented as a fresh run of the candidate wrapper.

## MaterialStats publication ownership

Original PublicWallState+188 contains two0x68 halves. Previous has Vec+08,
seed+20, seven f32+28..40, eight u32+44..60. Current repeats from+68. The
publisher moves old current into new previous and creates a fresh empty current
only when it republishes a row. `init_this_update` resets current only. Advancing
on every frame or inside init would erase meaningful history.

Candidate adds missing previous seed/counts, actually attaches this record to
PublicWallState and moves it at the successful row replacement owner. Original
invalid-source guard is retained; untouched rows keep both halves. This does not
complete constructor writers or material audio.

Fresh independent `probe_stats_publication_fragments.py` executes five exact
straight-line original move/reset instruction ranges with an old-present
fixture,128 arbitrary bit states. It does not run the full publisher, TLS,
allocation, query acquisition or source branch selection. Native output fixture
and standalone actual-candidate rustc comparator are stored beside this report.
**128/128 compared successfully**, including bitwise seven lengths/eight counts/
seed and independent preservation of the owning Vec pointer/content by move.

## Finite sparse pool and visibility

Native `gpu_clear_wall_gate_pieces140773c00` is misleadingly named: it clears
the wall's **WoodenPlankSsbo allocation at Public+130**, once in19 before22/28.
Lintel/fence posts/rails share that per-wall allocation. `141238d80` first loop
pushes fence posts64B; second pushes rails64B. Each push to14120c430 that fails
sets an aggregate error flag and processing continues. Successful prefix remains;
old geometry is already cleared. `construct_fences140765830` still writes stats
after emission and emits at most one enabled warning per invocation after all
wall rows: `Fence-pillar constructor ran out of fence pieces` (original format
table142a7dcf8→142a7dcc8,len48).

Current extra-output batch `replace_all` preserves old revisions on exhaustion
and partitions per-source allocations. That is a safe current API adaptation,
but **not proven equivalent to original finite-pool output/order/quota**. Correct
caller adaptation should use the existing sparse `clear`/`push`, preserve native
source ordering22→28 and per-wall owner, continue on failure and aggregate it.
No rhapsody or sparse library rewrite is required. Do not call this boundary
complete from CPU batch tests.

Graph integration should observe `WallConstructorOutput.gpu.failures` and
`last_device_error` after44 upload and before building W21/W27/Nani graph
callbacks. Main `WallBrickUploadStatus` cannot describe additional owners. An
upload error must remain observable as the existing graph/error result, not be
turned into an empty DrawLists success. No extra AssetMaintenance main sync is
needed: the current owner already runs one in PostUpdate Geometry.

T13 is excluded by the user and is absent from these contracts.
