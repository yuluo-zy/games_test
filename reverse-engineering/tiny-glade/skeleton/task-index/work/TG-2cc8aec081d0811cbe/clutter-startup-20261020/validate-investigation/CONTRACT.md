# validate_blueprints: B behavior and current-host adaptation

Design investigation only, 2026-10-20. No source candidate, production/Cargo/GPU/test execution or claim mutation. Central has subsequently authorized the location/audio/subtype/UI/outline/loader companion families; this document records the required split before implementation. Blueprint and Storage remain foreign/root-owned. The original application interfaces are semantic references, not an instruction to reproduce Bevy/hashbrown ABI.

## Evidence and phase boundary

Reuse 06's frozen `TG-827b71275936dda04f/rhapsody-correctness-20261019/decorator-phase-audit/CONTRACT.md` and corrected 539-byte `validate_blueprints@140827130` body, PDB471:31212. The former erroneous `826f30` request is excluded. B binary SHA is `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`; matching PDB SHA is `2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911`.

The game callback is in the standard iterator wrapper `1412b84f0/8161B`, PDB1576:6768; it cannot be discarded as dependency code. This task collected that complete body, its `closure$0::closure$0@1412ba810/473B`, `is_decorator_location_valid@14080f470/8143B`, `DecoratorCollisionOutline::display@140b33920/476B`, and the two collision helpers `14129be00/497B` and `14129c3d0/254B`. Three collector reports retain exact original bytes, matching identity, symbol/source frames, measured process/output costs and no target execution. Source-line tables do not supply author source or ABI. `bounded-tables-and-strings.json` retains three finite dispatch tables directly from the same PE.

06 owns overall phase and true cleanup/destructive-clear separation. In the original outer19 Update/set25 chain, member9 validation follows the first collider phase and flush; fullvisual/middle/copy are later in member14. Validation is not an end-of-frame cleanup node. Do not mount the old approximate `add_decorator_systems` shim or invent event/Changed gates. Do not change the foreign Storage implementation here. Its current method deletes authority entries and is unusable for original validation until its owner applies the real metadata cleanup contract.

## Main 539-byte control flow

The original 14 system inputs are current Blueprints, PublicWalls, mutable Storage, DeferredDecoratorEditHistory, RaycastWorld, real Roof query, DerivedDecoratorInfoResources, mutable Backup, UI dispatcher, UiSignifierStream, CachedDecoratorTransforms, OutlineWallCmd writer, CursorPositionSS and MutePreviewDecoratorOpSounds writer. Canonical resources come from the same actual App World. A host adapter may flatten DerivedDecoratorInfoResources into its actual subtype/mesh-library resources and use current mutable append-stream APIs; it must not build substitute libraries.

1. Unconditionally mark Storage changed through actual ResMut access, then call **true** Storage metadata cleanup. This precedes all deferred-work gates and preserves authority/d2s/info. Destructive clear is a separate function and restore concern.
2. Test `deferred.records.len()` (native +0x10), **not** `pending`, resource changed state, or surviving-set length. With no records, return after cleanup.
3. Iterate the potentially-surviving locator set in its existing traversal order and short-circuit on the first rejected locator. Do not sort, batch-copy a World, erase the records or drain this set. The iterator wrapper's native ControlFlow tag is0 on exhaustion (`1412b8b6a`) and1 on the first break (`1412b9bee`); this is not a returned `all()==true` success flag.
4. Only a rejection plus `backup.created` (+0x281) requests rollback. The per-iteration diagnostics still happen before the created guard. If no specific diagnostic was emitted, dispatch the original generic cursor icon. Then mark Backup changed and set `backup.state=2` (RESTORE), and emit exactly one **unit MutePreviewDecoratorOpSounds event**. The last native eight-byte push is its Bevy EventInstance event counter; it is not a decorator ID queue or a 0x28-byte UI operation.
5. Do not execute restore inline, clear history/pending, manufacture a backup, or mark a valid placement after rejection. Original later chain members consume the restore request; their actual registration remains governed by06's phase contract.

The generic icon selector uses the full UI state/payload: when the second qword is0 and state is2..6, table index is state-1; otherwise index0. Table142a91c00 maps indices0..5 to dispatcher byte offsets `[0x69,0x5c,0x60,0x5c,0x5c,None]`. Index5 skips the generic icon but still requests restore and emits mute. Call1408272e5 passes the selected icon byte, actual cursor x/y and signifier stream; it does not pass a DecoratorId. Current `show_decorator_icon(icon,pos,id,stream)` packs an invented ID/raw glyph record and is not this exact leaf.

## Iterator callback business

