# Save chain source recovery, 2026-10-20

Source frozen for central Cargo validation. This is not a complete game-load or
GPU acceptance claim. Only snapshot.rs and one native fixture are changed.

## Repaired production behavior

- AutoClutter was captured and retained in parsed sections but completely ignored
  by world application. The actual DeletedAutoClutter resource is now replaced
  with the captured unique u64 IDs, including an empty set. Invalid values fail
  parsing/application and do not replace that owner. No second deletion state or
  default resource is synthesized.
- Roofs now emit the real CreateRoofCmd, including the full identity, appearance,
  parameter block and restore flag. The previous bare Roof spawn skipped actual
  animation, tile simulation, trigger, Nani, geometry and sound creation.
- The three color families now emit their canonical commands with full u64 IDs
  and custom=false. Their real consumers own table and plaster side effects.
- Every tree now goes through InsertTreeCmd, preserving storage ID, signed variant,
  position, scale, rotation, preplaced flag and prefab name. Missing legacy names
  use the canonical missing-prefab sentinel and real tree consumer. The old manual
  entity creation bypassed ambience, conflict replacement, water and name rules.
- Terrain restores publish the incoming snapshot's stroke IDs before replacing
  TerrainEditingData. Raster restoration sends the original staging-upload unit
  event, including an empty raster list. No private Events resource is created.
- Apply order follows original native owner/event boundaries: walls, roof requests,
  decorators, three colors, terrain, removed supports, raster/upload, height data,
  trees, deleted clutter, stairs. Report roof/tree/color counts mean requests
  queued, not downstream geometry/GPU readiness.

## Original evidence and correction

Original B workspace/input/tiny-glade.exe SHA256
`f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`.
The complete original function1417a9c50/4143B has SHA256
`544fa462a845e4f7ff84cda328bed7edfebca351e91d5a490952072aa1bc4ec9`.
Retained original disassembly is at
`skeleton/task-index/work/TG-3adf041a3108edb59a/main-chain-boot-20261018/snapshot-init-rne.asm`.
These original bytes were read again directly for this batch, not inferred from
an old candidate source or the patched executable.

`native_snapshot_producers.py` executed roof producer1417aa042..1417aa196
and three color producers1417aa218..1417aa4ba on original B bytes:24 cases
each,32 roof records and96 color records retained. Preallocated actual event
containers avoid growth. Only the known heap deallocator is stubbed. This proves
bounded native field production, not the entire function, ECS, allocator failure
branches, whole game or Rust execution. Rust regressions consume those retained
original outputs through the actual production projection/queue path.

An older CONTRACT incorrectly said invalidation reads old world stroke IDs.
Original1417aa4ba reads LoadedSnapshotData+100 and1417aa4cf/+108 its map,
then publishes those IDs before owner replacement. This batch corrects that
interpretation; a regression distinguishes old ID9 from incoming IDs1/3.
Original1417aaa5e..1417aaaea copies the incoming deleted-clutter owner
at+148..178 over the real target. Original1417aa796..1417aa82b writes
rasters then publishes the staging request. Original resource borrows are not
serialized snapshot-header descriptors.

## Validation requested from central owner

Rust syntax parse via rustfmt returned0. No Cargo was run by this agent.
Run country-core save::snapshot::tests and the new central worker/reload test.
New tests cover owner replacement/empty/omitted/invalid/missing resource, actual
native roof/color outputs, full tree payload, terrain source IDs and empty-raster
upload. The previous world test now checks real requests and absence of bare
entities; only actual consumers may establish final entities or color effects.

Mandatory canonical channels:CreateRoofCmd,SetWallColorIdCmd,SetGateColorIdCmd,
SetHalftimberColorIdCmd,EvTerrainStrokeChanged,
UploadWorldRasterBuffersToStagingCmd,InsertTreeCmd. Central owner handles maintained
registration and real staging consumer assembly. Downstream roof/tree/color
consumer validation, format compatibility, Stairs cached-checksum restoration,
remaining snapshot gap policy and complete full-game reloading remain separate
acceptance boundaries. No global completion percentage is claimed.

