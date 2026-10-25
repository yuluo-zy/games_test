# Arrow gizmo animation consumer, original B

## Scope and current architecture

Complete the missing consumer inside the already claimed original ownerTG-7797c5bf7af7ea4ed0. The new source extends batch6b's same unit EnableArrowGizmoStretch event; it reuses the current renderer's public ArrowGizmoView component and current game Transform. No second gizmo component, rendering engine, native ECS/container or timing system is created. candidate.patch modifies only systems/arrow_gizmo_animation.rs from the integrated batch6b baseline. Shared module declaration/event registration is already in batch6b and is not changed here.

The integration owner should schedule arrow_gizmo_animation after ui_edit_terrain (and the input derivation that emits CursorMovedScreenSpace), before its renderer consumes ArrowGizmoView. This worker does not modify shared registration/schedules. Missing actual renderer/entity birth producer remains a separate explicit boundary; this consumer operates on real current component owners when supplied.

## Original rules

Original B tiny-glade.rne SHA256f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03; matching PDB SHA2562d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911. C behavior is unverified. The exact native direct generic caller identifies Query<(&utils::components::transform::Transform,&mutcomponents::ArrowGizmo)>, EventReader<EnableArrowGizmoStretch>, EventReader<CursorMovedScreenSpace>. Native loop loads only the Arrow8-byte fields; Transform is query membership. Current Query<&mutArrowGizmoView,With<Transform>> keeps that membership and same existing component identity, without an unused read of transform values.

Original0x140a931df..0x140a93642 is the complete business loop after optional profiling. It uses constant-0.1 per invocation (0xbdcccccd@1429eb6f8), cursor multiplier0.02 (0x3ca3d70a@1429cd2d0), max1.0 (@142925904), with no GameTime parameter or hidden elapsed-time multiplier. The current clock is not consulted because native business does not consult one.

For each matched entity, original EventReader enable is read/advanced inside the row loop. Cursor events are read only when this row sees an unread enable message; its last unread Vec2 is selected. With zero matched rows neither reader advances; without enable, cursor remains unread for a later enable. Thus moving reads outside the entity loop would change original behavior.

Two axis guards startfalse once before the entire query loop and persist across rows. If a row sees enabled cursor movement with strictly positive length (unordered NaN is rejected), abs(dy)>abs(dx) increases fieldf4, otherwise fieldf0 (ties choose horizontal). Increase is min(abs(delta*0.02)+old,1), preserving the original operand order/SSE source semantics. Its axis guard becomes true. Each axis whose guard remainsfalse decays bymax(old-0.1,0). Native source semantics choose finite1 for unordered increase result and+0 for unordered decay result. A guard already true skips that axis's later-row decay. Component change tick is written whenever native writes an axis, even if the value remains0. The candidate only takes mutable component deref when it has such a write; Bevy owns the actual change tick.

## Evidence and observations

native-manifest.json and native-pack/report.json collect one bounded0x770-byte original range plus3 sparse matching-PDB samples;2 analysis+2 version processes, exact original instruction bytes verified. The pack's function_boundary_verified remainsfalse; source/inline frames identify metadata, not author code. original-query-interface.json retains exact matching artifact/PDB binding and native generic caller candidates. original-constants.json binds3 raw rdata values.

animation/native_arrow_oracle.py executes full original dense-query row traversal, two native Bevy event-reader cursor algorithms, actual axis guards/numeric operations and component change-tick writes. It supplies one valid matched dense table/archetype with Arrow data column/tick array/entity-row mapping, and preallocated events; no business arithmetic is bridged or translated. The standard shared native harness only provides immutable image mapping/allocator/CRT boundaries, none of which the business loop needs to call. The optional profiling prefix is bypassed. No target process or GPU runs.

native-arrow-fixtures.json has56 full-loop cases:0/1/2/3 entities, enable0/2, cursor absent/zero/horizontal/vertical/ties/NaN/two unread deltas, including negative initial fields. It records every output field bit, every native changed tick and both resulting reader counters. Two additional real multi-invocation sequences prove empty query preserves both readers before a later spawn, and absent enable preserves cursor deltas until a later enable.

Candidate includes56 actual ECS system regressions comparing all output float bits to native snapshots, with an extra ArrowGizmoView entity lacking Transform that must remain unchanged; two cross-frame tests compare the original reader-lifecycle sequences. Original instruction probes completed exit0 with the approved external Python runtime because sandbox JIT is unavailable. This worker did not compile. Root owns current-crate compilation/test and independent review.

## Boundaries and status

Original dense-query lookup/reader implementation was executed as evidence, not rebuilt in Rust; actual ECS/filter/readers are the current dependency APIs. The native sparse-archetype dispatch path and profiling paths are not exercised; the same business loop and query membership are reused through Bevy. Full native entity allocation/Arrow birth producer, global renderer ordering and Windows runtime/GPU acceptance remain outside this bounded consumer. Only ordinary native f32 plus selected negative/NaN delta cases are exercised; all possible NaN payloads/overflow inputs are not claimed. Status implemented_unverified pending root tests and independent review. The former batch6b note that Arrow's consumer was open is superseded for this implemented system body, while birth/global-order boundaries remain explicit.
