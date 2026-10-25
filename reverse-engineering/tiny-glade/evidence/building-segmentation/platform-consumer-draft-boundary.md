# Draft expanded platform consumer boundary

This is research, outside accepted staged runners. Rust/Cargo and accepted oracles remain frozen for the parent's Stage4 replay.

The first expanded sixteen-case full source→steps→cells→rows→holes→stats attempt failed ten cases. The complete native outputs and candidate errors remain in `B-platform-stairs-consumer-first-failure.json`. The report is not an acceptance result.

A concrete source mismatch exists in the draft `src/bin/wall-platform-stairs-consumer-probe.rs`: it writes PublicWall+0x258 tag1, +0x264=.6, +0x26c=6, +0x270=4, but does not populate +0x25c/+0x260 rectangle orientation. `PublicWallBuildingState::shape()` copies +0x258..+0x274 directly. The candidate renderer therefore receives rectangle axes(0,0), center(.6,0), dimensions(6,4). The native renderer oracle receives axes(1,0), center(0,0), dimensions(6,4). The existing geometry-only fixture did not consume the rectangle renderer, so reusing its partial metadata for the new renderer boundary was incorrect.

`research_consumer_shape_mismatch.py` uses only frozen binaries and the eight persisted failing source cases. It produces each actual original cell and runs the shared rows binary twice: first with the same complete native shape, then with the draft's zero axes. Results will be retained separately. Repair requires a new full native comparison after freeze; no accepted report hash can substitute for running it.

Publisher reachability remains a separate scope qualification. Some expanded fixture cases explicitly resolve prepared pointY independently from encoded terrain and use regular constrained-extra .3, outside the already proven regular publication clamp .5..14. These are useful resolved-consumer stress inputs but do not establish a source publication scenario. Do not drop failed cases or silently clamp them to make a renderer report pass.

## Next original collision owner

The original creator's final physics branch is in `B-platform-constructor/1407744d0.asm.txt`:

- 1407771d2 tests accumulated point count, zero branches to remove.
- 14077721a calls 141202600 to materialize triangle chunks (dependency Vec mechanics are reused).
- 1407772c3 calls game `RaycastMeshData::new_indexed` 1408cee40.
- 14077736f..140777391 construct a mesh key with tag7 and WallId, then `RaycastMesh::new` 1408ceb10.
- 140777396..1407773b2 construct identity translation/quaternion.
- 1407773e9 calls wall-keyed `RaycastWorld::upsert_collider` 1412453c0.
- 140777429 calls `remove_collider` 141245220 on empty accumulated geometry.
- 1407774aa calls native `MaterialStats::add_stairs_length` 140a88590 after physics replacement/removal.

The existing CPU component and indexed cells prove geometry production; they do not prove original RaycastWorld insertion/query policy. Generic BVH/Arc/HashMap/triangle storage mechanics are dependencies, while game key/tag/filter/replacement choices require original evidence. No duplicate BVH implementation is authorized by this finding.

## Frozen-binary diagnosis result

`B-platform-consumer-shape-diagnostic.json` now records all eight persisted failed source cases: 71 actual original cells and 3331 complete ordered 96B renderer records. The frozen shared rows binary, given the complete native shape, matches every record byte. Feeding the same cells with the draft zero-axis shape produces `error InvalidInput` on all 71 cells. This isolates the first attempt's concrete fixture error without changing any Rust or accepted oracle. The complete sixteen-case draft must still be repaired and reexecuted after freeze; these 71 per-cell checks are not a replacement for that whole source-chain acceptance.

## Corrected whole source-consumer acceptance

After Stage4 freeze was lifted, the draft probe's complete rectangle axes were corrected to (1,0), leaving center(0,0) and dimensions(6,4), matching the original renderer's complete 28B shape. The probe was rebuilt in release and the entire sixteen-case original source chain was reexecuted twice (the second run followed Rust formatting). `B-platform-stairs-consumer-verification.json` now has zero differences across 48 actual materialized segments, 91 cells, 3701 ordered 96B render records, 48 holes, stair lengths and all four native MaterialStats fields. The first failure and per-cell diagnostic are retained. The scope remains resolved prepared/terrain/arch owner inputs, not proof that every stress fixture is emitted by the actual publisher.
