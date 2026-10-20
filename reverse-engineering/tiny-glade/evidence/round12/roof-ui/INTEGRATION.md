# Roof UI candidate checkpoint — original B

This directory contains source candidates, not a production completion claim.
No Cargo or LLVM code generation was started by this owner. Central integration
must compile and run the original-output tests and the full input/history chain.
T13 remains excluded. rhapsody and general dependency implementations were untouched.

## Owned source candidates

All paths below are rooted under `proposed/tiny-glade/crates/`.

1. `country-core/src/systems/ui/resources/widget_params.rs`: eight owned native
   volume variants, including TriangleMesh Vec ownership, ScreenAabb depth,
   canvas layer and PlaneDisc. Existing complete scalar sphere/canvas producers
   remain supported; an absent required owned payload fails explicitly. The
   native first24B owned `Vec<SignifierFactory>` is preserved as `signifiers`,
   with the `with_signifiers` builder; active Arc retention retains each factory
   closure. Params no longer derives fictional equality for arbitrary callables.
2. `country-core/src/systems/ui/resources/ui_state.rs`: full56B intersection,
   active strong Arc independent of frame registrations, ordered hover-result
   Vec, registry snapshot/clear API, phase4 Continuing. Clearing canvases now
   register canonical WidgetParams. The existing garden specialization supplies
   empty sounds; the complete96B sound API is `clearing_canvas_with_sounds`.
3. `systems/roof/src/ui_edit_roof.rs`: original mode0 producer and complete
   ordinary consumer, entity+phase event, actual world triangles, exact six-float
   plus discrete cache comparison, actual per-address collision archival,
   cursor ownership, wall crossing and both history classes.
4. `systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs`: original modes1/2,
   actual selected roof resolution, ridge center, camera upright box, widget,
   real signifier factory closure, distance grab offset, height projection,
   absolute EditRoofCmd, wall crossings and direct history on completion.
5. `systems/roof/src/ui_focused/native_helpers.rs`: private xz rotation policy
   using original glam operations and native24 corner/36 index order. These
   adapt private game utility policy; glam algorithms are reused.
6. `country-core/src/utils/project_cursor_onto_ray.rs`: restore complete original
   native constructor, pixel transform and723B project algorithm. The existing
   optional defensive try API remains a separately callable host adapter.
   Original-output tests replace tests which encoded invented fallback policy.

## Shared dependencies and merge order

- Apply the2 UI resource candidates before the ui-update owner's lossless
  PlayUiSound24B fragment. The resource candidate still contains the base32B
  definition; copying it over the later sound fix would regress that work.
- `roof_chain_full`: owns hover_intersection original geometry, ordering and
  world occlusion. `escape_menu_full`: owns ui_update/get_ui_volume, active Arc
  history-start/finish seam and exact event/audio migration. Neither owner may
  replace real input with manual interaction injection for chain acceptance.
- `independent_verification`: complete DecoratorArchivalResources with world and
  DecoratorPhysicsColliders, `archive_current_decorators_at_address` and mutable
  `decorator_cull_allow_wall`. Both consumers pass exact reason `edit_roof`.
  Native bodies1421a0470 and140b2bc00; relevant sites141a1a0f4/106 and1421adfa1/ba.
- Archival types are currently `country-core/native-physics` gated. The roof
  dependency must enable that real feature for full compilation. There is no
  absent-physics dummy archival path in these UI candidates.
- `save_migration_full`: complete typed SignifierEntityBuilder. Focused closure
  calls `from_mesh_name("brush_arrow_flat_single_up")`, duration0, captured
  rotation, captured scale*1.5 and actual position, then build(meshes,shaders,
  commands). It must use the complete builder candidate, not the old spawn-empty
  implementation. Factory name uses original glam DebugTuple formatting.
- Central shared onboarding type must preserve `NotifyHintSystemRoofShape {tag}`:
  ordinary movement writes1 after accumulated horizontal profile change>.2;
  finish with a populated cache writes0. This was previously a unit event.
- The ordinary and focused functions intentionally retain their cache after
  finish, matching original stores. The next start replaces it. History lifetime
  is controlled by original ui_update/history_start/history_finish scheduling.
- These candidates do not modify root schedule/history_runtime/snapshot. Root
  must register DoUiEditRoof, cache, both system functions and their complete
  event dependencies at the verified original sets/order.

## Native evidence and tests

`*.original.asm.txt` are fresh LLVM dumps from unmodified original RNE, not the
patched database's void-signature decompilation. Native fixtures execute those
unchanged bytes in Unicorn, with explicit PC stop hooks for reused regions.

- `native-drag-fixtures.json`:87 ordinary arithmetic/control-flow cases,67
  actual EditRoofCmd stores and4 actual requested InnerWalls height targets.
  Includes zero/NaN, horizontal/vertical equality, original wall-height guard,
  hint threshold and crossing boundaries. Actual wall-container setter is
  observed at its call boundary, not counted as an executed container test.
- `native-focused-helper-fixtures.json`:13 complete xz private function cases,
  plus5 full native dimension-to24-corner/36-index cases including nonfinite and
  signed-zero inputs. The box fixture stops before Transform-to-Mat4/allocation.
- `native-projection-fixtures.json`:146 complete original projection calls with
  original lerp calls, plus12 constructor+pixel transform calls, including zero
  windows, near-zero projected direction, perspective and behind-camera cases.
- `native-constants.json`: height inverse range0.4..12; range0..f32::MAX; arrow
  offset.75, box offset.5, hover scale1.2, factory scale1.5, dimensions.75/1/.5.
  Both reason literals are `edit_roof`, not `roof_edit`.
- `clearing-canvas.original.asm.txt`:14092e0e2 writes word0x0100 at params+a8;
  ArcInner header16B makes flags+a9 identical to original Arc+b9. Thus flags1
  permits history, while no_history_recording returns false. The sound records
  are all96B copied from the caller at14092e02a..0b0. Extra tag1/argument@+79,
  volume tag4/layer and130x130 are separately written at14092e0b6..0db.

All native fixture extraction succeeded. Rust fixture comparisons have not yet
run at this checkpoint; no passing Rust/e2e/GPU result is claimed.

## Still required for acceptance

Central candidate compilation and independent source/native comparison;
numeric fixture execution; true input→set6 picker→ui_update active Arc→history
start→roof start/move/finish→EditRoof authoritative consumer→visual/wall history
→undo/redo using real collision archive owners. A nonempty focused widget must
wake the true factory and produce its real drawable entity, then animation and
deletion must release ownership. No hand-written `replace_interaction` test is
a substitute for that acceptance path. Other roof ridge direction/length UI
files remain outside this candidate and remain unfinished in production.

## Reverse-engineering efficiency corrections

Use full original PROC boundaries before control-flow restoration; adjacent
switch tables can falsely extend enum variants. Prove ownership at each seam:
EventId suffix is not event payload, retained Arc is not current registry entry,
geometry tag is not enough to reconstruct owned mesh data, and a factory whose
builder spawns an empty entity is not a recovered visual. Read literal bytes and
constants before naming or choosing a familiar value. Freeze original fixtures
before Rust comparison; separating numeric execution scope from ECS/collision
scope prevents fixture counts from being overstated as whole-chain validation.
