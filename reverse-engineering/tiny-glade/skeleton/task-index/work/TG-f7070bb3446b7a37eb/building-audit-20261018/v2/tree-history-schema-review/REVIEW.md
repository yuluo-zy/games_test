# Independent B tree schema and wall-tree consumer review

Reviewed 2026-10-18 by building_audit. No production edits, Cargo invocation, new native execution, or author-frozen input changes. The review accepts this bounded identity/transport/schema repair and hide/UIdelete v3; it does not accept the complete tree tool, original global schedule, or unresolved preplaced filtering/render ownership.

## Frozen inputs and verification

- Tree schema 38-input manifest: `36c1afd8950751eeaafb81a80411c5f16fcc7bab11e800f289a25a1db9ae7d49`.
- Hide/UIdelete v2 15-input manifest: `ab58f6c8b9dc0dd8036be8a5fc1b0eb0458a034391d6888e1c222b795846424a`.
- Hide/UIdelete v3 15-input manifest: `3652539b3f2e76ef30693992af0e35772b22a5faaa23aacf3df864fc472e14b0`.

All paths, hashes, and declared sizes were independently checked. All three original LLVM packs (82 + 205 + 289 instructions, 354 + 876 + 1244 bytes) were compared instruction by instruction against the identity-bound B RNE. The 17 recorded RIP-relative references and their exact referenced key text were independently recalculated and compared against the PE. All six owned source bases matched current production when checked. The six owned patches, shared schema integration patch and root hide-v3 patch passed one combined `git apply --check`. Verification details and a repeatable static script are retained here.

## Original evidence, read before the implementation

B `14003d4e0` uses the six TreeData keys in this sequence: position, scale, rotation_degrees, id, prefab_name, is_preplaced. The id source is the i32 at data+2c; it is the placement variant. This alone does not describe the outer TreeEdit.

B `14003d650` independently proves Place/Delete each contain index and tree. Place reads index at enum+8 (`d7fc`), emits the index key at `d80a`, then tree at `d82e`; Delete uses `d91a/d928/d948`. Move reads index at enum+20 (`d6f1`), from at enum+4 (`d71f`), and to at enum+10 (`d743`). Its exact keys are index/from/to. Current custom outer serde follows these shapes and preserves the public enum constructors. Standalone TreeData decoding leaves storage_index unbound; save/replay never widens its placement variant into a unique index or assumes index zero.

Original replay `141bc34f0` uses Arc+18 for Place/Delete storage identity and Arc+30 for Move identity. The nested placement id is Arc+4c. The two Move directions load Vec3 from Arc+14 and Arc+20. Reinsertion loads scale/rotation at Arc+44/+48, variant at +4c and preplaced at +50. Original output slot is 72-byte EventInstance: the generated EventId is stored at +40 (`141bc39b5`), after the storage index at +38 (`39b0`). Treating +40 as tree height would be incorrect.

The reviewed candidate carries all these known values through existing InsertTreeCmd, SpawnPrefabCmd and TreeComponent, and uses the existing live 48-byte PlacementDataSsbo owner instead of stale storage positions or fabricated placement values. Move's inactive host fields are not serialized as native TreeData. Saved EditType integration uses the existing Tree tag12 and deserializes the outer identity. The converter's other unresolved filters remain explicitly outside this acceptance.

## Consumer v1 findings and v2/v3 closure

The earlier independent v1 review found two actual control-flow errors. Root v2 fixes both without changing the leaf geometry:

1. Original `141bd2579/2582` branches around UI acquisition when the mode is not Eraser(Tree), but still enters the PathDrawEvent reader. v2 gates only the canvas/cursor/brush branch and consumes path events in every mode.
2. Original `141bd2680..26eb` resumes reader iteration after a false observed flag updates center/radius. It drains both native event buckets, retaining the last false event; a later true event does not cancel the retained request. v2 removes the first-event break. The new actual World test sends false/false/true in Path mode, checks only the last false center is deleted, and checks a second frame does not add a repeated history edit.

The initial review's statement treating UIdelete's missing-query skip as an accepted integration policy was withdrawn after root requested the exact original policy. This was not a user constraint. Original `141bd28a8` (entity index), `28c1/28d8` (generation/invalid entity), `28e2/28fb` (query archetype membership) all reach `2b37/2b3d/2b44`, then `2b61`; `141bd2b8c` calls `1428d9760`. Independently resolving that address with LLVM identifies `core::result::unwrap_failed` at core/result.rs:1703. The referenced 43-byte message at `142ef5e98` is `called Result::unwrap() on an Err value` with the original backticks. v3 correctly replaces UIdelete's skip with Query::get(...).expect and adds a real missing-query should-panic test. Hide's independent native entity-query guard continues to skip unresolved entries; these two functions have different proven failure policies.

## Geometry, ownership and validation limits

The earlier 84-case native wall callback oracle and strict SSE-order distance review remain applicable; v2/v3 did not change that production geometry. Root's 61 arithmetic rows still correspond to the previously independently verified 10 edge + 51 finite original distance fixtures. The direct grid walk preserves cell order, WallId filtering before distance, strict 0.4, denominator MAXSS with 1e-20, signed-zero/NaN-aware clamp operand order and early stop. Generic WallSpatialHash circle query is not substituted.

The live query is the existing PrefabInstance wrapper over PlacementDataSsbo and the existing PrefabInstanceData ECS identity. Each valid hide request clears hidden state before its predicate; wall requests precede area requests. Area's height rejection is `limit > 0 && limit-tree_y > 4`; NaNs follow the original comparisons. Circle expansion doubles delta before with_expanded_diameter; rectangle expansion uses Vec2::splat(delta).

The schema candidate's three schema tests and actual replay test cover six direction combinations in source. Root consumer v3 has the arithmetic regression plus actual wall, area, path and failure regression sources. They have not been compiled or run by this reviewer. Source inspection and apply-check do not substitute for root's combined Cargo tests. No new tree native differential run was attempted because the earlier automatic approval usage rejection remains in force. This limitation does not erase the prior successful 84-case original callback run, nor does that leaf run establish complete tree behavior.

No further blocking finding was identified in these frozen candidates. Root was notified before integration.
