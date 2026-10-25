# B continuation Delete retains two independent heights

Target B only. Owner `core-wall-height-20261018`, root claim TG-eb56a08d047af785ea (original resources/history/wall_history.rs). Production and Cargo untouched. `candidate/resources/history/wall_edit.rs` adapts the current manual architecture. `integration-owner.patch` contains only necessary constructor/serde/API-test integration for root; the original replay/container groups remain main-chain-boot-owned. No ownership claim is made over those files.

## Original evidence and business rule

SHA-bound `native-manifest.json` / `native-pack` retain six original PE ranges, 381 instructions, 2,080 decoded bytes. The finish request has three explicitly reported instruction-overhang bytes; these are bounded spans, not claims to recover the entire functions. RNE SHA f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03; matching PDB SHA 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911. No original instructions were executed in this task.

1. Original finish_wall(false), invalid continuation: 140a81077 loads progress+94 desired height, 140a81129 writes it independently to the Delete record T+80. Previous WallState occupies T+8..78, wall u64 T+78, current color u32 T+84. 140a81199 assigns the Delete discriminator MIN+1; 140a811bf calls the existing builder next. In particular, overwriting previous.source.extra would lose the old undo source.
2. Original serializer 141400939 selects `DeleteWall`; 1414009b8 passes &T+78 as `wall_index`, 1414009e6 passes &T+80 as `wall_height`, 141400a0d passes &T+8 as `before`, and 141400a31 forwards T+84 to `color_id`. `serializer-keys.json` retains the original RIP target strings. The existing SavedWallEdit field/alias schema already decodes wall_height; the old conversion rejected a distinct height because its live owner was missing.
3. Original replay operation 2 entry 140ac04b3 restores on undo and removes on redo. **Some-source Delete undo preserves the source height**: 140ac12be loads Arc+6c = source+54; 140ac14c0 writes it into restored WallState+54; 140ac1461 calls new_wall_with_id. The independent Arc+90 = T+80 height is read at 140ac1322 only for the source-None fallback. Contrast Create's Some-source path at 140ac1252, which explicitly reloads its separate height. Do not copy Create's override into Delete.
4. Generic builder wall_deleted at 140a876a0 reads source+54 at 140a87706 and writes it twice: cloned source height at 140a87886 and T+80 at 140a87863. This proves the existing three-argument factory's default is source height. Continuation's inline producer is the exception requiring an explicit height.

## Candidate and compatibility

Delete gains `restored_height: f32` (original saved `wall_height`), retaining source separately. `record_deleted_wall` keeps its existing signature and delegates with source.source.extra. New `record_deleted_wall_with_height(wall, source, color_id, restored_height)` and the same method on RecordWallHistory carry the independent value for continuation. Its parameter order preserves the original factory's first three arguments. Some-source apply only adjusts its match pattern; it retains the existing source-height behavior.

Shared serde serializes/restores the new field independently, removes the now-unnecessary unequal-height rejection, preserves `undo_instant_state` alias and the existing format. The current mandatory source model still cannot import a native `before: null` / Option-none legacy record; this task does not manufacture an empty wall or pretend that compatibility branch is implemented. No new production producer fabricates a None source.

All current direct Delete literals were searched in core/systems: replay test, eraser archivist's two literals, bulldoze's one literal. Minimal API hunks set the new value to the captured height to preserve those existing producers; this is not a new audit or restoration of their eraser algorithms. The verified continuation owner separately changes its producer to the explicit factory with desired height.

## Verification and boundaries

- Owned and all five shared candidate files parse through rustfmt; both patches pass same-batch git apply --check against the captured source hashes.
- Added core record/replay regression retains 7, negative zero, NaN payload bits while Some-source undo restores old 3.9 and redo removes. The old factory defaults to the old height.
- Added serde regression round-trips old 3.9 / independent 7, both canonical `before` and old alias.
- Added actual existing ECS replay regression using old height 2 / independent 7; undo must restore 2, then redo removes. Existing restore-color/failure events are checked.
- These Rust regressions have not been compiled or executed here. Root owns Cargo validation. Native execution was not retried after the earlier automatic approval reviewer reported the account usage limit.
- Full native optional-source compatibility, the other unresolved wall variants, GPU publication and snapped/premade continuation branches are outside this bounded schema fix. The sibling continuation candidate remains responsible for its actual producer and schedule chain.