The callback reads a real **24-byte locator**: storage address16B and decorator ID in its trailing word. At1412b868e/8697 these are loaded independently; at86bc they are passed to Storage::decorator_info. An affected decorator no longer present is skipped. A still-present decorator without a corresponding current blueprint rejects with no specific-feedback flag; that enables the generic icon in the main wrapper.

For a present blueprint, the original calls full `is_decorator_location_valid`, resolves derived subtype, and requires the cached transform before testing the location result. Preserve that failure/order boundary. Inlined `is_valid` at1412b883d..8847 compares the result tag with **0xb**: tag11 is Valid; tags0..10 are rejection variants. The location function's direct tag3/tag4 destination dispatch writes0xb at14080f542, so terrain/stair location is accepted by this branch. Current Rust does the opposite and its corresponding tests encode that error.

An invalid location displays the intended placement using the actual collision outline and cached pose, sets the specific-feedback flag, then dispatches the actual reason payload. Tables142c2c364 and142c2c390 preserve reason tags0..10, their cloning/feedback destinations and source-line ranges. Effects include wall curve/height/rectangle feedback, actual outline events, dashed DebugLine factories, and a `decorator_delete_scribble` SignifierFactory closure. Tags7/8/9 go directly to rejection after intended-placement feedback. Other tags retain wall/coordinate/rectangle payloads; collapsing all reasons to `Option<Rectangle2d>` loses required behavior. Author variant names and the precise typed payload schema must come from the complete producer/consumer branches during the separately authorized location/feedback candidate; the current boolean result is not acceptable fallback behavior.

A location-valid non-wall locator continues without the wall-overlap query. A wall locator with no concrete blueprint collision Mesh likewise continues. For a real Mesh collision slot, construct the actual transformed shape from its real Arc geometry and pose. Invoke the original overlap helper with the current ID, actual wall and `ordered=false`. No hit continues. A hit sets the specific-feedback flag and uses the returned real source: for decorator source tag3 resolve the blocking decorator ID/address, its real derived subtype, collision outline, transform and current blueprint, display that blocking outline with the original flag, then display the intended/current outline and reject. Preserve missing-owner/lookup panic branches rather than fabricating an outline or accepting the placement. The full callback is scoped rejection/feedback logic; it does not directly remove authority decorators.

## Collision companion agreed with06

06 is implementing the root-owned physical query dependency, using these complete bodies without re-extraction. Agreed API:

```rust
#[cfg(feature = "native-physics")]
fn decorator_overlaps_with_others(
    &self,
    decorator: &OffsetColliderTransform,
    current_id: DecoratorId,
    wall: WallId,
    ordered: bool,
) -> Option<&RaycastGeoSource>;
```

Main call1412b8928 has RCX=actual RaycastWorld, RDX=the concrete transformed collider, R8d=current locator ID, R9=WallId, and stack fifth argument0. Original `Rapier::index@1408cc5a0` uses length17 and **slot2**, as independently checked by06. The filter resolves the real collider receipt/user_data through the existing source registry with generation validation, requires source tag3, requires candidate storage discriminant0 and the same wall; `ordered=false` accepts only candidate ID != current ID, while true accepts current ID > candidate ID. Return the first actual source or None, preserving stale source/invalid tag panic behavior. An all-mask bool scan loses self-exclusion, source ownership and blocking reason. Reuse the current canonical 24-byte RaycastGeoSource projection, actual collider set/query pipeline and receipts; no second collision World or synthetic reason table.

## Current interfaces: missing versus simplified

