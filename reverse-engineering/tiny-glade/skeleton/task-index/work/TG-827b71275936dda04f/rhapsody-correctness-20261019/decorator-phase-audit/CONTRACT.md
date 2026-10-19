# Decorator phase and cleanup audit — B, design only

This audit supplies original registration, cleanup, and actual-caller facts. It changes no production source and runs no Cargo/GPU/native target. The previous reverse-blur manifests and integration status remain frozen.

## Identities and scope

Original B RNE SHA256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`, matching PDB SHA256 `2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911`, RSDS GUID `09F1A02A-49F2-F8C8-4C4C-44205044422E`, age1. The collector binds original bytes and PDB identity; the complete-function boundaries additionally come from selected B PDB procedure records. No shared database is opened or changed. Field names remain recovered semantic layout, not a request to reproduce hashbrown ABI.

Plugin identity is **DecoratorPlugin::build**, original `0x140342c10`, 17521 bytes. There is no distinct DecoratorVisualPlugin in these observations. Reused TG889 plugin/outer-chain packs are hashed in `external-evidence.json`. New complete bodies: storage cleanup `0x140a1e720/791`, destructive clear `0x141257900/690`, snapshot copy `0x140826d70/957`, validation `0x140827130/539`, plus five bounded typed nested registration functions. `native-request-correction.json` preserves the initial manually mistranscribed validation request `0x826f30`; that initial range is excluded from all validation-body conclusions. Corrected full body is in `native-corrected-pack`.

## Actual cleanup

The inline **Decorators::clear_modification_tracking**, original source lines100/101, occurs inside the complete Storage cleanup. It clears **modified_ids at bucket+0x60 and removed_ids at bucket+0**, preserving authority `decorators` at bucket+0x30. Direct instructions at `0x140a1e8a1..e8e2` clear the modified hashset; `e8e6..e912` and `e7a0..e7c2` clear the removed hashset. Neither range empties the middle authority map.

The complete Storage method:

```rust
assert!(!self.immutable);
for bucket in all_live_wall_buckets_and_optional_terrain_and_stair_buckets {
    bucket.modified_ids.clear();
    bucket.removed_ids.clear();
}
self.removed.clear(); // removed destination -> ID-set registry
```

The native entry asserts `Storage+0x1b0==0` at `0x140a1e730..e737`, with the panic cold block `ea1e..ea36`. Iteration visits per-wall entries of stride0x98, then optional buckets at Storage+0x90/+0x120. The tail `e917..ea08` drops each ID-set in Storage+0x30 registry and clears that registry. It preserves live bucket ownership, authority decorator entries and all DecoratorInfo values, d2s at Storage+0x60, and immutable itself. It writes no game timestamps. Hashset/table counts/capacity bookkeeping are dependency internals, implemented using the existing containers.

The Bevy resource **change tick** is separate from a game timestamp: validation `0x14082714f..715c` writes the original ResMut changed tick before calling cleanup, even when tracking is empty. Do not add an equality/no-change gate merely to avoid this assignment.

### Present concrete source defect

At the frozen source baseline `country-core/.../decorator_storage.rs:211`, the bucket helper clears modified_ids and **authority decorators**, leaving removed_ids. The Storage helper at725 calls this for live buckets, omits the immutable assertion and omits the removed destination registry. Consequently wiring this existing method as original tracking cleanup would delete authoritative decorators, leave stale removal notifications, and break the d2s relation. This is a direct mismatch with the complete original body, not an interface-name inference.

The misleading retained decompiler labels are not accepted as identity: its `141257900` output says tracking cleanup, but exact PDB/symbolizer identifies **Decorators::clear<DecoratorToStorageMap>**, original source150..157. That destructive method additionally performs removed-ID collection and d2s interaction before clearing authority map. It is a separate operation.

The current bucket helper also has real restore consumers (`backup.rs:159`, `restore_archived_decorators.rs:112`) that rely on its current authority-map clearing. A correction cannot silently change those callers to metadata cleanup. Their original call graph targets destructive `141257900`, with its own d2s/removal contract. Split and reconnect the two meanings under the relevant source owner; retain replacement semantics and audit original d2s state branches before claiming those restore paths recovered. This audit does not propose a two-line global replacement.

## Original phase edges

Zero-based indices below refer to the exact outer tuple19 from TG889. Native metadata and physical configuration moves are frozen in `phase-edges.json`, `copy-chain-members.json`, `nested-registration-members.json`, `full-copy-chain-callsite.json`, and `exact-native-snippets.txt`.

1. Outer member5: tuple(clear_blueprints, clear transient visuals). Member6: tuple15 generators. Members5/6 use tuple siblings internally; do not invent serial edges inside those tuples.
2. Member7: independent **instantiate_blueprint_colliders** `0x140827510` using mutable RaycastWorld/collider owner and immutable current/previous blueprints. Member8 flushes RaycastWorld.
3. Member9: **validate_blueprints** `0x140827130`. It **unconditionally invokes true tracking cleanup at its entry** `0x14082715c`, then validates deferred potentially surviving affected decorators and has conditional backup/UI/audio/event effects. The exact typed run_unsafe `0x1403d3820` and without-deferred `0x14045c8c0` identify this member. An earlier provisional label “backup preview” for member9 is corrected.
4. Members10..13 are intermediate archive/cull/restore/edit systems retained in the outer chain. They cannot be collapsed away: later mutation can create notifications for the next generation pass.
5. Member14 is a pair of ScheduleConfigs. First branch is the18-member stairs chain. Second branch is **chain_ignore_deferred(fullvisual, middle visual group, copy_previous)**, constructed by `0x1403d6ce0/1898`. These two member14 branches are siblings; no order between them is asserted.
6. The fullvisual chain first member is **instantiate_blueprints** with original14 parameters, `0x140827910`; second member is tuple(dormer-roof ScheduleConfig, chimney bricks, door/stairs bricks, cloth upload); last is **copy_blueprints_to_prev** `0x140826d70`. No implicit command flush is inserted by this nested chain. Native constructor vtable `0x1429d2ce8` has run_unsafe slot+0x50=`0x1403d43f0`, whose original callsite `0x1403d4c21` targets fullvisual. Copy vtable `0x1429d2b78` slot+0x50=`0x1403d33e0`, whose callsite `0x1403d34df` targets copy. Selected function metadata and bounded original vtable bytes are retained.
7. Member15 unleashes deferred decorator operation events, member16 flushes RaycastWorld, members17/18 update lantern/stairs visualization.

Physical registration proof: Plugin `0x140344fae` calls `3d6ce0` with the prebuilt middle group. Its result at frame+0x320 is moved to frame+0x160 (`345015..5070`). The first member14 config is at frame+0xf0, second at+0x160; the plugin copies the pair (0xe0 bytes) to outer input+0x150 at `345188..519c`. Outer `0d52f0` consumes that pair at input+0x150 (`0d53f7..540b`); its typed member14 is exactly tuple2 ScheduleConfigs. Plugin `3451af` chains the outer19, `3451b4` supplies set25, `3451e2` applies in_set, and `345253` adds that exact configuration to **Update**.

Therefore cleanup is **before fullvisual and copy**, within validation after the first generation/collider stage. Do not move it to the end of frame. Fullvisual itself calls the collider helper again (independently frozen by02); the original stage7 wrapper and stage14 fullvisual are distinct intended moments separated by validation/editing. Do not register an adjacent extra collider wrapper merely because fullvisual contains the helper.

### Content consumption and snapshot timing

Dirty modified/removed sets and removed-destination registry are inputs to blueprint pruning and generation before validation clears them. Collider-only and fullvisual use current/previous blueprint maps, not these authority dirty sets; the fullvisual14 signature has no DecoratorStorage parameter. Fullvisual consumes its blueprint identity diff and also produces per-frame holes/window/cloth outputs. The existing same-Arc shortcut only skips entity difference work, not those outputs (02 complete CFG scope). Thus neither `Changed<Blueprints>` nor an operation-event run-if can be invented to skip fullvisual unchanged frames.

Copy writes the previous owner after fullvisual and middle visual consumers in that nested chain, and before outer unleash. The original copy marks PrevBlueprints changed (`826fc1..fc8`), calls clone_from on the current map (`826fca..fd6`) and copies the trailing map state (`826fdb..fdf`). Do not snapshot early: equal Arc identity is used by the next diff. Do not deep clone each blueprint merely to manufacture changes. This is a blueprint snapshot; it is not an authoritative DecoratorStorage copy and does not clear tracking.

### Triggers and World ownership

These selected nodes are direct typed function items in the original configuration. No on_event/Changed/run_if wrapper is present on the original selected collider-only, validation, fullvisual, or copy node, or added to the completed outer chain in the inspected plugin callsite. OnWallDeleted is an ordinary fullvisual EventReader input; it does not mean “run only when a wall was deleted.” Registration is in Update/set25. A distinct global schedule gate is outside this frozen proof; no universal whole-game frequency claim is made here.

Every Res/ResMut/EventReader/Writer is resolved in the same actual Update App/SubApp World. Use canonical existing resource types and locked Bevy APIs/derive. Do not create adapter copies of Storage/Blueprints/PrevBlueprints/events to satisfy parameters. Preserve outer normal chain versus inner chain_ignore_deferred and their command application boundaries.

## Current real caller and narrow integration obligations

At the frozen production baseline, `run_game.rs:366` calls **build_startup_resources**, which initializes canonical owners and UI resources. It does not call the old `decorator_plugin::build` helper. The actual set25 registration at805..811 currently mounts only unleash. Current `blueprint.rs` exposes clear/copy/collider model functions but has no validate_blueprints system. The old `add_decorator_systems` shim contains its own partial guessed order and is not evidence for reconnecting this complete native sequence.

Clear_blueprints has another concrete permission mismatch: exact original typed member5 is ResMut<Storage>,ResMut<Blueprints>. Original `82633e..6345` updates Storage's Bevy tick, then `826351` calls Storage::iter_mut `a1e630`, which enforces !immutable. Current wrapper only has Res<Storage>. This must be reviewed with the blueprint owner rather than ignored as a read-only algorithm.

The narrow design for subsequent owner work is:

- Correct actual tracking cleanup (both bucket sets, whole removed registry, immutable gate) and preserve unrelated authority/d2s/info. Keep destructive clear separate and repair its existing restore callers under their own proven contract; no silent method-behavior swap.
- Recover validation's actual remaining business before registering member9. A cleanup-only substitute cannot claim that member recovered.
- Connect a proven set25 chain through the actual runtime installer, keeping the real canonical owners and the original selected nested order. Do not activate the old approximate registration shim wholesale.
- Preserve the two collision phases at their original positions if the chain is mounted; do not add a third incidental invocation. Preserve fullvisual per-frame output and copy publication after actual consumers.

No source candidate is authorized or supplied by this audit. Root must coordinate legal source owners for any implementation.02 owns blueprint/fullvisual;03 owns WindowAutoClutterCandidates;05 owns the two differential trackers. Tracker logs are separate from Storage tracking; clearing them is not a replacement for this cleanup.

## Focused acceptance suggestions (not executed)

Use the actual App/World owner, not synthetic counters as recovery acceptance. At minimum verify: nonempty live wall/terrain/stair authority survives metadata cleanup; both ID sets and removed-destination registry clear; d2s/info values persist; immutable rejection occurs before cleanup; empty tracking still respects original Bevy tick semantics. Then exercise blueprint generation and collision at member7, validation mutation/cleanup, fullvisual at member14 with real unchanged-Arc per-frame outputs, and snapshot copy only after consumers. Include removal and a later restored/edit notification across two frames so stale removals or early copy cannot mask the diff. Restore tests must exclude old authority keys absent in the selected backup and preserve original d2s/removal behavior. Same canonical owner must be reused across all systems; a second World must remain isolated.

Compilation, source review and these suggested checks remain distinct from original native/gameplay/GPU acceptance. No result here claims full set25 or decorator main-chain recovery.