## Gap-state follow-up, awaiting central execution

Central reported snapshot_worker_reload1/1 passed over real worker/file/load/World
after this batch. Independently, current try_load_game_system had only logged a
nonempty apply report and then still sent OpenFile and reached Loaded. That could
publish a partial restored world as a valid save and emit LoadingGameComplete.
The load adapter now requires a complete report immediately after application,
before cursor progression/save opening; the existing terminal Failed handler
owns error cleanup. Required ActiveSession validation and original marker creation
are inside that same fallible boundary, so a missing owner cannot panic before
the handler or fabricate a successful loading state.

New integration target save_load_gap_state uses actual temporary Motomoto files,
the real load manager/base-state application and canonical owners/channels. Its
broken file declares PathRaster but contains no raster chunk: it must return an
error, clear the active request, stay Failed across later frames, and produce
neither OpenFile nor LoadingGameComplete. Its valid empty-raster counterpart
must continue through ApplyCounter/Loaded and the real completion event. This
is a load-state/transport regression, not an independent full save worker or GPU
test. Syntax parse returned0; central has not yet reported its execution.

## Independent test target and original staging schedule

Existing country-core library tests are blocked by six unrelated removed native
fixtures. The nine new snapshot regressions were moved, without disabling tests
or fabricating missing fixtures, to external target snapshot_native_restoration.
The roof oracle was rerun with valid finite canonical fields and enum tags;
the external regression now exercises actual Roof serde → World apply → canonical
CreateRoofCmd, rather than a private projection function. New manifest is
external-target-checkpoint.json. Central corrected the load-state test's initial
camera from zero-aspect MainCamera::default to the actual MainCamera::new(1.0);
production validation was preserved.

The actual staging upload consumer's registration was traced from original PE:
run_game_thread14059d450 → call14059fc87 with DL14 → converter1405c0400 →
vtable142a05448 → original run wrappers1405bef90/14045dc60 → upload leaf140957bf0.
The same converter interns GameSystemSet vtable14292af98. Original fmt140037760
decodes the original label tables14292b070/14292b1d8:14=HistoryStart,
36=PostGeneratorGameplay,44=PrepareRendering. See
raster-registration-manifest.json and six complete PE-exception-bounded assembly
files; no unavailable SDK/PDB lookup was claimed.

The separate decorator_visual reviewer independently closed Update identity:
add wrapper140327490 at140327551 calls140172580; that helper's1401725c8
uses schedule-label vtable1429f1e78, whose fmt140526ef0 references the original
`Update` string1429f1cb0/length6. Parent14059ff02..33 copies the actual upload
configuration into the tuple passed at14059ff83. The earlier observed128-bit
constant8e1bd251... queries the Schedules resource and is not itself proof of
Update. Central owns actual Update/GameSystemSet14 registration and independent
runtime validation over the six real mapped SSBO resources.

## Final bounded validation

Central actual Cargo execution, independently reread from
`D:/game/tmp/progress-20261020-save-integration-v2.log`, passed:

- snapshot_native_restoration:9/9,0 ignored;
- save_load_gap_state:1/1,0 ignored;
- snapshot_worker_reload:1/1,0 ignored.

The five frozen source/fixture hashes still match after that run. The complete
log hash and scoped result are recorded in external-target-checkpoint.json.
This proves the repaired CPU/data/worker/error-state paths; it does not establish
whole-game interaction, downstream roof/tree/color geometry or GPU acceptance.
The broad country-core unit suite remains blocked by six unrelated old missing
fixtures, which were not disabled or synthesized.

The separate original Update/set14 proof is now fully retained at
`../decorator-visual/raster-registration-independent-manifest.json` and its
`RASTER-REGISTRATION-INDEPENDENT-REVIEW.md`. Central reports mounting the real
upload consumer on Update/GameSystemSet14 with maintained events. Its physical
device presence guard is explicitly a current-engine adapter, not an original
native predicate. Full host compilation/runtime remains central validation.