| Current surface | What exists / exact remaining defect |
| --- | --- |
| `blueprint.rs` | Clear/copy/collider leaves exist; `validate_blueprints`, `show_intended_placement`, `show_max_wall_y` and its full reason-feedback consumer are missing. Blueprint belongs to02, not this writer. |
| `DecoratorLocator` / apply producers | Current Locator has only `addr`, so affected edits on the same address merge and lose ID. Place/change producers call `new(dst)` and cannot supply original24B identity. This is implemented simplification, not an absent file. Add ID under its current owner and repair only genuine producers; do not recover it later from one arbitrary record. |
| Deferred history | Canonical records/set/pending resource exists. Preserve it; no second validation queue. Current OpQueue is separate and must not carry mute events. `decorator_record_history` drains later according to its actual original stage. |
| `cull_oob_decorators.rs` | Function exists but is manually simplified: boolean+position+optional rectangle, wall/roof true, terrain/stair false and missing-cache behavior. Recover the full tagged result and wall/roof geometry against real PublicWalls/Roofs/subtypes/transforms. Merely flipping two booleans leaves it incorrect. |
| `Backup` | Public created/state fields suffice for a narrow restore-request write. Real create/restore is incomplete: auxiliary3 maps and events snapshot fields exist but are not copied, and current restore consumers misuse destructive cleanup. Do not label a state2 request as completed rollback. UI move is currently a gated no-op and does not create the native backup flow. |
| Preview/stair audio | Unit mute event type/registration and both native mute readers are absent. Preview currently documents the omitted entry stream as unknown and uses different frame state. Restore the actual event gate: consume pending mute notifications and return before consuming the ordinary op stream; do not silently discard unrelated pending op events or reset invented state. Full audio semantics beyond this narrow gate remain separately limited. |
| UI dispatcher / icon | State+payload+one `ty` byte does not preserve the original selected variant bytes. `show_decorator_icon` has a synthetic ID/raw-record implementation. Restore only proven schema/leaf behavior; no byte-offset access against the new host struct. |
| `DecoratorCollisionOutline` | The production module is entirely absent. Original476B display proves three states: point polyline, sphere radius, no display. Points are transformed in source order by the supplied actual affine basis, then DebugLine width2, ui_pass=true, dashed=argument. Sphere uses its actual radius and supplied translation; no-display emits nothing. Current `get_collision_outline` incorrectly returns Arc<DerivedDecoratorInfo> containing only a bbox. |
| Real outline producer | Indexed B `load_decorators@1407b66e0/17386B` calls actual library insert/insert_collision_and_interaction_as_same. Current `load_decorators.rs` only scans JSON into a raw MeshHost; it does not build the original collision/interaction/outline library and is uncalled by the actual App. Do not fabricate outline geometry from the bbox or claim an empty default library loaded. Its complete producer and actual Startup installation are a separate authorized batch. |
| Signifier/outline event | Canonical UiSignifierStream, closure factory, DebugLine and DebugSphere interfaces can be reused. `OutlineWallCmd` still has `_gap:u64`, so its true payload/consumer must be recovered before producing reason-specific wall feedback. Recovered `decorator_delete_scribble` uses a genuine factory/SignifierEntityBuilder path, not a raw empty record. |

Actual `run_game` calls only `decorator_plugin::build_startup_resources` and currently registers unleash in set25. The old partial generic installer is not called. No validator, backup/update/UI audio sequence or actual decorator mesh-library producer is registered merely by initializing startup resources. Missing callback source and incomplete current dependency algorithms must be tracked separately from this installation gap.

## Ownership and next bounded candidates

- Root/02: `blueprint.rs` TG-69cc2f973d7c14cb28 owns validate/feedback hunk. Root canonical `decorator.rs` TG-43142338addb95ca2d needs Locator ID; `apply_decorator_op.rs` TG-738de9fcef5acc1d6a needs narrow true place/change producer changes. Foreign Storage TG-889aa76ca030a2e3cd stays with root/06's cleanup/destructive contract.
- Central has claimed to `clutter-startup-20261020`: location TG-719dcf28a173a4b9b1; audio TG-c4a7246ce4d35f5279 and TG-b7e31a0de59cb27522; subtype TG-60901c6c8508b1af76; UI TG-2a3669083d5cb60c21 and TG-3ab1be37613b300ae5; collision-outline TG-da90ad5ff4af16fcb5; walls/mod TG-4509462cb49323f18d; actual loader TG-4b6d939d045d508224. Only their own isolated claim work may hold future candidates.
- Root-owned collision World TG-f7070bb3446b7a37eb is delegated to06 for the agreed Option/source API. This agent supplies evidence and consumes that contract, never edits the foreign World.
- OutlineWallCmd TG-d11abc3329e6f85879 remains root's `frameinputs-20261018` owner: request a schema/producer-consumer handoff, not foreign source edits. Current _gap event cannot be considered genuine feedback.

Stage the work: first full location result/schema and numerical algorithm; then native outline display/type and exact UI/mute leaves; then true library producers/installation; then the owner merges validate with its actual dependencies and original phase chain. Each batch may be reviewed separately. No partial batch is a completed validator or gameplay closure. Avoid a large speculative adapter trait, alternate World, manual scheduler, fake ready marker, or source-wide guessed registration.

## Focused verification plan (unexecuted)

Verify two IDs on one wall remain two locators; no-records still invokes correct metadata cleanup but skips validation; records with no surviving locators exhaust successfully; deleted affected objects skip; missing blueprint rejects. Exercise every original reason tag with its real typed payload and feedback, terrain/stair valid branch, wall/roof true geometry including boundary/NaN ordering, cache/owner failures, concrete collision versus disabled/unspecified, self-exclusion and same/different wall hits. Validate backup-created false versus true, state2 request plus exactly one mute event, generic icon selection including no-icon index5, and that mute consumers leave ordinary op cursors untouched. Check the actual App schedule with existing owners and phase ordering; do not inject fake loaded libraries or declare a restore request to be restored storage. CPU source/schema checks, actual rollback, actual feedback drawables, and world render/present remain separate acceptance levels.
